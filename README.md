<img style="width: 100%" src="https://github.com/ferrumc-rs/ferrumc/assets/README/header.svg?raw=true" alt="FerrumC Header">
<div align="center">
    <img src="https://img.shields.io/github/license/ferrumc-rs/ferrumc?style=for-the-badge&color=red" alt="License">
    <img src="https://img.shields.io/github/languages/code-size/ferrumc-rs/ferrumc?style=for-the-badge&color=red" alt="Code Size">
    <img src="https://www.aschey.tech/tokei/github.com/ferrumc-rs/ferrumc?style=for-the-badge&color=red" alt="Lines of Code">
    <img src="https://img.shields.io/badge/language-Rust-orange?style=for-the-badge&color=red" alt="Language">
    <a  href="https://discord.gg/qT5J8EMjwk">
    <img alt="Discord" src="https://img.shields.io/discord/1277314213878173726?style=for-the-badge&logo=discord&logoColor=red&color=red&link=https%3A%2F%2Fdiscord.gg%2FqT5J8EMjwk">
    </a>
</div>

<div align="center">
    <a href="#-about">About</a>
    •
    <a href="#-key-features">Features</a>
    •
    <a href="#-getting-started">Getting Started</a>
    •
    <a href="#%EF%B8%8F-development">Development</a>
    •
    <a href="#-license">License</a>
    •
    <a href="#-faq">FAQ</a>
</div>

## 📖 About

FerrumC is a **1.21.1** Minecraft server implementation written from the ground up in Rust. Leveraging the power of the
Rust
programming language, it is completely multithreaded and offers high performance as well as amazing memory efficiency!

<img src="https://github.com/ferrumc-rs/ferrumc/assets/README/in_game.png?raw=true" alt="In-game screenshot">

## ✨ Key Features

<ul>
   <li>
     <h4>🛈 Customizable server list</h4>
     <img src="https://github.com/ferrumc-rs/ferrumc/assets/README/server%20list.png?raw=true" alt="Server list">
   </li>
   <li>
     <h4>🚄 Extremely fast and adaptable update speeds</h4>
     <img src="https://github.com/ferrumc-rs/ferrumc/assets/README/mind%20boggling.gif?raw=true" alt="Mind boggling">
   </li>
   <li>
     <h4>🖥️ Highly efficient memory usage</h4>
     <img src="https://github.com/ferrumc-rs/ferrumc/assets/README/mem_use.png?raw=true" alt="Low memory usage">
   </li>
   <li>
     <h4>🗂️ Customizable configuration</h4>
     <img src="https://github.com/ferrumc-rs/ferrumc/assets/README/config.png?raw=true" alt="Configuration">
   </li>
   <li>
      <h4>🔄 Can import existing worlds from vanilla minecraft</h4>
      <img src="https://github.com/ferrumc-rs/ferrumc/assets/README/chunk_importing.gif?raw=true" alt="Configuration">
   </li>
   <li>
      <h4>🌐 Compatible with vanilla Minecraft clients (Currently only 1.21.1)</h4>
   </li>
   <li>
      <h4>💪 Powerful Entity Component System to handle high entity loads</h4>
      <img src="https://github.com/ferrumc-rs/ferrumc/assets/README/ECSBlockDiagram.png?raw=true" alt="Entity Component System">
      <p><i>ECS Block Diagram (credits: <a href="https://docs.unity3d.com/Packages/com.unity.entities@0.1/manual/ecs_core.html">Unity</a>)</i></p>
   </li>
   <li>
      <h4>📦 Fully multithreaded; Utilizes all available CPU cores, instead of a single "main" thread</h4>
   </li>
   <li>
      <h4>📝 Custom made network, NBT and Anvil encoding systems to allow for minimal I/O lag</h4>
   </li>
   <li>
      <h4>💾 Multiple database options to finetune the server to your needs</h4>
      <i>32 render distance*</i>
      <img src="https://github.com/ferrumc-rs/ferrumc/assets/README/chunk_loading.gif?raw=true" alt="Chunk Loading DEMO">
   </li>
</ul>

<h1>✅ Upcoming features</h1>

<ul>
   <li>
      <h4>Ability to view other players</h4>
   </li>
   <li>
      <h4>World modification (place / break blocks etc)</h4>
   </li>
    <li>
        <h4>Chat & Command system</h4>
   </li>
    <li>
        <h4>Optimizations</h4>
   </li>
    <li>
        <h4>Plugin support (JVM currently, other languages will be considered later)</h4>
    </li>
</ul>

## 🚀 Getting Started

### Prerequisites

- Rust compiler (latest nightly version)
- Cargo (comes with Rust)

## 📥 Installation

[//]: # (#### Option 1: Download pre-compiled binary &#40;Maybe outdated!&#41;)

[//]: # ()

[//]: # (1. Go to the [Releases]&#40;https://github.com/ferrumc-rs/ferrumc/releases&#41; page)

[//]: # (2. Download the latest version for your operating system)

[//]: # (3. Extract the archive to your desired location)

Unfortunately, the server is **not yet ready for production use**. We are still in the early
stages of development and are working hard to add more features and fix bugs.
For now, you can either **compile** the server from source or **download** from Github Actions.

### [Option 1] Download from Github Actions

![Where To Find](https://github.com/ferrumc-rs/ferrumc/assets/README/download_prebuilt.gif?raw=true)

1. Go to the [Actions](https://github.com/ferrumc-rs/ferrumc/actions) tab
2. Click on the latest build
3. Scroll all the way down to the `Artifacts` section
4. Download the artifact for your operating system (Windows, Linux, or macOS)
5. Follow the instructions in the `Usage` section

### [Option 2] Compile from source

##### Clone and build the project.

```bash
# Clone the repository
git clone https://github.com/ferrumc-rs/ferrumc
cd ferrumc

# Build the project
cargo build --release
```

#### The binary will be in target/release/

## 🖥️ Usage

1. Move the FerrumC binary (`ferrumc.exe` or `ferrumc` depending on the OS) to your desired server directory
2. Open a terminal in that directory
3. (Optional) Generate a config file: `./ferrumc --setup`
    - Edit the generated `config.toml` file to customize your server settings
4. Import an existing world: Either copy your world files to the server directory or specify the path to the world files
   in the `config.toml` file. This should be the root directory of your world files, containing the `region` directory
   as well as other folders like DIM1, playerdata, etc. The default import path is `import` so you should end up with a
   directory structure like this:
    ```
    server_directory
    ├── config.toml
    ├── ferrumc.exe
    ├── import
    │   ├── region
    │   │   ├── r.0.0.mca
    │   │   ├── r.0.1.mca
    │   │   ├── ...
    │   ├── DIM1
    │   ├── playerdata
    │   ├── ...
    ```
    - The location of these files is explained [here](https://minecraft.wiki/w/Region_file_format#Location).
5. Run the server:
    - Windows: `.\ferrumc.exe`
    - Linux/macOS: `./ferrumc`
    - You can change logging level by using `--log=<level>`:
        - e.g. `.\ferrumc.exe --log=info` for info level logging
        - Possible values:
            - `trace` (Extremely verbose)
            - `debug` (Default, Slightly verbose, used for debugging)
            - `info` (**Recommended**, useful information)
            - `warn` (Only warnings)
            - `error` (Only errors)

## 🛠️ Development

We welcome contributions! If you'd like to contribute to FerrumC, please follow these steps:

1. Fork the repository
2. Create a new branch for your feature
3. Implement your changes
4. Write or update tests as necessary
5. Submit a pull request

*Please* join our [Discord server](https://discord.gg/qT5J8EMjwk) to get help or discuss the project!
Also have a look at our [CONTRIBUTING.md](CONTRIBUTING.md) file for more information.

## ❔ FAQ

### How does this project differ from:

- **Valence**: Valence is a framework for building your own custom server by pulling in different components of their
  library. FerrumC aims to be a full replacement for the vanilla server. It's like
  the difference between buying the ingredients to make a meal yourself or just buying a pre-made meal.
- **Minestom**: Same as Valence, it's a framework to build your own server, which is different to what we are trying to
  do.
- **Paper/Spigot/Bukkit**: These are all great tools and have undoubtedly set the groundwork for projects like this to
  exist, but ultimately they are still somewhat bound to the original server implementation. We aim to write the entire
  server from the ground up, hopefully giving us a leg up.
- **Pumpkin**: It really doesn't differ that much. We are both trying to achieve the same thing. It's also not a
  competition, we are both aware of each other's progress and to be honest the Pumpkin team are doing really well. We
  won't tolerate any disrespect towards them as they are also undertaking the same monumental task.

### Will we be implementing terrain generation?

Yes! Not currently on our list of priorities and it's very unlikely that we will be able to have 1:1 terrain generation
with the vanilla server, but we do plan on implementing some sort of terrain generation as soon as we can.

### Will there be plugins? And how?

We do very much plan to have a plugin system and as of right now, our plan is to leverage the
JVM to allow for plugins to be written in Kotlin, Java, or any other JVM language. We are also considering other
languages
such as Rust, JavaScript and possibly other native languages, but that is a fair way off for now.

### What does 'FerrumC' mean?

It's a play on words. Ferrum is the Latin word for iron and it ***rust***s. And MC (Minecraft) in the end.
So it becomes Ferru*mc*. Get it? 😄

## 📜 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE) file for details.

## 🌟 Star History

<a href="https://star-history.com/#ferrumc-rs/ferrumc&Date">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=ferrumc-rs/ferrumc&type=Date&theme=dark" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=ferrumc-rs/ferrumc&type=Date" />
   <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=ferrumc-rs/ferrumc&type=Date" />
 </picture>
</a>