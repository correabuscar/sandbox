#You may want something like:

let
  nixpkgs = import <nixpkgs> {};
  cross = import <nixpkgs> {
    #crossSystem = { config = "aarch64-unknown-linux-gnu"; };
    crossSystem = { config = "aarch64-unknown-linux-musl"; };
  };
in
nixpkgs.mkShell {
  buildInputs = [
    cross.buildPackages.gcc
  ];
}

#You’re evaluating twice <nixpkgs>, once for your current architecture, once for your target architecture.
#Then, in buildInputs, you’re adding the compiler which can build (“buildPackages”) for your target architecture.
#src: https://discourse.nixos.org/t/adding-a-crosscompiler-into-nix-shell-environment/16324

