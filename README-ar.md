<div align="center">

<img src="NotchBuddy/Assets.xcassets/AppIcon.appiconset/icon_256x256.png" width="96" alt="أيقونة Coucou">

# Coucou

[🇮🇷 فارسی](README-fa.md) · [🇸🇦 العربية](README-ar.md) · [🇷🇺 Русский](README-ru.md) · [🇨🇳 中文](README-ch.md)

**المستودع الأصلي:** [Louis-CFM/coucou](https://github.com/Louis-CFM/coucou) — هذا فرع (fork) رُتّبَت فيه عمليات CI/CD لإصدارات متعددة المنصات.

**صديق صغير يعيش داخل فتحة الكاميرا (Notch) في جهاز Mac — أو في أعلى شاشتك على Windows وLinux — ويراقب جلسات Claude Code لديك.**

وافق على الأذونات، راقب وكلاءك أثناء العمل، أفلِت ملفاً، راسل Claude — كل ذلك دون أن تغادر ما كنت تفعله.

![macOS 15+](https://img.shields.io/badge/macOS-15%2B-black?logo=apple)
![Windows 10/11](https://img.shields.io/badge/Windows-10%2F11-0078D4?logo=windows&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-any-16A085?logo=linux&logoColor=white)
![Swift 6](https://img.shields.io/badge/Swift-6-F05138?logo=swift&logoColor=white)
![SwiftUI](https://img.shields.io/badge/SwiftUI-native-0A84FF)
![Tauri 2](https://img.shields.io/badge/Tauri-2-FFC131?logo=tauri&logoColor=black)
![License: MIT](https://img.shields.io/badge/license-MIT-green)
![GitHub stars](https://img.shields.io/github/stars/Louis-CFM/coucou?style=social)

<img src="docs/media/demo.gif" width="760" alt="Coucou أثناء العمل">

</div>

---

## لماذا؟

بعض الاستوديوهات أظهرت رفاقاً رائعين للفتحة… ثم لم يسمح لأحد باستخدامهم أبداً.
**Coucou هو النسخة المفتوحة.** كل سطر من الكود، وكل حركة، وكل صوت — حرّة الاستخدام والقراءة والتفرع والتعديل.

تعرّف على **Mochi**: مربّع دائري ناعم بعينين كبيرتين ينطّ خارج فتحتك، يلوّح لكي تحية، يتبع مؤشر الفأرة بعينيه، يستاء عندما تدغدغه (ويُصاب بالدوار إن أصررت)، ويخبرك في اللحظة التي يحتاج فيها Claude Code إليك.

## المزايا

- 🤖 **Claude Code وHermes Agent، مباشرة** — شاهد كل جلسة في فتحتك: ما تقرأه وما يحرره وما يشغّله، خطوة بخطوة. يعمل مع المحرّكين معاً. انتهى؟ يقفز Mochi قفزة صغيرة فرحة.
- ✅ **الموافقة من الفتحة** — تظهر طلبات الأذونات (من Claude Code أو Hermes) مع **السماح / الرفض**. نقرة واحدة، وتعود إلى عملك.
- 🧑‍💻 **الانتقال إلى الطرفية الصحيحة** — افتح نافذة الطرفية الفعلية لجلسة معيّنة *(macOS)*.
- 💬 **راسل Claude أو Hermes** — محادثة مدمجة، مباشرة من الفتحة. اختر النموذج في الإعدادات؛ القائمة تأتي من حساب Anthropic لديك أو وجّه Coucou نحو [Hermes Agent](https://github.com/NousResearch/hermes-agent) بعيد المدى.
- 🪽 **محادثة Hermes بملء الشاشة** — افتح نافذة محادثة بأسلوب سطح المكتب مع Hermes Agent. يمكن كذلك أن تُقدَّم محادثة الجزيرة عبر Hermes: تختار **المحرّك** (Claude / Hermes) و**الخيار الاحتياطي** (بدون / Claude / Hermes) في الإعدادات — ولا يتغيّر شيء من تلقاء نفسه. [التفاصيل](docs/HERMES.md).

- 📎 **أفلِت ملفاً على الفتحة** — يتحوّل Mochi إلى صندوق يبتلعه، ثم اسأل سؤالاً عنه أو أرسله بالبريد الإلكتروني *(البريد: macOS، Mail.app)*.
- 🪟 **اسحب Mochi فوق أي نافذة** — لإرفاق تلك النافذة كسياق لـ Claude *(macOS)*.
- 🔌 **التكاملات** — مدفوعات Stripe، وسير عمل n8n، وGitHub، ونشر Vercel، وبريد Resend، وNotion، وCal.com. لكل واحدة منها Mochi ملوّن خاص بها.
- 🎭 **شخصية حقيقية** — تنفّس أثناء الخمول، ورمشات، وعينان على كرة تتبعان الفأرة، وتعابير، و28 صوتاً مصنوعة يدوياً، وتحية عند الإطلاق.
- 🫥 **غير مرئي عندما يكون خاملاً** — يختفي حين لا يعمل شيء، ويطلّ عندما تمرّر مؤشرك فوق الفتحة (الحافة العلوية من الشاشة على Windows).
- 🖥️ **أي Mac، بفتحة أو بدونها** — على iMac أو Mac mini أو MacBook بغطاء مغلق أمام شاشة خارجية، يستقرّ Mochi في شريط صغير أعلى الشاشة.
- 🔒 **خصوصية بالتصميم** — لا تتبّع، ولا حساب. تعيش المفاتيح في Keychain الخاص بـ macOS أو Credential Manager الخاص بـ Windows. ولا يتواصل التطبيق إلا مع الخدمات التي تربطها بنفسك.

<table>
<tr>
<td><img src="docs/media/claude-code.png" alt="جلسة Claude Code"></td>
<td><img src="docs/media/stripe.png" alt="مدفوعات Stripe"></td>
</tr>
<tr>
<td><img src="docs/media/chat.png" alt="محادثة مع Claude"></td>
<td><img src="docs/media/dizzy.png" alt="ضربات كثيرة جداً"></td>
</tr>
</table>

## التثبيت

### التنزيل على macOS

1. حمّل أحدث ملف `Coucou.zip` من [Releases](https://github.com/Louis-CFM/coucou/releases).
2. فكّ الضغط وانقل **Coucou.app** إلى `/Applications`.
3. شغّله. هذا البناء لم يتم توثيقه (notarization) لدى Apple بعد، لذا في المرة الأولى تظهر رسالة من macOS بأنها لا تستطيع التحقق من المطوّر: افتح **System Settings → Privacy & Security**، ثم مرّر للأسفل واضغط **Open Anyway** (مرة واحدة فقط).

### Windows

مثبّت Windows **غير متاح مؤقتاً**. فأداة Microsoft Defender تصنّف المثبّت غير الموقّع خطأً بوصفه برمجية خبيثة؛ وقد رُفعت بلاغة إيجابية كاذبة للمراجعة لدى Microsoft، وسيعود المثبّت بعد أن تُحسم وتُوقَّع النسخة.
حتى ذلك الحين يمكنك [بناءه من المصدر](#البناء-من-المصدر).

لا توجد فتحة (Notch) على الحاسوب، لذا تنزلق الجزيرة من الحافة العلوية للشاشة بدلاً من الاختفاء داخلها. راجع [`windows/README.md`](windows/README.md) لمعرفة بقية الفروقات.

### Linux

تُبنى الحزم بواسطة CI مع كل دفع (push) — احصل على `Coucou-Linux-packages` من نزول عمل Workflow، ثم:

```bash
sudo apt install ./Coucou-Linux-X.Y.Z-amd64.deb    # Debian / Ubuntu / Mint
sudo dnf install ./Coucou-Linux-X.Y.Z-x86_64.rpm   # Fedora / RPM-based
```

لا توجد فتحة، ولا يسمح أي Compositor لعميل بتثبيت نافذة على الحافة العلوية في كل سطح مكتب — لذا تظهر الجزيرة كنافذة always-on-top تضبط موقعها بنفسها، مع الرجوع إلى XWayland في جلسات Wayland. راجع [`linux/README.md`](linux/README.md) لمعرفة الفروقات وما لم يُربط بعد.

### البناء من المصدر

**macOS** — المتطلبات: macOS 15+، وXcode 16+، و[XcodeGen](https://github.com/yonaskolb/XcodeGen).

```bash
brew install xcodegen
git clone https://github.com/Louis-CFM/coucou.git
cd coucou/NotchBuddy
xcodegen
open NotchBuddy.xcodeproj   # then ⌘R
```

**Windows** — المتطلبات: [Rust](https://rustup.rs)، وNode 20+، وأدوات بناء MSVC.

```powershell
git clone https://github.com/Louis-CFM/coucou.git
cd coucou/windows
npm install
npm run pack                # installer lands in windows/release/
```

**Linux** — المتطلبات: [Rust](https://rustup.rs)، وNode 20+، واعتماديات بناء Tauri (المدرجة في [`linux/README.md`](linux/README.md)).

```bash
git clone https://github.com/Louis-CFM/coucou.git
cd coucou/linux
npm install
npm run pack                # .deb + .rpm land in linux/release/
```

لا يُبنى شيء على جهاز التطوير: كل البناءات تجري في GitHub Actions.

## الإعداد

انقر على أيقونة Coucou في شريط القوائم (macOS) أو في صينية النظام (Windows، Linux) ← **Settings…**

| ماذا | لماذا | أين يذهب المفتاح |
|---|---|---|
| **خطافات Claude Code** | الجلسات المباشرة والموافقات (Claude Code) | **Install hooks** — يُنشئ Coucou نسخة احتياطية من `~/.claude/settings.json`، ويدمج خطافاته ويعرض لك الفرق قبل الكتابة |
| **خطافات Hermes** | الجلسات المباشرة والموافقات لـ Hermes Agent | **Install plugin** — يُنشئ Coucou نسخة احتياطية من `~/.hermes/config.yaml`، ويضيف مفاتيحه الثلاثة ويعرض لك الفرق قبل الكتابة |
| **مفتاح Anthropic API** | المحادثة والأسئلة عن الملفات | Keychain / Credential Manager / Secret Service |
| **عنوان Hermes + المفتاح** *(اختياري)* | محادثة Hermes بملء الشاشة، ومحادثة الجزيرة إن اخترت Hermes محرّكاً لها؛ وللحصول على ملفات محلية من الوكيل البعيد، أضِف MCP من جانب Hermes | Keychain / Credential Manager / Secret Service |
| Stripe وn8n وGitHub وVercel وResend وNotion وCal.com | حبّات التكامل (pills) | Keychain / Credential Manager / Secret Service، وكلها اختيارية |

إن لم يكن Coucou يعمل، يخرج الخطاف فوراً: **Claude Code لا يتعطّل أبداً.**

## جرّب ذلك

| افعل هذا | يفعل Mochi ذلك |
|---|---|
| مرّر فوق الفتحة (الحافة العلوية على Windows) | يطلّ ويقول مرحباً 👋 |
| انقر عليه | يُفتح |
| مرّر فوق Mochi | يرمش، وتكبر عيناه |
| انقر على Mochi | يُدهس + يستاء |
| انقر 3 مرات بسرعة | 😵‍💫 يصاب بالدوار لبضع ثوانٍ |
| أفلِت ملفاً على الجزيرة | يتحوّل إلى صندوق يبتلعه |
| اسحب Mochi فوق نافذة *(macOS)* | يرفقها كسياق |

## كيف يعمل

**macOS**

- **الجزيرة**: `NSPanel` بلا إطار يحتضن الفتحة، يقوده آلة حالة صغيرة (`hidden → petit → home`).
- **الشخصية**: تُرسم في SwiftUI عبر `Canvas` + `TimelineView` بمعدّل 60 إطاراً في الثانية — جسد squircle، وعينان مسقطتان على كرة، وحركة نابضة. لا Rive ولا Lottie ولا صور.
- **Claude Code وHermes**: سكربت صغير `nb-hook` / `coucou-hook` يستقبل أحداث الخطاف ويرسلها عبر Unix socket (أنبوب مُسمّى على Windows) إلى التطبيق، وإضافة Hermes في `~/.hermes/plugins/coucou/` تغذّي نفس الـ relay. وفي الموافقات ينتظر نقرتك ثم يردّ على الخطاف. يعمل مع المحرّكين معاً.
- **التكاملات**: مستطلعات خفيفة، تتوقّف حين لا يراقبها أحد.
- **الأصوات**: 28 ملف WAV قصيراً تُشغَّل عبر `AVAudioPlayer`s محمّلة مسبقاً.

تطبيق macOS أصلي بـ Swift 6 / SwiftUI / AppKit وب**صفر تبعيات من طرف ثالث**.

**Windows**

- تطبيق [Tauri 2](https://tauri.app) (Rust + TypeScript): الجزيرة نافذة شفافة always-on-top لا تسرق التركيز أبداً، ويُرسم Mochi في Canvas 2D بالأشكال والتوقيتات والأصوات نفسها كما على Mac.
- خطافات Claude Code تمرّ عبر `coucou-hook.exe` صغير وأنبوب مُسمّى، وملحق Hermes يستخدم نفس الـ relay؛ والمفاتيح تعيش في Windows Credential Manager.
- التفاصيل والفروقات في [`windows/README.md`](windows/README.md).

**Linux**

- نفس تطبيق Tauri 2 مع تبديل طبقة المنصة: يتحوّل الأنبوب المُسمّى إلى Unix socket في `$XDG_RUNTIME_DIR/coucou.sock` (يُتحقّق منه بـ `SO_PEERCRED`)، وتنتقل المسارات إلى XDG، وتذهب المفاتيح إلى Secret Service، ويحلّ `.deb`/`.rpm` محل مثبّت NSIS.
- التفاصيل والتغليف وما يزال ناقصاً في [`linux/README.md`](linux/README.md).

## المساهمة

نرحّب بشدة بتقديم Issues وPRs — تكاملات جديدة، وتعابير جديدة، وأصوات جديدة، وإصلاحات للأخطاء. راجع [CONTRIBUTING.md](CONTRIBUTING.md).

## الإقرارات

بُني بواسطة [Louis Raillé](https://louisraille.fr) مع Claude Code.
استُلهِم من مفاهيم رفاق الفتحة التي شاركتها استوديوهات التصميم — هذا المشروع مستقل وغير تابع لأي منهم.

## الرخصة

- **الكود:** [MIT](LICENSE) — استخدمه، وتفرّع منه، وتعلّم منه، فقط أبقِ إشعار حقوق النشر.
- **الاسم، وشخصية Mochi، والأيقونة، والأوساط (الوسائط):** © Louis Raillé، جميع الحقوق محفوظة — راجع [LICENSE-ASSETS.md](LICENSE-ASSETS.md). هل ستشحن نسخة forks خاصة بك؟ امنحها اسماً وشخصية خاصين بها.

<div align="center">

**إن كان Mochi أنقذ ابتسامتك، فـ ⭐ تُساعد كثيراً.**

[Website](https://louis-cfm.github.io/coucou/) · [Privacy](https://louis-cfm.github.io/coucou/privacy.html) · [Terms](https://louis-cfm.github.io/coucou/terms.html) · [Support](https://louis-cfm.github.io/coucou/support.html)

</div>
