#!/usr/bin/env bash
# 安装 Voquill deb 并交互式选择 pill 动画主题。
#
# 用法:
#   ./install-voquill.sh [deb路径]     # 省略路径时在脚本目录和当前目录找 voquill-desktop_*.deb
#   ./install-voquill.sh --theme-only  # 跳过 deb 安装，只选择/切换主题
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# ---------- 1. 安装 deb ----------
if [ "${1:-}" != "--theme-only" ]; then
    DEB="${1:-}"
    if [ -z "$DEB" ]; then
        DEB=$(ls "$SCRIPT_DIR"/voquill-desktop_*.deb ./voquill-desktop_*.deb 2>/dev/null | head -1 || true)
    fi
    if [ -z "$DEB" ] || [ ! -f "$DEB" ]; then
        echo "错误: 未找到 voquill-desktop_*.deb，请传入路径: $0 <deb路径>"
        echo "（只想切换主题请用: $0 --theme-only）"
        exit 1
    fi
    echo "安装 $DEB（需要 sudo 密码）..."
    sudo apt-get install -y "$DEB"
fi

# ---------- 2. 选择主题 ----------
styles=()
for d in "$SCRIPT_DIR"/style*/; do
    [ -d "$d/frames" ] && styles+=("$(basename "$d")")
done

if [ ${#styles[@]} -eq 0 ]; then
    echo "未找到主题目录，使用 deb 内置默认动画（跳舞杰瑞/躺汤姆）"
    exit 0
fi

echo
echo "选择录音悬浮窗动画主题:"
echo "  0) 默认（deb 内置，同 style3：跳舞杰瑞 / 躺汤姆，不写用户配置）"
i=1
for s in "${styles[@]}"; do
    case "$s" in
        style1) desc="杰瑞吐舌狂奔 / 躺汤姆" ;;
        style2) desc="Morty 狂奔 / 传送门竖大拇指" ;;
        style3) desc="跳舞杰瑞 / 躺汤姆" ;;
        *) desc="" ;;
    esac
    echo "  $i) $s  $desc"
    i=$((i+1))
done

if [ -t 0 ]; then
    read -rp "输入编号 [0]: " choice
else
    read -r choice || choice=0
fi
choice="${choice:-0}"

if [ "$choice" = "0" ]; then
    echo "✔ 使用默认动画"
    exit 0
fi

idx=$((choice-1))
[ "$idx" -ge 0 ] && [ "$idx" -lt ${#styles[@]} ] || { echo "错误: 无效编号 $choice"; exit 1; }
exec "$SCRIPT_DIR/apply.sh" "${styles[$idx]}"
