# Ferrumc Rewrite [v3]

> [!WARNING]
> FerrumC is going through a major rewrite **for 1.21.1**. This branch contains all of the new code.\
> To view the original code, go to the [dev branch](https://github.com/ferrumc-rs/ferrumc/tree/dev).

# Roadmap
To view the roadmap, see [plans.md](assets/plans/plans.md)

> [!IMPORTANT]
> Use pull requests instead of direct pushes.

## Contributing
**Want to contribute to FerrumC?**\
Make sure to check out [CONTRIBUTING.md](CONTRIBUTING.md).\
We would highly recommend you join our [Discord](https://discord.gg/FqT5J8EMjwk).

## 📥 Installation/ 🖥️ Usage
### Use docker

This method comes with a default world and might be easier assuming you arent doing development and dont have cargo already installed. Just run the following command 
```bash
docker run -d -p 25565:25565 -v ferrumc/ferrumc-example:latest
```


// TODO: Throw the images in dockerhub under the ferrumc username.

### Build from Source .

```bash
# Clone the repository
git clone https://github.com/Sweattypalms/ferrumc
cd ferrumc

# Build the project
cargo build --release
```

##### The binary will be in target/release/

Then 

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

---

## Nightly Dev Server

There is a nightly deployed version of rewrite/v3 running at `ferrumc.nimq.xyz`.

## Docs

Documentation for rewrite/v3 can be found at:

- Unsecure https://docs.ferrumc.com/
- HTTPS https://docs.nimq.xyz/

## Funding / Donations

If you would like to donate to the development of FerrumC, you can do so via the following methods:

- OpenCollective: https://opencollective.com/ferrumc


