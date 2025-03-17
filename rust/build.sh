# Move to the correct directory.
cd /root/shared/myplot

# Build the project. If it fails, exit.
cargo build --target wasm32-unknown-unknown || exit 1

# Move the output file to the correct location.
mv /root/shared/myplot/target/wasm32-unknown-unknown/debug/*.wasm /root/shared/out.wasm
