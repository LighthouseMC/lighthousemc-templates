# This is where your code is. This is most likely necessary.
cd /root/shared/code



# Add your script for building a WASM binary here.

# This build script will be run on a virtual machine running Alpine Linux 3.21 or above.
#  https://alpinelinux.org/

# The resulting code is run inside of a WASM container with heavily restricted permissions.
#  Threads, internet access, etc are disallowed.

# If the build fails, exit with a non-zero status.

# Packages are available at
#  https://pkgs.alpinelinux.org/packages

# The build is allowed to take a maximum of 10 seconds before it is cancelled.


# You might want to start by looking here:
#  https://webassembly.org/getting-started/developers-guide/

# If you add support for a new language, consider contributing a PR at
#  https://github.com/LighthouseMC/LighthouseTemplates



# Move the output file to the correct location.
# As soon as the target file exists, the build is considered to be completed and the VM is shut down.
mv /root/shared/code/path/to/out.wasm /root/shared/out.wasm 
#  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
# Replace this path with the location of the output compiled WASM binary.
