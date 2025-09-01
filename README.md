# JoyMouse 🎮🐭

_Turn your mouse into a virtual gamepad!_

**JoyMouse** is a lightweight, cross-platform input daemon
that emulates a virtual game controller, mapping mouse movement to analog stick input.
Ideal for emulators like [RPCS3](https://rpcs3.net/),
particularly in first-person shooters
or camera-driven games where mouse precision is key.

---

## ✨ Features

-   🖱️ **Mouse → Right Analog Stick** mapping
-   🎮 **Virtual Controller Emulation**
    -   **Linux:** Uses `evdev` for input parsing and `uinput` to create a virtual controller
    -   **Windows:** Uses `device query` for input and [`ViGEmClient`](https://github.com/ViGEm/ViGEmClient) + [ViGEmBus](https://github.com/ViGEm/ViGEmBus) for virtual gamepad creation
-   ⚙️ **Customizable Settings**
-   🧭 Auto recenter analog stick after idle
-   🐧 **Linux support** (fully functional)
-   💻 **Windows support** (in progress)
-   🔒 Built with **modern Rust** (≥1.87.0)

---

## 🖥️ How It Works

-   **Linux:**

    -   Reads input events using [`evdev`](https://www.freedesktop.org/wiki/Software/libevdev/)
    -   Creates a virtual device via [`uinput`](https://www.kernel.org/doc/html/latest/input/uinput.html)
    -   Emits events as if from a real controller

-   **Windows:**
    -   Parses events using device APIs
    -   Emulates a virtual controller using [`ViGEmClient`](https://github.com/ViGEm/ViGEmClient)
        and [`ViGEmBus`](https://github.com/ViGEm/ViGEmBus)

---

## 🏗 Architecture

```text
+---------------------+
|    JoyMouse App     |
+---------------------+
          |
          v
+---------------------------+
|  PlatformControllerManager|
|  (Cross-platform logic)   |
+---------------------------+
          |
          v
+-------------------------------+
| VirtualController             |
| - left stick / right stick    |
| - handle events               |
| - emit events                 |
+-------------------------------+
          |
          v
+-------------------------+
| PlatformControllerOps   |
| (OS-specific backend)   |
|                         |
| LinuxOps:               |
|  - evdev (Input Read)   |
|  - uinput (Virtual Pad) |
|                         |
| WindowsOps (WIP):       |
|  - Device API (Input)   |
|  - ViGEmClient + Bus    |
+-------------------------+
```

---

## 📸 Demo (Coming Soon)

> ![Demo GIF](./assets/demo.gif)

---

## 🚀 Getting Started

### 🔽 Downloads

The latest releases can be downloaded from the [Downloads section](https://github.com/clemenscodes/joymouse-rs/releases/tag/latest).

For Windows, you can directly download the binary here:  
[joymouse.exe](https://github.com/clemenscodes/joymouse-rs/releases/download/latest/joymouse.exe)
To create the virtual controller, installing the ViGEmBus Driver is required. You can download the driver [here](https://github.com/nefarius/ViGEmBus/releases/download/v1.22.0/ViGEmBus_1.22.0_x64_x86_arm64.exe).
Just double click the executable file. It should open a window that says that JoyMouse started.

For Linux, you can directly download the binary here:  
[joymouse](https://github.com/clemenscodes/joymouse-rs/releases/download/latest/joymouse-musl-static)
For a seemless experience, it is recommended to add your user to the `input` group. This allows reading `/dev/input` without needing `sudo` and thus running the binary without `sudo`.
To add your user to the `input` group, run the following command `sudo usermod -a -G input $USER`.
Then simply run the downloaded binary in the terminal.

### 🛠️ Build Instructions

```bash
git clone https://github.com/clemenscodes/joymouse.git
cd joymouse
cargo build --release
```

---

## ⚙️ Configuration

JoyMouse uses two TOML configuration files located in your OS-specific configuration directory:

-   **Linux:** `~/.config/joymouse/`
-   **Windows:** `%APPDATA%\joymouse\`

### **1. joymouse.toml** – Mouse & Stick Settings

This file defines settings for right analog stick emulation and tuning.

```toml
tickrate = 16
mouse_idle_timeout = 64
max_tilt_range = 32767.0
min_tilt_range = 13107.0
sensitivity = 7.0
blend = 0.2
diagonal_boost = 1.41
angle_delta_limit = 0.5
speed_stabilize_threshold = 200.0
min_speed_clamp = 1.0
max_speed_clamp = 500.0
motion_threshold_micro_macro = 0.025
motion_threshold_macro_flick = 0.5
motion_threshold_macro_micro = 0.03
motion_threshold_micro_macro_recover = 0.01
```

#### **Key Settings**

| Setting                                | Description                                               |
| -------------------------------------- | --------------------------------------------------------- |
| `tickrate`                             | Update interval in milliseconds (lower = faster response) |
| `mouse_idle_timeout`                   | Time in ms before the stick auto-centers when idle        |
| `max_tilt_range`                       | Maximum analog stick tilt value                           |
| `min_tilt_range`                       | Minimum analog stick tilt value (deadzone)                |
| `sensitivity`                          | Mouse-to-stick sensitivity multiplier                     |
| `blend`                                | Smoothing factor between micro and macro movement         |
| `diagonal_boost`                       | Multiplier for diagonal movement                          |
| `angle_delta_limit`                    | Max allowed angle change per update                       |
| `speed_stabilize_threshold`            | Speed where input stabilizes                              |
| `min_speed_clamp`                      | Minimum clamped speed                                     |
| `max_speed_clamp`                      | Maximum clamped speed                                     |
| `motion_threshold_micro_macro`         | Threshold to switch micro → macro motion                  |
| `motion_threshold_macro_flick`         | Threshold for fast flick detection                        |
| `motion_threshold_macro_micro`         | Threshold to switch macro → micro motion                  |
| `motion_threshold_micro_macro_recover` | Threshold to recover from macro to micro                  |

### **2. bindings.toml** – Button Mappings

This file defines which keyboard keys or mouse buttons map to virtual controller buttons.

```toml
south = ["space"]
east = ["left_ctrl"]
north = ["f"]
west = ["c", "mouse_side"]

up = ["up", "k", "2"]
down = ["down", "j", "4"]
left = ["left", "h", "1"]
right = ["right", "l", "3"]

forward = ["w"]
backward = ["s"]
starboard = ["d"]
port = ["a"]

l1 = ["mouse_right"]
r1 = ["mouse_left"]
l2 = ["q", "mouse_extra"]
r2 = ["x"]

l3 = ["left_alt"]
r3 = ["v"]

start = ["enter"]
select = ["tab"]
```

#### **Supported Actions**

| Action                                     | Description            |
| ------------------------------------------ | ---------------------- |
| `south`, `east`, `north`, `west`           | Face buttons (A/B/X/Y) |
| `up`, `down`, `left`, `right`              | D-Pad directions       |
| `forward`, `backward`, `starboard`, `port` | Movement keys          |
| `l1`, `r1`                                 | Shoulder buttons       |
| `l2`, `r2`                                 | Triggers               |
| `l3`, `r3`                                 | Stick click buttons    |
| `start`, `select`                          | Menu buttons           |

### **Key Notes**

-   Multiple keys can be bound to the same action by listing them in an array.
-   Supported input names:
    -   **Keyboard keys**: `a`, `space`, `enter`, `tab`, `up`, `down`, etc.
    -   **Mouse buttons**: `mouse_left`, `mouse_right`, `mouse_side`, `mouse_extra`.

## Contributing

Contributions are welcome!
Please have a look at the [issues](https://github.com/clemenscodes/joymouse-rs/issues)
to see what currently needs help
or what features are planned and just pick what you like.
