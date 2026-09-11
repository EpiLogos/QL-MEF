//! Exercise the producer through the shipped C++ header/static-library boundary.
//! No source-tree include or direct compilation of C implementation is permitted
//! in the consumer: successful in-tree C parity is not installed ABI acceptance.
use std::{fs, path::PathBuf, process::Command};

fn checked(command: &mut Command) {
    let result = command.output().expect("start native consumer command");
    assert!(
        result.status.success(),
        "{command:?}\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
}

struct World(PathBuf);
impl Drop for World {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn installed_cpp_correspondence_and_pitch_api_preserves_source_identity_and_failure_outputs() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let directory = std::env::temp_dir().join(format!("ql-m2-installed-{}", std::process::id()));
    // create_dir intentionally refuses to reuse a stale or concurrent World.
    fs::create_dir(&directory).expect("fresh installed-consumer World");
    let world = World(directory);
    let prefix = world.0.join("package");
    checked(
        Command::new("make")
            .arg("-C")
            .arg(root.join("c"))
            .arg("install")
            .arg(format!("BUILD_DIR={}", world.0.join("build").display()))
            .arg(format!("PREFIX={}", prefix.display())),
    );
    let source = world.0.join("consumer.cpp");
    fs::write(
        &source,
        r##"#include <ql/m2.h>
#include <cassert>
#include <cmath>
#include <cstring>
#include <type_traits>
static_assert(std::is_standard_layout<QL_M2_Correspondence>::value);
int main() {
    assert(ql_m2_table(QL_M2_MEF_TABLE)->row_count == 72);
    const auto *r = ql_m2_correspondence(2, 0);
    assert(r && r->maqam_id && r->planet_id && r->chakra_id);
    assert(r->maqam_id == ql_m_resolve("#2-4.3-0-3")->id);
    assert(r->musical_relation_id && r->planetary_relation_id);
    assert(r->fibre == 1 && r->colour_name && std::strcmp(r->colour_name, "red") == 0);
    double pitch = -17.0;
    assert(ql_m2_correspondence_pitch(2, 0, 1, 0, 220.0, &pitch) == QL_M2_OK);
    assert(pitch == 220.0);
    assert(ql_m2_correspondence_pitch(2, 0, 1, 7, 220.0, &pitch) == QL_M2_OK);
    assert(std::abs(pitch - 440.0) < 1e-10);
    const double prior = pitch;
    assert(ql_m2_correspondence_pitch(2, 0, 1, 8, 220.0, &pitch) == QL_M2_INVALID);
    assert(pitch == prior);
    assert(ql_m2_correspondence_pitch(2, 0, 1, 0, NAN, &pitch) == QL_M2_NONFINITE);
    assert(pitch == prior);
    assert(ql_m2_correspondence_pitch(2, 0, 1, 0, 220.0, nullptr) == QL_M2_INVALID);
    assert(ql_m2_correspondence(72, 0) == nullptr);
    assert(ql_m2_correspondence(2, 2) == nullptr);
    const auto count = ql_m2_correspondence_count();
    assert(count > 0 && ql_m2_correspondence_at(count) == nullptr);
    for (std::size_t i = 0; i < count; ++i) {
        const auto *row = ql_m2_correspondence_at(i);
        assert(row && row == ql_m2_correspondence(row->maqam_index, row->role));
        assert(row->musical_relation_id && row->planetary_relation_id);
    }
}
"##,
    )
    .expect("write independent C++ consumer");
    let binary = world.0.join("consumer");
    checked(
        Command::new(std::env::var("CXX").unwrap_or_else(|_| "c++".into()))
            .current_dir(&world.0)
            .args(["-std=c++17", "-Wall", "-Wextra", "-Werror", "-pedantic"])
            .arg(format!("-I{}", prefix.join("include").display()))
            .arg(&source)
            .arg(format!("-L{}", prefix.join("lib").display()))
            .args(["-lql-mef-c", "-lm", "-o"])
            .arg(&binary),
    );
    checked(Command::new(&binary).current_dir(&world.0));
}
