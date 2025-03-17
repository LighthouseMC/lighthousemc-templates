const std = @import("std");


pub fn build(b: *std.Build) void {
    const exe = b.addExecutable(.{
        .name    = "myplot",
        .version = .{ .major = 0, .minor = 1, .patch = 0 },

        .root_source_file = b.path("src/lib.zig"),
        .target           = b.resolveTargetQuery(.{ .cpu_arch = .wasm32, .os_tag = .freestanding }),
        .optimize         = .ReleaseSmall,
    });
    exe.entry    = .disabled;
    exe.rdynamic = true;

    b.installArtifact(exe);
}
