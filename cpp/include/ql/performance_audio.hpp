#ifndef QL_PERFORMANCE_AUDIO_HPP
#define QL_PERFORMANCE_AUDIO_HPP
// Native M1 excitation, consuming prepared M2 musical targets. Preparation and
// source joining run on the control owner. This callback contains no theory,
// graph, JSON, device configuration, allocation, locks or UI-owned clock.
#include <array>
#include <atomic>
#include <algorithm>
#include <cmath>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <limits>
#include <memory>
#include <stdexcept>
#include <type_traits>
#include <ql/m_tree_live.h>

namespace ql::performance {
inline constexpr const char* contract = "ql.performance-audio/v1";
inline constexpr std::size_t max_frames=512, max_voices=24, max_touches=96,
    max_tails=16, queue_capacity=256, capture_capacity=16;
inline constexpr double tau=6.283185307179586476925286766559;
using Ref=std::array<char,256>;
inline Ref reference(const char* value) {
    if (!value || !*value || std::strlen(value)>=256) throw std::invalid_argument("bounded performance reference required");
    Ref out{}; std::memcpy(out.data(),value,std::strlen(value)); return out;
}
inline bool valid_ref(const Ref& ref) noexcept {
    return ref[0] && std::find(ref.begin(),ref.end(),'\0')!=ref.end();
}
struct Identity {
    Ref instance{}, event{}, subject{};
    std::uint64_t m1_revision=0, m2_generation=0;
    bool operator==(const Identity& other) const noexcept {
        return instance==other.instance && event==other.event && subject==other.subject && m1_revision==other.m1_revision && m2_generation==other.m2_generation;
    }
};
struct NodalBoundary { std::uint8_t position=0, face=0, m=0, n=0; };
// Native M1/K and actual M2/B producer outputs, copied once after their complete
// joined validation. The receipt names coherence; it never implies permission
// or live graph authentication. Source face is independent of temporal phase.
struct Determination {
    Identity identity{};
    Ref m1_coordinate{}, m2_writer{}, registry_revision{}, source_revision{},
        relation_plan_ref{}, tuning_ref{}, native_receipt_ref{}, body_preparation_ref{}, body_state_ref{};
    std::uint8_t m1_face=0, m2_face=1, tick12=0, basis=0, lens12=0, context_frame=1;
    std::uint16_t degree720=0;
    std::array<double,8> audio_octet_hz{};
    std::array<NodalBoundary,4> nodal_quartet{};
    std::uint64_t body_revision=0;
    bool tuning_available=false;
};
struct NoteTarget {
    Identity identity{};
    Ref source_coordinate{}, tuning_ref{};
    std::uint64_t member=0, touch=0, ratio_numerator=0, ratio_denominator=0;
    std::uint8_t key=0, position=0, coordinate_face=0, source_face=0, pitch_class=0;
    std::int8_t register_octave=0;
    double fundamental_hz=0, hertz=0, phase_sin=0, phase_cos=1;
    bool exact_ratio=false;
};
// A scalar Newton force along P's prepared node/axis projection. P is the sole
// physical numerical owner and exposes pickup/displacement from its same q/v.
// All function pointers must refer to that resident owner for the full lifetime.
struct PhysicalPort {
    void* owner=nullptr;
    bool (*advance)(void*,const double*,float*,std::size_t,std::uint64_t,std::uint64_t) noexcept=nullptr;
    std::uint64_t (*revision)(const void*) noexcept=nullptr;
    std::uint64_t (*cursor)(const void*) noexcept=nullptr;
    double max_force_newtons=0;
    unsigned sample_rate=0;
    Ref event{}, subject{}, preparation{}, state{};
    // Acquired/released on the control thread only. Device output requires
    // resident ownership so even a refused OS stop cannot free callback data.
    std::shared_ptr<void> custody{};
};
enum class Kind : std::uint8_t { NoteOn, NoteOff, Sustain, Expression, Panic, Parameter, Determination };
enum class Parameter : std::uint8_t { ForceNewtons, AttackSeconds, ReleaseSeconds, CutoffHertz, MasterLinear, BodyLinear, MonitorLinear };
struct Operation {
    Kind kind=Kind::Panic;
    Identity identity{};
    std::uint64_t sequence=0, sample=0, touch=0;
    NoteTarget note{};
    // NoteOn velocity [0,1]; expression pressure [0,1] and prepared pitch Hz;
    // sustain 0/1; parameter in the units stated by its enum. No raw MIDI pitch.
    double value=0, pitch_hz=0;
    Parameter parameter=Parameter::MasterLinear;
    Determination determination{};
    bool late_admitted=false;
};
struct ReleaseOperation {
    Kind kind=Kind::NoteOff;
    Identity identity{};
    std::uint64_t sequence=0, sample=0, touch=0;
    bool late_admitted=false;
};
enum class Result : std::uint8_t { Accepted, Invalid, Stale, Late, Order, Overflow, Unavailable, Exhausted };
struct Parameters {
    double force_newtons=0.01, attack_seconds=0.003, release_seconds=0.05,
        cutoff_hertz=18000, master_linear=0.25, body_linear=1, monitor_linear=0;
};
struct VoiceReadback {
    std::uint64_t member=0, m1_revision=0, m2_generation=0;
    double effective_hertz=0, target_hertz=0, envelope=0;
    std::uint32_t held_touches=0;
    bool releasing=false;
};
struct Readback {
    Identity identity{};
    std::uint64_t samples_elapsed=0, body_revision=0, last_sequence=0, refused=0,
        late=0, overflows=0, stolen=0, dropped_readbacks=0, dropped_captures=0,
        clipping_samples=0, force_limited_samples=0;
    std::uint32_t active_voices=0, active_touches=0;
    double peak=0, rms=0;
    Parameters source{}, effective{};
    std::array<VoiceReadback,max_voices> voices{};
    bool sustain=false, available=false;
};
struct Capture {
    Identity identity{}, end_identity{};
    std::uint64_t start_sample=0, body_revision=0;
    std::uint32_t frames=0;
    std::array<double,max_frames> force_newtons{};
    std::array<float,max_frames> pickup_linear{}, output_linear{};
};
template<class T,std::size_t N> class Spsc {
    static_assert(N>1 && std::is_trivially_copyable_v<T>);
    std::array<T,N> storage_{};
    alignas(64) std::atomic<std::uint64_t> write_{0};
    alignas(64) std::atomic<std::uint64_t> read_{0};
public:
    bool push(const T& item) noexcept {
        const auto w=write_.load(std::memory_order_relaxed), r=read_.load(std::memory_order_acquire);
        if (w-r>=N) return false;
        storage_[w%N]=item; write_.store(w+1,std::memory_order_release); return true;
    }
    const T* peek() const noexcept {
        const auto r=read_.load(std::memory_order_relaxed);
        if (r==write_.load(std::memory_order_acquire)) return nullptr;
        return &storage_[r%N];
    }
    void pop() noexcept { read_.fetch_add(1,std::memory_order_release); }
    bool take(T& out) noexcept { const auto* p=peek(); if (!p) return false; out=*p; pop(); return true; }
};
static_assert(std::atomic<std::uint64_t>::is_always_lock_free && std::atomic<bool>::is_always_lock_free,
    "performance callback requires native lock-free atomics");

// One producer (the existing serial control owner), one callback consumer.
// Source decisions can be replaced at a precise sample using a prepared event;
// a whole determination cannot change physical preparation implicitly.
class Engine {
    struct Voice {
        bool active=false, release=false;
        NoteTarget note{};
        std::uint64_t born=0;
        double sine=0, cosine=1, envelope=0, velocity=0, pressure=1,
            frequency=0, target_frequency=0, filter=0;
        std::uint32_t release_remaining=0;
    };
    struct Touch { std::uint64_t token=0, member=0; double velocity=0, pressure=1; };
    struct Tail { Voice voice{}; std::uint32_t left=0; };
    Determination determination_{};
    // Owned only by the serial producer; callback never touches this copy.
    Determination producer_determination_{};
    Identity producer_identity_{};
    struct ScheduledDetermination { Determination value{}; std::uint64_t from_sample=0; };
    std::array<ScheduledDetermination,8> source_schedule_{};
    std::size_t source_schedule_size_=1;
    unsigned rate_;
    PhysicalPort body_{};
    std::array<Voice,max_voices> voices_{};
    std::array<Touch,max_touches> touches_{};
    std::array<Tail,max_tails> tails_{};
    Spsc<Operation,queue_capacity> operations_{};
    struct PendingOperation { bool active=false; Operation operation{}; };
    std::array<PendingOperation,queue_capacity> pending_operations_{};
    std::array<std::size_t,queue_capacity> operation_heap_{};
    std::size_t heap_size_=0;
    Spsc<ReleaseOperation,64> releases_{};
    struct PendingRelease { bool active=false; ReleaseOperation operation{}; };
    std::array<PendingRelease,64> pending_releases_{};
    Spsc<Readback,64> readbacks_{};
    Spsc<Capture,capture_capacity> captures_{};
    Parameters source_{}, effective_{};
    std::uint64_t cursor_=0, accepted_sequence_=0, accepted_sample_=0, applied_sequence_=0,
        refused_=0, late_=0, stolen_=0, dropped_readbacks_=0, dropped_captures_=0, clipping_=0, force_limited_=0;
    std::atomic<std::uint64_t> published_cursor_{0}, overflow_count_{0}, published_sequence_{0}, panic_fence_{0};
    std::atomic<bool> emergency_{false}, capture_{false}, fault_{false};
    bool sustain_=false;
    static bool scalar(double x,double lo,double hi) noexcept { return std::isfinite(x) && x>=lo && x<=hi; }
    static bool same_lineage(const Identity& a,const Identity& b) noexcept { return a.instance==b.instance && a.event==b.event && a.subject==b.subject; }
    bool valid_determination(const Determination& d) const noexcept {
        if (!valid_ref(d.identity.instance) || !valid_ref(d.identity.event) || !valid_ref(d.identity.subject) || !valid_ref(d.m1_coordinate)
            || !valid_ref(d.registry_revision) || !valid_ref(d.source_revision) || !valid_ref(d.relation_plan_ref)
            || !valid_ref(d.tuning_ref) || !valid_ref(d.native_receipt_ref) || !valid_ref(d.body_preparation_ref)
            || !valid_ref(d.body_state_ref) || !valid_ref(d.m2_writer) || d.m1_face>1 || d.m2_face!=1
            || std::strcmp(d.m2_writer.data(),"#2-1")!=0 || d.tick12>=12 || d.lens12>=12 || d.basis>1
            || d.context_frame<1 || d.context_frame>7 || d.degree720>=720 || !d.tuning_available) return false;
        for (double hz:d.audio_octet_hz) if (!scalar(hz,0.001,rate_*0.45)) return false;
        for (const auto& n:d.nodal_quartet) if ((n.position!=0 && n.position!=5) || n.face>1 || n.m<1 || n.m>12 || n.n<1 || n.n>12) return false;
        const auto* source=ql_m_live_resolve(d.m1_coordinate.data());
        if (!source || source->root_position!=1 || !ql_m_live_accepts_base(d.registry_revision.data())
            || d.identity.m2_generation>9007199254740991ULL) return false;
        return true;
    }
    const Determination& source_at(std::uint64_t sample) const noexcept {
        const Determination* selected=&source_schedule_[0].value;
        for(std::size_t i=1;i<source_schedule_size_;++i) if(source_schedule_[i].from_sample<=sample) selected=&source_schedule_[i].value;
        return *selected;
    }
    void retire_source_schedule(std::uint64_t cursor) noexcept {
        std::size_t current=0;
        for(std::size_t i=1;i<source_schedule_size_;++i) if(source_schedule_[i].from_sample<=cursor) current=i;
        if(current) {
            for(std::size_t i=current;i<source_schedule_size_;++i) source_schedule_[i-current]=source_schedule_[i];
            source_schedule_size_-=current;
        }
    }
    bool valid_note(const NoteTarget& n,const Determination& source) const noexcept {
        if (!(n.identity==source.identity) || n.source_coordinate!=source.m1_coordinate
            || n.tuning_ref!=source.tuning_ref || n.source_face!=source.m1_face
            || !n.member || !n.touch || n.key>=12 || n.position!=n.key/2 || n.coordinate_face!=n.key%2
            || n.pitch_class>=12 || !scalar(n.hertz,0.001,rate_*0.45) || !scalar(n.fundamental_hz,0.001,rate_*0.45)
            || !scalar(n.phase_sin,-1,1) || !scalar(n.phase_cos,-1,1)
            || std::abs(n.phase_sin*n.phase_sin+n.phase_cos*n.phase_cos-1)>1e-10) return false;
        if (n.exact_ratio) {
            if (!n.ratio_numerator || !n.ratio_denominator) return false;
            const double target=n.fundamental_hz*(double(n.ratio_numerator)/double(n.ratio_denominator));
            if (!std::isfinite(target) || std::abs(target-n.hertz)>1e-11*n.hertz) return false;
        } else if (n.ratio_numerator || n.ratio_denominator) return false;
        return true;
    }
    static bool valid_parameter(Parameter p,double x,unsigned rate) noexcept {
        switch(p) {
        case Parameter::ForceNewtons: return scalar(x,0,100);
        case Parameter::AttackSeconds: return scalar(x,0.0001,10);
        case Parameter::ReleaseSeconds: return scalar(x,0.001,30);
        case Parameter::CutoffHertz: return scalar(x,1,rate*0.45);
        case Parameter::MasterLinear: case Parameter::BodyLinear: case Parameter::MonitorLinear: return scalar(x,0,1);
        } return false;
    }
    static double& field(Parameters& p,Parameter parameter) noexcept {
        switch(parameter) {
        case Parameter::ForceNewtons:return p.force_newtons;
        case Parameter::AttackSeconds:return p.attack_seconds;
        case Parameter::ReleaseSeconds:return p.release_seconds;
        case Parameter::CutoffHertz:return p.cutoff_hertz;
        case Parameter::MasterLinear:return p.master_linear;
        case Parameter::BodyLinear:return p.body_linear;
        case Parameter::MonitorLinear:return p.monitor_linear;
        } return p.master_linear;
    }
    void clear() noexcept {
        voices_.fill({}); touches_.fill({}); tails_.fill({}); sustain_=false;
    }
    void fence_attacks(std::uint64_t sequence) noexcept {
        auto previous=panic_fence_.load(std::memory_order_relaxed);
        while(previous<sequence && !panic_fence_.compare_exchange_weak(previous,sequence,std::memory_order_release,std::memory_order_relaxed)) {}
    }
    bool operation_before(std::size_t a,std::size_t b) const noexcept {
        const auto& x=pending_operations_[a].operation; const auto& y=pending_operations_[b].operation;
        return x.sample<y.sample || (x.sample==y.sample && x.sequence<y.sequence);
    }
    bool prepare_operation(const Operation& operation) noexcept {
        if(heap_size_==queue_capacity) return false;
        std::size_t index=0; while(pending_operations_[index].active) ++index;
        pending_operations_[index]=PendingOperation{true,operation};
        auto child=heap_size_++; operation_heap_[child]=index;
        while(child) {
            const auto parent=(child-1)/2;
            if(!operation_before(operation_heap_[child],operation_heap_[parent])) break;
            std::swap(operation_heap_[child],operation_heap_[parent]); child=parent;
        } return true;
    }
    void consume_operation() noexcept {
        pending_operations_[operation_heap_[0]].active=false;
        --heap_size_; if(!heap_size_) return;
        operation_heap_[0]=operation_heap_[heap_size_]; std::size_t parent=0;
        for(;;) {
            auto child=parent*2+1; if(child>=heap_size_) break;
            if(child+1<heap_size_ && operation_before(operation_heap_[child+1],operation_heap_[child])) ++child;
            if(!operation_before(operation_heap_[child],operation_heap_[parent])) break;
            std::swap(operation_heap_[child],operation_heap_[parent]); parent=child;
        }
    }
    void release(Voice& voice) noexcept {
        if (voice.release) return;
        voice.release=true;
        voice.release_remaining=std::uint32_t(std::ceil(source_.release_seconds*rate_));
    }
    void refresh(Voice& voice) noexcept {
        bool held=false; double velocity=0,pressure=0;
        for (const auto& t:touches_) if (t.token && t.member==voice.note.member) {
            held=true; velocity=std::max(velocity,t.velocity); pressure=std::max(pressure,t.pressure);
        }
        if (held) { voice.velocity=velocity; voice.pressure=pressure; }
        else if (!sustain_) release(voice);
    }
    void retire(Voice& voice) noexcept {
        auto slot=std::min_element(tails_.begin(),tails_.end(),[](const Tail& a,const Tail& b){return a.left<b.left;});
        // A fixed 64-sample steal fade is declared click-control, not body decay.
        *slot=Tail{voice,64};
        for (auto& t:touches_) if (t.member==voice.note.member) t={};
        voice={}; ++stolen_;
    }
    void apply(const Operation& op) noexcept {
        // Maximum acknowledged sequence; release admission can overtake a
        // future automation sequence without moving that automation's sample.
        applied_sequence_=std::max(applied_sequence_,op.sequence);
        switch(op.kind) {
        case Kind::Panic:
            fence_attacks(op.sequence); touches_.fill({}); sustain_=false; for (auto& v:voices_) if (v.active) release(v); break;
        case Kind::Parameter: field(source_,op.parameter)=op.value; break;
        case Kind::Determination:
            determination_=op.determination; break;
        case Kind::Sustain:
            sustain_=op.value==1; if (!sustain_) for (auto& v:voices_) if (v.active) refresh(v); break;
        case Kind::NoteOff:
            for (auto& t:touches_) if (t.token==op.touch) {
                const auto member=t.member; t={};
                for (auto& v:voices_) if (v.active && v.note.member==member) refresh(v);
                break;
            } break;
        case Kind::Expression:
            for (auto& t:touches_) if (t.token==op.touch) {
                t.pressure=op.value;
                for (auto& v:voices_) if (v.active && v.note.member==t.member) {
                    refresh(v); if (op.pitch_hz>0) { v.target_frequency=op.pitch_hz; v.note=op.note; }
                }
            } break;
        case Kind::NoteOn: {
            if (op.sequence<=panic_fence_.load(std::memory_order_relaxed)) return;
            for (const auto& t:touches_) if (t.token==op.note.touch) { ++refused_; return; }
            auto touch=std::find_if(touches_.begin(),touches_.end(),[](const Touch& t){return !t.token;});
            if (touch==touches_.end()) { ++refused_; return; }
            auto voice=std::find_if(voices_.begin(),voices_.end(),[&](const Voice& v){return v.active && !v.release && v.note.member==op.note.member;});
            if (voice==voices_.end()) {
                voice=std::find_if(voices_.begin(),voices_.end(),[](const Voice& v){return !v.active;});
                if (voice==voices_.end()) {
                    voice=std::min_element(voices_.begin(),voices_.end(),[](const Voice& a,const Voice& b){
                        if (a.release!=b.release) return a.release;
                        if (a.release && a.envelope!=b.envelope) return a.envelope<b.envelope;
                        return a.born<b.born;
                    }); retire(*voice);
                }
                *voice=Voice{}; voice->active=true; voice->note=op.note; voice->born=op.sample;
                voice->sine=op.note.phase_sin; voice->cosine=op.note.phase_cos;
                voice->frequency=voice->target_frequency=op.note.hertz;
            } else if (voice->note.key!=op.note.key || voice->note.register_octave!=op.note.register_octave
                || std::abs(voice->target_frequency-op.note.hertz)>1e-10) { ++refused_; return; }
            *touch=Touch{op.note.touch,op.note.member,op.value,1}; refresh(*voice); break;
        }
        }
    }
    void apply_release(const ReleaseOperation& op) noexcept {
        if (!same_lineage(op.identity,determination_.identity)) {++refused_;return;}
        applied_sequence_=std::max(applied_sequence_,op.sequence);
        if(op.kind==Kind::Panic) {
            fence_attacks(op.sequence); touches_.fill({}); sustain_=false;
            for(auto& v:voices_) if(v.active) release(v);
        } else if(op.kind==Kind::Sustain) {
            sustain_=false; for(auto& v:voices_) if(v.active) refresh(v);
        } else {
            for(auto& t:touches_) if(t.token==op.touch) {
                const auto member=t.member; t={};
                for(auto& v:voices_) if(v.active && v.note.member==member) refresh(v);
                break;
            }
        }
    }
    double sample_voice(Voice& v,double smoothing,double filter_coefficient) noexcept {
        if (!v.active) return 0;
        v.frequency+=(v.target_frequency-v.frequency)*smoothing;
        const double angle=tau*v.frequency/rate_, c=std::cos(angle), s=std::sin(angle);
        const double next_sine=v.sine*c+v.cosine*s;
        v.cosine=v.cosine*c-v.sine*s; v.sine=next_sine;
        // Renormalize this source quadrature every sample; no phase reset on edit.
        const double norm=std::hypot(v.sine,v.cosine); v.sine/=norm; v.cosine/=norm;
        if (v.release) {
            if (!v.release_remaining) { v={}; return 0; }
            v.envelope*=double(v.release_remaining-1)/double(v.release_remaining); --v.release_remaining;
        } else v.envelope=std::min(1.0,v.envelope+1.0/(effective_.attack_seconds*rate_));
        const double raw=v.sine*v.envelope*v.velocity*v.pressure;
        v.filter+=(raw-v.filter)*filter_coefficient;
        return v.filter;
    }
public:
    Engine(Determination determination,unsigned sample_rate,PhysicalPort body,Parameters parameters={})
        :determination_(determination),producer_determination_(determination),producer_identity_(determination.identity),rate_(sample_rate),body_(body),source_(parameters),effective_(parameters) {
        if (rate_<8000 || rate_>192000 || !valid_determination(determination_)
            || !body_.owner || !body_.advance || !body_.revision || !body_.cursor
            || body_.revision(body_.owner)!=determination_.body_revision
            || body_.event!=determination_.identity.event || body_.subject!=determination_.identity.subject
            || body_.preparation!=determination_.body_preparation_ref || body_.state!=determination_.body_state_ref
            || body_.sample_rate!=rate_ || !scalar(body_.max_force_newtons,1e-12,1e9)) throw std::invalid_argument("disconnected prepared native audio/body");
        for (unsigned id=0;id<=unsigned(Parameter::MonitorLinear);++id)
            if (!valid_parameter(Parameter(id),field(source_,Parameter(id)),rate_)) throw std::invalid_argument("invalid performance parameter");
        cursor_=body_.cursor(body_.owner); accepted_sample_=cursor_; published_cursor_.store(cursor_);
        source_schedule_[0]=ScheduledDetermination{determination_,cursor_};
    }
    // Single control owner only. Failed admission consumes no source sequence.
    // A full queue additionally requests safe all-notes-off out of band, so a
    // lost NoteOff can never leave a permanently sounding excitation.
    Result enqueue(const Operation& op) noexcept {
        if (fault_.load(std::memory_order_acquire)) return Result::Unavailable;
        const auto cursor=published_cursor_.load(std::memory_order_acquire);
        retire_source_schedule(cursor);
        if (accepted_sequence_==std::numeric_limits<std::uint64_t>::max()) return Result::Exhausted;
        if (unsigned(op.kind)>unsigned(Kind::Determination)) return Result::Invalid;
        if (op.sequence!=accepted_sequence_+1) return Result::Order;
        const bool late=op.sample<cursor;
        const bool critical=op.kind==Kind::NoteOff || op.kind==Kind::Panic || (op.kind==Kind::Sustain && op.value==0);
        const auto& source=source_at(std::max(op.sample,cursor));
        if(critical) {
            if(!same_lineage(op.identity,producer_identity_) || op.identity.m1_revision>producer_identity_.m1_revision
                || op.identity.m2_generation>producer_identity_.m2_generation) return Result::Stale;
        } else if(!(op.identity==source.identity)) return Result::Stale;
        if (late && !critical) return Result::Late;
        const auto limit=cursor>std::numeric_limits<std::uint64_t>::max()-std::uint64_t(rate_)*2
            ? std::numeric_limits<std::uint64_t>::max() : cursor+std::uint64_t(rate_)*2;
        if (op.sample>limit) return Result::Late;
        Operation admitted=op;
        admitted.late_admitted=late;
        if (late) admitted.sample=cursor;
        const bool separate_release=critical;
        if ((op.kind==Kind::NoteOn && (!valid_note(op.note,source) || !scalar(op.value,0,1)))
            || (op.kind==Kind::NoteOff && !op.touch)
            || (op.kind==Kind::Expression && (!op.touch || !scalar(op.value,0,1) || !scalar(op.pitch_hz,0,rate_*0.45)
                || (op.pitch_hz>0 && (!valid_note(op.note,source) || op.note.touch!=op.touch || op.note.hertz!=op.pitch_hz))))
            || (op.kind==Kind::Sustain && op.value!=0 && op.value!=1)
            || (op.kind==Kind::Parameter && !valid_parameter(op.parameter,op.value,rate_))) return Result::Invalid;
        if (op.kind==Kind::Determination && (!valid_determination(op.determination)
            || !same_lineage(op.determination.identity,producer_identity_)
            || op.determination.identity.m2_generation<=producer_identity_.m2_generation
            || op.determination.identity.m1_revision<producer_identity_.m1_revision
            || op.determination.body_revision!=producer_determination_.body_revision
            || op.determination.body_state_ref!=producer_determination_.body_state_ref
            || op.determination.body_preparation_ref!=producer_determination_.body_preparation_ref)) return Result::Invalid;
        if(op.kind==Kind::Determination) {
            if(op.sample<source_schedule_[source_schedule_size_-1].from_sample) return Result::Order;
            if(source_schedule_size_==source_schedule_.size()) return Result::Exhausted;
        }
        const bool queued=separate_release
            ? releases_.push(ReleaseOperation{op.kind,op.identity,op.sequence,admitted.sample,op.touch,late})
            : operations_.push(admitted);
        if (!queued) { overflow_count_.fetch_add(1); emergency_.store(true,std::memory_order_release); return Result::Overflow; }
        accepted_sequence_=op.sequence; if(!separate_release) accepted_sample_=admitted.sample;
        published_sequence_.store(accepted_sequence_,std::memory_order_release);
        if (op.kind==Kind::Determination) {
            producer_identity_=op.determination.identity; producer_determination_=op.determination;
            source_schedule_[source_schedule_size_++]=ScheduledDetermination{op.determination,op.sample};
        }
        return Result::Accepted;
    }
    // Out-of-band panic cancels previously queued attacks while preserving
    // prepared source changes and physical tails. A subsequent human attack
    // gets a higher sequence and can play normally.
    void request_panic() noexcept {
        fence_attacks(published_sequence_.load(std::memory_order_acquire));
        emergency_.store(true,std::memory_order_release);
    }
    void enable_capture(bool enabled) noexcept { capture_.store(enabled,std::memory_order_release); }
    std::uint64_t samples_elapsed() const noexcept { return published_cursor_.load(std::memory_order_acquire); }
    unsigned sample_rate() const noexcept { return rate_; }
    // Serial control owner only: the admitted source for the NEXT native
    // sample, including a determination already due at a block boundary.
    const Determination& current_source() const noexcept { return source_at(published_cursor_.load(std::memory_order_acquire)); }
    bool validate_note_target(const NoteTarget& note) const noexcept { return valid_note(note,source_at(published_cursor_.load(std::memory_order_acquire))); }
    bool has_physical_custody() const noexcept { return bool(body_.custody); }
    bool available() const noexcept { return !fault_.load(std::memory_order_acquire); }
    bool pop_readback(Readback& out) noexcept { return readbacks_.take(out); }
    bool pop_capture(Capture& out) noexcept { return captures_.take(out); }
    // Callback only; input cursor must be the actual body's sample cursor.
    // A detached/refused body emits silence, latches unavailable, and needs an
    // explicit stopped-owner recovery. It never advances a parallel clock.
    bool render(float* output,std::size_t frames,std::uint64_t start_sample) noexcept {
        if (!output || frames==0 || frames>max_frames) return false;
        std::fill_n(output,frames,0);
        if (fault_.load(std::memory_order_relaxed) || start_sample!=cursor_
            || cursor_>std::numeric_limits<std::uint64_t>::max()-frames) return false;
        if (body_.revision(body_.owner)!=determination_.body_revision || body_.cursor(body_.owner)!=cursor_) {
            fault_.store(true,std::memory_order_release); return false;
        }
        Capture capture{}; capture.identity=determination_.identity; capture.start_sample=cursor_;
        capture.body_revision=determination_.body_revision; capture.frames=std::uint32_t(frames);
        std::array<double,max_frames> body_gain{}, monitor_gain{}, force_scale{};
        const double smoothing=-std::expm1(-1.0/(0.005*rate_));
        ReleaseOperation arriving{};
        while(releases_.take(arriving)) {
            auto slot=std::find_if(pending_releases_.begin(),pending_releases_.end(),[](const PendingRelease& p){return !p.active;});
            if(slot==pending_releases_.end()) {
                overflow_count_.fetch_add(1); clear(); fault_.store(true,std::memory_order_release); return false;
            }
            *slot=PendingRelease{true,arriving};
        }
        Operation arriving_operation{};
        while(operations_.take(arriving_operation)) if(!prepare_operation(arriving_operation)) {
            overflow_count_.fetch_add(1); clear(); fault_.store(true,std::memory_order_release); return false;
        }
        for (std::size_t i=0;i<frames;++i) {
            if (emergency_.exchange(false,std::memory_order_acq_rel)) {
                if (overflow_count_.load(std::memory_order_relaxed)) {
                    // Queued source generations may be ahead of the callback.
                    // Do not pretend that discarding them leaves a coherent
                    // playing owner. Explicit stopped-owner recovery is needed.
                    clear(); fault_.store(true,std::memory_order_release); return false;
                }
                touches_.fill({}); sustain_=false; for (auto& v:voices_) if (v.active) release(v);
            }
            // Bounded sample/sequence order, independent of arrival order.
            // Live playing and releases can overtake future clip/automation.
            for(;;) {
                PendingRelease* due=nullptr;
                for(auto& pending:pending_releases_) if(pending.active && pending.operation.sample<=cursor_+i
                    && (!due || pending.operation.sample<due->operation.sample
                        || (pending.operation.sample==due->operation.sample && pending.operation.sequence<due->operation.sequence))) due=&pending;
                const Operation* op=heap_size_?&pending_operations_[operation_heap_[0]].operation:nullptr;
                if(op && op->sample>cursor_+i) op=nullptr;
                if(!due && !op) break;
                if(due && (!op || due->operation.sample<op->sample
                    || (due->operation.sample==op->sample && due->operation.sequence<op->sequence))) {
                    if(due->operation.late_admitted || due->operation.sample<cursor_+i) ++late_;
                    apply_release(due->operation); due->active=false;
                } else {
                    if(op->late_admitted || op->sample<cursor_+i) ++late_;
                    if(op->identity==determination_.identity) apply(*op); else ++refused_;
                    consume_operation();
                }
            }
            for (unsigned id=0;id<=unsigned(Parameter::MonitorLinear);++id) {
                auto& x=field(effective_,Parameter(id)); x+=(field(source_,Parameter(id))-x)*smoothing;
            }
            const double filter=-std::expm1(-tau*effective_.cutoff_hertz/rate_);
            double force=0;
            for (auto& v:voices_) force+=sample_voice(v,smoothing,filter);
            for (auto& tail:tails_) if (tail.left) {
                force+=sample_voice(tail.voice,smoothing,filter)*(double(tail.left)/64); --tail.left;
            }
            // Fixed headroom bound, independent of current polyphony. Newton
            // scale is declared material policy and visible in readback.
            force*=effective_.force_newtons/double(max_voices+max_tails);
            if (std::abs(force)>body_.max_force_newtons) ++force_limited_;
            force=std::clamp(force,-body_.max_force_newtons,body_.max_force_newtons);
            capture.force_newtons[i]=force;
            body_gain[i]=effective_.master_linear*effective_.body_linear;
            monitor_gain[i]=effective_.master_linear*effective_.monitor_linear;
            force_scale[i]=effective_.force_newtons;
        }
        capture.end_identity=determination_.identity;
        if (!body_.advance(body_.owner,capture.force_newtons.data(),capture.pickup_linear.data(),frames,determination_.body_revision,cursor_)) {
            clear(); fault_.store(true,std::memory_order_release); return false;
        }
        double peak=0,power=0;
        for (std::size_t i=0;i<frames;++i) {
            const double monitor=force_scale[i]>0 ? capture.force_newtons[i]/force_scale[i] : 0;
            const double raw=body_gain[i]*capture.pickup_linear[i]+monitor_gain[i]*monitor;
            if (!std::isfinite(raw)) { fault_.store(true,std::memory_order_release); clear(); std::fill_n(output,frames,0); return false; }
            if (std::abs(raw)>0.98) ++clipping_;
            output[i]=float(std::clamp(raw,-0.98,0.98)); capture.output_linear[i]=output[i];
            peak=std::max(peak,std::abs(double(output[i]))); power+=double(output[i])*output[i];
        }
        cursor_+=frames; published_cursor_.store(cursor_,std::memory_order_release);
        Readback receipt{}; receipt.identity=determination_.identity; receipt.samples_elapsed=cursor_;
        receipt.body_revision=determination_.body_revision; receipt.last_sequence=applied_sequence_;
        receipt.refused=refused_; receipt.late=late_; receipt.overflows=overflow_count_.load(); receipt.stolen=stolen_;
        receipt.dropped_readbacks=dropped_readbacks_; receipt.dropped_captures=dropped_captures_; receipt.clipping_samples=clipping_;
        receipt.force_limited_samples=force_limited_;
        for (const auto& v:voices_) if (v.active) {
            auto& observed=receipt.voices[receipt.active_voices++];
            observed.member=v.note.member; observed.m1_revision=v.note.identity.m1_revision;
            observed.m2_generation=v.note.identity.m2_generation;
            observed.effective_hertz=v.frequency; observed.target_hertz=v.target_frequency;
            observed.envelope=v.envelope; observed.releasing=v.release;
            for (const auto& t:touches_) observed.held_touches+=t.token && t.member==v.note.member;
        }
        for (const auto& t:touches_) receipt.active_touches+=bool(t.token);
        receipt.peak=peak; receipt.rms=std::sqrt(power/frames); receipt.source=source_; receipt.effective=effective_;
        receipt.sustain=sustain_; receipt.available=available();
        if (!readbacks_.push(receipt)) ++dropped_readbacks_;
        if (capture_.load(std::memory_order_relaxed) && !captures_.push(capture)) ++dropped_captures_;
        return true;
    }
};
} // namespace ql::performance
#endif
