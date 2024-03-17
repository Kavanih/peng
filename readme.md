admin = add admin address


run cargo build

docker run --rm -v "$(pwd)":/code   --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry   cosmwasm/rust-optimizer:0.15.0

Post-processing artifacts...
df64791ae3e2e96558f4b42201b9ed92b11b0d57ee12fd33d9a27c936a45305f  bluemove_launchpad.wasm
done