#!/usr/bin/env bash

# Auto quit if exits non-zero
set -e

# Color code
GREEN='\033[0;32m'
RED='\033[0;31m'
# Default
NC='\033[0m'

if [ "$1" == "--help" ] || [ "$1" == "-h" ]; then
    echo "Usage:"
    echo ""
    echo "-h / --help                         Show help"
    echo "-i / --ignore-mingw-missing         Ignores MingW dependency missing"
    echo "                                    (This flag is used, just in case"
    echo "                                    that mingw is called differently"
    echo "                                    on other distro)"
    echo "-d / --debug                        Set compiled result to Debug"
    echo "-r / --release                      Set compiled result to Release"
    exit
fi

function check_dependencies () {
    local binary=$1

    printf "[-] Checking for %s \n" "$binary"
    if command -v "$binary" &> /dev/null; then
        printf "[${GREEN}OK${NC}] $binary found\n"
    else
        printf "[${RED}X${NC}] $binary not found, please install the dependency." &>2
        return 1
    fi
}

check_dependencies "rustc" || exit 1
check_dependencies "cargo" || exit 1

# Clear cache
rm -rf target

if [ "$1" == "-i" ] || [ "$1" == "--ignore-mingw-missing" ]; then
    check_dependency "x86_64-w64-mingw32-gcc" || exit 1
fi

echo -ne "[${GREEN}OK${NC}] All deps have satisfied, starts compiling...\n"
printf "Checking Rust Windows target compilation support... "

if rustup target list | grep -q "x86_64-pc-windows-gnu (installed)"; then
    echo -e "${GREEN}READY${NC}"
else
    echo -e "${RED}NOT INSTALLED${NC}"
    echo "[*] Attempting to install missing Windows compilation target..."
    rustup target add x86_64-pc-windows-gnu
fi

printf "[-] Starting Build..."

pwd = "$({pwd})"
echo $pwd

case $1 in
    -r|--release)
        cargo build --release --target x86_64-pc-windows-gnu
        printf "[-] Cargo concluded. Check target dir\n"
        ;;
    -d|--debug)
        cargo build --target x86_64-pc-windows-gnu
        printf "[-] Cargo concluded. Check target dir\n"
        ;;
    *)
        cargp build --target x86_64-pc-windows-gnu
        printf "[-] Cargo concluded. Check target dir\n"
        ;;
esac

printf "[${GREEN}OK${NC}] Done"

