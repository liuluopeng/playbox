#!/bin/bash

# 遍历当前目录下的所有子目录
for dir in */ ; do
    # 检查是否存在`Cargo.toml`文件，这通常表示一个Rust项目
    if [ -f "$dir/Cargo.toml" ]; then
        # 进入项目目录
        cd "$dir"
        # 执行cargo clean命令
        cargo clean
        # 返回到原始目录
        cd ..
    fi
done

echo "所有Rust项目的target目录已清理完成。"
