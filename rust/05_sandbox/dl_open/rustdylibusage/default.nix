with import <nixpkgs> {};

let
  mybin = rustPlatform.buildRustPackage rec {
    name = "mybin";
    version = "0.1.0";
    src = ./mybin/mybin-${version}.tar.gz;
    #cargoSha256 = "0qbdcw601m09fw7vqmafvw5q7w17dh37fbbycs6is3rff5qlmbyw"; #mybin
    #cargoSha256 = "sha256-i9TVX8LydRtaiEF0SAz0dKbdW2wMxrdisXkY293tI0Q";
    cargoHackageDeps = false;
    cargoBuildFlags = [ "--release" ];
    #cargoDepInputs = [ mylib ];
    buildInputs = [ mylib ];
    sourceRoot = "./";
    cargoLock = {
      lockFile = ./mybin/Cargo.lock;
    };
    libPath = lib.makeLibraryPath [ mylib ];
    fixupPhase = ''
      patchelf --set-rpath $libPath \
      $out/bin/mybin
      '';
#    fixupPhase = ''
#      patchelf --add-rpath $mylib/lib/libmylib.so \
#      $out/bin/mybin
#      '';

  };

  mylib = rustPlatform.buildRustPackage rec {
    version = "0.1.0";
    name = "mylib";
    src = ./mylib/mylib-${version}.tar.gz;
    #cargoSha256 = "1zwcx7637zl6ka058hfgwzmnxln1y592gfwiycxiz65qygxph383"; #mylib
    #cargoSha256 = "sha256-7AfdCKEh4xTyRir/hmA8I6WYnoPYAQH8NN1Jx19SYDc";
    cargoHackageDeps = false;
    cargoBuildFlags = [ "--release" ];
    sourceRoot = "./";
    cargoLock = {
      lockFile = ./mylib/Cargo.lock;
    };
  };

in
  mybin

