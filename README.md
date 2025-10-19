<img style="width: 100%" src="https://github.com/ferrumc-rs/ferrumc/blob/master/assets/README/header.svg?raw=true" alt="FerrumC Header">
<div align="center">
    <img src="https://img.shields.io/github/license/ferrumc-rs/ferrumc?style=for-the-badge&color=red" alt="License">
    <img src="https://img.shields.io/github/languages/code-size/ferrumc-rs/ferrumc?style=for-the-badge&color=red" alt="Code Size">
    <img src="https://www.aschey.tech/tokei/github.com/ferrumc-rs/ferrumc?style=for-the-badge&color=red" alt="Lines of Code">
    <img src="https://img.shields.io/badge/language-Rust-orange?style=for-the-badge&color=red" alt="Language">
    <a  href="https://discord.gg/qT5J8EMjwk">
    <img alt="Discord" src="https://img.shields.io/discord/1277314213878173726?style=for-the-badge&logo=discord&logoColor=red&color=red&link=https%3A%2F%2Fdiscord.gg%2FqT5J8EMjwk">
    </a>
</div>

## 📖 About

FerrumC is a **1.21.8** Minecraft server implementation written from the ground up in Rust. Leveraging the power of the
Rust programming language, it is completely multithreaded and offers high performance as well as amazing memory
efficiency!

Visit **[ferrumc.com](https://www.ferrumc.com)** for more information. The official **[Docs](https://docs.ferrumc.com)**
are currently under construction, but you can join our **[Discord server](https://discord.gg/qT5J8EMjwk)** for help or
to discuss the project!

<img src="https://github.com/ferrumc-rs/ferrumc/blob/master/assets/README/in_game.png?raw=true" alt="In-game screenshot">

## 🔗 Project Links

* **Official Website:** **[ferrumc.com](https://www.ferrumc.com)**
* **Documentation:** **[docs.ferrumc.com](https://docs.ferrumc.com)**
* **Discord Community:** **[Join our Discord](https://discord.gg/qT5J8EMjwk)**
* **GitHub Repository:** **[ferrumc-rs/ferrumc](https://github.com/ferrumc-rs/ferrumc)**

## ✨ Key Features

<ul>
    <li>
        <h4>🛈 Customizable server list</h4>
        <img src="https://github.com/ferrumc-rs/ferrumc/blob/master/assets/README/server%20list.png?raw=true" alt="Server list">
    </li>
    <li>
        <h4>🚄 Extremely fast </h4>
        <img src="https://github.com/ferrumc-rs/ferrumc/blob/master/assets/README/mind%20boggling.gif?raw=true" alt="Mind boggling">
    </li>
    <li>
        <h4>🖥️ Highly efficient memory usage</h4>
        <img src="https://github.com/ferrumc-rs/ferrumc/blob/master/assets/README/mem_use.png?raw=true" alt="Low memory usage">
    </li>
    <li>
        <h4>🗂️ Straightforward Configuration</h4>
        <img src="https://github.com/ferrumc-rs/ferrumc/blob/master/assets/README/config.toml.png?raw=true" alt="Configuration">
    </li>
    <li>
      <h4>🔄 Can import existing worlds from vanilla minecraft</h4>
      <img src="https://github.com/ferrumc-rs/ferrumc/blob/master/assets/README/chunk_importing.gif?raw=true" alt="Configuration">
    </li>
    <li>
      <h4>🌐 Compatible with vanilla Minecraft clients (Version 1.21.8)</h4>
    </li>
    <li>
      <h4>📦 Fully multithreaded; Utilizes all available CPU cores, instead of a single "main" thread</h4>
    </li>
    <li>
      <h4>📝 Custom made network, NBT and Anvil encoding systems to allow for minimal I/O lag</h4>
    </li>
    <li>
        <h4>💾 Crazy fast K/V database </h4>
        <i>32 render distance*</i>
        <img src="https://github.com/ferrumc-rs/ferrumc/blob/master/assets/README/chunk_loading.gif?raw=true" alt="Chunk Loading DEMO">
    </li>
    <li>
        <h4>🎮 Bevy ECS for smart, lockless concurrency driven by a massive community</h4>
    </li>
</ul>

<h2>✅ Upcoming features</h2>

<ul>
   <li>
      <h4>PvE mechanics, and entities.</h4>
   </li>
    <li>
        <h4>Web based server dashboard</h4>
   </li>
    <li>
        <h4>Optimizations</h4>
   </li>
    <li>
        <h4>Plugin support (Rust via FFI currently, other languages will be considered later)</h4>
    </li>
</ul>

## Goals

- **Performance**: FerrumC aims to be the fastest Minecraft server implementation available, with a focus on low latency
  and high throughput.
- **Memory Efficiency**: FerrumC is designed to use as little memory as possible while still providing a full-featured
  server experience.
- **Not just a faster replacement**: FerrumC is not intended to be a perfect match for the vanilla server. We aim to
  improve on the original server in ways other than just performance. This includes things like setup and usage,
  configurability, plugins and more. Simply speeding up the server feels like a waste of the opportunity to do something
  new and exciting.
- **Ease of Use**: While it will certainly possible to run FerrumC as the backend for highly complex servers such as
  Hypixel through the use of plugins, the main intended audience is the average user who wants to run a server for their
  friends
  and family. We want to make it as easy as possible to set up and run a server, while still allowing the flexibility
  and power that advanced users need.
- **Not taking the easy route**: We aren't scared of a little unsafe Rust or some raw SIMD instructions. If we need
  to take some risks to get the performance we want, or need to write our own versions of libraries to get the features
  we need, we will do it. We already have custom-made Anvil and NBT libraries that use experimental APIs and some raw
  assembly because the existing ones just weren't up to scratch.

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

![Where To Find](https://github.com/ferrumc-rs/ferrumc/blob/master/assets/README/download_prebuilt.gif?raw=true)

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

```plaintext
Usage: ferrumc.exe [OPTIONS] [COMMAND]

Commands:
setup   Sets up the config
import  Import the world data
run     Start the server (default, if no command is given)
help    Print this message or the help of the given subcommand(s)

Options:
--log <LOG>  [default: debug] [possible values: trace, debug, info, warn, error]
-h, --help       Print help
```

1. Move the FerrumC binary (`ferrumc.exe` or `ferrumc` depending on the OS) to your desired server directory
2. Open a terminal in that directory
3. (Optional) Generate a config file: `./ferrumc setup`
    - Edit the generated `config.toml` file to customize your server settings
4. Run the server:
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
  library. FerrumC aims to be a full replacement for the vanilla server. It's like the difference between buying the
  ingredients to make a meal yourself or just buying a pre-made meal.
- **Minestom**: Same as Valence, it's a framework to build your own server, which is different to what we are trying to
  do.
- **Paper/Spigot/Bukkit**: These are all great tools and have undoubtedly set the groundwork for projects like this to
  exist, but ultimately they are still somewhat bound to the original server implementation. We aim to write the entire
  server from the ground up, hopefully giving us a leg up.
- **Pumpkin**: Pumpkin are a lot more focused on matching the vanilla server as close as possible, only improving the
  performance.
  We are trying to improve most aspects of the game, including ease of use, performance, memory usage and extensibility.
  That being said, we are trying to achieve similar things, and it's not a competition, we are both aware of each
  other's
  progress and to be honest the Pumpkin team are doing really well. We won't tolerate any disrespect towards them as
  they are also undertaking the same monumental task.

### Will we be implementing terrain generation?

Yes! We currently have some very rudimentary terrain generation and vanilla terrain is currently being worked on.
However,
we will be implementing optimizations and cutting corners to improve performance. This could lead to the world not being
*exactly* the same as vanilla and differences may not be fixed if they would lead to performance issues that we deem to
outweigh the benefits of vanilla accuracy. That being said, we will try to make it as close to vanilla as possible
without
sacrificing performance.

### Will there be plugins? And how?

We do very much plan to have a plugin system and as of right now we are planning to use
some kind of FFI (foreign function interface) to allow for plugins to be written in Rust. Plugins are not our top
priority
right now, and we want to make sure the API is designed well before we start implementing it to avoid breaking changes
later.
We are open to suggestions and ideas from the community on how to best implement this.

### Will I be able to use plugins or mods from paper/spigot/bukkit/forge/fabric etc.?

No. Even if we did implement a perfect 1:1 API match for the vanilla server, the underlying implementation is still
completely different.
Java plugins and mods rely heavily on Java features such as reflection and dynamic class loading, which simply aren't
possible in Rust.
If we made a Java translation layer, it would be extremely slow and only the most basic plugins and mods would work. If
a plugin
or mod is basic enough to work through a translation layer, it would be much better to just rewrite it in Rust for
performance
and compatibility reasons.

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

## 📊 Stats

[![Timeline graph](https://images.repography.com/59032276/ferrumc-rs/ferrumc/recent-activity/J6CgGhzs6y3LXRuADz1QpSUriBC3ix9DXnPUbbljruA/O-qGFiSVQmksFEaX7mVQ4jY3lppUTK2xUw4CpqZ3oUk_timeline.svg)](https://github.com/ferrumc-rs/ferrumc/commits)
[![Issue status graph](https://images.repography.com/59032276/ferrumc-rs/ferrumc/recent-activity/J6CgGhzs6y3LXRuADz1QpSUriBC3ix9DXnPUbbljruA/O-qGFiSVQmksFEaX7mVQ4jY3lppUTK2xUw4CpqZ3oUk_issues.svg)](https://github.com/ferrumc-rs/ferrumc/issues)
[![Pull request status graph](https://images.repography.com/59032276/ferrumc-rs/ferrumc/recent-activity/J6CgGhzs6y3LXRuADz1QpSUriBC3ix9DXnPUbbljruA/O-qGFiSVQmksFEaX7mVQ4jY3lppUTK2xUw4CpqZ3oUk_prs.svg)](https://github.com/ferrumc-rs/ferrumc/pulls)
[![Top contributors](https://images.repography.com/59032276/ferrumc-rs/ferrumc/recent-activity/J6CgGhzs6y3LXRuADz1QpSUriBC3ix9DXnPUbbljruA/O-qGFiSVQmksFEaX7mVQ4jY3lppUTK2xUw4CpqZ3oUk_users.svg)](https://github.com/ferrumc-rs/ferrumc/graphs/contributors)
