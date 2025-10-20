# Building Audio Plugins in Rust: Project Setup and Foundation

**Part 1 of the Naughty and Tender Development Series**

---

## What You'll Learn

By the end of this article, you'll understand:

- ✅ Why Rust is a compelling choice for audio plugin development
- ✅ How to set up a complete Rust development environment for audio work
- ✅ The structure of a Cargo workspace for plugin projects
- ✅ How to integrate the nih-plug framework
- ✅ Best practices for organizing shared code and build tooling

**Prerequisites:**
- [ ] Basic programming experience (any language)
- [ ] Familiarity with command-line tools
- [ ] Willingness to learn Rust fundamentals
- [ ] A DAW (Digital Audio Workstation) for testing - we use Reaper

**Time Estimate:** 45-60 minutes

---

## Introduction: Why Rust for Audio Plugins?

Audio plugins have traditionally been written in C++. The VST SDK, Audio Units, and most major plugin frameworks are C++ codebases. So why consider Rust?

### The C++ Legacy

> **💡 Tooltip: What's a VST/Audio Plugin?**
>
> VST (Virtual Studio Technology) plugins are software components that process audio in real-time within a DAW. They can be synthesizers (generate sound), effects (modify sound), or MIDI processors. Learn more in the [Steinberg VST documentation](https://www.steinberg.net/vst-developer/).

C++ dominates audio development for good reasons:
- **Performance**: Direct memory control, zero-cost abstractions
- **Ecosystem**: Decades of libraries, frameworks, and examples
- **Real-time safety**: Manual memory management allows precise control

But C++ also brings challenges:
- **Memory safety**: Manual memory management leads to bugs (use-after-free, double-free, buffer overflows)
- **Undefined behavior**: Easy to write code that compiles but behaves unpredictably
- **Concurrency**: Data races are easy to introduce and hard to debug

### Enter Rust

Rust offers a compelling alternative:

**1. Memory Safety Without Garbage Collection**
```rust
// This won't compile - Rust prevents use-after-free at compile time
let data = vec![1.0, 2.0, 3.0];
let reference = &data[0];
drop(data);  // ❌ ERROR: cannot move out of `data` because it is borrowed
println!("{}", reference);
```

> **💡 Tooltip: What's a Garbage Collector?**
>
> A garbage collector (GC) automatically frees unused memory, but introduces unpredictable pauses - unacceptable in real-time audio. Rust achieves memory safety through *ownership rules* checked at compile time, not runtime. Learn more in [The Rust Book](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html).

**2. Fearless Concurrency**
```rust
// Rust prevents data races at compile time
let mut buffer = vec![0.0; 1024];
let thread = std::thread::spawn(|| {
    // ❌ ERROR: cannot move `buffer` while borrowed mutably
    process_audio(&mut buffer);
});
buffer[0] = 1.0; // Already borrowed by thread - compiler catches this!
```

**3. Modern Tooling**
- **Cargo**: Dependency management that Just Works™
- **rustfmt**: Automatic code formatting
- **clippy**: Linting that catches common mistakes
- **cargo doc**: Built-in documentation generation

**4. Real-Time Guarantees**
Rust's ownership system makes it easier to write code that's safe for real-time audio:
- No hidden allocations
- No garbage collection pauses
- Explicit control over memory layout
- Compiler enforces thread safety

### The Trade-offs

Rust isn't perfect for audio:
- **Smaller ecosystem**: Fewer audio libraries than C++
- **Learning curve**: Ownership and borrowing take time to internalize
- **Compilation time**: Rust can be slower to compile than C++

But for this project, we're betting that Rust's safety guarantees and modern tooling will pay dividends as the codebase grows.

> **📚 Further Reading:**
> - [The Rust Book](https://doc.rust-lang.org/book/) - Comprehensive Rust introduction
> - [Rust for Audio - Blog Series](https://www.seventeencups.net/posts/category-theory-and-audio-programming-rust/) - Why Rust for audio
> - [nih-plug Philosophy](https://github.com/robbert-vdh/nih-plug#why) - Framework design rationale

---

## System Setup

Let's get your development environment ready.

### Step 1: Install Rust

**All Platforms:**

Visit [rustup.rs](https://rustup.rs/) and follow the instructions, or:

```bash
# Unix-like systems (Linux, macOS)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# Download and run rustup-init.exe from rustup.rs
```

This installs:
- `rustc` - The Rust compiler
- `cargo` - Rust's build tool and package manager
- `rustup` - Toolchain manager (for updating Rust)

**Verify installation:**
```bash
rustc --version
# Should output: rustc 1.XX.X (commit-hash date)

cargo --version
# Should output: cargo 1.XX.X (commit-hash date)
```

> **💡 Tooltip: What's a Toolchain?**
>
> A toolchain is a collection of tools for building software. Rust's toolchain includes the compiler (`rustc`), package manager (`cargo`), and standard library. `rustup` manages which version you're using.

### Step 2: Install Git

**Why Git?** The nih-plug framework and its dependencies are distributed via GitHub. Cargo will clone these repositories during builds.

**Installation:**

```bash
# Check if already installed
git --version

# If not installed:
# Ubuntu/Debian
sudo apt-get install git

# macOS (via Homebrew)
brew install git

# Windows
# Download from https://git-scm.com/download/win
```

### Step 3: Platform-Specific Dependencies

**Linux (Ubuntu/Debian):**
```bash
# Build essentials
sudo apt-get install build-essential

# ALSA development files (for audio I/O)
sudo apt-get install libasound2-dev

# X11 development files (for GUI)
sudo apt-get install libx11-dev libxcursor-dev libxrandr-dev libxi-dev
```

**macOS:**
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

**Windows:**
```bash
# Install Visual Studio Build Tools or Visual Studio Community
# https://visualstudio.microsoft.com/downloads/
#
# During installation, select:
# - "Desktop development with C++"
# - Windows 10/11 SDK
```

> **⚠️ Troubleshooting: "linker not found" on Windows**
>
> If you see linker errors, ensure Visual Studio Build Tools are installed and in your PATH. Restart your terminal after installation.

### Step 4: Install a DAW

For testing plugins, you'll need a VST3-compatible DAW. We use **Reaper**:

- **Download**: [reaper.fm](https://www.reaper.fm/)
- **Why Reaper?** Lightweight, fully-featured, excellent VST3 support
- **Alternatives**: Bitwig, Ableton Live, FL Studio, Ardour

Install and launch Reaper to ensure it works. We'll load our plugin here later.

---

## Project Structure: The Cargo Workspace

Audio plugin projects benefit from a **workspace structure** - multiple related packages in one repository.

### Why a Workspace?

Imagine building three synthesizer plugins. Each needs:
- MIDI note-to-frequency conversion
- Common DSP utilities
- Shared real-time safety checks

Without a workspace, you'd duplicate this code three times. With a workspace:

```
audio-experiments/
├── Cargo.toml           # Workspace manifest
├── naughty-and-tender/  # Plugin 1
├── future-plugin-2/     # Plugin 2
└── shared/
    └── core/            # Shared utilities
```

All packages share dependencies, build artifacts, and common code.

> **💡 Tooltip: What's a Package vs. a Crate?**
>
> In Rust, a **package** is a bundle of code with a `Cargo.toml` file. It contains one or more **crates** (compilation units). A crate can be a library (`lib.rs`) or binary (`main.rs`). Learn more in [The Cargo Book](https://doc.rust-lang.org/cargo/reference/workspaces.html).

### Creating the Workspace

Let's build this from scratch.

**Step 1: Create the root directory**

```bash
mkdir audio-experiments
cd audio-experiments
```

**Step 2: Create the workspace `Cargo.toml`**

This file lives at the repository root and defines the workspace:

```toml
# Cargo.toml (root)

[workspace]
resolver = "2"
members = [
    "naughty-and-tender",
    "shared/*",
]
exclude = ["xtask"]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
license = "MIT OR Apache-2.0"

[workspace.dependencies]
# nih-plug framework
nih_plug = { git = "https://github.com/robbert-vdh/nih-plug.git" }

# Shared utilities
shared-core = { path = "shared/core" }

[profile.release]
lto = "thin"
strip = "symbols"

[profile.profiling]
inherits = "release"
debug = true
strip = "none"
```

**Let's break this down:**

```toml
[workspace]
resolver = "2"
```
> **What's `resolver = "2"`?** Cargo's dependency resolver algorithm. Version 2 (introduced in Rust 1.51) handles feature unification better. Always use "2" for new projects. [Resolver documentation](https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions)

```toml
members = [
    "naughty-and-tender",
    "shared/*",
]
```
> **Workspace members**: Packages that are part of this workspace. `shared/*` is a glob pattern - any directory in `shared/` is a member. This lets us add `shared/dsp`, `shared/gui`, etc. later.

```toml
exclude = ["xtask"]
```
> **Why exclude?** `xtask` is a build tool that needs its own dependency tree. Excluding it prevents cargo from treating it as a workspace member.

```toml
[workspace.package]
version = "0.1.0"
edition = "2021"
```
> **Shared metadata**: All workspace members inherit these values unless they override them. Keeps version numbers in sync.

```toml
[workspace.dependencies]
nih_plug = { git = "https://github.com/robbert-vdh/nih-plug.git" }
shared-core = { path = "shared/core" }
```
> **Dependency deduplication**: Define dependencies once, reference them in members with `{ workspace = true }`. Ensures all crates use the same version.

```toml
[profile.release]
lto = "thin"
strip = "symbols"
```
> **Release optimizations**:
> - `lto = "thin"`: Link-Time Optimization - makes binaries smaller and faster
> - `strip = "symbols"`: Remove debug symbols from final binary (reduces size)

```toml
[profile.profiling]
inherits = "release"
debug = true
strip = "none"
```
> **Profiling profile**: Optimized like release, but keeps debug symbols for profilers like `cargo flamegraph`. Use this when hunting performance issues.

---

## Creating the Shared Core Library

Before building the plugin, let's create shared utilities that future plugins can reuse.

**Step 1: Create directory structure**

```bash
mkdir -p shared/core/src
```

**Step 2: Create `shared/core/Cargo.toml`**

```toml
# shared/core/Cargo.toml

[package]
name = "shared-core"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
```

> **Note**: `version.workspace = true` inherits from the workspace `Cargo.toml`. No need to repeat metadata.

**Step 3: Implement `shared/core/src/lib.rs`**

```rust
//! Shared core utilities for audio DSP experiments
//!
//! This crate provides common utilities used across multiple plugin projects.

#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

/// Common audio constants
pub mod constants {
    /// Standard sample rates
    pub const SAMPLE_RATE_44100: f32 = 44100.0;
    pub const SAMPLE_RATE_48000: f32 = 48000.0;
    pub const SAMPLE_RATE_96000: f32 = 96000.0;

    /// MIDI constants
    pub const MIDI_NOTE_OFF: u8 = 0x80;
    pub const MIDI_NOTE_ON: u8 = 0x90;
    pub const MIDI_CC: u8 = 0xB0;

    /// MIDI note range
    pub const MIDI_MIN: u8 = 0;
    pub const MIDI_MAX: u8 = 127;
}

/// Utility functions for real-time safe operations
pub mod util {
    /// Convert MIDI note number to frequency in Hz
    /// Uses equal temperament tuning with A4 = 440 Hz
    #[inline]
    #[must_use]
    pub fn midi_note_to_freq(note: u8) -> f32 {
        // f = 440 * 2^((note - 69) / 12)
        const A4_FREQ: f32 = 440.0;
        const A4_NOTE: i32 = 69;

        A4_FREQ * 2.0_f32.powf((f32::from(note) - A4_NOTE as f32) / 12.0)
    }

    /// Clamp a value between min and max
    #[inline]
    #[must_use]
    pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midi_to_freq() {
        // A4 (MIDI 69) should be 440 Hz
        let freq = util::midi_note_to_freq(69);
        assert!((freq - 440.0).abs() < 0.01);

        // C4 (MIDI 60) should be ~261.63 Hz
        let freq = util::midi_note_to_freq(60);
        assert!((freq - 261.63).abs() < 0.1);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(util::clamp(5, 0, 10), 5);
        assert_eq!(util::clamp(-1, 0, 10), 0);
        assert_eq!(util::clamp(15, 0, 10), 10);
    }
}
```

**Code Analysis:**

```rust
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
```
> **Clippy lints**: These activate Rust's linter to catch common mistakes and non-idiomatic code. `pedantic` is strict but teaches good Rust style. [Clippy documentation](https://doc.rust-lang.org/clippy/)

```rust
#[inline]
#[must_use]
pub fn midi_note_to_freq(note: u8) -> f32 {
```
> **Attributes explained**:
> - `#[inline]`: Suggests to the compiler to inline this function (important for hot paths in audio processing)
> - `#[must_use]`: Compiler warning if return value is ignored (prevents bugs)
> - `pub`: Makes function public (callable from other crates)

```rust
const A4_FREQ: f32 = 440.0;
const A4_NOTE: i32 = 69;

A4_FREQ * 2.0_f32.powf((f32::from(note) - A4_NOTE as f32) / 12.0)
```
> **MIDI to frequency formula**: Equal temperament tuning. MIDI note 69 = A4 = 440 Hz. Each semitone up multiplies frequency by 2^(1/12). Learn more: [MIDI Tuning Standard](https://en.wikipedia.org/wiki/MIDI_tuning_standard)

```rust
#[cfg(test)]
mod tests {
```
> **Testing in Rust**: The `#[cfg(test)]` attribute means this module only compiles when running `cargo test`. Rust's built-in test framework encourages testing everything. [Testing documentation](https://doc.rust-lang.org/book/ch11-00-testing.html)

**Verify it works:**

```bash
cargo test --package shared-core
```

You should see:
```
running 2 tests
test tests::test_clamp ... ok
test tests::test_midi_to_freq ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## The nih-plug Framework

Now we need a framework to bridge our Rust code to the VST3/CLAP plugin APIs.

### Why nih-plug?

Several Rust plugin frameworks exist:
- **vst-rs**: Mature, VST 2.4 only (deprecated format)
- **rust-vst3**: Low-level VST3 bindings
- **nih-plug**: Modern, supports VST3 and CLAP, excellent ergonomics

We chose **nih-plug** because:
1. **Actively maintained** (as of 2024-2025)
2. **Supports modern formats** (VST3, CLAP)
3. **Excellent documentation** and examples
4. **Rust-idiomatic API** (not a C++ API wrapper)
5. **Built-in GUI support** (egui integration)

> **📚 nih-plug Resources:**
> - [GitHub Repository](https://github.com/robbert-vdh/nih-plug)
> - [API Documentation](https://nih-plug.robbertvanderhelm.nl/)
> - [Example Plugins](https://github.com/robbert-vdh/nih-plug/tree/master/plugins)

### Adding nih-plug to the Workspace

We already added nih-plug to `[workspace.dependencies]`:

```toml
nih_plug = { git = "https://github.com/robbert-vdh/nih-plug.git" }
```

> **Why `git =` instead of `version =`?** nih-plug isn't published to crates.io yet. We pull directly from GitHub. This is common for fast-moving Rust libraries.

---

## Setting Up the Build System: xtask

VST3 plugins aren't normal binaries - they're specially packaged bundles with metadata. We need a build tool.

### What is xtask?

`xtask` is a Rust pattern for project-specific build tasks. Instead of shell scripts or Makefiles, we write a Rust binary that lives in the repository.

> **💡 Tooltip: Why Not Just Cargo?**
>
> `cargo build` produces a `.dll`/`.so`/`.dylib`. VST3 needs a directory bundle (`.vst3`) with specific structure and metadata files. `xtask` packages everything correctly.

**Step 1: Create xtask structure**

```bash
mkdir -p xtask/src
```

**Step 2: Create `xtask/Cargo.toml`**

```toml
# xtask/Cargo.toml

[package]
name = "xtask"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]
nih_plug_xtask = { git = "https://github.com/robbert-vdh/nih-plug.git" }
```

> **`publish = false`**: Prevents accidentally publishing this internal tool to crates.io.

**Step 3: Create `xtask/src/main.rs`**

```rust
// xtask/src/main.rs

fn main() {
    nih_plug_xtask::main();
}
```

That's it! `nih_plug_xtask` provides all the bundling logic.

**How to use it:**

```bash
# From xtask directory
cd xtask
cargo run --release -- bundle <plugin-name> --release

# This will:
# 1. Build the plugin in release mode
# 2. Create VST3 bundle at target/bundled/<plugin-name>.vst3
# 3. Create CLAP bundle at target/bundled/<plugin-name>.clap
```

---

## Project Structure Summary

You should now have:

```
audio-experiments/
├── Cargo.toml              # Workspace root
├── shared/
│   └── core/
│       ├── Cargo.toml      # Shared utilities package
│       └── src/
│           └── lib.rs      # MIDI conversion, constants
├── xtask/
│   ├── Cargo.toml          # Build tool
│   └── src/
│       └── main.rs         # Delegates to nih_plug_xtask
└── naughty-and-tender/     # (We'll create this in Article 2)
```

**Verify the workspace:**

```bash
cargo check --workspace
```

This should download dependencies (might take a few minutes the first time) and verify everything compiles.

---

## Troubleshooting

### "cannot find linker" Error

**Symptoms:**
```
error: linker `link.exe` not found
  |
  = note: The system cannot find the file specified. (os error 2)
```

**Solution (Windows):**
1. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
2. Select "Desktop development with C++"
3. Restart terminal
4. Retry `cargo check`

**Solution (Linux):**
```bash
sudo apt-get install build-essential
```

### Git Clone Failures

**Symptoms:**
```
error: failed to clone repository `https://github.com/robbert-vdh/nih-plug.git`
```

**Solution:**
1. Check internet connection
2. Verify Git is installed: `git --version`
3. Clear cargo cache: `rm -rf ~/.cargo/git/checkouts/`
4. Retry

### Compilation Takes Forever

**Symptoms:** First build takes 10+ minutes

**Explanation:** Rust compiles all dependencies from source. First build is slow, subsequent builds are incremental (fast).

**Mitigation:**
```bash
# Use sccache for distributed caching (optional)
cargo install sccache
export RUSTC_WRAPPER=sccache
```

### Clippy Warnings Overwhelming

**Symptoms:** Hundreds of warnings from dependencies

**Solution:** Clippy only warns about *your* code, not dependencies. Add to `Cargo.toml`:

```toml
[workspace.lints.clippy]
# Only lint workspace code
workspace = true
```

Or run clippy only on your packages:
```bash
cargo clippy --package shared-core
```

---

## What's Next?

In **Article 2**, we'll build on this foundation to create the actual plugin shell:
- Implementing the `Plugin` trait
- Setting up the audio processing callback
- Creating a parameter system
- Building a basic GUI with egui
- Packaging and loading the plugin in Reaper

You now have a solid workspace structure and understand why Rust is a great fit for audio development. The hard part (project setup) is done!

---

## Quick Reference

**Essential Commands:**

```bash
# Check entire workspace
cargo check --workspace

# Test a specific package
cargo test --package shared-core

# Build plugin bundle
cd xtask && cargo run --release -- bundle naughty-and-tender --release

# Run clippy (linter)
cargo clippy --package shared-core

# Format code
cargo fmt --all
```

**Project Structure at a Glance:**

- `Cargo.toml` (root) → Workspace configuration
- `shared/core/` → Shared utilities across plugins
- `xtask/` → Build system for bundling VST3/CLAP
- `naughty-and-tender/` → The plugin itself (next article!)

---

## Further Reading

**Rust Fundamentals:**
- [The Rust Book](https://doc.rust-lang.org/book/) - Start here for Rust basics
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by doing
- [Cargo Book](https://doc.rust-lang.org/cargo/) - Deep dive on Cargo

**Audio Development:**
- [nih-plug Documentation](https://nih-plug.robbertvanderhelm.nl/)
- [VST3 SDK Documentation](https://steinbergmedia.github.io/vst3_doc/)
- [Real-Time Audio Programming 101](http://www.rossbencina.com/code/real-time-audio-programming-101-time-waits-for-nothing) - Essential reading

**Rust Audio Ecosystem:**
- [rust-audio Discourse](https://rust-audio.discourse.group/) - Community forum
- [Are We Audio Yet?](https://areweaudioyet.com/) - Rust audio crate directory

---

**Next Article:** [Building Audio Plugins in Rust: The Plugin Shell](./02-plugin-shell-architecture.md)

---

*Written as part of the Naughty and Tender development series - a journey building a MIDI synthesizer plugin in Rust.*
