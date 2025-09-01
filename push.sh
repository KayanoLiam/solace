#!/bin/bash
# 推送脚本 - 手动执行

echo "准备推送solace_rust项目到GitHub..."

# 检查git状态
git status

# 添加所有更改
git add .

# 提交更改（如果已有提交则跳过）
if git diff-index --quiet HEAD --; then
    echo "没有新的更改需要提交"
else
    git commit -m "Update: $(date '+%Y-%m-%d %H:%M:%S')"
fi

# 设置远程仓库（如果不存在）
if ! git remote | grep -q origin; then
    git remote add origin https://github.com/KayanoLiam/solace.git
fi

echo "请执行以下命令完成推送："
echo "1. 使用HTTPS: git push -u origin master --force"
echo "2. 或使用SSH: git@github.com:KayanoLiam/solace.git"
echo "3. 或使用GitHub CLI: gh repo create KayanoLiam/solace --push"