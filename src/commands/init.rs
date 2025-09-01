use std::fs;
use std::path::Path;
use crate::utils::write_file;

pub fn run() {
    if Path::new("build.zig.zon").exists() {
        eprintln!("Error: build.zig.zon already exists.");
        std::process::exit(1);
    }

    // 创建目录
    fs::create_dir_all("src").expect("Failed to create src folder");
    fs::create_dir_all("deps").expect("Failed to create deps folder");

    // 创建 main.zig
    let main_zig = r#"const std = @import("std");

pub fn main() void {
    std.debug.print("Hello, Solace!\n", .{});
}"#;

    write_file("src/main.zig", main_zig);

    // 创建 build.zig.zon
    let build_data = r#"{
  "dependencies": {}
}"#;

    write_file("build.zig.zon", build_data);

    // 创建 solace.lock
    fs::File::create("solace.lock").expect("Failed to create solace.lock");

    println!("✅ Project structure created successfully.");
}
