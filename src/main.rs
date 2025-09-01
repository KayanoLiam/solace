use clap::{Parser, Subcommand};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

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

    // 创建项目目录
    fs::create_dir_all(root.join("src")).expect("Failed to create project directory");

    // 创建 src/main.zig
    let mut main_zig = File::create(root.join("src/main.zig")).unwrap();
    writeln!(
        main_zig,
        r#"const std = @import("std");

pub fn main() void {{
    std.debug.print("Hello, {0}!\n", .{{ "{0}" }});
}}"#,
        project_name
    )
    .unwrap();

    // 创建 build.zig.zon 配置文件
    let mut build_zon = File::create(root.join("build.zig.zon")).unwrap();
    writeln!(
        build_zon,
        r#"{{
    .name = "{0}",
    .version = "0.0.1",
}}"#,
        project_name
    )
    .unwrap();

    // 创建 solace.lock
    let mut lock = File::create(root.join("solace.lock")).unwrap();
    writeln!(lock, "# Solace lock file for project '{}'", project_name).unwrap();

    // 创建 build.zig
    let mut build_zig = File::create(root.join("build.zig")).unwrap();
    writeln!(
        build_zig,
        r#"const std = @import("std");

pub fn build(b: *std.build.Builder) void {{
    const mode = b.standardReleaseOptions();
    const target = b.standardTargetOptions(.{});
    const exe = b.addExecutable("{0}", "src/main.zig");
    exe.setBuildMode(mode);
    exe.setTarget(target);
    b.installArtifact(exe);
}}"#,
        project_name
    )
    .unwrap();

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
