#WARNING: This script is NOT meant for normal installation, it's dedicated
# to the compilation of all supported targets.
# This is a long process and it involves specialized toolchains.
# For usual compilation do
#     cargo build --release
# or read all possible installation solutions on
# https://dystroy.org/rhit/install

H1="\n\e[30;104;1m\e[2K\n\e[A" # style first header
H2="\n\e[30;104m\e[1K\n\e[A" # style second header
EH="\e[00m\n\e[2K" # end header
NAME=rhit
version=$(./version.sh)

echo -e "${H1}Compilation of all targets for $NAME $version${EH}"
 
# Clean previous build
rm -rf build
mkdir build
echo "   build cleaned"

# Build versions for other platforms using cargo cross
cross_build() {
    target_name="$1"
    target="$2"
    echo -e "${H2}Compiling the $target_name version for target $target ${EH}"
    cargo clean
    cross build --target "$target" --release
    mkdir "build/$target"
    if [[ $target_name == 'Windows' ]]
    then
        exec="$NAME.exe"
    else
        exec="$NAME"
    fi
    cp "target/$target/release/$exec" "build/$target/"
}
cross_build "Windows" "x86_64-pc-windows-gnu"
cross_build "MUSL" "x86_64-unknown-linux-musl"
cross_build "Linux GLIBC" "x86_64-unknown-linux-gnu"
cross_build "Raspberry 32" "armv7-unknown-linux-gnueabihf" ""
 
# Build the default linux version
# recent glibc
echo -e "${H2}Compiling the standard linux version${EH}"
cargo build --release
strip "target/release/$NAME"
mkdir build/x86_64-linux/
cp "target/release/$NAME" build/x86_64-linux/

# add a summary of content
echo '
This archive contains pre-compiled binaries

For more information, or if you prefer to compile yourself, see https://dystroy.org/rhit/install
' > build/install.md

echo -e "${H1}FINISHED${EH}"
