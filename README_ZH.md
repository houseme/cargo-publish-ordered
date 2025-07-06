# Cargo-Publish-Ordered

[English](README.md) | 简体中文

一个 Cargo 子命令，用于按拓扑顺序发布 Cargo 工作区 (workspace) 中的包。

当您的工作区中存在本地路径依赖时，这个工具非常有用。它能确保在发布依赖于它们的包之前，先发布它们的依赖项，从而避免因依赖版本未找到而导致的发布失败。

## 功能

* **自动解析工作区**：自动读取 `Cargo.toml` 并解析工作区内的所有包。
* **依赖图构建**：根据包之间的路径依赖关系，构建一个有向无环图 (DAG)。
* **拓扑排序**：计算出正确的发布顺序，确保依赖项总是先于依赖它的包被发布。
* **安全演练 (Dry Run)**：提供 `--dry-run` 标志，可以在不实际发布到 crates.io 的情况下，预览将要执行的发布命令和顺序。
* **实际发布**：在验证顺序无误后，执行 `cargo publish` 命令将所有包发布。

## 安装

当项目发布到 crates.io 后，您可以通过以下命令安装：

```bash
cargo install cargo-publish-ordered
```

或者，您可以从源码安装：

```bash
git clone https://github.com/houseme/cargo-publish-ordered.git # 请替换为您的仓库地址
cd cargo-publish-ordered
cargo install --path .
```

## 使用方法

导航到您的 Cargo 工作区根目录。

### 演练 (Dry Run)

建议先执行一次演练来检查发布顺序和将被执行的命令：

```bash
cargo publish-ordered --dry-run
```

### 正式发布

确认顺序无误后，执行发布命令。您可能需要提供 crates.io 的令牌。

```bash
cargo publish-ordered --token ${CRATES_IO_TOKEN}
```

## 贡献

欢迎任何形式的贡献！如果您发现 bug 或有功能建议，请随时创建 Issue。如果您想贡献代码，请 Fork 本仓库并提交 Pull Request。

## 许可证

本项目采用 [Apache-2.0](LICENSE) 许可。