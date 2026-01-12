# Allocator Project Processing Script

这个脚本自动化处理 `verified-memory-allocator` 项目，将其拆分成较小的自包含 Rust 文件用于测试。

## 功能

该脚本执行以下步骤：

1. **调用 Claude API** - 使用 Claude 分析项目结构并生成拆分策略
2. **拆分文件** - 根据 Claude 的建议将大型文件拆分成较小的文件（目标约 400 行/文件）
3. **内联依赖** - 使用 `inline-crate` 工具消除包依赖
4. **代码标注** - 使用 `line_count` 工具标记代码类型（exec/spec/proof 等）
5. **生成报告** - 创建处理结果的汇总报告

## 前置要求

### 1. Python 依赖

安装 Anthropic Python SDK：

```bash
pip install anthropic
```

### 2. 构建必要工具

确保以下工具已构建：

```bash
# 构建 line_count 工具
cd /home/chuyue/verus/source/tools/line_count
cargo build

# 确保 inline-crate 可用
cd /home/chuyue/inline-crate
cargo build
```

### 3. Anthropic API Key

你需要一个 Anthropic API key。可以从 [Anthropic Console](https://console.anthropic.com/) 获取。

## 使用方法

### 基本用法

```bash
cd /home/chuyue/verus
./process_allocator_project.py --api-key YOUR_API_KEY
```

### 使用环境变量

```bash
export ANTHROPIC_API_KEY="your-api-key-here"
./process_allocator_project.py --api-key $ANTHROPIC_API_KEY
```

### 详细输出模式

```bash
./process_allocator_project.py --api-key YOUR_API_KEY --verbose
```

### 查看帮助

```bash
./process_allocator_project.py --help
```

## 输出

### 输出目录

所有处理后的文件将保存到：
```
/home/chuyue/verus/tests/split_allocator/
```

### 输出文件

- **`*.rs`** - 拆分和处理后的 Rust 文件
- **`SUMMARY.md`** - 包含所有文件统计信息的汇总报告

### 汇总报告内容

报告包含：
- 源项目信息
- 创建的文件列表
- 每个文件的描述
- 代码统计（exec/spec/proof 等行数）

## 处理流程详解

### 步骤 1: 项目分析

脚本首先读取 `/home/chuyue/verified-memory-allocator/verus-mimalloc/` 中的所有 Rust 文件：

- 统计每个文件的行数
- 收集文件结构信息

### 步骤 2: Claude 规划

向 Claude API 发送请求，要求其：
- 分析项目结构
- 提出将大文件拆分成小文件的策略
- 考虑功能相关性和自包含性
- 目标：每个文件约 400 行

### 步骤 3: 文件拆分

根据 Claude 的规划：
- 合并相关的源文件
- 创建新的拆分文件
- 保留原始注释和文档

### 步骤 4: 依赖内联

对每个拆分文件：
- 使用 `inline-crate` 工具
- 消除外部 crate 依赖
- 创建自包含的单文件

### 步骤 5: 代码标注

使用 Verus 的 `line_count` 工具：
- 分析每一行代码
- 标记为不同类型：
  - **exec** - 可执行代码
  - **spec** - 规范代码
  - **proof** - 证明代码
  - **trusted** - 受信任代码
  - **comment** - 注释
  - **layout** - 布局/格式

### 步骤 6: 生成报告

创建 Markdown 格式的汇总报告，包含统计信息。

## 配置选项

可以在脚本开头修改以下配置：

```python
SOURCE_PROJECT = "/home/chuyue/verified-memory-allocator"
OUTPUT_DIR = f"{VERUS_DIR}/tests/split_allocator"
TARGET_LINES_PER_FILE = 400  # 每个文件的目标行数
```

## 故障排除

### 问题：API key 无效

确保你的 API key 是有效的 Anthropic API key。

### 问题：line_count 工具未找到

运行：
```bash
cd /home/chuyue/verus/source/tools/line_count
cargo build
```

### 问题：inline-crate 失败

如果 inline-crate 对某些文件失败，脚本会自动回退到使用原始文件。

### 问题：输出目录权限

确保你有权限写入输出目录：
```bash
mkdir -p /home/chuyue/verus/tests/split_allocator
chmod 755 /home/chuyue/verus/tests/split_allocator
```

## 示例输出

```
=== Allocator Project Processor ===

[Step 1] Reading project files...
Found 33 Rust files in verus-mimalloc

[Step 2] Consulting Claude API for splitting strategy...
Split plan created: 18 output files

[Step 3] Creating split files...
Creating allocator_core_01.rs: Core allocation structures and types
Creating allocator_core_02.rs: Page organization and management
...

[Step 4] Processing each file...
  [1/18] Processing allocator_core_01.rs...
  [2/18] Processing allocator_core_02.rs...
  ...

[Step 5] Creating summary report...

=== Processing Complete ===
Output directory: /home/chuyue/verus/tests/split_allocator
Files created: 18
```

## 高级用法

### 只处理特定文件

修改脚本中的 `read_project_files()` 方法来过滤文件：

```python
def read_project_files(self):
    verus_mimalloc_dir = f"{SOURCE_PROJECT}/verus-mimalloc"
    files_info = []

    # 只处理特定文件
    target_files = ["page.rs", "segment.rs", "types.rs"]

    for file_name in target_files:
        file_path = Path(verus_mimalloc_dir) / file_name
        if file_path.exists():
            # ... 处理文件
```

### 调整拆分粒度

修改 `TARGET_LINES_PER_FILE` 来改变目标文件大小：

```python
TARGET_LINES_PER_FILE = 600  # 更大的文件
# 或
TARGET_LINES_PER_FILE = 200  # 更小的文件
```

## 技术细节

### Claude API 使用

- 模型：`claude-sonnet-4-20250514`
- Max tokens：8000
- 输出格式：JSON

### inline-crate 工具

- 创建临时 crate 结构
- 超时时间：60 秒
- 失败时回退到原始文件

### line_count 工具

- 使用 `--one-file` 模式处理单个文件
- 输出 JSON 格式统计
- 超时时间：120 秒

## 许可证

该脚本与 Verus 项目使用相同的许可证（MIT）。

## 支持

如有问题或建议，请查看：
- Verus 文档：`/home/chuyue/verus/README.md`
- inline-crate 文档：`/home/chuyue/inline-crate/README.md`
