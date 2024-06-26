#!/usr/bin/env bash
function generate_did() {
    local canister=$1
    canister_root="src/$canister"

    cargo build --manifest-path="$canister_root/Cargo.toml" \
    --target wasm32-unknown-unknown \
    --release --package "$canister"

    candid-extractor "target/wasm32-unknown-unknown/release/$canister.wasm" > "$canister_root/$canister.did"
}



# List of canisters to generate candid files for 
# (comma separated list of canister names)

CANISTERS=ic_stable_structure_implementation_backend

for canister in $(echo $CANISTERS | sed "s/,/ /g")
do
  generate_did "$canister"
done