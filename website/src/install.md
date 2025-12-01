
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

Binaries are made available at every release in [download](https://dystroy.org/rhit/download). They are also available on [GitHub releases](https://github.com/Canop/rhit/releases).

The archives contain precompiled binaries, as well as the licenses and other files.

You may also directly download the executable files below, depending on your system:

Target|Download
-|-
x86-64 Linux gnu | [x86_64-unknown-linux-gnu](https://dystroy.org/rhit/download/x86_64-unknown-linux-gnu/rhit)
x86-64 Linux musl  | [x86_64-unknown-linux-musl](https://dystroy.org/rhit/download/x86_64-unknown-linux-musl/rhit)
ARM32 Linux gnu | [armv7-unknown-linux-gnueabihf](https://dystroy.org/rhit/download/armv7-unknown-linux-gnueabihf/rhit)
ARM32 Linux musl | [armv7-unknown-linux-musleabi](https://dystroy.org/rhit/download/armv7-unknown-linux-musleabi/rhit)
ARM64 Linux gnu | [aarch64-unknown-linux-gnu](https://dystroy.org/rhit/download/aarch64-unknown-linux-gnu/rhit)
ARM64 Linux musl | [aarch64-unknown-linux-musl](https://dystroy.org/rhit/download/aarch64-unknown-linux-musl/rhit)
Windows 10+ (experimental) | [x86_64-pc-windows-gnu](https://dystroy.org/rhit/download/x86_64-pc-windows-gnu/rhit.exe)

On linux, if you have an old system, the "musl" versions may work when the "gnu" ones ask for a version of glibc that you don't have.


When you download executable files, you'll have to ensure the shell can find them. An easy solution on linux is for example to put them in `/usr/local/bin`. You may also have to set them executable using `chmod +x rhit`.

As I can't compile myself for all possible systems, you'll need to compile rhit yourself or use a third-party repository (see below) if your system isn't in the list above.

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

# Third party repositories

Those packages are maintained by third parties and may be less up to date.

[![Packaging status](https://repology.org/badge/vertical-allrepos/rhit.svg)](https://repology.org/project/rhit/versions)
## Homebrew

```bash
brew install rhit
```
## APT / Deb

Ubuntu and Debian users may use this apt repository: [https://packages.azlux.fr/](https://packages.azlux.fr/)

## Arch Linux

Arch Linux users may use `pacman` to install [rhit](https://archlinux.org/packages/extra/x86_64/rhit/) from the extra repository:

```bash
pacman -S rhit
```
