
The current version of Rhit works on linux and mac.


# From precompiled binaries

Binaries are made available at every release in [download](https://dystroy.org/rhit/download).

Direct links:

Target|Files
-|-
Linux | [x86_64-linux](https://dystroy.org/rhit/download/x86_64-linux/rhit)
Linux/musl | [x86_64-unknown-linux-musl](https://dystroy.org/rhit/download/x86_64-unknown-linux-musl/rhit)
Windows (experimental) | [x86_64-pc-windows-gnu](https://dystroy.org/rhit/download/x86_64-pc-windows-gnu/rhit.exe)

When you download executable files, you'll have to ensure the shell can find them. An easy solution is to put them in `/usr/local/bin`. You may also have to set them executable using `chmod +x rhit`.

# From crates.io

You'll need to have the [Rust development environment](https://www.rustup.rs) installed and up to date.

Once it's installed, use cargo to install rhit:

    cargo install rhit

# From source

You'll need to have the [Rust development environment](https://www.rustup.rs) installed.

Fetch the [Canop/rhit](https://github.com/Canop/rhit) repository, move to the rhit directory, then run

```bash
cargo install --path .
```

