# JoyMouse 🎮🐭

**JoyMouse** is a lightweight cross-platform input daemon
that lets you control analog stick input using your mouse.

JoyMouse emulates a virtual game controller and maps relative mouse movement
to the **right analog stick** — perfect for use with emulators
like [RPCS3](https://rpcs3.net/), especially in FPS or camera-based games.

---

## ✨ Features

- 🖱️ Mouse → Right Stick input translation
- 🎮 Evdev based virtual controller (cross-platform)
- 🧭 Recenter analog stick after idle
- ⚙️ Configurable sensitivity and deadzone
- 🐧 Linux support (via evdev/uinput)
- 💡 Built with modern Rust (1.87.0)

---

## 📸 Demo (Coming Soon)

> ![Demo GIF](./assets/demo.gif)

---

## 🚀 Getting Started

### 🛠️ Build Instructions

```bash
git clone https://github.com/clemenscodes/joymouse.git
cd joymouse
cargo build --release
```

## Contributing

Contributions are welcome!
Please have a look at the [issues](https://github.com/clemenscodes/joymouse-rs/issues)
to see what currently needs help
or what features are planned and just pick what you like.
