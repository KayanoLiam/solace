# Solace Rust - Zig包管理工具

## 项目概述

Solace Rust是一个用Rust编写的Zig包管理工具，旨在为Zig项目提供类似Cargo的包管理功能。该项目目前处于早期开发阶段，主要提供项目初始化和依赖管理功能。

### 核心功能
- **项目初始化**: 创建新的Zig项目结构
- **依赖管理**: 管理Zig项目的依赖关系
- **构建集成**: 与Zig的构建系统集成

## 项目结构

```
solace_rust/
├── src/
│   ├── main.rs          # 主程序入口
│   ├── utils.rs         # 工具函数
│   └── commands/        # 命令模块
│       ├── init.rs      # 初始化命令
│       ├── add.rs       # 添加依赖命令
│       └── list.rs      # 列出依赖命令
├── Cargo.toml           # Rust项目配置
├── README.md           # 项目文档
└── target/             # 构建输出目录
```

## 技术栈

- **语言**: Rust 2024 edition
- **CLI框架**: clap 4.5 (带derive特性)
- **序列化**: serde + serde_json
- **随机数**: rand 0.8

## 构建和运行

### 开发环境设置
```bash
# 安装Rust (如果尚未安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 克隆项目
git clone https://github.com/KayanoLiam/solace.git
cd solace_rust
```

### 构建项目
```bash
# 调试构建
cargo build

# 发布构建
cargo build --release
```

### 运行项目
```bash
# 直接运行
cargo run -- init

# 使用构建后的二进制文件
./target/debug/solace init
./target/release/solace init
```

### 测试
```bash
# 运行测试套件
cargo test

# 运行特定测试
cargo test test_name
```

## 使用方法

### 初始化新项目
```bash
solace init
```

该命令会创建以下Zig项目结构：
```
my_zig_project/
├── src/
│   ├── main.zig    # 主程序文件
│   └── root.zig    # 库根文件
├── build.zig       # 构建配置
├── build.zig.zon   # 包配置
└── solace.lock     # 锁文件
```

### 添加依赖
```bash
# TODO: 待实现
solace add <package-name>
```

### 列出依赖
```bash
# TODO: 待实现
solace list
```

## 开发约定

### 代码风格
- 遵循Rust标准代码风格
- 使用clap的derive特性进行CLI参数解析
- 错误处理使用expect进行快速原型开发

### 文件组织
- 命令相关的代码放在`src/commands/`目录下
- 工具函数放在`src/utils.rs`
- 每个命令对应一个独立的模块文件

### 构建配置
- 使用Cargo.toml管理Rust依赖
- 生成的Zig项目使用build.zig.zon进行包管理
- 支持Zig 0.16.0-dev.43+版本

## 项目状态

- ✅ 基础项目结构
- ✅ 初始化命令实现
- 🔄 添加依赖命令 (待完善)
- 🔄 列出依赖命令 (待完善)
- 📋 测试套件 (待添加)
- 📋 文档完善 (进行中)

## 贡献指南

1. Fork项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建Pull Request

## 许可证

MIT License - 详见项目根目录的LICENSE文件