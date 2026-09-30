#!/bin/bash
# Independent Omarchy replay of the O:I K² browser trace (O:I #335 / QL-MEF #135).
set -u
export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$HOME/.local/share/mise/shims:$PATH" CARGO_TARGET_DIR=/mnt/hdd/vm/cargo-target
QW=~/Central/worktrees/ql-m123-replay; OW=~/Central/worktrees/oi-m123-replay
cd ~/Central/Work/Quaternal-Logic && git fetch -q origin feat/m123-engine-binding && cd "$QW" && git checkout -q --detach FETCH_HEAD 2>/dev/null || git -C "$QW" checkout -q --detach origin/feat/m123-engine-binding
cd ~/Central/Work/O-I && git fetch -q origin feat/m123-expression-instrument
[ -d "$OW" ] || git worktree add --detach "$OW" FETCH_HEAD
cd "$OW" && git checkout -q --detach FETCH_HEAD
echo "QL $(git -C $QW rev-parse --short HEAD)  O:I $(git rev-parse --short HEAD)  $(uname -sm)"
step() { echo "=== $1"; shift; "$@" 2>&1 | tail -${TAILN:-5}; echo "rc=${PIPESTATUS[0]}"; }
step "QL owner (worker, host, ql)" sh -c "make -C $QW/cpp test >/dev/null && cargo build -q --locked --release --manifest-path $QW/Cargo.toml -p ql-mef --bin ql-field-host -p ql-cli --bin ql && echo built"
step "cradle deps" npm ci --prefix desktop/cradle --no-audit --no-fund
step "expressions app build" sh -c "cd desktop/cradle && npm run build:expressions"
step "walk-bridge" cargo build -q --release --manifest-path desktop/cradle/kernel/Cargo.toml --bin walk-bridge
step "playwright chromium" sh -c "cd desktop/cradle && npx playwright install chromium"
export NATIVE_EXPRESSION_BRIDGE=$CARGO_TARGET_DIR/release/walk-bridge OI_QL_BIN=$CARGO_TARGET_DIR/release/ql OI_QL_FIELD_HOST_BIN=$CARGO_TARGET_DIR/release/ql-field-host OI_QL_FIELD_WORKER_BIN=$QW/cpp/build/ql-field-worker
cd desktop/cradle
NATIVE_EXPRESSION_OUT=/tmp/oi-k2-replay node tests/native-expression-k2-browser.mjs > /tmp/oi-k2-replay.log 2>&1; echo "=== K² trace rc=$?"
K2_DISCONNECT_TARGETS=1 NATIVE_EXPRESSION_OUT=/tmp/oi-k2-replay-disc node tests/native-expression-k2-browser.mjs > /tmp/oi-k2-replay-disc.log 2>&1; echo "=== disconnected rc=$? (must be non-zero)"
python3 - <<'PY'
import json
for p in ('/tmp/oi-k2-replay/k2-trace.json','/tmp/oi-k2-replay-disc/k2-trace.json'):
    try: d=json.load(open(p))
    except Exception as e: print(p,'unreadable',e); continue
    g=d.get('gpu',{}); a=d.get('audio',{}); c=d.get('cadence',{})
    print(p,'pass',d.get('pass'),'renderer',d.get('webgl',{}).get('renderer'),'gpu mean',g.get('effect_mean_abs_difference'),'pixels',d.get('pixels',{}).get('effect_differing_channels'),'hz',a.get('varied_measured_hz'))
    for k,v in c.items(): print('  ',k,{x:v.get(x) for x in ('applied','achieved_ticks_per_second','audio_device_epoch_before','audio_device_epoch','device_epoch_before','device_epoch_after')})
PY
echo DONE
