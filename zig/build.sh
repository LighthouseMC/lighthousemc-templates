# Move to the correct directory.
cd /root/shared/myplot

# Build the project. If it fails, exit.
zig build || exit 1

# Move the output file to the correct location.
mv /root/shared/myplot/zig-out/bin/*.wasm /root/shared/out.wasm
