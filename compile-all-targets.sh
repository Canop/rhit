# WARNING: This script is NOT meant for normal installation, it's dedicated
# to the compilation of all supported targets, from a linux machine.

H1="\n\e[30;104;1m\e[2K\n\e[A" # style first header
H2="\n\e[30;104m\e[1K\n\e[A" # style second header
EH="\e[00m\n\e[2K" # end header

version=$(sed 's/version = "\([0-9.]\{1,\}\(-[a-z]\+\)\?\)"/\1/;t;d' Cargo.toml | head -1)
echo -e "${H1}Compilation of all targets for rhit $version${EH}"
 
# clean previous build
rm -rf build
mkdir build
echo "   build cleaned"

# build the windows version
# use cargo cross
echo -e "${H2}Compiling the Windows version${EH}"
cargo clean
cross build --target x86_64-pc-windows-gnu --release
mkdir build/x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/rhit.exe build/x86_64-pc-windows-gnu/

# build a musl version
echo -e "${H2}Compiling the MUSL version${EH}"
cross build --release --target x86_64-unknown-linux-musl
mkdir build/x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/rhit build/x86_64-unknown-linux-musl

# build the linux version
echo -e "${H2}Compiling the linux version${EH}"
cargo build --release
strip target/release/rhit
mkdir build/x86_64-linux/
cp target/release/rhit build/x86_64-linux/
