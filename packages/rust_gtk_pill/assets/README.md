# 自定义 pill 动画素材

- `frames/` — 录音阶段默认素材：跳舞杰瑞（16 帧，源自 Tom and Jerry 表情包 GIF）
- `frames-loading/` — 转写阶段默认素材：躺着的汤姆（10 帧）
- `style1/ style2/ style3/` — 可切换主题，每套含自己的 `frames/` 与 `frames-loading/`：

| 主题 | 录音 | 转写 |
|---|---|---|
| style1 | 杰瑞吐舌狂奔（20 帧） | 躺着的汤姆（10 帧） |
| style2 | Morty 狂奔（45 帧） | 传送门竖大拇指（38 帧） |
| style3 | 跳舞杰瑞（16 帧） | 躺着的汤姆（10 帧，同默认） |

## 安装 / 切换主题

pill 进程启动时从 `~/.config/voquill-pill/` 加载，用户目录为空时回退到
deb 内置的 `/usr/lib/voquill-desktop/resources/pill-assets/`（见 `src/custom_anim.rs`）。

```bash
./apply.sh            # 列出可用主题
./apply.sh style2     # 切换主题（拷贝帧 + 自动重启 Voquill）
```

或手动安装默认素材：

```bash
mkdir -p ~/.config/voquill-pill
cp -r assets/frames assets/frames-loading ~/.config/voquill-pill/
# 重启 Voquill 生效
```

## 素材要求（换图时）

PNG 序列帧、真 Alpha 透明背景、按文件名排序播放；建议 ≤256×256、≤45 帧
（长 GIF 先抽稀）。GIF 转帧：`ffmpeg -i xxx.gif frames/f%02d.png`
