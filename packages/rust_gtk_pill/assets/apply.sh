#!/usr/bin/env bash
# 切换 Voquill 录音悬浮窗（pill）的动画主题。
# 用法: ./apply.sh style1|style2|style3   （不带参数列出可用主题及当前状态）
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEST="$HOME/.config/voquill-pill"

if [ $# -eq 0 ]; then
    echo "可用主题:"
    for d in "$SCRIPT_DIR"/style*/; do
        s=$(basename "$d")
        rec=$(ls "$d/frames" 2>/dev/null | wc -l)
        load=$(ls "$d/frames-loading" 2>/dev/null | wc -l)
        echo "  $s  (录音 ${rec} 帧 / 转写 ${load} 帧)"
    done
    echo "用法: $0 <主题名>"
    exit 0
fi

STYLE="$1"
SRC="$SCRIPT_DIR/$STYLE"
[ -d "$SRC/frames" ] || { echo "错误: 主题 $STYLE 不存在或缺少 frames/"; exit 1; }

mkdir -p "$DEST"
rm -rf "$DEST/frames" "$DEST/frames-loading"
cp -r "$SRC/frames" "$DEST/frames"
[ -d "$SRC/frames-loading" ] && cp -r "$SRC/frames-loading" "$DEST/frames-loading"

# pill 进程在启动时加载帧，需要重启 Voquill 生效
if pgrep -x voquill-desktop >/dev/null; then
    pkill -x voquill-desktop
    sleep 1
    setsid nohup /usr/bin/voquill-desktop >/dev/null 2>&1 < /dev/null &
    echo "✔ 已切换到 $STYLE 并重启 Voquill"
else
    echo "✔ 已切换到 $STYLE（Voquill 未运行，下次启动生效）"
fi
