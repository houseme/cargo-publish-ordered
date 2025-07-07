# Cargo-Publish-Ordered

English｜[简体中文](README_ZH.md)

A Cargo subcommand to publish packages in a Cargo workspace in topological order.

This tool is useful when you have local path dependencies in your workspace. It ensures that dependencies are published
before the packages that depend on them, avoiding publishing failures due to missing dependency versions.

## Features

* **Automatic Workspace Parsing**: Automatically reads `Cargo.toml` and parses all packages within the workspace.
* **Dependency Graph Construction**: Builds a directed acyclic graph (DAG) based on the path dependencies between
  packages.
* **Topological Sorting**: Calculates the correct publishing order, ensuring dependencies are always published before
  the packages that depend on them.
* **Safe Dry Run**: Provides a `--dry-run` flag to preview the publishing commands and order without actually publishing
  to crates.io.
* **Flexible Registry Support**: Allows specifying a target registry via the `--registry` flag, defaulting to
  `crates-io` to avoid local configuration issues.
* **Actual Publishing**: After verifying the order, it executes `cargo publish` commands to publish all packages.

## Installation

Once the project is published to crates.io, you can install it with:

```bash
cargo install cargo-publish-ordered
```

Alternatively, you can install from source:

```bash
git clone https://github.com/houseme/cargo-publish-ordered.git
cd cargo-publish-ordered
cargo install --path .
```

## Usage

Navigate to your Cargo workspace root directory.

### Dry Run

It is recommended to perform a dry run first to check the publishing order and the commands that will be executed:

```bash
cargo publish-ordered --dry-run
```

### Publishing

After confirming the order is correct, execute the publish command. You may need to provide a crates.io token. By
default, this tool publishes to `crates-io`.

```bash
cargo publish-ordered --token ${CRATES_IO_TOKEN}
```

### Publishing to a different registry

If you want to publish to an alternative registry (e.g., a private one), use the `--registry` flag:

```bash
cargo publish-ordered --token ${MY_REGISTRY_TOKEN} --registry my-private-registry
```

## Contributing

Contributions of any kind are welcome! If you find a bug or have a feature suggestion, please feel free to create an
Issue. If you want to contribute code, please fork the repository and submit a Pull Request.

## License

This project is dual-licensed under [Apache-2.0](LICENSE) licenses.