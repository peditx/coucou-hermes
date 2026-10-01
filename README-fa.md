<div align="center">

<img src="NotchBuddy/Assets.xcassets/AppIcon.appiconset/icon_256x256.png" width="96" alt="آیکون Coucou">

# Coucou

[🇮🇷 فارسی](README-fa.md) · [🇸🇦 العربية](README-ar.md) · [🇷🇺 Русский](README-ru.md) · [🇨🇳 中文](README-ch.md)

**مخزن اصلی:** [Louis-CFM/coucou](https://github.com/Louis-CFM/coucou) — این یک fork است که CI/CD آن برای انتشار چند-سکویی بازچینش شده است.

**یک دوست کوچک که در notch مک شما زندگی می‌کند — یا در بالای صفحه در Windows و Linux — و جلسات Claude Code شما را زیر نظر دارد.**

مجوزها را تأیید کنید، کار agentها را تماشا کنید، فایلی را رها کنید، با Claude چت کنید — همه بدون اینکه کاری را در دست دارید رها کنید.

![macOS 15+](https://img.shields.io/badge/macOS-15%2B-black?logo=apple)
![Windows 10/11](https://img.shields.io/badge/Windows-10%2F11-0078D4?logo=windows&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-any-16A085?logo=linux&logoColor=white)
![Swift 6](https://img.shields.io/badge/Swift-6-F05138?logo=swift&logoColor=white)
![SwiftUI](https://img.shields.io/badge/SwiftUI-native-0A84FF)
![Tauri 2](https://img.shields.io/badge/Tauri-2-FFC131?logo=tauri&logoColor=black)
![License: MIT](https://img.shields.io/badge/license-MIT-green)
![GitHub stars](https://img.shields.io/github/stars/Louis-CFM/coucou?style=social)

<img src="docs/media/demo.gif" width="760" alt="Coucou در حال کار">

</div>

---

## چرا

بعضی استودیوها همراه‌های notch را با شکوه نشان دادند… و هرگز اجازه ندادند کسی از آن‌ها استفاده کند.
**Coucou نسخهٔ بازِ همان است.** هر خط کد، هر انیمیشن، هر صدا — آزاد برای استفاده، خواندن، fork کردن و بازآفرینی.

با **Mochi** آشنا شوید: یک squircle نرم و کوچک با چشم‌های درشت که از notch شما بیرون می‌زند، دست تکان می‌دهد، با چشم‌هایش دنبال مکان‌نما می‌گردد، وقتی نیشگونش می‌گیرید عصبانی می‌شود (و اگر اصرار کنید سرگیجه می‌گیرد)، و همان لحظه‌ای که Claude Code به شما نیاز دارد خبر می‌دهد.

## امکانات

- 🤖 **Claude Code و Hermes Agent، زنده** — هر جلسه را در notch خود ببینید: چه می‌خواند، چه ویرایش می‌کند و چه اجرا می‌کند، قدم‌به‌قدم. با هر دو موتور کار می‌کند. تمام شد؟ Mochi یک پرش کوچک خوشحالی می‌کند.
- ✅ **تأیید از درون notch** — درخواست‌های مجوز (Claude Code یا Hermes) با **اجازه / رد** نمایان می‌شوند. یک کلیک، و برمی‌گردید سر کارتان.
- 🧑‍💻 **پرش به ترمینال درست** — پنجرهٔ دقیق ترمینالِ یک جلسه را باز کنید *(macOS)*.
- 💬 **چت با Claude یا Hermes** — چت داخلی، مستقیم از notch. مدل را در Settings انتخاب کنید؛ فهرست از حساب Anthropic شما می‌آید، یا Coucou را به یک [Hermes Agent](https://github.com/NousResearch/hermes-agent) راه‌دور اشاره دهید.
- 🪽 **چت تمام‌صفحهٔ Hermes** — یک پنجرهٔ چت به شکل دسکتاپ با Hermes Agent باز کنید. چت جزیره هم می‌تواند توسط Hermes سرو شود: شما **Engine** (Claude / Hermes) و **Fallback** (None / Claude / Hermes) را در Settings انتخاب می‌کنید — هیچ‌چیز خودبه‌خود عوض نمی‌شود. [جزئیات](docs/HERMES.md).

- 📎 **رها کردن فایل روی notch** — Mochi به یک جعبه تبدیل می‌شود و آن را می‌بلعد، بعد درباره‌اش سؤال بپرسید یا ایمیلش کنید *(ایمیل: macOS، Mail.app)*.
- 🪟 **کشیدن Mochi روی هر پنجره** — آن پنجره را به‌عنوان context برای Claude پیوست کنید *(macOS)*.
- 🔌 **یکپارچه‌سازی‌ها** — پرداخت‌های Stripe، گردش‌کارهای n8n، GitHub، استقرارهای Vercel، ایمیل‌های Resend، Notion، Cal.com. هر کدام Mochi رنگیِ کوچک خودش را دارد.
- 🎭 **یک شخصیت واقعی** — تنفس در حالت بیکاری، پلک زدن، چشم‌هایی روی یک کره که موس شما را دنبال می‌کنند، حالت‌های احساسی، ۲۸ صدای دست‌ساز، و یک سلام هنگام راه‌اندازی.
- 🫥 **نامرئی هنگام بیکاری** — وقتی چیزی در حال اجرا نیست پنهان می‌شود، و وقتی نشانگر را روی notch می‌برید (لبهٔ بالایی صفحه در Windows) سرک می‌کشد.
- 🖥️ **هر مکی، با notch یا بی‌آنکه** — روی یک iMac، یک Mac mini، یا یک MacBook با درِ بسته روی نمایشگر خارجی، Mochi در یک نوار کوچک در بالای صفحه می‌نشیند.
- 🔒 **محرمانه از پایه** — بدون telemetry، بدون حساب. کلیدها در Keychain مک شما یا Windows Credential Manager زندگی می‌کنند. برنامه فقط با سرویس‌هایی که وصلشان می‌کنید حرف می‌زند.

<table>
<tr>
<td><img src="docs/media/claude-code.png" alt="جلسهٔ Claude Code"></td>
<td><img src="docs/media/stripe.png" alt="پرداخت‌های Stripe"></td>
</tr>
<tr>
<td><img src="docs/media/chat.png" alt="چت با Claude"></td>
<td><img src="docs/media/dizzy.png" alt="ضرباتِ زیاد"></td>
</tr>
</table>

## نصب

### دانلود برای macOS

1. آخرین `Coucou.zip` را از [Releases](https://github.com/Louis-CFM/coucou/releases) بردارید.
2. آن را از حافظهٔ فشرده دربیاورید و **Coucou.app** را به `/Applications` منتقل کنید.
3. اجرا کنید. این بیلد هنوز توسط Apple notarize نشده است، بنابراین بار اول macOS می‌گوید نمی‌تواند توسعه‌دهنده را تأیید کند: **System Settings → Privacy & Security** را باز کنید، به پایین بروید و **Open Anyway** را بزنید (فقط یک بار).

### Windows

نصب‌کنندهٔ Windows **موقتاً در دسترس نیست**. Microsoft Defender به‌اشتباه
نصب‌کنندهٔ امضا‌نشده را malware اعلام کرده است؛ یک گزارش false-positive در حال بررسی
نزد Microsoft است و نصب‌کننده پس از تأیید و امضا شدن برمی‌گردد.
تا آن زمان می‌توانید [از منبع بسازیدش](#ساخت-از-منبع).

notch روی PC وجود ندارد، بنابراین جزیره به‌جای پنهان شدن در آن، از لبهٔ بالایی
صفحه بیرون می‌لغزد. بقیهٔ تفاوت‌ها را در [`windows/README.md`](windows/README.md)
ببینید.

### Linux

بسته‌ها در هر push توسط CI ساخته می‌شوند — `Coucou-Linux-packages` را از یک اجرای
workflow بردارید، بعد:

```bash
sudo apt install ./Coucou-Linux-X.Y.Z-amd64.deb    # Debian / Ubuntu / Mint
sudo dnf install ./Coucou-Linux-X.Y.Z-x86_64.rpm   # Fedora / RPM-based
```

notch در کار نیست، و هیچ compositorی به یک client اجازه نمی‌دهد پنجره‌ای را روی لبهٔ
بالایی در هر دسکتاپی بچسباند — بنابراین جزیره یک پنجرهٔ همیشه-روی-بالا
(always-on-top) است که خودش را جا می‌دهد، و در نشست‌های Wayland به XWayland
باز می‌گردد. تفاوت‌ها و اینکه هنوز چه چیزی وصل نشده را در
[`linux/README.md`](linux/README.md) ببینید.

### ساخت از منبع

**macOS** — پیش‌نیازها: macOS 15+، Xcode 16+، [XcodeGen](https://github.com/yonaskolb/XcodeGen).

```bash
brew install xcodegen
git clone https://github.com/Louis-CFM/coucou.git
cd coucou/NotchBuddy
xcodegen
open NotchBuddy.xcodeproj   # then ⌘R
```

**Windows** — پیش‌نیازها: [Rust](https://rustup.rs)، Node 20+، و ابزارهای build مربوط به MSVC.

```powershell
git clone https://github.com/Louis-CFM/coucou.git
cd coucou/windows
npm install
npm run pack                # installer lands in windows/release/
```

**Linux** — پیش‌نیازها: [Rust](https://rustup.rs)، Node 20+، و وابستگی‌های build مربوط به
Tauri (فهرست‌شده در [`linux/README.md`](linux/README.md)).

```bash
git clone https://github.com/Louis-CFM/coucou.git
cd coucou/linux
npm install
npm run pack                # .deb + .rpm land in linux/release/
```

هیچ‌چیز روی ماشین توسعه کامپایل نمی‌شود: هر build در GitHub Actions
اجرا می‌شود.

## راه‌اندازی

روی آیکون Coucou در نوار منو (macOS) یا system tray (Windows، Linux) کلیک کنید → **Settings…**

| چه چیزی | چرا | کلید کجا می‌رود |
|---|---|---|
| **هوک‌های Claude Code** | جلسات زنده و تأییدها (Claude Code) | **Install hooks** — Coucou از `~/.claude/settings.json` نسخهٔ پشتیبان می‌گیرد، هوک‌هایش را ادغام می‌کند و پیش از نوشتنِ هر چیزی، diff را به شما نشان می‌دهد |
| **هوک‌های Hermes** | جلسات زنده و تأییدها برای Hermes Agent | **Install plugin** — Coucou از `~/.hermes/config.yaml` نسخهٔ پشتیبان می‌گیرد، سه کلیدش را اضافه می‌کند و پیش از نوشتن، diff را به شما نشان می‌دهد |
| **کلید Anthropic API** | چت و پرسش دربارهٔ فایل‌ها | Keychain / Credential Manager / Secret Service |
| **URL و کلید Hermes** *(اختیاری)* | چت تمام‌صفحهٔ Hermes، و چت جزیره اگر Hermes را به‌عنوان موتورش انتخاب کنید؛ برای فایل‌های محلی از agent راه‌دور، در سمت Hermes مقدار MCP اضافه کنید | Keychain / Credential Manager / Secret Service |
| Stripe، n8n، GitHub، Vercel، Resend، Notion، Cal.com | قرص‌های یکپارچه‌سازی | Keychain / Credential Manager / Secret Service، همه اختیاری |

اگر Coucou در حال اجرا نباشد، هوک فوراً خارج می‌شود: **Claude Code هرگز مسدود نمی‌شود.**

## چیزهایی برای امتحان کردن

| این کار را بکنید | Mochi این کار را می‌کند |
|---|---|
| نشانگر را روی notch ببرید (لبهٔ بالایی در Windows) | سرک می‌کشد و سلام می‌کند 👋 |
| رویش کلیک کنید | باز می‌شود |
| نشانگر را روی Mochi ببرید | پلک می‌زند، چشم‌ها بزرگ می‌شوند |
| روی Mochi کلیک کنید | له می‌شود + عصبانی |
| سه بار سریع کلیک کنید | 😵‍💫 چند ثانیه سرگیجه |
| فایلی را روی جزیره بکشید | به جعبه تبدیل می‌شود و آن را می‌بلعد |
| Mochi را روی یک پنجره بکشید *(macOS)* | آن را به‌عنوان context پیوست می‌کند |

## چگونه کار می‌کند

**macOS**

- **جزیره**: یک `NSPanel` بدون حاشیه که به notch می‌چسبد و با یک ماشین وضعیتِ کوچک (`hidden → petit → home`) هدایت می‌شود.
- **شخصیت**: با SwiftUI `Canvas` و `TimelineView` با نرخ ۶۰ فریم بر ثانیه رسم می‌شود — بدنهٔ squircle، چشم‌های پروژکت‌شده روی یک کره، انیمیشن‌های فنری. بدون Rive، بدون Lottie، بدون تصویر.
- **Claude Code و Hermes**: یک اسکریپت کوچک `nb-hook` / `coucou-hook` رویدادهای هوک را دریافت می‌کند و از طریق یک Unix socket (در Windows، named pipe) به برنامه می‌فرستد، و افزونهٔ Hermes در `~/.hermes/plugins/coucou/` همین relay را تغذیه می‌کند. برای تأییدها منتظر کلیک شما می‌ماند و بعد به هوک پاسخ می‌دهد. با هر دو موتور کار می‌کند.
- **یکپارچه‌سازی‌ها**: pollerهای سبک‌وزن، که وقتی چیزی زیر نظر نیست متوقف می‌شوند.
- **صداها**: ۲۸ فایل WAV کوتاه که از طریق `AVAudioPlayer`های از قبل بارگذاری‌شده پخش می‌شوند.

برنامهٔ macOS بومی Swift 6 / SwiftUI / AppKit است و **صفر وابستگیِ شخص ثالث** دارد.

**Windows**

- یک برنامهٔ [Tauri 2](https://tauri.app) (Rust + TypeScript): جزیره یک پنجرهٔ شفاف و always-on-top است که هرگز focus را نمی‌دزدد، و Mochi در Canvas 2D با همان شکل‌ها، زمان‌بندی‌ها و صداهایی که روی مک هست رسم می‌شود.
- هوک‌های Claude Code از یک `coucou-hook.exe` کوچک و یک named pipe می‌گذرند و افزونهٔ Hermes از همین relay استفاده می‌کند؛ کلیدها در Windows Credential Manager زندگی می‌کنند.
- جزئیات و تفاوت‌ها در [`windows/README.md`](windows/README.md).

**Linux**

- همان برنامهٔ Tauri 2 با لایهٔ سکوییِ عوض‌شده: named pipe به یک Unix socket در `$XDG_RUNTIME_DIR/coucou.sock` تبدیل می‌شود (که با `SO_PEERCRED` بررسی می‌شود)، مسیرها به XDG منتقل می‌شوند، کلیدها به Secret Service می‌روند، و `.deb`/`.rpm` جای نصب‌کنندهٔ NSIS را می‌گیرند.
- جزئیات، بسته‌بندی و اینکه هنوز چه چیزی جا مانده، در [`linux/README.md`](linux/README.md).

## مشارکت

Issues و PRها بسیار خوش‌آیندند — یکپارچه‌سازی‌های جدید، حالت‌های احساسی جدید، صداهای جدید، رفع اشکال. [CONTRIBUTING.md](CONTRIBUTING.md) را ببینید.

## قدردانی

ساختهٔ [Louis Raillé](https://louisraille.fr) با Claude Code.
الهام‌گرفته از مفاهیم همراهِ notch که استودیوهای طراحی منتشر کرده‌اند — این پروژه مستقل است و به هیچ‌کدام از آن‌ها وابستگی ندارد.

## مجوز

- **کد:** [MIT](LICENSE) — از آن استفاده کنید، fork کنید، از آن بیاموزید، فقط اعلامیهٔ copyright را نگه دارید.
- **نام، شخصیت Mochi، آیکون، صداها و رسانه‌ها:** © Louis Raillé، تمامی حقوق محفوظ است — [LICENSE-ASSETS.md](LICENSE-ASSETS.md) را ببینید. fork خودتان را منتشر می‌کنید؟ نام و شخصیت خودتان را به آن بدهید.

<div align="center">

**اگر Mochi لبخند بر لب شما نشاند، یک ⭐ خیلی کمک می‌کند.**

[وب‌سایت](https://louis-cfm.github.io/coucou/) · [حریم خصوصی](https://louis-cfm.github.io/coucou/privacy.html) · [شرایط](https://louis-cfm.github.io/coucou/terms.html) · [پشتیبانی](https://louis-cfm.github.io/coucou/support.html)

</div>
