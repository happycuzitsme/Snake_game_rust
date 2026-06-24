
# 🐍 Snake Game in Rust

A classic Snake game built in Rust, running in the terminal with real-time keyboard controls.

---

## 📋 Table of Contents

- [Features](#features)
- [Screenshots](#screenshots)
- [Installation](#installation)
- [How to Play](#how-to-play)
- [Controls](#controls)
- [Project Structure](#project-structure)
- [Technologies Used](#technologies-used)
- [Build Instructions](#build-instructions)
- [Development Stages](#development-stages)
- [Learnings](#learnings)
- [License](#license)

---

## ✨ Features

- 🎮 Classic snake movement with arrow keys
- 🍎 Random food spawning
- 📊 Score tracking
- 💥 Wall and self-collision detection
- 🔄 Game restart with 'r' key
- 🚪 Quit with 'q' key
- 🎨 Terminal-based rendering with borders
- ⚡ Real-time keyboard input (raw mode)

---

## 📸 Screenshots

```
#########################################
#                                       #
#              *                        #
#                                       #
#         oooo                          #
#         O                             #
#                                       #
#                                       #
#                                       #
#                                       #
#                                       #
#                                       #
#                                       #
#                                       #
#                                       #
#                                       #
#                                       #
#                                       #
#                                       #
#########################################
Score: 5
Controls: Arrow keys to move, q to quit
```

---

## 🚀 Installation

### Prerequisites

- Rust installed (https://rustup.rs/)
- Cargo (comes with Rust)

### Steps

```bash
# Clone the repository
git clone https://github.com/yourusername/snake_game.git
cd snake_game

# Build and run
cargo run

# Or build a release version
cargo build --release
./target/release/snake_game
```

---

## 🎮 How to Play

1. Run `cargo run`
2. Use **arrow keys** to control the snake
3. Eat the `*` (food) to grow and increase score
4. Avoid hitting walls or your own tail
5. Press **'r'** to restart after game over
6. Press **'q'** to quit anytime

---

## 🎯 Controls

| Key | Action |
|-----|--------|
| ⬆️ Up Arrow | Move Up |
| ⬇️ Down Arrow | Move Down |
| ⬅️ Left Arrow | Move Left |
| ➡️ Right Arrow | Move Right |
| **r** | Restart Game |
| **q** | Quit Game |

---

## 📁 Project Structure

```
snake_game/
├── src/
│   ├── snake/
│   │   └── mod.rs          # Snake logic (movement, direction, body)
│   ├── food/
│   │   └── mod.rs          # Food spawning (random positions)
│   ├── game/
│   │   └── mod.rs          # Game state (score, collisions, reset)
│   ├── ui/
│   │   └── mod.rs          # Terminal rendering
│   └── main.rs             # Game loop and input handling
├── Cargo.toml              # Dependencies and metadata
├── Cargo.lock              # Locked dependency versions
├── README.md               # This file
└── LICENSE                 # MIT License
```

---

## 🛠️ Technologies Used

| Technology | Purpose |
|------------|---------|
| **Rust** | Main programming language |
| **Cargo** | Build system and package manager |
| **rand** | Random food generation |
| **crossterm** | Terminal input/output handling |

### Dependencies

```toml
[dependencies]
rand = "0.8"        # Random number generation
crossterm = "0.27"  # Cross-platform terminal control
```

---

## 🔧 Build Instructions

### Debug Build (Fast Compile)

```bash
cargo build
./target/debug/snake_game
```

### Release Build (Fast Runtime)

```bash
cargo build --release
./target/release/snake_game
```

### Run Directly

```bash
cargo run
```

### Check Without Running

```bash
cargo check
```

### Clean Build Artifacts

```bash
cargo clean
```

---

## 📚 Development Stages

| Stage | What Was Built | Key Learnings |
|-------|----------------|---------------|
| **1** | Environment Setup | Cargo, toolchain, project creation |
| **2** | Project Structure | Modules, dependencies, file organization |
| **3** | Snake Logic | Structs, enums, vectors, movement, ownership |
| **4** | Food & Scoring | Random generation, Option type, error handling |
| **5** | Game Mechanics | Collision detection, game over, reset |
| **6** | UI & Polish | Terminal rendering, raw mode, keyboard input |
| **7** | Documentation | README, build instructions, final polish |

---

## 🧠 Key Learnings

### Rust Concepts Practiced

- ✅ Structs and Enums
- ✅ Impl blocks and methods
- ✅ Ownership and Borrowing
- ✅ Vectors and Slicing
- ✅ Pattern Matching (`match`)
- ✅ Error Handling (`Result`, `?`)
- ✅ Modules and Crates
- ✅ Trait implementations
- ✅ Closures and Iterators
- ✅ Terminal I/O with crossterm

### Game Development Concepts

- ✅ Game loop architecture
- ✅ State management
- ✅ Collision detection
- ✅ Real-time input handling
- ✅ Rendering to terminal

---

## 🐛 Troubleshooting

### "command not found: cargo"

```bash
source $HOME/.cargo/env
# Or add to ~/.bashrc
```

### "Permission denied" when running

```bash
chmod +x target/release/snake_game
```

### Compilation errors

```bash
cargo clean
cargo build
```

### Terminal not responding to keys

```bash
# Check raw mode is enabled
# Try running without raw mode:
cargo run --features no-raw
```

---

## 🚀 Future Improvements

- [ ] High score tracking (save to file)
- [ ] Speed increase as score grows
- [ ] Multiple difficulty levels
- [ ] Colorful rendering
- [ ] Sound effects
- [ ] Pause functionality
- [ ] Leaderboard
- [ ] WASM version for web

---

## 📝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 👨‍💻 Author

[Your Name]
- GitHub: [@yourusername](https://github.com/yourusername)

---

## 🙏 Acknowledgments

- Rust Book for guidance
- crossterm for terminal handling
- rand crate for random generation

---

## 🎯 Quick Commands Reference

```bash
# Build and run
cargo run

# Build release
cargo build --release

# Run release
./target/release/snake_game

# Check code
cargo check

# Clean build
cargo clean

# Update dependencies
cargo update

# Generate docs
cargo doc --open
```

---

## ⭐ Star the Repo

If you found this project helpful, please give it a ⭐ on GitHub!

---

**Happy Gaming! 🐍**
```

---

## 📝 Quick Command to Create README

```bash
cd ~/snake_game
cat > README.md << 'EOF'
# [paste the entire README above]
EOF
```

Or just paste it in nano:

```bash
nano README.md
# Paste everything, save with Ctrl+O, Ctrl+X
```

---

