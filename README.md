cargo-publish-ordered/
├── Cargo.toml # 项目元数据和依赖
├── src/
│ ├── main.rs # 程序入口，处理命令行参数和调用核心逻辑
│ ├── cli.rs # 定义命令行接口 (使用 clap)
│ ├── workspace.rs # 负责解析 Cargo 工作区和构建依赖图
│ ├── publisher.rs # 负责执行发布流程 (dry-run, publish)
│ └── error.rs # 定义自定义错误类型
└── .github/
└── workflows/
└── release.yml # 使用我们自己的工具来发布自己