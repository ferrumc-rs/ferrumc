<div style="width: 100%">
   <img src="https://github.com/Sweattypalms/ferrumc/blob/dev/README/assets/header.svg?raw=true" alt="FerrumC Header">
</div>


<p align="center">
  <img src="https://img.shields.io/github/license/Sweattypalms/ferrumc" alt="License">
  <img src="https://img.shields.io/github/languages/code-size/Sweattypalms/ferrumc" alt="Code Size">
  <img src="https://img.shields.io/badge/language-Rust-orange" alt="Language">
</p>
<p align="center">
  <a href="https://discord.gg/qT5J8EMjwk">
    <img src="https://img.shields.io/discord/1277314213878173726?color=7289DA&label=Join%20our%20Discord&logo=discord&logoColor=white" alt="Join our Discord&style=for-the-badge">
  </a>
</p>


## 📖 About

FerrumC is a Minecraft server implementation written from the ground up in Rust. Leveraging the power of the Rust programming language, it achieves high performance and low latency as well as amazing memory efficiency!

![Minecraft Character](https://github.com/Sweattypalms/ferrumc/blob/dev/README/assets/in_game.png?raw=true)
![Server list](https://github.com/Sweattypalms/ferrumc/blob/dev/README/assets/server%20list.png?raw=true)
*FerrumC in action! This is updating almost 50k blocks in 10th of a second*
![Cool stuff](https://github.com/Sweattypalms/ferrumc/blob/dev/README/assets/mind%20boggling.gif?raw=true)

### ✨ Key Features

- 🚀 High performance and low latency as well as efficient resource usage
![Low memory usage](https://github.com/Sweattypalms/ferrumc/blob/dev/README/assets/low_mem_usage.png?raw=true)
- 🔄 Drop-in replacement for vanilla Minecraft server (kinda)
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
- NBT serialization and deserialization

</details>


<details>
<summary><b>🔨 In Progress</b></summary>

- World stuff (chunks loading, saving, etc.)
- Database integration (embedded)
- Entities and physics

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

Join our [Discord server](https://discord.gg/qT5J8EMjwk) to get help or discuss the project!


## 📜 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.


## 🙏 Acknowledgments

- [wiki.vg](https://wiki.vg): Used for protocol documentation
- [Tokio Runtime](https://github.com/tokio-rs/tokio): Asynchronous runtime for Rust
- [Valence](https://github.com/valence-rs/valence): VarInt/VarLong encoding and decoding
