#src: https://stackoverflow.com/questions/77680023/rust-compiling-to-windows-not-working-under-nixos
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {

  shellHook = ''
    export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS="-L native=${pkgs.pkgsCross.mingwW64.windows.pthreads}/lib"
  '';
}
