<div align="center">

<img src="NotchBuddy/Assets.xcassets/AppIcon.appiconset/icon_256x256.png" width="96" alt="Иконка Coucou">

# Coucou

[🇮🇷 فارسی](README-fa.md) · [🇸🇦 العربية](README-ar.md) · [🇷🇺 Русский](README-ru.md) · [🇨🇳 中文](README-ch.md)

**Оригинальный репозиторий:** [Louis-CFM/coucou](https://github.com/Louis-CFM/coucou) — это форк с перестроенным CI/CD под мультиплатформенные релизы.

**Крошечный друг, живущий в вырезе экрана вашего Mac — или в верхней части экрана в Windows и Linux — который следит за вашими сессиями Claude Code.**

Подтверждайте разрешения, наблюдайте за работой агентов, скидывайте файл, общайтесь с Claude — и всё это, не отрываясь от того, чем занимаетесь.

![macOS 15+](https://img.shields.io/badge/macOS-15%2B-black?logo=apple)
![Windows 10/11](https://img.shields.io/badge/Windows-10%2F11-0078D4?logo=windows&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-any-16A085?logo=linux&logoColor=white)
![Swift 6](https://img.shields.io/badge/Swift-6-F05138?logo=swift&logoColor=white)
![SwiftUI](https://img.shields.io/badge/SwiftUI-native-0A84FF)
![Tauri 2](https://img.shields.io/badge/Tauri-2-FFC131?logo=tauri&logoColor=black)
![License: MIT](https://img.shields.io/badge/license-MIT-green)
![GitHub stars](https://img.shields.io/github/stars/Louis-CFM/coucou?style=social)

<img src="docs/media/demo.gif" width="760" alt="Coucou в действии">

</div>

---

## Зачем

Некоторые студии показали великолепных компаньонов для выреза… и так и не разрешили ими пользоваться.
**Coucou — открытая версия.** Каждая строка кода, каждая анимация, каждый звук — свободно использовать, читать, форкать и переделывать.

Знакомьтесь: **Mochi** — мягкий сквиркл с большими глазами, который выныривает из вашего выреза, машет приветствие, следит за курсором глазами, злится, когда вы его щёлкаете (и закружается, если продолжаете), и сообщает в тот самый момент, когда Claude Code вас позвал.

## Возможности

- 🤖 **Claude Code и Hermes Agent в реальном времени** — видна каждая сессия в вашем вырезе: что она читает, редактирует и запускает, шаг за шагом. Работает с обоими движками. Закончила? Mochi радостно подпрыгивает.
- ✅ **Подтверждение прямо из выреза** — запросы разрешений (Claude Code или Hermes) появляются с кнопками **Allow / Deny**. Один клик — и снова к работе.
- 🧑‍💻 **Переход к нужному терминалу** — открывает ровно то окно терминала, где идёт сессия *(macOS)*.
- 💬 **Чат с Claude или Hermes** — встроенный чат прямо из выреза. Модель выбирается в настройках; список берётся из вашей учётной записи Anthropic, либо направьте Coucou на удалённый [Hermes Agent](https://github.com/NousResearch/hermes-agent).
- 🪽 **Полноэкранный чат с Hermes** — открывает окно чата в стиле рабочего стола с Hermes Agent. Чатом на острове тоже может служить Hermes: вы выбираете **Engine** (Claude / Hermes) и **Fallback** (None / Claude / Hermes) в настройках — ничего не переключается само. [Подробнее](docs/HERMES.md).

- 📎 **Перетащите файл на вырез** — Mochi превращается в коробку и проглатывает его, после чего можно задать вопрос о нём или отправить по почте *(почта: macOS, Mail.app)*.
- 🪟 **Перетащите Mochi на любое окно** — прикрепляет это окно как контекст для Claude *(macOS)*.
- 🔌 **Интеграции** — платежи Stripe, сценарии n8n, GitHub, деплои Vercel, письма Resend, Notion, Cal.com. У каждой свой маленький цветной Mochi.
- 🎭 **Настоящий персонаж** — дыхание в покое, моргание, глаза на сфере, следящие за мышью, эмоции, 28 авторских звуков, приветствие при запуске.
- 🫥 **Невидим в покое** — прячется, когда ничего не выполняется, и выглядывает, когда вы наводите курсор на вырез (на верхний край экрана в Windows).
- 🖥️ **Любой Mac, с вырезом или без** — на iMac, Mac mini или MacBook с закрытой крышкой под внешним дисплеем Mochi сидит в небольшой полоске в верхней части экрана.
- 🔒 **Приватность по замыслу** — никакой телеметрии, никаких аккаунтов. Ключи хранятся в Keychain на macOS или в Credential Manager в Windows. Приложение общается только с теми службами, которые вы к нему подключаете.

<table>
<tr>
<td><img src="docs/media/claude-code.png" alt="Сессия Claude Code"></td>
<td><img src="docs/media/stripe.png" alt="Платежи Stripe"></td>
</tr>
<tr>
<td><img src="docs/media/chat.png" alt="Чат с Claude"></td>
<td><img src="docs/media/dizzy.png" alt="Слишком много ударов"></td>
</tr>
</table>

## Установка

### Загрузка для macOS

1. Скачайте последний `Coucou.zip` из [Releases](https://github.com/Louis-CFM/coucou/releases).
2. Распакуйте архив и перетащите **Coucou.app** в `/Applications`.
3. Запустите. Эта сборка ещё не нотаризована Apple, поэтому в первый раз macOS сообщит, что не может проверить разработчика: откройте **Системные настройки → Конфиденциальность и безопасность**, прокрутите вниз и нажмите **Всё равно открыть** (лишь один раз).

### Windows

Установщик Windows **временно недоступен**. Microsoft Defender ошибочно помечает
неподписанный установщик как вредоносное ПО; запрос о ложном срабатывании
рассматривается в Microsoft, и установщик вернётся, когда его очистят и подпишут.
Пока что можно [собрать его из исходников](#сборка-из-исходников).

На ПК нет выреза, поэтому остров выезжает из верхнего края экрана
вместо того, чтобы прятаться внутри него. О прочих отличиях — в
[`windows/README.md`](windows/README.md).

### Linux

Пакеты собираются CI при каждом пуше — возьмите `Coucou-Linux-packages` из
запуска workflow и затем:

```bash
sudo apt install ./Coucou-Linux-X.Y.Z-amd64.deb    # Debian / Ubuntu / Mint
sudo dnf install ./Coucou-Linux-X.Y.Z-x86_64.rpm   # Fedora / RPM-based
```

Выреза здесь нет, и ни один композитор не даёт клиенту прикрепить окно
к верхнему краю на любом рабочем столе — поэтому остров это окно
always-on-top, которое позиционирует себя само, а в сессиях Wayland
с откатом на XWayland. Об отличиях и о том, что ещё не подключено, —
в [`linux/README.md`](linux/README.md).

### Сборка из исходников

**macOS** — требования: macOS 15+, Xcode 16+, [XcodeGen](https://github.com/yonaskolb/XcodeGen).

```bash
brew install xcodegen
git clone https://github.com/Louis-CFM/coucou.git
cd coucou/NotchBuddy
xcodegen
open NotchBuddy.xcodeproj   # then ⌘R
```

**Windows** — требования: [Rust](https://rustup.rs), Node 20+, MSVC build tools.

```powershell
git clone https://github.com/Louis-CFM/coucou.git
cd coucou/windows
npm install
npm run pack                # installer lands in windows/release/
```

**Linux** — требования: [Rust](https://rustup.rs), Node 20+ и зависимости для сборки
Tauri (перечислены в [`linux/README.md`](linux/README.md)).

```bash
git clone https://github.com/Louis-CFM/coucou.git
cd coucou/linux
npm install
npm run pack                # .deb + .rpm land in linux/release/
```

Ничего не компилируется на машине разработчика: каждая сборка выполняется в GitHub
Actions.

## Настройка

Нажмите на иконку Coucou в строке меню (macOS) или в системном трее (Windows, Linux) → **Settings…**

| Что | Зачем | Куда уходит ключ |
|---|---|---|
| **Хуки Claude Code** | сессии в реальном времени и подтверждения (Claude Code) | **Install hooks** — Coucou делает резервную копию `~/.claude/settings.json`, объединяет с ним свои хуки и показывает вам diff, прежде чем что-либо записывать |
| **Хуки Hermes** | сессии в реальном времени и подтверждения для Hermes Agent | **Install plugin** — Coucou делает резервную копию `~/.hermes/config.yaml`, добавляет туда свои три ключа и показывает вам diff, прежде чем что-либо записывать |
| **Ключ Anthropic API** | чат и вопросы о файлах | Keychain / Credential Manager / Secret Service |
| **URL и ключ Hermes** *(необязательно)* | полноэкранный чат с Hermes, а также чат на острове, если вы выбрали Hermes в качестве его движка; для доступа к локальным файлам из удалённого агента добавьте MCP на стороне Hermes | Keychain / Credential Manager / Secret Service |
| Stripe, n8n, GitHub, Vercel, Resend, Notion, Cal.com | пилюли интеграций | Keychain / Credential Manager / Secret Service — всё необязательно |

Если Coucou не запущен, хук завершается сразу: **Claude Code никогда не блокируется.**

## Что попробовать

| Делайте так | Mochi делает так |
|---|---|
| Наведите курсор на вырез (на верхний край в Windows) | выглядывает и здоровается 👋 |
| Нажмите на него | открывается |
| Наведите курсор на Mochi | моргает, глаза увеличиваются |
| Нажмите на Mochi | сплющивается + злится |
| Быстро нажмите 3 раза | 😵‍💫 кружится несколько секунд |
| Перетащите файл на остров | превращается в коробку и проглатывает его |
| Перетащите Mochi на окно *(macOS)* | прикрепляет его как контекст |

## Как это работает

**macOS**

- **Остров**: бескаркасная `NSPanel`, облегающая вырез, управляется небольшой конечной машиной (`hidden → petit → home`).
- **Персонаж**: рисуется в SwiftUI `Canvas` + `TimelineView` со 60 кадрами в секунду — тело-сквиркл, глаза, спроецированные на сферу, пружинные анимации. Без Rive, без Lottie, без изображений.
- **Claude Code и Hermes**: крошечный скрипт `nb-hook` / `coucou-hook` принимает события хуков и пересылает их через Unix-сокет (именованный канал в Windows) в приложение, а плагин Hermes в `~/.hermes/plugins/coucou/` питает тот же relay. Для подтверждений он ждёт вашего клика и лишь затем отвечает хуку. Работает с обоими движками.
- **Интеграции**: лёгкие поллеры, которые останавливаются, когда за ними никто не наблюдает.
- **Звуки**: 28 коротких WAV, воспроизводимых через предзагруженные `AVAudioPlayer`.

Приложение для macOS — нативное, Swift 6 / SwiftUI / AppKit, **без единой сторонней зависимости**.

**Windows**

- Приложение на [Tauri 2](https://tauri.app) (Rust + TypeScript): остров это прозрачное окно always-on-top, которое никогда не перехватывает фокус, а Mochi рисуется в Canvas 2D с теми же формами, таймингами и звуками, что и на Mac.
- Хуки Claude Code идут через крошечный `coucou-hook.exe` и именованный канал, а плагин Hermes использует тот же relay; ключи хранятся в Windows Credential Manager.
- Подробности и отличия — в [`windows/README.md`](windows/README.md).

**Linux**

- То же приложение Tauri 2 со сменённым платформенным слоем: именованный канал становится Unix-сокетом в `$XDG_RUNTIME_DIR/coucou.sock` (проверка через `SO_PEERCRED`), пути переходят в XDG, ключи уходят в Secret Service, а установщик NSIS заменяют `.deb`/`.rpm`.
- Подробности, упаковка и то, чего ещё нет, — в [`linux/README.md`](linux/README.md).

## Участие

Issues и PR очень желательны — новые интеграции, новые эмоции, новые звуки, исправления ошибок. См. [CONTRIBUTING.md](CONTRIBUTING.md).

## Благодарности

Создано [Louis Raillé](https://louisraille.fr) вместе с Claude Code.
Вдохновлено концепциями компаньонов для выреза, которыми делились дизайн-студии — этот проект независим и не аффилирован ни с одной из них.

## Лицензия

- **Код:** [MIT](LICENSE) — используйте, форкайте, учитесь на нём, только сохраните уведомление об авторских правах.
- **Название, персонаж Mochi, иконка, звуки и медиа:** © Louis Raillé, все права защищены — см. [LICENSE-ASSETS.md](LICENSE-ASSETS.md). Выпускаете собственный форк? Дайте ему собственное имя и персонажа.

<div align="center">

**Если Mochi вызвал у вас улыбку, ⭐ очень поможет.**

[Website](https://louis-cfm.github.io/coucou/) · [Privacy](https://louis-cfm.github.io/coucou/privacy.html) · [Terms](https://louis-cfm.github.io/coucou/terms.html) · [Support](https://louis-cfm.github.io/coucou/support.html)

</div>
