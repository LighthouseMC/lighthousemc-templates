# Move to the correct directory.
cd /root/shared/code

# Build the project. If it fails, exit.
cargo build --target wasm32-unknown-unknown || exit 1

# Move the output file to the correct location.
mv /root/shared/code/target/wasm32-unknown-unknown/debug/*.wasm /root/shared/out.wasm
