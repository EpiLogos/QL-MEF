#!/bin/bash
# Independent replay of EpiLogos/QL-MEF#251 on Omarchy (Linux) — QL-MEF #135.
set -u
export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH" CARGO_TARGET_DIR=/mnt/hdd/vm/cargo-target
cd ~/Central/Work/Quaternal-Logic && git fetch -q origin feat/m123-engine-binding
W=~/Central/worktrees/ql-m123-replay
[ -d "$W" ] || git worktree add --detach "$W" origin/feat/m123-engine-binding
cd "$W" && git checkout -q --detach origin/feat/m123-engine-binding
echo "HEAD $(git rev-parse HEAD) host $(hostname) $(uname -sm)"
step() { echo "=== $1"; shift; "$@" 2>&1 | tail -${TAILN:-4}; echo "rc=${PIPESTATUS[0]}"; }
step "C++ native acceptance" make -C cpp test install PREFIX="$W/target/k8-cpp"
export QL_FIELD_WORKER="$W/target/k8-cpp/bin/ql-field-worker"
step "k8_reshape (real worker)" cargo test --locked -q -p ql-mef --test k8_reshape -- --ignored
step "k2_instrument (real worker)" cargo test --locked -q -p ql-mef --test k2_instrument -- --ignored
step "k2_nara_reception (real worker)" cargo test --locked -q -p ql-mef --test k2_nara_reception -- --ignored
step "continuous lib" cargo test --locked -q -p ql-mef --lib continuous
step "m2_engine" cargo test --locked -q -p ql-mef --test m2_engine
step "k8_coupled" cargo test --locked -q -p ql-mef --test k8_coupled
step "census" python3 scripts/k8-census.py --check
step "adapter" node --test adapters/retained-field/instrument-session.test.mjs
step "fmt" cargo fmt --all -- --check
TAILN=12 step "install script (sky companion)" sh scripts/oi-source-install.sh
echo '{"schema":"ql.sky-request/v1","epoch":"2026-09-27T12:00:00Z","timezone":"UTC","mode":"historical","perspective":"Apparent Geocentric","zodiac":"Tropical","ayanamsha":null,"observer":null,"max_age_seconds":60,"backend_policy":"allow-moshier"}' > /tmp/replay-sky-req.json
step "ql-sky" sh -c "$CARGO_TARGET_DIR/release/ql-sky /tmp/replay-sky-req.json > /tmp/replay-sky.json && python3 -c \"import json;d=json.load(open('/tmp/replay-sky.json'));print('sun',d['bodies'][0]['longitude_degrees'])\""
echo DONE
