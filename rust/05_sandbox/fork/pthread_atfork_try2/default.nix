#src: https://stackoverflow.com/questions/77680023/rust-compiling-to-windows-not-working-under-nixos
{ pkgs ? import <nixpkgs> {} }:

let
  winee = pkgs.wine.override { wineBuild = "wine64"; }; #this is 64 bits, good! thanks to: https://github.com/NixOS/nixpkgs/issues/50615#issuecomment-439732094
in
pkgs.mkShell {

  shellHook = ''
    set -e
    export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS="-L native=${pkgs.pkgsCross.mingwW64.windows.pthreads}/lib"
    export PATH="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin:$PATH"
    export PATH="${winee}/bin:$PATH"
    export PATH="${pkgs.rustup}/bin:$PATH"

    rustup default nightly
    rustup target add x86_64-pc-windows-gnu
  '';

  #you can put the following in /etc/nixos/configuration.nix but then you don't have to set the above PATHs
  builtInputs = [
    pkgs.pkgsCross.mingwW64.stdenv.cc
    pkgs.pkgsCross.mingwW64.windows.pthreads

    pkgs.rustup #needed for the proper rust to target windows while compiling on linux, so for target x86_64-pc-windows-gnu

    winee
    #(pkgs.wine.override { wineBuild = "wine64"; }) #this is 64 bits, good! thanks to: https://github.com/NixOS/nixpkgs/issues/50615#issuecomment-439732094
    #note: 'wine' as 'pkgs.wine' is 32 bits wine, no good!
  ];
}
