with import <nixpkgs> {};

mkShell {
    buildInputs = [ 
        gnumake
        grub2
        xorriso
        qemu
        rustup
        python3
        gdb
    ];
}