# 自定义 pill 动画素材

- `frames/` — 录音阶段：跳舞杰瑞（16 帧，源自 Tom and Jerry 表情包 GIF）
- `frames-loading/` — 转写阶段：躺着的汤姆（10 帧）

## 安装

pill 进程启动时从 `~/.config/voquill-pill/` 加载（见 `src/custom_anim.rs`）：

```bash
mkdir -p ~/.config/voquill-pill
cp -r assets/frames assets/frames-loading ~/.config/voquill-pill/
# 重启 Voquill 生效
```

## 素材要求（换图时）

PNG 序列帧、真 Alpha 透明背景、按文件名排序播放；建议 ≤256×256。
GIF 转帧：`ffmpeg -i xxx.gif frames/f%02d.png`
