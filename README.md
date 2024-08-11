[//]: # (<div align="center" style="background: linear-gradient&#40;to bottom, #0d0f10 80%, rgba&#40;13,15,16,0&#41;&#41;; padding: 60px 0 40px; width: auto;">)

[//]: # (    <div style="width: 200px; height: 200px; margin: 0 auto; position: relative;">)

[//]: # (        <div style="position: absolute; top: 0; left: 0; right: 0; bottom: 0; background: #ff6600; filter: blur&#40;20px&#41; opacity&#40;0.7&#41;; border-radius: 20px;"></div>)

[//]: # (        <img src="icon.png" alt="FerrumC Icon" width="200" style="position: relative; border-radius: 20px;">)

[//]: # (    </div>)

[//]: # (    <h1 style="color: #ff6600; font-size: 38px; margin-top: 30px; letter-spacing: 2px;">FerrumC</h1>)

[//]: # (    <p style="color: #ffa366; font-size: 14px; max-width: 600px; margin: 10px auto;">A high-performance Minecraft server implementation, crafted in Rust for unparalleled speed and efficiency</p>)

[//]: # (</div>)

<div style="width: 100%">
   <img src="README/assets/header.svg" alt="FerrumC Header">
</div>


<p align="center">
  <img src="https://img.shields.io/github/license/Sweattypalms/ferrumc" alt="License">
  <img src="https://img.shields.io/github/languages/code-size/Sweattypalms/ferrumc" alt="Code Size">
  <img src="https://img.shields.io/badge/language-Rust-orange" alt="Language">
</p>

## 📖 About

FerrumC is a Minecraft server implementation written from the ground up in Rust with performance in mind. Leveraging the power of the Rust programming language, FerrumC achieves high performance and low latency, making it an ideal choice for Minecraft server hosting.

### ✨ Key Features

- 🚀 High performance and low latency
- 🔄 Drop-in replacement for vanilla Minecraft server
- 🌐 Compatible with all vanilla Minecraft clients (Version: 1.20.1)
- 🛠 Open-source for community contributions and customization
- ⚡ Built with Rust for memory safety and concurrency

## 🎯 Current Features and Roadmap

<details>
<summary><b>✅ Implemented Features</b></summary>

- Basic server setup and configuration
- Server list ping
- Player connection and authentication
- Entity Component System
- Packet handling, serialization, and deserialization
- Great logging system
- Keep-alive system

</details>

<details>
<summary><b>🔨 In Progress</b></summary>

- NBT serialization and deserialization
- World stuff (chunks loading, saving, saving etc.)
- Database integration (embedded)

</details>

<details>
<summary><b>📅 Planned Features</b></summary>

- Chat system
- Advanced world generation
- Plugin support + API (Rust and Lua)
- Multi-world support
- Performance optimizations

</details>

## 🚀 Getting Started

### Prerequisites

- Rust compiler (latest stable version)
- Cargo (comes with Rust)

### 📥 Installation

#### Option 1: Download pre-compiled binary (Maybe outdated!)

1. Go to the [Releases](https://github.com/Sweattypalms/ferrumc/releases) page
2. Download the latest version for your operating system
3. Extract the archive to your desired location

#### Option 2: Compile from source (Bleeding edge updates, always up-to-date)

```bash
# Clone the repository
git clone https://github.com/Sweattypalms/ferrumc
cd ferrumc

# Build the project
cargo build --release
```
### The binary will be in target/release/


### 🖥️ Usage

1. Move the FerrumC binary to your desired server directory
2. Open a terminal in that directory
3. Run the server:
    - Windows: `./ferrumc.exe`
    - Linux/macOS: `./ferrumc`
4. (Optional) Generate a config file: `./ferrumc --setup`
5. Edit the generated `config.toml` file to customize your server settings

## 🛠️ Development

We welcome contributions! If you'd like to contribute to FerrumC, please follow these steps:

1. Fork the repository
2. Create a new branch for your feature
3. Implement your changes
4. Write or update tests as necessary
5. Submit a pull request


## 📜 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.


## 🙏 Acknowledgments

- [wiki.vg](https://wiki.vg): Used for protocol documentation
- [Tokio Runtime](https://github.com/tokio-rs/tokio): Asynchronous runtime for Rust
- [Valence](https://github.com/valence-rs/valence): VarInt/VarLong encoding and decoding