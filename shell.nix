with import <nixpkgs> {};

mkShell {
    buildInputs = [ 
        gnumake
        grub2
        xorriso
        qemu
        rustup
    ];
}