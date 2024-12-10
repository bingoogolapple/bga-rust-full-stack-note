# bga-rust-full-stack-note

Rust 全栈开发学习笔记

- [官网](https://www.rust-lang.org/zh-CN/learn/get-started)
- 国内镜像代理: [RsProxy](https://rsproxy.cn)
- 安装 Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- 安装完成后在 `~/.cargo/bin` 目录中包含以下命令

  - [cargo](https://doc.rust-lang.org/cargo/index.html)
    - Rust 的构建工具和包管理器
    - 查看 Cargo 版本: `cargo --version`
    - 创建新的二进制项目: `cargo new --bin rust-binary` 或 `cargo new rust-binary`
    - 创建新的库项目: `cargo new --lib rust-library`
    - 仅构建项目: 执行 `cargo build` 会生成 debug 模式的二进制可执行文件 `./target/debug/rust-binary`，然后执行 `./target/debug/rust-binary` 即可运行生成的可执行文件
      - 执行 `cargo build -r` 会以 release 模式的生成二进制可执行文件 `./target/release/rust-binary`
    - 构建并运行项目: `cargo run`
    - 测试项目: `cargo test`
    - 更新依赖库到最新版: `cargo update`
    - 检查代码是否可编译: `cargo check` 该命令快速检查代码确保其可以编译，但并不产生二进制可执行文件
    - 为项目构建文档: `cargo doc --open`
    - 发布库到[crates.io](https://crates.io/): `cargo publish`，在 Rust 中，通常把包称作 crate
    - 向当前 Rust 项目的 Cargo.toml 文件添加新的依赖: `cargo add cratename`
    - 全局安装 Rust 二进制文件: `cargo install cratename`
  - cargo-miri
  - [rust-analyzer](https://github.com/rust-lang/rust-analyzer)
    - A Rust compiler front-end for IDEs
    - 开发工具
      - VS Code
      - JetBrains 全家桶中的 [RustRover](https://www.jetbrains.com/rust/)
    - VS Code 中的插件名称也叫 rust-analyzer
  - rust-lldb
  - [rustfmt](https://github.com/rust-lang/rustfmt)
    - 用于自动格式化 Rust 代码
  - [cargo-clippy](https://github.com/rust-lang/rust-clippy)
    - 代码检查 lint 工具，通过静态分析，来检查代码中有问题或不符合指定规范的代码
  - clippy-driver
  - rust-gdb
  - rustc
    - 查看 rustc 版本: `rustc --version`
    - 编译生成二进制文件: 执行 `rustc -o main src/main.rs` 后会生成二进制可执行文件 main，然后执行 `./main` 即可运行生成的可执行文件
    - 编译生成库文件：执行 `rustc --crate-type lib src/lib.rs` 后会生成库文件 liblib.rlib
  - [rustup](https://rust-lang.github.io/rustup/)
    - Rust 安装器和版本管理工具: 用于管理不同平台下的 Rust 构建版本并使其互相兼容， 支持安装由 Beta 和 Nightly 频道发布的版本，并支持其他用于交叉编译的编译版本
    - 升级 Rust: `rustup update`
    - 查看 rustup 版本: `rustup --version`
    - 卸载 Rust: `rustup self uninstall`
    - 打开本地的 Rust 文档: `rustup doc`
    - 显示当前安装的 Rust 版本信息（包括默认的工具链、安装的工具链列表、默认的目标平台等）: `rustup show`
  - cargo-fmt
  - rls
  - rust-gdbgui
  - rustdoc
    - 生成 API 文档，还可以通过 [docs.rs](https://docs.rs/) 在线获取公开的 crates 文档

- 项目结构
  - Cargo.toml: Rust 的清单文件，其中包含了项目的元数据和依赖库
  - Cargo.lock: 由 Cargo 自动生成的，用于记录项目依赖的精确版本信息。当你构建项目时，Cargo 会查看 Cargo.lock 文件来确定应该使用哪个版本的依赖库。如果 Cargo.lock 文件不存在，Cargo 会在构建项目时创建它，并在其中记录当前使用的依赖库版本。如果 Cargo.lock 文件已经存在，Cargo 会使用文件中记录的版本，以确保构建的一致性。这意味着，当你首次构建项目时，Cargo 会解析 Cargo.toml 文件，下载依赖库的最新版本，并将这些信息记录在 Cargo.lock 文件中。在后续的构建中，即使依赖库发布了新版本，Cargo 也会继续使用 Cargo.lock 文件中记录的版本。如果你想更新依赖库到最新版本，你可以运行 cargo update 命令，这会使 Cargo 更新 Cargo.lock 文件中的版本信息
  - src: 源码目录
    - main.rs: 二进制项目入库文件
    - lib.rs: 库项目入库文件
  - target: 构建输出目录
    - debug: 开发模式构建输出目录
      - .fingerprint: TODO
      - build: TODO
      - deps: TODO
      - examples: TODO
      - incremental: TODO
      - .cargo-lock: TODO
      - rust-binary: TODO
      - rust-binary.d: TODO
      - librust_library.d: TODO
      - librust_library.rlib: TODO
    - release: 发布模式构建输出目录
      - .fingerprint: TODO
      - build: TODO
      - deps: TODO
      - examples: TODO
      - incremental: TODO
      - .cargo-lock: TODO
      - rust-binary: TODO
      - rust-binary.d: TODO
      - librust_library.d: TODO
      - librust_library.rlib: TODO
    - doc: 执行 `cargo doc` 后生成的项目文档
    - .rustc_info.json: TODO
    - CACHEDIR.TAG: TODO

## 作者联系方式

| 个人主页 | 邮箱 |
| ------------- | ------------ |
| <a  href="https://www.bingoogolapple.cn" target="_blank">bingoogolapple.cn</a>  | <a href="mailto:bingoogolapple@gmail.com" target="_blank">bingoogolapple@gmail.com</a> |

| 个人微信号 | 微信群 | 公众号 |
| ------------ | ------------ | ------------ |
| <img width="180" alt="个人微信号" src="https://github.com/bingoogolapple/bga-god-assistant-config/raw/main/images/BGAQrCode.png"> | <img width="180" alt="微信群" src="https://github.com/bingoogolapple/bga-god-assistant-config/raw/main/images/WeChatGroup1QrCode.jpg"> | <img width="180" alt="公众号" src="https://github.com/bingoogolapple/bga-god-assistant-config/raw/main/images/GongZhongHao.png"> |

| 个人 QQ 号 | QQ 群 |
| ------------ | ------------ |
| <img width="180" alt="个人 QQ 号" src="https://github.com/bingoogolapple/bga-god-assistant-config/raw/main/images/BGAQQQrCode.jpg"> | <img width="180" alt="QQ 群" src="https://github.com/bingoogolapple/bga-god-assistant-config/raw/main/images/QQGroup1QrCode.jpg"> |

## 打赏支持作者

如果您觉得 BGA 系列开源库或工具软件帮您节省了大量的开发时间，可以扫描下方的二维码打赏支持。您的支持将鼓励我继续创作，打赏后还可以加我微信免费开通一年 [上帝小助手浏览器扩展/插件开发平台](https://github.com/bingoogolapple/bga-god-assistant-config) 的会员服务

| 微信 | QQ | 支付宝 |
| ------------- | ------------- | ------------- |
| <img width="180" alt="微信" src="https://github.com/bingoogolapple/bga-god-assistant-config/raw/main/images/donate-wechat.jpg"> | <img width="180" alt="QQ" src="https://github.com/bingoogolapple/bga-god-assistant-config/raw/main/images/donate-qq.jpg"> | <img width="180" alt="支付宝" src="https://github.com/bingoogolapple/bga-god-assistant-config/raw/main/images/donate-alipay.jpg"> |

## 作者项目推荐

* 欢迎您使用我开发的第一个独立开发软件产品 [上帝小助手浏览器扩展/插件开发平台](https://github.com/bingoogolapple/bga-god-assistant-config)
