<img style="width: 100%" src="https://github.com/Sweattypalms/ferrumc/blob/dev/README/assets/header.svg?raw=true" alt="FerrumC Header">
<div align="center">
    <img src="https://img.shields.io/github/license/Sweattypalms/ferrumc?style=for-the-badge&color=red" alt="License">
    <img src="https://img.shields.io/github/languages/code-size/Sweattypalms/ferrumc?style=for-the-badge&color=red" alt="Code Size">
    <img src="https://www.aschey.tech/tokei/github.com/Sweattypalms/ferrumc?style=for-the-badge&color=red" alt="Lines of Code">
    <img src="https://img.shields.io/badge/language-Rust-orange?style=for-the-badge&color=red" alt="Language">
</div>
<p align="center">
  <a href="https://discord.gg/qT5J8EMjwk">
    <img src="https://img.shields.io/discord/1277314213878173726?color=7289DA&label=Join%20our%20Discord&logo=discord&logoColor=white" alt="Join our Discord&style=for-the-badge">
  </a>
</p>

<div align="center">
    <a href="#-about">About</a>
    •
    <a href="#-current-features-and-roadmap">Features</a>
    •
    <a href="#-getting-started">Getting Started</a>
    •
    <a href="#-development">Development</a>
    •
    <a href="#-license">License</a>
    •
    <a href="#-acknowledgments">Acknowledgments</a>

</div>

## 📖 About

FerrumC is a Minecraft server implementation written from the ground up in Rust. Leveraging the power of the Rust
programming language, it achieves high performance and low latency as well as amazing memory efficiency!

<img src="https://github.com/Sweattypalms/ferrumc/blob/dev/README/assets/in_game.png?raw=true" alt="In-game screenshot">


<h1>✨ Key Features</h1>

<ul>
   <li>
     <h4>🛈 Customizable server list</h4>
     <img src="https://github.com/Sweattypalms/ferrumc/blob/dev/README/assets/server%20list.png?raw=true" alt="Server list">
   </li>
   <li>
     <h4>🚄 Extremely fast and adaptable update speeds</h4>
     <img src="https://github.com/Sweattypalms/ferrumc/blob/dev/README/assets/mind%20boggling.gif?raw=true" alt="Mind boggling">
   </li>
   <li>
     <h4>🖥️ Highly efficient memory usage</h4>
     <img src="https://github.com/Sweattypalms/ferrumc/blob/dev/README/assets/mem_use.png?raw=true" alt="Low memory usage">
   </li>
   <li>
     <h4>🗂️ Customizable configuration</h4>
     <img src="https://github.com/Sweattypalms/ferrumc/blob/dev/README/assets/config.png?raw=true" alt="Configuration">
   </li>
   <li>
      <h4>🔄 Can import existing worlds from vanilla minecraft</h4>
      <img src="https://github.com/Sweattypalms/ferrumc/blob/dev/README/assets/importing_chunks.png?raw=true" alt="Configuration">
   </li>
   <li>
      <h4>🌐 Compatible with vanilla Minecraft clients (Currently only 1.20.1)</h4>
   </li>
   <li>
      <h4>🛠 Open-source for community contributions and customization</h4>
   </li>
   <li>
      <h4>⚡ Built with Rust for memory safety and concurrency</h4>
   </li>
   <li>
      <h4>💪 Powerful Entity Component System to handle high entity loads</h4>
      <img src="https://github.com/Sweattypalms/ferrumc/blob/dev/README/assets/ECSBlockDiagram.png?raw=true" alt="Entity Component System">
      <p><i>ECS Block Diagram (credits: unity)</i></p>
   </li>
   <li>
      <h4>📦 Asynchronous networking stack to compartmentalise individual players</h4>
   </li>
   <li>
      <h4>📝 Custom made network and NBT encoding system to allow for minimal I/O lag</h4>
   </li>
   <li>
      <h4>💾 Lighting fast database to ensure optimal world loading speeds</h4>
      <h6><i>[Currently using RocksDB](https://github.com/facebook/rocksdb)</i></h6>
      <img src="https://github.com/Sweattypalms/ferrumc/blob/dev/README/assets/chunk_loading.gif?raw=true" alt="Chunk Loading DEMO">
   </li>
</ul>

<h1>✅ Upcoming features</h1>

<ul>
   <li>
      <h4>IP and account based whitelisting/blacklisting</h4>
   </li>
   <li>
      <h4>Cross-platform WASM plugin system</h4>
   </li>
   <li>
      <h4>SIMD accelerated lighting calculations</h4>
   </li>
   <li>
      <h4>Multithreaded terrain generation</h4>
   </li>
   <li>
      <h4>Custom dimensions</h4></li>
   <li>
      <h4>Parallel physics processing</h4>
   </li>
</ul>

## 🚀 Getting Started

### Prerequisites

- Rust compiler (latest nightly version)
- Cargo (comes with Rust)
- LLVM (required for RocksDB compilation)

### 📥 Installation

[//]: # (#### Option 1: Download pre-compiled binary &#40;Maybe outdated!&#41;)

[//]: # ()

[//]: # (1. Go to the [Releases]&#40;https://github.com/Sweattypalms/ferrumc/releases&#41; page)

[//]: # (2. Download the latest version for your operating system)

[//]: # (3. Extract the archive to your desired location)

<p>
Unfortunately, the server is not yet ready for production use. If you want to try it out, you can compile it from source.
</p>

#### Compile from source (Bleeding edge updates, always up-to-date)

1. Ensure you have LLVM installed on your system. This is required for RocksDB compilation. 
   - The env variable `LIBCLANG_PATH` must be set to the path of the `[LLVM path]/bin`. 

2. Clone and build the project.

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
    - Windows: `.\ferrumc.exe`
    - Linux/macOS: `./ferrumc`
4. (Optional) Generate a config file: `./ferrumc --setup`
5. Edit the generated `config.toml` file to customize your server settings
6. (Optional) Import an existing world: Place the region files (`.mca`) in the folder named `import` then run
   `./ferrumc --import`.
   The location of these files is explained [here](https://minecraft.wiki/w/Region_file_format#Location).

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
