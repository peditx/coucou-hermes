<div align="center">

<img src="NotchBuddy/Assets.xcassets/AppIcon.appiconset/icon_256x256.png" width="96" alt="Coucou 图标">

# Coucou

[🇮🇷 فارسی](README-fa.md) · [🇸🇦 العربية](README-ar.md) · [🇷🇺 Русский](README-ru.md) · [🇨🇳 中文](README-ch.md)

**原始仓库：** [Louis-CFM/coucou](https://github.com/Louis-CFM/coucou) —— 这是一个 fork，重新编排了 CI/CD 以支持多平台发布。

**一个住在你 Mac 刘海里的小家伙 —— 在 Windows 和 Linux 上则住在屏幕顶端 —— 帮你盯着 Claude Code 会话。**

批准权限、观看智能体干活、丢一个文件过去、和 Claude 聊天 —— 全程不用离开你手头的事。

![macOS 15+](https://img.shields.io/badge/macOS-15%2B-black?logo=apple)
![Windows 10/11](https://img.shields.io/badge/Windows-10%2F11-0078D4?logo=windows&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-any-16A085?logo=linux&logoColor=white)
![Swift 6](https://img.shields.io/badge/Swift-6-F05138?logo=swift&logoColor=white)
![SwiftUI](https://img.shields.io/badge/SwiftUI-native-0A84FF)
![Tauri 2](https://img.shields.io/badge/Tauri-2-FFC131?logo=tauri&logoColor=black)
![License: MIT](https://img.shields.io/badge/license-MIT-green)
![GitHub stars](https://img.shields.io/github/stars/Louis-CFM/coucou?style=social)

<img src="docs/media/demo.gif" width="760" alt="运行中的 Coucou">

</div>

---

## 为什么

有些工作室做出过漂亮的刘海伴侣……却从没让任何人真正用上。
**Coucou 是它的开源版本。** 每一行代码、每一段动画、每一个音效 —— 都可以自由使用、阅读、fork 和再创作。

来认识一下 **Mochi**：一个柔软的圆角方块，长着大眼睛，从你的刘海里蹦出来，挥手问好，用眼睛跟着你的光标，被戳的时候会生气（你要是戳个不停它还会晕），并在 Claude Code 需要你的那一刻告诉你。

## 功能特性

- 🤖 **Claude Code 与 Hermes Agent，实时同步** —— 在刘海里看到每一个会话：它读了什么、改了什么、跑了什么，一步一步。两个引擎都支持。干完了？Mochi 会开心地跳一下。
- ✅ **在刘海里直接批准** —— 权限请求（Claude Code 或 Hermes）会带着 **允许 / 拒绝** 出现。点一下，回去继续干活。
- 🧑‍💻 **跳到对应的终端** —— 直接打开该会话所在的那个终端窗口 *（macOS）*。
- 💬 **和 Claude 或 Hermes 聊天** —— 内置聊天，直接从刘海里发起。在设置里挑选模型；模型列表来自你的 Anthropic 账户，或者把 Coucou 指向一个远程 [Hermes Agent](https://github.com/NousResearch/hermes-agent)。
- 🪽 **全屏 Hermes 聊天** —— 打开一个桌面式的聊天窗口与 Hermes Agent 对话。小岛聊天同样可以由 Hermes 承担：你在设置里选择 **引擎**（Claude / Hermes）和 **回退**（无 / Claude / Hermes）—— 任何一方都不会自动切换。[详情](docs/HERMES.md)。

- 📎 **把文件丢到刘海里** —— Mochi 变成一个盒子把它吞掉，然后你可以就它提个问题，或者用邮件把它发出去 *（邮件：macOS、Mail.app）*。
- 🪟 **把 Mochi 拖到任意窗口上** —— 把那个窗口作为上下文附加给 Claude *（macOS）*。
- 🔌 **集成** —— Stripe 支付、n8n 工作流、GitHub、Vercel 部署、Resend 邮件、Notion、Cal.com。每一个都有属于自己的彩色小 Mochi。
- 🎭 **一个真正的角色** —— 待机时的呼吸、眨眼、在球面上跟着鼠标转的眼睛、表情动作、28 个手工制作的音效、启动时的一声问候。
- 🫥 **空闲时隐形** —— 没有任务在跑时就藏起来，把光标悬停到刘海上（Windows 上是屏幕顶边）才探出头。
- 🖥️ **任何 Mac，有刘海没刘海都行** —— 在 iMac、Mac mini，或者合上盖子外接显示器的 MacBook 上，Mochi 会待在屏幕顶端的一条小横条里。
- 🔒 **从设计上就是私密的** —— 没有遥测，没有账户。密钥存在你的 macOS Keychain 或 Windows Credential Manager 里。应用只与你接入的服务通信。

<table>
<tr>
<td><img src="docs/media/claude-code.png" alt="Claude Code 会话"></td>
<td><img src="docs/media/stripe.png" alt="Stripe 支付"></td>
</tr>
<tr>
<td><img src="docs/media/chat.png" alt="和 Claude 聊天"></td>
<td><img src="docs/media/dizzy.png" alt="被戳太多次"></td>
</tr>
</table>

## 安装

### 下载 macOS 版

1. 从 [Releases](https://github.com/Louis-CFM/coucou/releases) 拿到最新的 `Coucou.zip`。
2. 解压，把 **Coucou.app** 移到 `/Applications`。
3. 启动。这个构建还没有经过 Apple 公证，所以第一次运行时 macOS 会说无法验证开发者：打开 **系统设置 → 隐私与安全性**，向下滚动并点击 **仍要打开**（只需一次）。

### Windows

Windows 安装程序**暂时不可用**。Microsoft Defender 错误地把这个未签名的
安装程序标记为恶意软件；一份误报申诉正在 Microsoft 审核中，
安装程序会在通过审核并完成签名后回归。
在此期间，你可以[从源码构建](#从源码构建)。

PC 上没有刘海，所以小岛是从屏幕顶边滑出来，
而不是藏进刘海里。其余差异见
[`windows/README.md`](windows/README.md)。

### Linux

软件包由 CI 在每次 push 时构建 —— 从某次 workflow 运行里取回
`Coucou-Linux-packages`，然后：

```bash
sudo apt install ./Coucou-Linux-X.Y.Z-amd64.deb    # Debian / Ubuntu / Mint
sudo dnf install ./Coucou-Linux-X.Y.Z-x86_64.rpm   # Fedora / RPM-based
```

这里没有刘海，也没有哪个合成器允许客户端
在所有桌面环境下把窗口钉在屏幕顶端 —— 所以小岛是一个
自我定位的置顶窗口，在 Wayland 会话下回退到 XWayland。
差异以及尚未接通的部分见
[`linux/README.md`](linux/README.md)。

### 从源码构建

**macOS** —— 环境要求：macOS 15+、Xcode 16+、[XcodeGen](https://github.com/yonaskolb/XcodeGen)。

```bash
brew install xcodegen
git clone https://github.com/Louis-CFM/coucou.git
cd coucou/NotchBuddy
xcodegen
open NotchBuddy.xcodeproj   # then ⌘R
```

**Windows** —— 环境要求：[Rust](https://rustup.rs)、Node 20+、MSVC 构建工具。

```powershell
git clone https://github.com/Louis-CFM/coucou.git
cd coucou/windows
npm install
npm run pack                # installer lands in windows/release/
```

**Linux** —— 环境要求：[Rust](https://rustup.rs)、Node 20+，以及 Tauri 的构建
依赖（列在 [`linux/README.md`](linux/README.md) 中）。

```bash
git clone https://github.com/Louis-CFM/coucou.git
cd coucou/linux
npm install
npm run pack                # .deb + .rpm land in linux/release/
```

开发机上不编译任何东西：
每一次构建都在 GitHub Actions 里完成。

## 设置

点击菜单栏（macOS）或系统托盘（Windows、Linux）里的 Coucou 图标 → **设置…**

| 项目 | 用途 | 密钥存放在哪里 |
|---|---|---|
| **Claude Code hooks** | 实时会话与批准（Claude Code + Hermes） | **安装 hooks** —— Coucou 会备份 `~/.claude/settings.json`，合并它的 hooks，并在写入任何内容之前把差异给你看 |
| **Anthropic API key** | 聊天，以及针对文件的提问 | Keychain / Credential Manager / Secret Service |
| **Hermes URL + key** *（可选）* | 全屏 Hermes 聊天；如果你把 Hermes 选作小岛聊天的引擎，也包括小岛聊天；要让远程智能体访问本地文件，请在 Hermes 那一侧添加 MCP | Keychain / Credential Manager / Secret Service |
| Stripe、n8n、GitHub、Vercel、Resend、Notion、Cal.com | 各个集成的胶囊按钮 | Keychain / Credential Manager / Secret Service，全部可选 |

如果 Coucou 没在运行，hook 会立刻退出：**Claude Code 绝不会被阻塞。**

## 试试这些

| 这样做 | Mochi 就会那样 |
|---|---|
| 悬停在刘海上（Windows 上是顶边） | 探出头来打招呼 👋 |
| 点击它 | 展开 |
| 悬停在 Mochi 上 | 眨眼，眼睛变大 |
| 点击 Mochi | 被压扁 + 生气 |
| 快速连点 3 次 | 😵‍💫 晕上几秒 |
| 把文件拖到小岛上 | 变成盒子把它吞掉 |
| 把 Mochi 拖到某个窗口上 *（macOS）* | 把它作为上下文附加进去 |

## 工作原理

**macOS**

- **小岛**：一个贴合刘海的无边框 `NSPanel`，由一台小型状态机驱动（`hidden → petit → home`）。
- **角色**：用 SwiftUI `Canvas` + `TimelineView` 以 60 fps 绘制 —— squircle 身体、投射到球面上的眼睛、弹簧动画。没有 Rive，没有 Lottie，没有图片。
- **Claude Code 与 Hermes**：一个小小的 `nb-hook` / `coucou-hook` 脚本接收 hook 事件，再通过 Unix socket（Windows 上是命名管道）转发给应用。遇到批准时它会等你点击，然后再回复 hook。两个引擎都支持。
- **集成**：轻量级轮询器，无人关注时暂停。
- **音效**：28 个短 WAV，通过预加载的 `AVAudioPlayer` 播放。

macOS 应用是原生的 Swift 6 / SwiftUI / AppKit，**零第三方依赖**。

**Windows**

- 一个 [Tauri 2](https://tauri.app) 应用（Rust + TypeScript）：小岛是一个透明、置顶、从不抢焦点的窗口，Mochi 用 Canvas 2D 绘制，形状、时序和音效都与 Mac 版一致。
- Claude Code hooks 经由一个小小的 `coucou-hook.exe` 和一条命名管道传递；密钥存在 Windows Credential Manager 中。
- 细节与差异见 [`windows/README.md`](windows/README.md)。

**Linux**

- 同一个 Tauri 2 应用，只是把平台层换掉了：命名管道变成位于 `$XDG_RUNTIME_DIR/coucou.sock` 的 Unix socket（用 `SO_PEERCRED` 校验），路径改到 XDG 下，密钥交给 Secret Service，`.deb`/`.rpm` 取代 NSIS 安装程序。
- 细节、打包方式以及还缺什么，见 [`linux/README.md`](linux/README.md)。

## 贡献

非常欢迎 issue 和 PR —— 新的集成、新的表情、新的音效、bug 修复。见 [CONTRIBUTING.md](CONTRIBUTING.md)。

## 致谢

由 [Louis Raillé](https://louisraille.fr) 使用 Claude Code 打造。
灵感来自设计工作室们分享的刘海伴侣概念 —— 本项目是独立的，与其中任何一家都没有关联。

## 许可证

- **代码：** [MIT](LICENSE) —— 随便用、随便 fork、随便学习，只要保留版权声明即可。
- **名称、Mochi 角色、图标、音效与媒体：** © Louis Raillé，保留所有权利 —— 见 [LICENSE-ASSETS.md](LICENSE-ASSETS.md)。要发布你自己的 fork？那就给它取你自己的名字和角色。

<div align="center">

**如果 Mochi 让你笑了，一个 ⭐ 就帮了大忙。**

[网站](https://louis-cfm.github.io/coucou/) · [隐私](https://louis-cfm.github.io/coucou/privacy.html) · [条款](https://louis-cfm.github.io/coucou/terms.html) · [支持](https://louis-cfm.github.io/coucou/support.html)

</div>
