#ifndef QL_PHYSICAL_BODY_HPP
#define QL_PHYSICAL_BODY_HPP
// Physical preparation is a control-thread operation. This is an additive
// continuation of the existing native field owner, not a symbolic registry,
// clock, renderer or independently integrated visible body.
#include <ql/continuous_field.hpp>
#include <algorithm>
#include <cstddef>
#include <numeric>
#include <cstring>

namespace ql {
inline constexpr const char* physical_body_contract = "ql.physical-body/v1";
inline constexpr std::size_t physical_max_nodes = 32, physical_max_edges = 96;
inline constexpr std::size_t physical_max_dofs = 3 * physical_max_nodes;
inline constexpr std::size_t physical_max_frames = 512;
enum class BodyFamily { AxialTruss, PrestressedTensionNetwork };
struct PhysicalMaterial {
    std::string reference, revision, source_ref, standing;
    double young_modulus_pa, density_kg_per_m3;
    // Rayleigh C = alpha M + beta K. alpha s^-1, beta s.
    double damping_alpha_per_second, damping_beta_seconds;
};
struct PhysicalNode {
    std::uint64_t identity;
    std::string constituent;
    Vec3 rest_metres;
    double additional_mass_kg;
    std::array<bool,3> fixed;
};
struct PhysicalEdge {
    std::size_t first, second;
    double section_m2, prestress_newtons;
};
struct PhysicalProjection {
    Vec3 axis;
    // Nonnegative partition of unity: a distributed spatial force or pickup.
    std::vector<double> node_weights;
};
struct PhysicalBodyInput {
    std::string event_ref, subject_ref, source_coordinate, source_revision;
    std::string geometry_ref, geometry_revision, geometry_source_ref, geometry_standing;
    std::string preparation_ref, state_ref;
    std::uint64_t source_generation, body_revision;
    unsigned sample_rate;
    bool pratibimba;
    BodyFamily family;
    PhysicalMaterial material;
    std::vector<PhysicalNode> nodes;
    std::vector<PhysicalEdge> edges;
    PhysicalProjection exciter, pickup;
    double pickup_linear_per_metre, max_force_newtons, max_impulse_newton_seconds, max_displacement_metres;
};
struct PhysicalObservation {
    const std::string* preparation_ref;
    const std::string* state_ref;
    std::uint64_t body_revision, samples_elapsed;
    double pickup_linear, mechanical_energy_joules;
};
enum class PhysicalFormTransition { ProjectCorrespondingNodes, ExplicitReset };
struct PhysicalFormTransitionReceipt {
    std::uint64_t before_revision, after_revision, samples_elapsed;
    PhysicalFormTransition policy;
    double before_energy_joules, after_energy_joules, external_work_joules;
};
inline constexpr const char* physical_checkpoint_contract = "ql.physical-body-checkpoint/v1";
struct PhysicalBodyCheckpoint {
    std::uint32_t version=1;
    std::string contract=physical_checkpoint_contract;
    std::string event_ref,subject_ref,preparation_ref,state_ref;
    std::string source_coordinate,source_revision,geometry_ref,geometry_revision;
    std::string geometry_source_ref,geometry_standing,material_ref,material_revision,material_source_ref,material_standing;
    std::string eigenbasis_identity;
    bool pratibimba=false;
    std::uint64_t source_generation=0,body_revision=0,samples_elapsed=0;
    unsigned sample_rate=0;
    std::string displacement_unit="m",velocity_unit="m/s",pickup_unit="linear";
    double generalized_modal_mass_kg=1;
    std::vector<double> displacement_modal_metres,velocity_modal_metres_per_second;
    double last_pickup_linear=0;
};
struct PhysicalStep {
    double a11, a12, a21, a22, bq, bv;
};
inline PhysicalStep physical_step(double lambda, double gamma, double dt) {
    require(std::isfinite(lambda) && lambda >= 0 && std::isfinite(gamma) && gamma >= 0,
            "invalid physical eigenvalue or damping");
    if (lambda == 0) {
        const double rate = 2 * gamma;
        if (rate == 0) return {1,dt,0,1,0.5*dt*dt,dt};
        const double bv = -std::expm1(-rate*dt)/rate;
        double bq;
        if (rate*dt < 1e-4) {
            const double z = rate*dt;
            bq = dt*dt*(0.5-z/6+z*z/24-z*z*z/120);
        } else bq = (dt-bv)/rate;
        return {1,bv,0,std::exp(-rate*dt),bq,bv};
    }
    const double delta = lambda-gamma*gamma;
    double c, s;
    if (std::abs(delta)*dt*dt < 1e-12) {
        c = std::exp(-gamma*dt);
        s = c*dt;
    } else if (delta > 0) {
        const double w = std::sqrt(delta), decay = std::exp(-gamma*dt);
        c = decay*std::cos(w*dt); s = decay*std::sin(w*dt)/w;
    } else {
        const double r = std::sqrt(-delta);
        // Stable real-root form; never exp(+r*dt) before damping cancels it.
        const double e1 = std::exp((-gamma+r)*dt), e2 = std::exp((-gamma-r)*dt);
        c = 0.5*(e1+e2); s = (e1-e2)/(2*r);
    }
    const double a11 = c+gamma*s, a21 = -lambda*s;
    double bq = (1-a11)/lambda;
    // Avoid cancellation for a stiff mass sampled at a very small dt.
    if (lambda*dt*dt < 1e-8 && gamma*dt < 1e-4)
        bq = dt*dt*(0.5-gamma*dt/3+(4*gamma*gamma-lambda)*dt*dt/24);
    return {a11,s,a21,c-gamma*s,bq,s};
}

class PreparedPhysicalBody {
    friend class PhysicalBody;
    PhysicalBodyInput input_;
    std::string eigenbasis_identity_;
    std::vector<double> node_mass_, lambda_, gamma_, excitation_, pickup_, shape_peak_;
    // Modal shapes dimensionless, normalized to one kilogram generalized mass.
    std::vector<std::vector<Vec3>> shapes_;
    std::vector<PhysicalStep> step_;
    std::vector<std::pair<std::size_t,unsigned>> dofs_;
    // A deterministic binary fingerprint of the admitted preparation and
    // actually derived basis. This is compatibility evidence, not a signature
    // or host authentication. IEEE binary64 values are hashed in fixed byte
    // order; different numerical bases deliberately cannot restore each other.
    std::string fingerprint() const {
        static_assert(sizeof(double)==8 && std::numeric_limits<double>::is_iec559);
        std::uint64_t hash=14695981039346656037ULL;
        const auto byte=[&](unsigned char x){hash^=x;hash*=1099511628211ULL;};
        const auto integer=[&](std::uint64_t x){for(unsigned i=0;i<8;++i){byte(static_cast<unsigned char>(x&255));x>>=8;}};
        const auto real=[&](double x){std::uint64_t bits;std::memcpy(&bits,&x,8);integer(bits);};
        const auto text=[&](const std::string& x){integer(x.size());for(unsigned char b:x)byte(b);};
        const auto& in=input_;text("ql.physical-eigenbasis/fnv1a64-v1");
        for(const auto* value:{&in.event_ref,&in.subject_ref,&in.source_coordinate,&in.source_revision,&in.geometry_ref,
            &in.geometry_revision,&in.geometry_source_ref,&in.geometry_standing,&in.preparation_ref,&in.state_ref,
            &in.material.reference,&in.material.revision,&in.material.source_ref,&in.material.standing})text(*value);
        integer(in.source_generation);integer(in.body_revision);integer(in.sample_rate);integer(in.pratibimba);integer(static_cast<unsigned>(in.family));
        for(double x:{in.material.young_modulus_pa,in.material.density_kg_per_m3,in.material.damping_alpha_per_second,
            in.material.damping_beta_seconds,in.pickup_linear_per_metre,in.max_force_newtons,in.max_impulse_newton_seconds,in.max_displacement_metres})real(x);
        integer(in.nodes.size());for(const auto& node:in.nodes){integer(node.identity);text(node.constituent);for(double x:node.rest_metres)real(x);
            real(node.additional_mass_kg);for(bool fixed:node.fixed)integer(fixed);}
        integer(in.edges.size());for(const auto& edge:in.edges){integer(edge.first);integer(edge.second);real(edge.section_m2);real(edge.prestress_newtons);}
        for(const auto* projection:{&in.exciter,&in.pickup}){for(double x:projection->axis)real(x);integer(projection->node_weights.size());for(double x:projection->node_weights)real(x);}
        for(const auto* values:{&node_mass_,&lambda_,&gamma_,&excitation_,&pickup_,&shape_peak_}){integer(values->size());for(double x:*values)real(x);}
        integer(shapes_.size());for(const auto& mode:shapes_){integer(mode.size());for(const auto& node:mode)for(double x:node)real(x);}
        for(const auto& step:step_)for(double x:{step.a11,step.a12,step.a21,step.a22,step.bq,step.bv})real(x);
        constexpr char digits[]="0123456789abcdef";std::string result="ql.physical-eigenbasis/fnv1a64-v1:";
        for(int i=15;i>=0;--i)result.push_back(digits[(hash>>(4*i))&15]);return result;
    }
    static void magnitude(double x, double low, double high, const char* message) {
        require(std::isfinite(x) && x >= low && x <= high, message);
    }
    static void projection(const PhysicalProjection& p, std::size_t count) {
        require(p.node_weights.size() == count, "spatial projection/node mismatch");
        double norm=0,sum=0;
        for (double x:p.axis) { magnitude(x,-1,1,"invalid physical projection axis"); norm+=x*x; }
        require(std::abs(norm-1) <= 1e-10, "physical projection axis must be normalized");
        for (double w:p.node_weights) { magnitude(w,0,1,"invalid projection weight"); sum+=w; }
        require(std::abs(sum-1) <= 1e-10, "physical projection weights must sum to one");
    }
public:
    explicit PreparedPhysicalBody(PhysicalBodyInput input):input_(std::move(input)) {
        const auto& in=input_;
        for (const auto* ref:{&in.event_ref,&in.subject_ref,&in.source_coordinate,&in.source_revision,
                &in.geometry_ref,&in.geometry_revision,&in.geometry_source_ref,&in.geometry_standing,
                &in.preparation_ref,&in.state_ref,&in.material.reference,&in.material.revision,
                &in.material.source_ref,&in.material.standing}) reference(*ref);
        const auto* source=ql_m_live_resolve(in.source_coordinate.c_str());
        require(source && source->root_position==3,"body requires exact native M3 source coordinate");
        require(in.sample_rate>=8000 && in.sample_rate<=192000,"unsupported body sample rate");
        require(in.source_generation<=9007199254740991ULL && in.body_revision<=9007199254740991ULL,
                "physical generation/revision exceeds exact transport integer");
        require(in.nodes.size()>=2 && in.nodes.size()<=physical_max_nodes && !in.edges.empty()
                && in.edges.size()<=physical_max_edges,"physical preparation resource budget exceeded");
        require(in.family==BodyFamily::AxialTruss || in.family==BodyFamily::PrestressedTensionNetwork,
                "unsupported physical constitutive family");
        magnitude(in.material.young_modulus_pa,1,1e13,"invalid elastic modulus Pa");
        magnitude(in.material.density_kg_per_m3,1e-3,1e6,"invalid density kg/m3");
        magnitude(in.material.damping_alpha_per_second,0,1e6,"invalid Rayleigh alpha s^-1");
        magnitude(in.material.damping_beta_seconds,0,1,"invalid Rayleigh beta seconds");
        magnitude(in.max_force_newtons,1e-12,1e9,"invalid Newton force limit");
        magnitude(in.max_impulse_newton_seconds,1e-12,1e9,"invalid Newton-second impulse limit");
        magnitude(in.max_displacement_metres,1e-12,1e3,"invalid displacement limit metres");
        magnitude(in.pickup_linear_per_metre,-1e9,1e9,"invalid pickup gain per metre");
        projection(in.exciter,in.nodes.size()); projection(in.pickup,in.nodes.size());
        node_mass_.assign(in.nodes.size(),0);
        std::array<int,physical_max_dofs> map; map.fill(-1);
        for (std::size_t i=0;i<in.nodes.size();++i) {
            const auto& node=in.nodes[i]; reference(node.constituent);
            require(ql_m_live_resolve(node.constituent.c_str()),"unknown exact body constituent");
            require(i==0 || in.nodes[i-1].identity<node.identity,"body node identities must be ordered unique");
            require(node.identity<=9007199254740991ULL,"body node identity exceeds exact range");
            for (double x:node.rest_metres) magnitude(x,-1e3,1e3,"invalid body geometry metres");
            magnitude(node.additional_mass_kg,0,1e9,"invalid point mass kg");
            node_mass_[i]=node.additional_mass_kg;
            for (unsigned axis=0;axis<3;++axis) if (!node.fixed[axis]) {
                map[3*i+axis]=static_cast<int>(dofs_.size()); dofs_.push_back({i,axis});
            }
        }
        require(!dofs_.empty(),"body has no unconstrained degrees of freedom");
        const std::size_t n=dofs_.size();
        std::vector<double> matrix(n*n,0), vectors(n*n,0);
        for (std::size_t e=0;e<in.edges.size();++e) {
            const auto& edge=in.edges[e];
            require(edge.first<in.nodes.size() && edge.second<in.nodes.size() && edge.first!=edge.second,
                    "invalid physical edge endpoints");
            for (std::size_t j=0;j<e;++j) {
                const auto& other=in.edges[j];
                require(!((other.first==edge.first && other.second==edge.second)
                        || (other.first==edge.second && other.second==edge.first)),"duplicate physical edge");
            }
            magnitude(edge.section_m2,1e-12,1e3,"invalid edge section m2");
            magnitude(edge.prestress_newtons,0,1e9,"compressive prestress requires a buckling solver");
            require(in.family!=BodyFamily::AxialTruss || edge.prestress_newtons==0,
                    "axial truss cannot silently use tension-network law");
            Vec3 u{}; double length=0;
            for (unsigned a=0;a<3;++a) { u[a]=in.nodes[edge.second].rest_metres[a]-in.nodes[edge.first].rest_metres[a]; length+=u[a]*u[a]; }
            length=std::sqrt(length); magnitude(length,1e-6,1e3,"degenerate/excessive edge length metres");
            for (double& x:u) x/=length;
            const double mass=in.material.density_kg_per_m3*edge.section_m2*length/2;
            node_mass_[edge.first]+=mass; node_mass_[edge.second]+=mass;
            const double axial=in.material.young_modulus_pa*edge.section_m2/length;
            const double transverse=edge.prestress_newtons/length;
            for (unsigned a=0;a<3;++a) for (unsigned b=0;b<3;++b) {
                const double stiffness=axial*u[a]*u[b]+transverse*((a==b?1.0:0.0)-u[a]*u[b]);
                for (unsigned endA=0;endA<2;++endA) for (unsigned endB=0;endB<2;++endB) {
                    const auto nodeA=endA?edge.second:edge.first, nodeB=endB?edge.second:edge.first;
                    const int row=map[3*nodeA+a], col=map[3*nodeB+b];
                    if (row>=0 && col>=0) matrix[std::size_t(row)*n+std::size_t(col)] += (endA==endB?1:-1)*stiffness;
                }
            }
        }
        for (double mass:node_mass_) magnitude(mass,1e-12,1e12,"body node has missing/excessive physical mass");
        for (std::size_t i=0;i<n;++i) {
            vectors[i*n+i]=1;
            for (std::size_t j=0;j<n;++j) matrix[i*n+j]/=std::sqrt(node_mass_[dofs_[i].first]*node_mass_[dofs_[j].first]);
        }
        double scale=0; for (double x:matrix) scale=std::max(scale,std::abs(x));
        require(scale>0 && std::isfinite(scale),"body has no finite elastic eigenstructure");
        bool converged=false;
        for (unsigned sweep=0;sweep<80;++sweep) {
            double off=0;
            for (std::size_t p=0;p<n;++p) for (std::size_t q=p+1;q<n;++q) off=std::max(off,std::abs(matrix[p*n+q]));
            if (off<=scale*1e-12) { converged=true; break; }
            for (std::size_t p=0;p<n;++p) for (std::size_t q=p+1;q<n;++q) {
                const double apq=matrix[p*n+q]; if (std::abs(apq)<=scale*1e-14) continue;
                const double tau=(matrix[q*n+q]-matrix[p*n+p])/(2*apq);
                const double t=std::copysign(1.0,tau)/(std::abs(tau)+std::hypot(1.0,tau));
                const double c=1/std::sqrt(1+t*t), s=t*c;
                const double app=matrix[p*n+p], aqq=matrix[q*n+q];
                matrix[p*n+p]=app-t*apq; matrix[q*n+q]=aqq+t*apq; matrix[p*n+q]=matrix[q*n+p]=0;
                for (std::size_t k=0;k<n;++k) {
                    if (k!=p && k!=q) {
                        const double akp=matrix[k*n+p], akq=matrix[k*n+q];
                        matrix[k*n+p]=matrix[p*n+k]=c*akp-s*akq;
                        matrix[k*n+q]=matrix[q*n+k]=s*akp+c*akq;
                    }
                    const double vkp=vectors[k*n+p], vkq=vectors[k*n+q];
                    vectors[k*n+p]=c*vkp-s*vkq; vectors[k*n+q]=s*vkp+c*vkq;
                }
            }
        }
        require(converged || n==1,"bounded physical eigensolver did not converge");
        std::vector<std::size_t> order(n); std::iota(order.begin(),order.end(),0);
        std::sort(order.begin(),order.end(),[&](auto a,auto b){return matrix[a*n+a]<matrix[b*n+b];});
        for (auto index:order) {
            double eigen=matrix[index*n+index];
            require(eigen>=-scale*1e-10 && std::isfinite(eigen),"unstable physical stiffness eigenvalue");
            if (std::abs(eigen)<scale*1e-12) eigen=0;
            require(std::sqrt(eigen)/(2*pi)<in.sample_rate*0.45,"physical mode above admitted audio band; refine model/rate explicitly");
            lambda_.push_back(eigen);
            const double gamma=(in.material.damping_alpha_per_second+in.material.damping_beta_seconds*eigen)/2;
            gamma_.push_back(gamma); step_.push_back(physical_step(eigen,gamma,1.0/in.sample_rate));
            std::vector<Vec3> shape(in.nodes.size(),Vec3{0,0,0});
            for (std::size_t row=0;row<n;++row) shape[dofs_[row].first][dofs_[row].second]=vectors[row*n+index]/std::sqrt(node_mass_[dofs_[row].first]);
            double ex=0, pick=0;
            for (std::size_t node=0;node<in.nodes.size();++node) for (unsigned a=0;a<3;++a) {
                ex+=shape[node][a]*in.exciter.axis[a]*in.exciter.node_weights[node];
                pick+=shape[node][a]*in.pickup.axis[a]*in.pickup.node_weights[node];
            }
            double peak=0;
            for (const auto& vector:shape) peak=std::max(peak,std::sqrt(vector[0]*vector[0]+vector[1]*vector[1]+vector[2]*vector[2]));
            shape_peak_.push_back(peak);
            excitation_.push_back(ex); pickup_.push_back(pick*in.pickup_linear_per_metre); shapes_.push_back(std::move(shape));
        }
        eigenbasis_identity_=fingerprint();
    }
    const PhysicalBodyInput& input() const noexcept { return input_; }
    const std::string& eigenbasis_identity() const noexcept { return eigenbasis_identity_; }
    std::size_t mode_count() const noexcept { return lambda_.size(); }
    double frequency_hz(std::size_t mode) const { return std::sqrt(lambda_.at(mode))/(2*pi); }
    double node_mass_kg(std::size_t node) const { return node_mass_.at(node); }
    const std::vector<Vec3>& mode_shape(std::size_t mode) const { return shapes_.at(mode); }
};

class PhysicalBody {
    PreparedPhysicalBody prepared_;
    std::vector<double> q_,v_,scratch_q_,scratch_v_;
    std::array<float,physical_max_frames> scratch_audio_{};
    std::uint64_t elapsed_=0;
    double last_=0;
    bool admissible(const std::vector<double>& q,const std::vector<double>& v) const noexcept {
        // Triangle bound guarantees every nodal displacement is inside the
        // declared limit. O(modes) per sample, rather than an audio-rate mesh
        // reconstruction. It may conservatively refuse a cancelling state.
        double envelope=0,energy=0;
        for (std::size_t m=0;m<q.size();++m) {
            if (!std::isfinite(q[m]) || !std::isfinite(v[m])) return false;
            envelope+=std::abs(q[m])*prepared_.shape_peak_[m];
            energy+=v[m]*v[m]+prepared_.lambda_[m]*q[m]*q[m];
        }
        return std::isfinite(envelope) && std::isfinite(energy)
                && envelope<=prepared_.input_.max_displacement_metres;
    }
public:
    explicit PhysicalBody(PreparedPhysicalBody prepared):prepared_(std::move(prepared)) {
        q_.assign(prepared_.mode_count(),0); v_=q_; scratch_q_=q_; scratch_v_=q_;
    }
    const PreparedPhysicalBody& preparation() const noexcept { return prepared_; }
    std::uint64_t body_revision() const noexcept { return prepared_.input_.body_revision; }
    std::uint64_t samples_elapsed() const noexcept { return elapsed_; }
    const std::string& preparation_ref() const noexcept { return prepared_.input_.preparation_ref; }
    const std::string& state_ref() const noexcept { return prepared_.input_.state_ref; }
    bool advance_force_block(const double* force_newtons,float* pickup_linear,std::size_t frames,
            std::uint64_t expected_body_revision,std::uint64_t start_sample) noexcept {
        if (!force_newtons || !pickup_linear || frames==0 || frames>physical_max_frames
                || expected_body_revision!=body_revision() || start_sample!=elapsed_
                || elapsed_>std::numeric_limits<std::uint64_t>::max()-frames) return false;
        for (std::size_t i=0;i<frames;++i) if (!std::isfinite(force_newtons[i])
                || std::abs(force_newtons[i])>prepared_.input_.max_force_newtons) return false;
        std::copy(q_.begin(),q_.end(),scratch_q_.begin()); std::copy(v_.begin(),v_.end(),scratch_v_.begin());
        for (std::size_t i=0;i<frames;++i) {
            double pickup=0;
            for (std::size_t m=0;m<q_.size();++m) {
                const auto& a=prepared_.step_[m]; const double force=force_newtons[i]*prepared_.excitation_[m];
                const double q=scratch_q_[m],v=scratch_v_[m];
                scratch_q_[m]=a.a11*q+a.a12*v+a.bq*force;
                scratch_v_[m]=a.a21*q+a.a22*v+a.bv*force;
                pickup+=scratch_q_[m]*prepared_.pickup_[m];
            }
            if (!admissible(scratch_q_,scratch_v_) || !std::isfinite(pickup)
                    || std::abs(pickup)>std::numeric_limits<float>::max()) return false;
            scratch_audio_[i]=static_cast<float>(pickup);
        }
        q_.swap(scratch_q_); v_.swap(scratch_v_); elapsed_+=frames; last_=scratch_audio_[frames-1];
        std::copy_n(scratch_audio_.begin(),frames,pickup_linear); return true;
    }
    bool apply_impulse_newton_seconds(double impulse,std::uint64_t revision,std::uint64_t sample) noexcept {
        if (revision!=body_revision() || sample!=elapsed_ || !std::isfinite(impulse)
                || std::abs(impulse)>prepared_.input_.max_impulse_newton_seconds) return false;
        std::copy(v_.begin(),v_.end(),scratch_v_.begin());
        for (std::size_t m=0;m<v_.size();++m) scratch_v_[m]+=impulse*prepared_.excitation_[m];
        if (!admissible(q_,scratch_v_)) return false;
        v_.swap(scratch_v_); return true;
    }
    bool write_displacements(Vec3* out,std::size_t count,std::uint64_t revision,std::uint64_t sample) const noexcept {
        if (!out || count!=prepared_.input_.nodes.size() || revision!=body_revision() || sample!=elapsed_) return false;
        for (std::size_t node=0;node<count;++node) {
            out[node]={0,0,0};
            for (std::size_t m=0;m<q_.size();++m) for (unsigned a=0;a<3;++a) out[node][a]+=q_[m]*prepared_.shapes_[m][node][a];
        }
        return true;
    }
    bool write_visible_positions(Vec3* out,std::size_t count,std::uint64_t revision,std::uint64_t sample) const noexcept {
        if (!write_displacements(out,count,revision,sample)) return false;
        for (std::size_t node=0;node<count;++node) for (unsigned a=0;a<3;++a) out[node][a]+=prepared_.input_.nodes[node].rest_metres[a];
        return true;
    }
    double mechanical_energy_joules() const noexcept {
        double energy=0; for (std::size_t m=0;m<q_.size();++m) energy+=0.5*(v_[m]*v_[m]+prepared_.lambda_[m]*q_[m]*q_[m]); return energy;
    }
    PhysicalObservation observation() const noexcept {
        return {&preparation_ref(),&state_ref(),body_revision(),elapsed_,last_,mechanical_energy_joules()};
    }
    // Off callback with stopped or callback-acknowledged exclusive custody.
    // A snapshot allocates bounded control-thread vectors; restore copies into
    // existing vectors and validates the entire candidate before any mutation.
    // The host checkpoints its note/queue state at this same cursor separately.
    PhysicalBodyCheckpoint checkpoint() const {
        const auto& in=prepared_.input_;PhysicalBodyCheckpoint result;
        result.event_ref=in.event_ref;result.subject_ref=in.subject_ref;result.preparation_ref=in.preparation_ref;result.state_ref=in.state_ref;
        result.source_coordinate=in.source_coordinate;result.source_revision=in.source_revision;result.pratibimba=in.pratibimba;
        result.geometry_ref=in.geometry_ref;result.geometry_revision=in.geometry_revision;
        result.geometry_source_ref=in.geometry_source_ref;result.geometry_standing=in.geometry_standing;
        result.material_ref=in.material.reference;result.material_revision=in.material.revision;
        result.material_source_ref=in.material.source_ref;result.material_standing=in.material.standing;
        result.eigenbasis_identity=prepared_.eigenbasis_identity_;result.source_generation=in.source_generation;
        result.body_revision=in.body_revision;result.samples_elapsed=elapsed_;result.sample_rate=in.sample_rate;
        result.displacement_modal_metres=q_;result.velocity_modal_metres_per_second=v_;result.last_pickup_linear=last_;return result;
    }
    bool restore_checkpoint(const PhysicalBodyCheckpoint& saved,std::uint64_t expected_body_revision,
            std::uint64_t expected_samples_elapsed) noexcept {
        const auto& in=prepared_.input_;
        if(expected_body_revision!=body_revision() || expected_samples_elapsed!=elapsed_ || saved.version!=1
            || saved.contract!=physical_checkpoint_contract || saved.event_ref!=in.event_ref || saved.subject_ref!=in.subject_ref
            || saved.preparation_ref!=in.preparation_ref || saved.state_ref!=in.state_ref || saved.source_coordinate!=in.source_coordinate
            || saved.source_revision!=in.source_revision || saved.pratibimba!=in.pratibimba || saved.source_generation!=in.source_generation
            || saved.geometry_ref!=in.geometry_ref || saved.geometry_revision!=in.geometry_revision
            || saved.geometry_source_ref!=in.geometry_source_ref || saved.geometry_standing!=in.geometry_standing
            || saved.material_ref!=in.material.reference || saved.material_revision!=in.material.revision
            || saved.material_source_ref!=in.material.source_ref || saved.material_standing!=in.material.standing
            || saved.eigenbasis_identity!=prepared_.eigenbasis_identity_ || saved.body_revision!=in.body_revision || saved.sample_rate!=in.sample_rate
            || saved.displacement_unit!="m" || saved.velocity_unit!="m/s" || saved.pickup_unit!="linear" || saved.generalized_modal_mass_kg!=1
            || saved.displacement_modal_metres.size()!=q_.size() || saved.velocity_modal_metres_per_second.size()!=v_.size()
            || !std::isfinite(saved.last_pickup_linear) || !admissible(saved.displacement_modal_metres,saved.velocity_modal_metres_per_second))return false;
        double pickup=0;for(std::size_t m=0;m<q_.size();++m)pickup+=saved.displacement_modal_metres[m]*prepared_.pickup_[m];
        if(!std::isfinite(pickup) || (saved.last_pickup_linear!=pickup && saved.last_pickup_linear!=static_cast<double>(static_cast<float>(pickup))))return false;
        std::copy(saved.displacement_modal_metres.begin(),saved.displacement_modal_metres.end(),q_.begin());
        std::copy(saved.velocity_modal_metres_per_second.begin(),saved.velocity_modal_metres_per_second.end(),v_.begin());
        elapsed_=saved.samples_elapsed;last_=saved.last_pickup_linear;return true;
    }
    // Off callback, preserving the same geometry/constraints/state/event/time.
    // The complete basis is projected with the new physical mass inner product.
    bool replace_material(PreparedPhysicalBody next,std::uint64_t expected_revision) {
        const auto& before=prepared_.input_; const auto& after=next.input_;
        if (expected_revision!=body_revision() || after.body_revision<=body_revision()
                || before.event_ref!=after.event_ref || before.subject_ref!=after.subject_ref
                || before.state_ref!=after.state_ref || before.geometry_ref!=after.geometry_ref
                || before.geometry_revision!=after.geometry_revision || before.nodes.size()!=after.nodes.size()
                || before.edges.size()!=after.edges.size() || before.sample_rate!=after.sample_rate
                || before.source_coordinate!=after.source_coordinate || before.pratibimba!=after.pratibimba
                || before.family!=after.family || before.source_generation!=after.source_generation
                || before.source_revision!=after.source_revision
                || before.exciter.axis!=after.exciter.axis || before.exciter.node_weights!=after.exciter.node_weights
                || before.pickup.axis!=after.pickup.axis || before.pickup.node_weights!=after.pickup.node_weights
                || before.pickup_linear_per_metre!=after.pickup_linear_per_metre) return false;
        for (std::size_t n=0;n<before.nodes.size();++n)
            if (before.nodes[n].identity!=after.nodes[n].identity || before.nodes[n].rest_metres!=after.nodes[n].rest_metres
                    || before.nodes[n].fixed!=after.nodes[n].fixed || before.nodes[n].constituent!=after.nodes[n].constituent
                    || before.nodes[n].additional_mass_kg!=after.nodes[n].additional_mass_kg) return false;
        for (std::size_t e=0;e<before.edges.size();++e) if (before.edges[e].first!=after.edges[e].first
                || before.edges[e].second!=after.edges[e].second || before.edges[e].section_m2!=after.edges[e].section_m2
                || before.edges[e].prestress_newtons!=after.edges[e].prestress_newtons) return false;
        std::vector<Vec3> displacement(before.nodes.size(),Vec3{0,0,0}),velocity=displacement;
        for (std::size_t n=0;n<before.nodes.size();++n) for (std::size_t m=0;m<q_.size();++m) for (unsigned a=0;a<3;++a) {
            displacement[n][a]+=prepared_.shapes_[m][n][a]*q_[m]; velocity[n][a]+=prepared_.shapes_[m][n][a]*v_[m];
        }
        std::vector<double> q(next.mode_count(),0),v=q;
        for (std::size_t m=0;m<q.size();++m) for (std::size_t n=0;n<before.nodes.size();++n) for (unsigned a=0;a<3;++a) {
            q[m]+=next.node_mass_[n]*next.shapes_[m][n][a]*displacement[n][a];
            v[m]+=next.node_mass_[n]*next.shapes_[m][n][a]*velocity[n][a];
        }
        // Validate the whole candidate before touching resident state.
        PhysicalBody candidate(std::move(next)); candidate.q_=std::move(q); candidate.v_=std::move(v);
        if (!candidate.admissible(candidate.q_,candidate.v_)) return false;
        candidate.elapsed_=elapsed_; candidate.last_=0;
        for (std::size_t m=0;m<candidate.q_.size();++m) candidate.last_+=candidate.q_[m]*candidate.prepared_.pickup_[m];
        *this=std::move(candidate); return true;
    }
    // An actual M3 form operation can replace geometry. This is explicit
    // control-thread work, not an automatic event/clock/material reset. Retained
    // node IDs are the only correspondence authority; no nearest-neighbour map.
    // Work added/removed by changed constraints or elasticity is reported.
    bool replace_form(PreparedPhysicalBody next,std::uint64_t expected_revision,
            std::uint64_t expected_sample,PhysicalFormTransition policy,
            PhysicalFormTransitionReceipt& receipt) {
        const auto& before=prepared_.input_; const auto& after=next.input_;
        if (expected_revision!=body_revision() || expected_sample!=elapsed_
                || after.body_revision<=body_revision() || after.source_generation<before.source_generation
                || after.event_ref!=before.event_ref || after.subject_ref!=before.subject_ref
                || after.state_ref!=before.state_ref || after.sample_rate!=before.sample_rate
                || (policy!=PhysicalFormTransition::ProjectCorrespondingNodes && policy!=PhysicalFormTransition::ExplicitReset)) return false;
        const double energy=mechanical_energy_joules(); const auto revision=body_revision();
        PhysicalBody candidate(std::move(next));
        if (policy==PhysicalFormTransition::ProjectCorrespondingNodes) {
            // All destination nodes require exact source identity. Extra or
            // removed nodes need a separately declared mapping/reset policy.
            if (before.nodes.size()!=candidate.prepared_.input_.nodes.size()) return false;
            std::vector<Vec3> displacement(before.nodes.size(),Vec3{0,0,0}),velocity=displacement;
            for (std::size_t node=0;node<before.nodes.size();++node) {
                if (before.nodes[node].identity!=candidate.prepared_.input_.nodes[node].identity) return false;
                for (std::size_t m=0;m<q_.size();++m) for (unsigned axis=0;axis<3;++axis) {
                    displacement[node][axis]+=q_[m]*prepared_.shapes_[m][node][axis];
                    velocity[node][axis]+=v_[m]*prepared_.shapes_[m][node][axis];
                }
            }
            for (std::size_t m=0;m<candidate.q_.size();++m) for (std::size_t node=0;node<before.nodes.size();++node)
                for (unsigned axis=0;axis<3;++axis) {
                    const double shape=candidate.prepared_.shapes_[m][node][axis]*candidate.prepared_.node_mass_[node];
                    candidate.q_[m]+=shape*displacement[node][axis]; candidate.v_[m]+=shape*velocity[node][axis];
                }
        }
        if (!candidate.admissible(candidate.q_,candidate.v_)) return false;
        candidate.elapsed_=elapsed_; candidate.last_=0;
        for (std::size_t m=0;m<candidate.q_.size();++m) candidate.last_+=candidate.q_[m]*candidate.prepared_.pickup_[m];
        const double next_energy=candidate.mechanical_energy_joules();
        if (!std::isfinite(next_energy) || !std::isfinite(candidate.last_)) return false;
        const PhysicalFormTransitionReceipt result{revision,candidate.body_revision(),elapsed_,policy,energy,next_energy,next_energy-energy};
        *this=std::move(candidate); receipt=result; return true;
    }
};
} // namespace ql
#endif
