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
    fs::create_dir(root).expect("Failed to create project directory");

    fs::create_dir(root.join("src")).expect("Failed to create src directory");

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

    let mut lock = File::create(root.join("solace.lock")).unwrap();
    writeln!(lock, "# Solace lock file for project '{}'", project_name).unwrap();

    let mut build_zig = File::create(root.join("build.zig")).unwrap();
    writeln!(
        build_zig,
        r#"const std = @import("std");

pub fn build(b: *std.build.Builder) void {{
    const exe = b.addExecutable("{0}", "src/main.zig");
    exe.setTarget(b.standardTargetOptions(.{{}}));
    exe.setBuildMode(b.standardReleaseOptions());
    b.installArtifact(exe);
}}"#,
        project_name
    )
    .unwrap();
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
