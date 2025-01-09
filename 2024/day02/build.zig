const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const lib_mod = b.createModule(.{
        .root_source_file = b.path("../lib/aoc/src/root.zig"),
        .target = target,
        .optimize = optimize,
    });

    const mecha_mod = b.createModule(.{
        .root_source_file = b.path("../lib/mecha/mecha.zig"),
        .target = target,
        .optimize = optimize,
    });

    const part1_mod = b.createModule(.{
        .root_source_file = b.path("src/part1.zig"),
        .target = target,
        .optimize = optimize,
    });
    const part2_mod = b.createModule(.{
        .root_source_file = b.path("src/part2.zig"),
        .target = target,
        .optimize = optimize,
    });

    part1_mod.addImport("aoc_lib", lib_mod);
    part2_mod.addImport("aoc_lib", lib_mod);
    part2_mod.addImport("mecha", mecha_mod);

    const part1 = b.addExecutable(.{
        .name = "day2",
        .root_module = part1_mod,
    });
    const part2 = b.addExecutable(.{
        .name = "day2",
        .root_module = part2_mod,
    });

    b.installArtifact(part1);
    b.installArtifact(part2);

    const install_part1 = b.addInstallArtifact(part1, .{});
    const install_part2 = b.addInstallArtifact(part2, .{});

    const part1_cmd = b.addRunArtifact(part1);
    const part2_cmd = b.addRunArtifact(part2);

    part1_cmd.step.dependOn(&install_part1.step);
    part2_cmd.step.dependOn(&install_part2.step);

    if (b.args) |args| {
        part1_cmd.addArgs(args);
        part2_cmd.addArgs(args);
    }

    // TODO: properly separate part1 and part2 to have their own commands
    const part1_step = b.step("part1", "Run the part1 app");
    const part2_step = b.step("part2", "Run the part2 app");
    part1_step.dependOn(&part1_cmd.step);
    part2_step.dependOn(&part2_cmd.step);

    const part1_unit_tests = b.addTest(.{
        .root_module = part1_mod,
    });
    const part2_unit_tests = b.addTest(.{
        .root_module = part2_mod,
    });

    const run_part1_unit_tests = b.addRunArtifact(part1_unit_tests);
    const run_part2_unit_tests = b.addRunArtifact(part2_unit_tests);

    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_part1_unit_tests.step);
    test_step.dependOn(&run_part2_unit_tests.step);
}
