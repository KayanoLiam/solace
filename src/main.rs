use clap::{Parser, Subcommand};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use rand::Rng;

#[derive(Parser)]
#[command(name = "solace")]
#[command(author = "Your Name <youremail@example.com>")]
#[command(version = "1.0")]
#[command(about = "A CLI tool for zig")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Init,
}

fn init() {
    let project_name = "my_zig_project";
    let root = Path::new(project_name);

    std::fs::create_dir_all(root.join("src")).expect("Failed to create project directory");

    // 创建 src/main.zig
    let mut main_zig = File::create(root.join("src/main.zig")).unwrap();
    writeln!(
        main_zig,
        r#"const std = @import("std");
const root = @import("root");

pub fn main() void {{
    std.debug.print("Hello, {0}!\n", .{{ "{0}" }});
}}"#,
        project_name
    )
    .unwrap();

    // 创建 src/root.zig（使用你提供的内容）
    let root_content = r#"//! By convention, root.zig is the root source file when making a library.
const std = @import("std");

pub fn bufferedPrint() !void {
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    try stdout.print("Run `zig build test` to run the tests.\n", .{});
    try stdout.flush();
}

pub fn add(a: i32, b: i32) i32 {
    return a + b;
}

test "basic add functionality" {
    try std.testing.expect(add(3, 7) == 10);
}
"#;

    let mut root_zig = File::create(root.join("src/root.zig")).unwrap();
    root_zig.write_all(root_content.as_bytes()).unwrap();

    // build.zig
    let mut build_zig = File::create(root.join("build.zig")).unwrap();
    writeln!(
        build_zig,
        r#"const std = @import("std");

pub fn build(b: *std.build.Builder) void {{
    const target = b.standardTargetOptions(.{{}});
    const mode = b.standardReleaseOptions();

    const exe = b.addExecutable("{0}", "src/main.zig");
    exe.setBuildMode(mode);
    exe.setTarget(target);

    b.installArtifact(exe);
}}"#,
        project_name
    )
    .unwrap();

    // 随机生成 fingerprint
    let fingerprint: u64 = rand::thread_rng().r#gen(); 

    // build.zig.zon
    let mut build_zon = File::create(root.join("build.zig.zon")).unwrap();
    writeln!(
        build_zon,
        r#".{{
    .name = .{0},
    .version = "0.0.0",
    .fingerprint = 0x{1:x},
    .minimum_zig_version = "0.16.0-dev.43+99b2b6151",
    .dependencies = .{{}},
    .paths = .{{
        "build.zig",
        "build.zig.zon",
        "src",
    }},
}}"#,
        project_name, fingerprint
    )
    .unwrap();

    // solace.lock
    let mut lock = File::create(root.join("solace.lock")).unwrap();
    writeln!(lock, "# Solace lock file for project '{}'", project_name).unwrap();

    println!("Project '{}' initialized successfully!", project_name);
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Init => {
            init();
        }
    }
    Ok(())
}
