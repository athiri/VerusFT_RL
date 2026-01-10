# Quick Start Guide

## 快速使用

### 方法 1：使用 Shell 包装脚本（推荐）

```bash
cd /home/chuyue/verus/tests

# 使用命令行传入 API key
./run_allocator_processing.sh YOUR_API_KEY

# 或者使用环境变量
export ANTHROPIC_API_KEY="your-api-key-here"
./run_allocator_processing.sh
```

### 方法 2：直接使用 Python 脚本

```bash
cd /home/chuyue/verus/tests

# 确保已安装依赖
pip3 install anthropic

# 运行脚本
./process_allocator_project.py --api-key YOUR_API_KEY --verbose
```

## 输出位置

处理后的文件将保存在：
```
/home/chuyue/verus/tests/split_allocator/
```

查看汇总报告：
```bash
cat /home/chuyue/verus/tests/split_allocator/SUMMARY.md
```

## 工作流程

脚本会自动完成以下任务：

1. ✅ 分析 `/home/chuyue/verified-memory-allocator` 项目
2. ✅ 调用 Claude API 生成拆分策略
3. ✅ 创建较小的自包含 Rust 文件
4. ✅ 使用 `inline-crate` 消除依赖
5. ✅ 使用 `line_count` 标注代码类型
6. ✅ 生成汇总报告

## 预计处理时间

- 分析项目：~30 秒
- Claude API 调用：~10-20 秒
- 文件处理：~5-10 分钟（取决于文件数量）

## 详细文档

查看完整文档：
```bash
cat /home/chuyue/verus/tests/ALLOCATOR_PROCESSING_README.md
```

## 获取 API Key

访问 [Anthropic Console](https://console.anthropic.com/) 获取 API key。
