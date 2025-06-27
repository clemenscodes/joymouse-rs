# JoyMouse 🎮🐭

**JoyMouse** is a lightweight cross-platform input daemon
that lets you control analog stick input using your mouse.

Built with modern Rust and [SDL3](https://github.com/libsdl-org/SDL),
JoyMouse emulates a virtual game controller and maps relative mouse movement
to the **right analog stick** — perfect for use with emulators
like [RPCS3](https://rpcs3.net/), especially in FPS or camera-based games.

---

## ✨ Features

- 🖱️ Mouse → Right Stick input translation
- 🎮 SDL3-based virtual controller (cross-platform)
- 🧭 Recenter analog stick after idle
- ⚙️ Configurable sensitivity and deadzone
- 🐧 Linux and 🪟 Windows support (via SDL3)
- 💡 Built with modern, clean C++ (C++17+)
- 📦 Easy to build with [Meson](https://mesonbuild.com/)

---

## 📸 Demo (Coming Soon)

> GIF of mouse input being translated to analog stick movement in an emulator window.

---

## 🚀 Getting Started

### 🧱 Dependencies

- [SDL3](https://github.com/libsdl-org/SDL) (dev package)

### 🛠️ Build Instructions

```bash
git clone https://github.com/clemenscodes/joymouse.git
cd joymouse
cargo build --release
```
