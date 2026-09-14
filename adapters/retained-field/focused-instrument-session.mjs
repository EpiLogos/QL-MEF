/**
 * K9 browser-side source for one already-open `ql-focused-host` owner.
 *
 * This object is deliberately source-shaped so O:I can register it directly as
 * a FocusedInstrumentSource, but it lives beside the accepted K8 retained-field
 * adapters because it owns no React/desktop state. One host sequence serialises
 * Bimba/focus/clock commands and native field advances. K8's existing
 * RetainedFieldBinding + NativeAudioBinding receive the same acknowledged field
 * block; O:I supplies only a lease to its already-created Global Expression
 * Stage. No executable discovery, second native owner, renderer, clock, graph or
 * audio synthesis is introduced here.
 */
import { RetainedFieldBinding } from './retained-field.mjs';
import { NativeAudioBinding } from './native-audio.mjs';

const HOST_REQUEST = 'ql.focused-host-request/v1';
const HOST_RECEIPT = 'ql.focused-host-receipt/v1';
const SNAPSHOT_CONTRACT = 'ql.focused-instrument/v1';
const BIMBA_CONTRACT = 'ql.focused-host-bimba-navigation/v1';
const U64 = (1n << 64n) - 1n;
const need = (ok, message) => { if (!ok) throw new Error(message); };
const clone = value => structuredClone(value);
const reference = (value, label) => {
  need(typeof value === 'string' && value.trim().length > 0 && value.length <= 4096 &&
    !/[\u0000-\u001f\u007f]/u.test(value), `invalid ${label}`);
  return value;
};
const cursor = value => {
  need(typeof value === 'string' && /^(0|[1-9][0-9]{0,19})$/.test(value), 'invalid focused-host cursor');
  const result = BigInt(value); need(result <= U64, 'focused-host cursor overflow'); return result;
};
function snapshot(value) {
  need(value?.schema === SNAPSHOT_CONTRACT && value.event && value.live_cursor && value.presented_cursor,
    'focused host did not return a focused-instrument snapshot');
  reference(value.event.event_ref, 'focused event ref');
  reference(value.event.subject_ref, 'focused subject ref');
  need(Number.isSafeInteger(value.event.profile_generation) && value.event.profile_generation >= 0,
    'invalid focused profile generation');
  cursor(value.live_cursor.field_generation); cursor(value.live_cursor.samples_elapsed);
  return value;
}
function navigation(value) {
  need(value?.contract === BIMBA_CONTRACT && Array.isArray(value.items) && typeof value.source_revision === 'string',
    'focused host did not return rooted Bimba navigation');
  return value;
}
function standing(status) {
  if (status === 'ok') return 'applied';
  if (status === 'refused') return 'refused';
  return 'unknown';
}
function mapped(command) {
  switch (command?.kind) {
    case 'set-focus': return { operation: 'set-focus', focus: command.focus };
    case 'select-bimba': return { operation: 'select-bimba', selection_ref: command.selection?.selection_ref };
    case 'clear-selection': return { operation: 'clear-selection' };
    case 'set-tracking': return { operation: 'set-tracking', tracking: command.tracking };
    case 'freeze': return { operation: 'freeze' };
    case 'resume-live': return { operation: 'resume-live' };
    case 'assemble-clock': return { operation: 'assemble-clock' };
    case 'explode-clock': return { operation: 'explode-clock', pair: command.pair ?? null };
    case 'set-clock-axis': return { operation: 'set-clock-axis', axis: command.axis, phase: clone(command.phase) };
    case 'advance': return { operation: 'advance', frames: command.frames, muted: command.muted };
    default: throw new Error(`unsupported focused instrument command: ${String(command?.kind)}`);
  }
}
function fieldOperation(command) {
  return command.operation === 'advance' || command.operation === 'set-clock-axis';
}

export class FocusedInstrumentSession {
  #transport; #receipt; #snapshot; #bimba; #sequence; #tail = Promise.resolve();
  #listeners = new Set(); #notifyTimer = null; #disposed = false; #uncertain = false;
  #context; #audioOwner = {}; #audio = null; #field = null; #lease = null;
  #checkpoint = null; #stopRecovery = null; #presentQueue = []; #presentTimer = null;
  #running = false; #runTimer = null; #resumeAfterRecovery = false; #recovering = false;
  #correspondence; #timeout; #block; #period; #lookahead; #maxBlocks; #audioOptions;

  constructor({ ref, title = 'Epi / Nara', transport, initialReceipt, audioContext = null,
    correspondence, accompanying, timeoutMs = 5000, blockFrames = 512,
    periodMs = 8, lookaheadSeconds = 0.1, maxPresentationBlocks = 16,
    audio = {}, autostart = false } = {}) {
    this.ref = reference(ref, 'focused source ref');
    this.title = reference(title, 'focused source title');
    need(transport && typeof transport.request === 'function', 'authorised focused-host transport required');
    need(initialReceipt?.schema === HOST_RECEIPT && initialReceipt.status === 'ready' &&
      initialReceipt.available === true && typeof initialReceipt.instance_ref === 'string',
      'focused-host ready receipt required');
    this.#receipt = clone(initialReceipt);
    this.#snapshot = clone(snapshot(initialReceipt.snapshot));
    this.#bimba = clone(navigation(initialReceipt.bimba));
    this.#sequence = cursor(initialReceipt.last_request_id);
    need(Number.isInteger(timeoutMs) && timeoutMs >= 100 && timeoutMs <= 120000, 'invalid focused-host timeout');
    need(Number.isInteger(blockFrames) && blockFrames >= 128 && blockFrames <= 8192, 'invalid focused block size');
    need(Number.isInteger(periodMs) && periodMs >= 4 && periodMs <= 100, 'invalid focused driver period');
    need(Number.isFinite(lookaheadSeconds) && lookaheadSeconds >= 0.01 && lookaheadSeconds <= 0.5,
      'invalid focused lookahead');
    need(Number.isInteger(maxPresentationBlocks) && maxPresentationBlocks >= 1 && maxPresentationBlocks <= 128,
      'invalid focused presentation queue ceiling');
    need(correspondence && typeof correspondence === 'object' && Array.isArray(correspondence.slotsA) &&
      Array.isArray(correspondence.slotsB), 'source-qualified retained sample correspondence required');
    reference(correspondence.sourceRef, 'sample correspondence source');
    reference(correspondence.revision, 'sample correspondence revision');
    this.#transport = transport; this.#context = audioContext; this.#correspondence = clone(correspondence);
    this.#timeout = timeoutMs; this.#block = blockFrames; this.#period = periodMs;
    this.#lookahead = lookaheadSeconds; this.#maxBlocks = maxPresentationBlocks; this.#audioOptions = clone(audio);
    this.autostart = autostart === true;
    if (accompanying !== undefined) {
      need(accompanying && typeof accompanying === 'object', 'invalid Epii AgentSession reference');
      this.accompanying = Object.freeze({ ref: reference(accompanying.ref, 'AgentSession ref'),
        project: reference(accompanying.project, 'AgentSession project'), space: reference(accompanying.space, 'AgentSession space') });
    }
  }

  get reading() {
    return Object.freeze({ schema: 'ql.focused-browser-session/v1', ref: this.ref,
      available: !this.#disposed && !this.#uncertain && this.#snapshot.available === true,
      running: this.#running, recovering: this.#recovering, attached: !!this.#field,
      host_sequence: this.#sequence.toString(), native_cursor: clone(this.#snapshot.live_cursor),
      presented_blocks: this.#presentQueue.length,
      correspondence: { sourceRef: this.#correspondence.sourceRef, revision: this.#correspondence.revision },
      audio: this.#audio?.lastReceipt ?? null, retained: this.#lease?.inspect?.() ?? null });
  }

  async read() { this.#alive(); return clone(this.#snapshot); }
  async readBimba() { this.#alive(); return clone(this.#bimba); }
  subscribe(listener) { this.#alive(); need(typeof listener === 'function', 'focused source listener required'); this.#listeners.add(listener); return () => this.#listeners.delete(listener); }

  async command(command) {
    this.#alive();
    const host = mapped(command);
    if (command.kind === 'select-bimba') {
      const known = this.#bimba.items.find(item => item.selection?.selection_ref === command.selection?.selection_ref);
      need(known, 'Bimba selection was not disclosed by this focused owner');
    }
    return this.#serial(async () => {
      const reply = await this.#exchange(host);
      if (reply.status === 'ok' && fieldOperation(host) && reply.owner_receipt && this.#field) {
        await this.#admitField(reply.owner_receipt);
      }
      return { standing: standing(reply.status), operation: command.kind,
        error: reply.error ?? null, snapshot: clone(this.#snapshot), owner_receipt: clone(reply.owner_receipt ?? null) };
    });
  }

  /** Attach the accepted K8 receivers to O:I's already-created expression stage.
   * The first operation is an actual zero-frame native acknowledgement on the
   * same focused-host sequence, not a cached frame or a second worker. */
  async attachExpression(lease) {
    this.#alive(); need(!this.#field && !this.#lease, 'focused expression already attached');
    need(this.#context && typeof this.#context.createBufferSource === 'function', 'audio context required to attach focused expression');
    need(lease && typeof lease.retainedTargetPort === 'function' && typeof lease.checkpointRetainedField === 'function' &&
      typeof lease.restoreRetainedField === 'function' && typeof lease.onRecoveryRequired === 'function',
      'O:I retained expression lease required');
    const reply = await this.#serial(() => this.#exchange({ operation: 'advance', frames: 0, muted: true }));
    need(reply.status === 'ok' && reply.owner_receipt, 'focused owner refused initial retained-field read');
    const frame = reply.owner_receipt;
    need(Array.isArray(frame.audio) && frame.audio.length === 0, 'focused attachment requires zero-PCM native read');
    const port = lease.retainedTargetPort();
    const size = port.texWidth * port.texHeight;
    need(this.#correspondence.slotsA.length === size && this.#correspondence.slotsB.length === size,
      'sample correspondence does not cover this O:I retained texture');
    const field = new RetainedFieldBinding(port, { initialFrame: frame, targetA: port.targetA, targetB: port.targetB,
      slotsA: this.#correspondence.slotsA, slotsB: this.#correspondence.slotsB });
    let audio;
    try {
      audio = new NativeAudioBinding(this.#context, { owner: this.#audioOwner, initialFrame: frame, ...this.#audioOptions });
      this.#lease = lease; this.#field = field; this.#audio = audio;
      this.#checkpoint = lease.checkpointRetainedField(field);
      this.#stopRecovery = lease.onRecoveryRequired(phase => { void this.#recoveryPhase(phase); });
      if (this.autostart) this.start();
      this.#notify();
    } catch (error) {
      audio?.dispose(); field.dispose(); this.#lease = null; this.#field = null; this.#audio = null; throw error;
    }
    return () => this.detachExpression();
  }

  detachExpression() {
    this.stop(); this.#clearPresentation();
    this.#stopRecovery?.(); this.#stopRecovery = null;
    this.#audio?.dispose(); this.#audio = null;
    this.#field?.dispose(); this.#field = null;
    this.#checkpoint = null; this.#lease = null; this.#recovering = false;
    this.#notify();
  }

  start(periodMs = this.#period) {
    this.#alive(); need(this.#field && this.#audio && this.#lease, 'attach the focused expression before starting');
    need(Number.isInteger(periodMs) && periodMs >= 4 && periodMs <= 100, 'invalid focused driver period');
    this.#period = periodMs; if (this.#running) return;
    this.#running = true; this.#scheduleRun(0); this.#notify();
  }
  stop() { this.#running = false; if (this.#runTimer !== null) clearTimeout(this.#runTimer); this.#runTimer = null; }

  async #pump() {
    this.#presentDue();
    if (!this.#running || this.#recovering || this.#uncertain || !this.#audio || !this.#field || !this.#lease) return;
    if (this.#context.state !== 'running') return;
    const capacity = this.#audio.capacity;
    if (capacity.held || capacity.closed || capacity.frames < this.#block || capacity.blocks < 1 || this.#presentQueue.length >= this.#maxBlocks) return;
    const last = this.#audio.lastReceipt;
    if (last && last.target_context_seconds - this.#context.currentTime > this.#lookahead) return;
    const reply = await this.#serial(() => this.#exchange({ operation: 'advance', frames: this.#block, muted: false }));
    if (reply.status === 'refused') { this.#hold(`native-advance-refused: ${reply.error ?? 'refused'}`); return; }
    if (reply.status !== 'ok' || !reply.owner_receipt) return;
    await this.#admitField(reply.owner_receipt);
  }

  async #admitField(frame) {
    need(this.#field && this.#audio, 'focused presentation receiver unavailable');
    try {
      this.#field.validate(frame); this.#audio.validate(frame);
      const audioReceipt = this.#audio.apply(frame);
      const targetTime = audioReceipt.interval?.target_context_seconds ?? audioReceipt.target_context_seconds;
      need(Number.isFinite(targetTime), 'native field has no attributable device target time');
      need(this.#presentQueue.length < this.#maxBlocks, 'focused presentation queue full');
      this.#presentQueue.push({ frame: clone(frame), targetTime });
      this.#presentQueue.sort((a, b) => a.targetTime - b.targetTime);
      this.#armPresentation();
    } catch (error) {
      this.#uncertain = true; this.#hold(`presentation-admission-unknown: ${String(error)}`); this.#transport.close?.(); throw error;
    }
  }

  #presentDue() {
    if (!this.#field || !this.#lease || !this.#audio || this.#recovering) return;
    let selected = null;
    while (this.#presentQueue.length && this.#presentQueue[0].targetTime <= this.#context.currentTime) selected = this.#presentQueue.shift();
    if (!selected) { this.#armPresentation(); return; }
    try {
      this.#field.apply(selected.frame);
      this.#checkpoint = this.#lease.checkpointRetainedField(this.#field);
      this.#notify();
    } catch (error) {
      this.#uncertain = true; this.#hold(`retained-field-application-unknown: ${String(error)}`); this.#transport.close?.();
    }
    this.#armPresentation();
  }

  #armPresentation() {
    if (this.#presentTimer !== null) clearTimeout(this.#presentTimer);
    this.#presentTimer = null;
    if (!this.#presentQueue.length || this.#recovering || !this.#audio) return;
    const delay = Math.max(0, (this.#presentQueue[0].targetTime - this.#context.currentTime) * 1000);
    this.#presentTimer = setTimeout(() => { this.#presentTimer = null; this.#presentDue(); }, Math.min(1000, Math.ceil(delay)));
  }

  #hold(reason) {
    this.stop(); this.#clearPresentation();
    try { if (this.#audio && !this.#audio.capacity.closed) this.#audio.hold(reason); } catch { /* already held/disposed */ }
    try { this.#lease?.pause(true); } catch { /* stage may already be lost */ }
    this.#notify();
  }

  async #recoveryPhase(phase) {
    if (this.#disposed || this.#uncertain || !this.#field || !this.#audio || !this.#lease) return;
    if (phase === 'lost') {
      this.#resumeAfterRecovery = this.#running;
      this.#recovering = true;
      this.#hold('oi-expression-context-lost');
      this.#recovering = true;
      return;
    }
    if (phase !== 'restored' || !this.#recovering) return;
    const shouldResume = this.#resumeAfterRecovery;
    try {
      await this.#serial(async () => {
        need(this.#checkpoint, 'no acknowledged retained GPU checkpoint is available');
        this.#lease.restoreRetainedField(this.#field, this.#checkpoint);
        const reply = await this.#exchange({ operation: 'advance', frames: 0, muted: true });
        need(reply.status === 'ok' && reply.owner_receipt && Array.isArray(reply.owner_receipt.audio) && reply.owner_receipt.audio.length === 0,
          'focused re-entry requires an explicit zero-frame owner acknowledgement');
        this.#audio.rebase(reply.owner_receipt, 'oi-expression-context-restored');
        this.#field.apply(reply.owner_receipt);
        this.#checkpoint = this.#lease.checkpointRetainedField(this.#field);
        this.#recovering = false; this.#resumeAfterRecovery = false;
        this.#lease.resume();
      });
      if (shouldResume) this.start();
      this.#notify();
    } catch (error) {
      this.#uncertain = true; this.#recovering = false; this.#resumeAfterRecovery = false;
      this.#hold(`focused-reentry-unknown: ${String(error)}`); this.#transport.close?.();
    }
  }

  #scheduleRun(delay) {
    if (!this.#running || this.#disposed) return;
    if (this.#runTimer !== null) clearTimeout(this.#runTimer);
    this.#runTimer = setTimeout(async () => {
      this.#runTimer = null;
      try { await this.#pump(); }
      catch (error) { if (!this.#uncertain) this.#hold(`focused-driver-held: ${String(error)}`); }
      this.#scheduleRun(this.#period);
    }, delay);
  }

  async #exchange(command) {
    this.#alive(); need(!this.#uncertain, 'focused owner standing unknown; no automatic retry');
    const expected = this.#snapshot.live_cursor;
    const next = this.#sequence + 1n; need(next <= U64, 'focused-host request sequence exhausted');
    const request = { schema: HOST_REQUEST, instance_ref: this.#receipt.instance_ref,
      event_ref: this.#snapshot.event.event_ref, subject_ref: this.#snapshot.event.subject_ref,
      profile_generation: this.#snapshot.event.profile_generation, request_id: next.toString(),
      expected_generation: expected.field_generation, expected_samples_elapsed: expected.samples_elapsed,
      command: clone(command) };
    let timer;
    try {
      const reply = await Promise.race([
        Promise.resolve().then(() => this.#transport.request(request)),
        new Promise((_, reject) => { timer = setTimeout(() => reject(new Error('focused-host acknowledgement timed out')), this.#timeout); })
      ]);
      need(reply?.schema === HOST_RECEIPT && reply.instance_ref === request.instance_ref &&
        reply.request_id === request.request_id && reply.last_request_id === request.request_id,
        'foreign or reordered focused-host acknowledgement');
      need(['ok', 'refused', 'unavailable'].includes(reply.status), 'unknown focused-host standing');
      const nextSnapshot = snapshot(reply.snapshot);
      need(nextSnapshot.event.event_ref === request.event_ref && nextSnapshot.event.subject_ref === request.subject_ref &&
        nextSnapshot.event.profile_generation === request.profile_generation,
        'focused-host acknowledgement changed event/subject/world identity');
      this.#sequence = next; this.#receipt = clone(reply); this.#snapshot = clone(nextSnapshot);
      if (reply.bimba) this.#bimba = clone(navigation(reply.bimba));
      else this.#bimba = { ...this.#bimba, selected_ref: this.#snapshot.selection?.selection_ref ?? null };
      if (reply.status === 'unavailable' || reply.available !== true) {
        this.#uncertain = true; this.#hold(reply.error ?? 'focused native owner unavailable'); this.#transport.close?.();
      }
      this.#notify();
      return reply;
    } catch (error) {
      this.#uncertain = true; this.#hold(`focused-host-transport-unknown: ${String(error)}`); this.#transport.close?.(); throw error;
    } finally { clearTimeout(timer); }
  }

  #serial(work) {
    const result = this.#tail.then(work, work);
    this.#tail = result.catch(() => {});
    return result;
  }
  #clearPresentation() {
    if (this.#presentTimer !== null) clearTimeout(this.#presentTimer);
    this.#presentTimer = null; this.#presentQueue = [];
  }
  #notify() {
    if (this.#notifyTimer !== null || this.#disposed) return;
    this.#notifyTimer = setTimeout(() => { this.#notifyTimer = null; for (const listener of this.#listeners) listener(); }, 40);
  }
  #alive() { need(!this.#disposed, 'focused instrument session disposed'); }

  dispose() {
    if (this.#disposed) return;
    this.stop(); this.detachExpression(); this.#disposed = true;
    if (this.#notifyTimer !== null) clearTimeout(this.#notifyTimer); this.#notifyTimer = null;
    this.#listeners.clear(); this.#transport.close?.();
  }
}
