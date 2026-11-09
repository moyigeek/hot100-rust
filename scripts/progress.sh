#!/usr/bin/env bash
# 统计完成进度：文件中不再包含 todo!() 视为已写实现。
set -u
cd "$(dirname "$0")/.."

total=$(find src/bin -name '*.rs' | wc -l | tr -d ' ')
remaining=$(grep -l 'todo!()' src/bin/*.rs 2>/dev/null | wc -l | tr -d ' ')
done=$((total - remaining))

printf '进度：%s/%s 题已写实现' "$done" "$total"
if [ "$total" -gt 0 ]; then
  printf '（%s%%）' "$((done * 100 / total))"
fi
printf '\n'

if [ "$remaining" -gt 0 ]; then
  echo
  echo "还没开始（仍有 todo!()）："
  grep -l 'todo!()' src/bin/*.rs | sed 's|src/bin/||; s|\.rs$||' | sed 's/^/  - /'
fi
