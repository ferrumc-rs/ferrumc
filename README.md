<img style="width: 100%" src="https://github.com/ferrumc-rs/ferrumc/blob/dev/README/assets/header.svg?raw=true" alt="FerrumC Header">
<div align="center">
    <img src="https://img.shields.io/github/license/Sweattypalms/ferrumc?style=for-the-badge&color=red" alt="License">
    <img src="https://img.shields.io/github/languages/code-size/Sweattypalms/ferrumc?style=for-the-badge&color=red" alt="Code Size">
    <img src="https://www.aschey.tech/tokei/github.com/Sweattypalms/ferrumc?style=for-the-badge&color=red" alt="Lines of Code">
    <img src="https://img.shields.io/badge/language-Rust-orange?style=for-the-badge&color=red" alt="Language">
    <a  href="https://discord.gg/qT5J8EMjwk">
    <img alt="Discord" src="https://img.shields.io/discord/1277314213878173726?style=for-the-badge&logo=discord&logoColor=red&color=red&link=https%3A%2F%2Fdiscord.gg%2FqT5J8EMjwk">
    </a>
</div>

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
    •
    <a href="#-faq">FAQ</a>


</div>

## 📖 About

FerrumC is a Minecraft server implementation written from the ground up in Rust. Leveraging the power of the Rust
programming language, it is completely multithreaded; and offers high performance as well as amazing memory efficiency!

<img src="https://github.com/ferrumc-rs/ferrumc/blob/dev/README/assets/in_game.png?raw=true" alt="In-game screenshot">


## ✨ Key Features

<ul>
   <li>
     <h4>🛈 Customizable server list</h4>
     <img src="https://github.com/ferrumc-rs/ferrumc/blob/dev/README/assets/server%20list.png?raw=true" alt="Server list">
   </li>
   <li>
     <h4>🚄 Extremely fast and adaptable update speeds</h4>
     <img src="https://github.com/ferrumc-rs/ferrumc/blob/dev/README/assets/mind%20boggling.gif?raw=true" alt="Mind boggling">
   </li>
   <li>
     <h4>🖥️ Highly efficient memory usage</h4>
     <img src="https://github.com/ferrumc-rs/ferrumc/blob/dev/README/assets/mem_use.png?raw=true" alt="Low memory usage">
   </li>
   <li>
     <h4>🗂️ Customizable configuration</h4>
     <img src="https://github.com/ferrumc-rs/ferrumc/blob/dev/README/assets/config.png?raw=true" alt="Configuration">
   </li>
   <li>
      <h4>🔄 Can import existing worlds from vanilla minecraft</h4>
      <img src="https://github.com/ferrumc-rs/ferrumc/blob/dev/README/assets/importing/chunk_importing.gif?raw=true" alt="Configuration">
   </li>
   <li>
      <h4>🌐 Compatible with vanilla Minecraft clients (Currently only 1.20.1)</h4>
   </li>
   <li>
      <h4>💪 Powerful Entity Component System to handle high entity loads</h4>
      <img src="https://github.com/ferrumc-rs/ferrumc/blob/dev/README/assets/ECSBlockDiagram.png?raw=true" alt="Entity Component System">
      <p><i>ECS Block Diagram (credits: <a href="https://docs.unity3d.com/Packages/com.unity.entities@0.1/manual/ecs_core.html">Unity</a>)</i></p>
   </li>
   <li>
      <h4>📦 Fully multithreaded; Utilizes all available CPU cores, instead of a single "main" thread</h4>
   </li>
   <li>
      <h4>📝 Custom made network and NBT encoding system to allow for minimal I/O lag</h4>
   </li>
   <li>
      <h4>💾 Lighting fast database to ensure extremely fast world loading speeds</h4>
      <h6><i><a href="https://www.symas.com/lmdb">Currently using LMDB</a></i></h6>
      <i>32 render distance*</i>
      <img src="https://github.com/ferrumc-rs/ferrumc/blob/dev/README/assets/chunk_loading.gif?raw=true" alt="Chunk Loading DEMO">
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
        <h4>Plugin support (Javascript, Rust, and other WASM support languages)</h4>
    </li>
</ul>

## 🚀 Getting Started

### Prerequisites

- Rust compiler (latest nightly version)
- Cargo (comes with Rust)

## 📥 Installation

[//]: # (#### Option 1: Download pre-compiled binary &#40;Maybe outdated!&#41;)

[//]: # ()

[//]: # (1. Go to the [Releases]&#40;https://github.com/Sweattypalms/ferrumc/releases&#41; page)

[//]: # (2. Download the latest version for your operating system)

[//]: # (3. Extract the archive to your desired location)

Unfortunately, the server is **not yet ready for production use**. We are still in the early
stages of development and are working hard to add more features and fix bugs.
For now, you can either **compile** the server from source or **download** from Github Actions.

### [Option 1] Download from Github Actions

![Where To Find](https://github.com/ferrumc-rs/ferrumc/blob/dev/README/assets/download_prebuilt.gif?raw=true)

1. Go to the [Actions](https://github.com/sweattypalms/ferrumc/actions) tab
2. Click on the latest build
3. Scroll all the way down to the `Artifacts` section
4. Download the artifact for your operating system (Windows, Linux, or macOS)
5. Follow the instructions in the `Usage` section


### [Option 2] Compile from source 

##### Clone and build the project.

```bash
# Clone the repository
git clone https://github.com/Sweattypalms/ferrumc
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
4. Import an existing world: Place the region files (`.mca`) in the folder named `import` then run
   `./ferrumc --import`.
   - The location of these files is explained [here](https://minecraft.wiki/w/Region_file_format#Location).
   - If you want to modify batch size (default 150), you can use `./ferrumc --import --batch_size=<num>`.
     - Basically the number of chunks to import at once, higher => faster but more CPU intensive.
     - Max is 1024, since that's the max number of chunks in a region(`.mca`) file.
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

*Note: You can specify the directory to treat as the root directory (the place where the config files, data files,
etc. live) by setting an environment variable `FERRUMC_ROOT` to the path of the directory. For example, I run
`set FERRUMC_ROOT=C:\Users\ReCor\Documents\Code\Rust\ferrumc` before running the server. This is useful if you
can't move the place the binary is executed from (`cargo run` for example).*

## 🛠️ Development

We welcome contributions! If you'd like to contribute to FerrumC, please follow these steps:

1. Fork the repository
2. Create a new branch for your feature
3. Implement your changes
4. Write or update tests as necessary
5. Submit a pull request

*Please* join our [Discord server](https://discord.gg/qT5J8EMjwk) to get help or discuss the project!

## ❔ FAQ

### How does this project differ from:

- **Valence**: Valence is a framework for building your own custom server by pulling in different components of their
  library. FerrumC is a fully built server designed to act as a potential replacement for the vanilla server. It's like
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
awesome [Extism](https://extism.org/) project to allow for plugins to be written in a multitude of languages (Rust, Go,
JS/TS, Zig and more) while not losing out on the performance gains of native code.

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
