
**Rhit** works on linux and mac.

Current version: **<a id=current-version href=../download>download</a>**
<script>
console.log("in script");
fetch("../download/version")
    .then(response => response.text())
    .then(version => {
        version = version.trim();
        if (!/^\d+(\.\d+)*(-\w+)?$/.test(version)) {
            console.warn("invalid version in download/version");
            return;
        }
        document.getElementById("current-version").textContent = version;
    })
</script>

[CHANGELOG](https://github.com/Canop/rhit/blob/main/CHANGELOG.md)


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

# From third party repositories

## Homebrew

```bash
brew install rhit
```
## APT / Deb

Ubuntu and Debian users may use this apt repository: [https://packages.azlux.fr/](https://packages.azlux.fr/)

## Arch Linux

Arch Linux users may use `pacman` to install [rhit](https://archlinux.org/packages/community/x86_64/rhit/) from the community repository:

```bash
pacman -S rhit
```
