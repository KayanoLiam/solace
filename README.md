# Solace - Zig Package Manager

> A modern package manager for the Zig programming language, written in Rust.

[🇺🇸 English](#english) | [🇨🇳 中文](#中文) | [🇯🇵 日本語](#日本語)

---

## English

### 🚀 Quick Start

```bash
# Install solace
cargo install solace_rust

# Create a new Zig project
solace init my_project
cd my_project

# Build and run
zig build run
```

### 📦 Installation

#### From Source
```bash
git clone https://github.com/KayanoLiam/solace.git
cd solace_rust
cargo install --path .
```

#### From Crates.io (Coming Soon)
```bash
cargo install solace_rust
```

### 🎯 Features

- **🎉 Project Initialization**: Create new Zig projects with sensible defaults
- **📚 Dependency Management**: Add, remove, and update Zig packages
- **🔒 Lock Files**: Ensure reproducible builds with `solace.lock`
- **⚡ Fast & Lightweight**: Built in Rust for maximum performance
- **🛠️ Build Integration**: Seamless integration with Zig's build system

### 📋 Commands

| Command | Description |
|---------|-------------|
| `solace init [name]` | Initialize a new Zig project |
| `solace add <package>` | Add a dependency |
| `solace remove <package>` | Remove a dependency |
| `solace list` | List all dependencies |
| `solace update` | Update dependencies |

### 🏗️ Project Structure

```
my_project/
├── src/
│   ├── main.zig    # Main application entry
│   └── root.zig    # Library root
├── build.zig       # Build configuration
├── build.zig.zon   # Package configuration
└── solace.lock     # Dependency lock file
```

### 🔧 Development

```bash
# Clone the repository
git clone https://github.com/KayanoLiam/solace.git
cd solace_rust

# Build for development
cargo build

# Run tests
cargo test

# Install locally
cargo install --path .
```

### 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 中文

### 🚀 快速开始

```bash
# 安装 solace
cargo install solace_rust

# 创建新的 Zig 项目
solace init my_project
cd my_project

# 构建并运行
zig build run
```

### 📦 安装

#### 从源码安装
```bash
git clone https://github.com/KayanoLiam/solace.git
cd solace_rust
cargo install --path .
```

#### 从 Crates.io 安装（即将推出）
```bash
cargo install solace_rust
```

### 🎯 特性

- **🎉 项目初始化**：使用合理的默认值创建新的 Zig 项目
- **📚 依赖管理**：添加、删除和更新 Zig 包
- **🔒 锁定文件**：通过 `solace.lock` 确保可重现的构建
- **⚡ 快速轻量**：使用 Rust 构建，性能卓越
- **🛠️ 构建集成**：与 Zig 构建系统无缝集成

### 📋 命令

| 命令 | 描述 |
|---------|-------------|
| `solace init [name]` | 初始化新的 Zig 项目 |
| `solace add <package>` | 添加依赖 |
| `solace remove <package>` | 移除依赖 |
| `solace list` | 列出所有依赖 |
| `solace update` | 更新依赖 |

### 🏗️ 项目结构

```
my_project/
├── src/
│   ├── main.zig    # 主应用程序入口
│   └── root.zig    # 库根目录
├── build.zig       # 构建配置
├── build.zig.zon   # 包配置
└── solace.lock     # 依赖锁定文件
```

### 🔧 开发

```bash
# 克隆仓库
git clone https://github.com/KayanoLiam/solace.git
cd solace_rust

# 开发构建
cargo build

# 运行测试
cargo test

# 本地安装
cargo install --path .
```

### 🤝 贡献

1. Fork 这个仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开 Pull Request

### 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

---

## 日本語

### 🚀 クイックスタート

```bash
# solace をインストール
cargo install solace_rust

# 新しい Zig プロジェクトを作成
solace init my_project
cd my_project

# ビルドして実行
zig build run
```

### 📦 インストール

#### ソースからインストール
```bash
git clone https://github.com/KayanoLiam/solace.git
cd solace_rust
cargo install --path .
```

#### Crates.io からインストール（近日公開）
```bash
cargo install solace_rust
```

### 🎯 機能

- **🎉 プロジェクト初期化**：適切なデフォルト値で新しい Zig プロジェクトを作成
- **📚 依存関係管理**：Zig パッケージの追加、削除、更新
- **🔒 ロックファイル**：`solace.lock` で再現可能なビルドを確保
- **⚡ 高速軽量**：Rust で構築され、最大のパフォーマンスを実現
- **🛠️ ビルド統合**：Zig のビルドシステムとのシームレスな統合

### 📋 コマンド

| コマンド | 説明 |
|---------|-------------|
| `solace init [name]` | 新しい Zig プロジェクトを初期化 |
| `solace add <package>` | 依存関係を追加 |
| `solace remove <package>` | 依存関係を削除 |
| `solace list` | すべての依存関係を一覧表示 |
| `solace update` | 依存関係を更新 |

### 🏗️ プロジェクト構造

```
my_project/
├── src/
│   ├── main.zig    # メインアプリケーションエントリ
│   └── root.zig    # ライブラリルート
├── build.zig       # ビルド設定
├── build.zig.zon   # パッケージ設定
└── solace.lock     # 依存関係ロックファイル
```

### 🔧 開発

```bash
# リポジトリをクローン
git clone https://github.com/KayanoLiam/solace.git
cd solace_rust

# 開発ビルド
cargo build

# テストを実行
cargo test

# ローカルインストール
cargo install --path .
```

### 🤝 貢献

1. リポジトリをフォーク
2. 機能ブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを開く

### 📄 ライセンス

このプロジェクトは MIT ライセンスの下でライセンスされています - 詳細は [LICENSE](LICENSE) ファイルを参照してください。

---

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/KayanoLiam/solace/issues)
- **Discussions**: [GitHub Discussions](https://github.com/KayanoLiam/solace/discussions)
- **Email**: Kayano04@outlook.jp

## 🌟 Star History

[![Star History Chart](https://api.star-history.com/svg?repos=KayanoLiam/solace&type=Date)](https://star-history.com/#KayanoLiam/solace&Date)