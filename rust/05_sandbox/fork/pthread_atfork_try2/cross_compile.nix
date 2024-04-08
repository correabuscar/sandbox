# ok this totally fails, badly because it's linking with 'cc' instead of my specified linker! but the global thing in /etc/nixos/configuration.nix and the default.nix for a nix-shell, instead of this $0, succeeds!

let
  nixpkgs = import <nixpkgs> {};
  cross = import <nixpkgs> {
    #crossSystem = { config = "aarch64-unknown-linux-gnu"; };
    #crossSystem = { config = "x86_64-pc-windows-gnu"; }; #bad!
    crossSystem = { config = "x86_64-w64-mingw32"; };
  };
in
nixpkgs.mkShell {
#  buildInputs = with nixpkgs.pkgsCross.mingwW64.windows; [
#    #gcc
#    pthreads
#  ];

  shellHook = ''
    #works:
    #export RUSTC_WRAPPER="`realpath ./rustc_wrapper.sh`"

    #no effect:
    #export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER="`realpath ./linker.sh`"
    #export CARGO_TARGET_X86_64-PC-WINDOWS-GNU_LINKER="`realpath ./linker.sh`"
    #export CARGO_LINKER="`realpath ./linker.sh`"

    #src:https://stackoverflow.com/questions/77680023/rust-compiling-to-windows-not-working-under-nixos
    #export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS="-L native=/nix/store/k4qm1y42z1nrl6gcqhmlnk5kcsi967zs-mingw-w64-x86_64-w64-mingw32-11.0.1/lib -L native=${nixpkgs.pkgsCross.mingwW64.windows.pthreads}/lib -lucrt -lucrtapp -lucrtbase -lmsvcrt -lmsvcrt-os -lntdllcrt -llibucrt.a -lwtf -linexistentlib -C default-linker-libraries=yes -C link-arg=shie -C linker=shiet"
    #XXX: allowing this RUSTFLAGS overwrites the above target specific one!
    #export RUSTFLAGS="-C linker=crap -C default-linker-libraries=yes -C link-self-contained=yes"
    #fine then, the hard way 'cc' exists in ./  AND yet it doesn't run it! lol!
    #export PATH="/home/user/sandbox/pthread_atfork_try2/path:/nix/store/pz7rb86xdq7jk4z5lrn0qlbs63pcsg4m-rustup-1.26.0/bin" #works
    #this was the only way to make it use the linker that I want, instead of just 'cc'
    export PATH="/home/user/sandbox/pthread_atfork_try2/path:$PATH"
    #XXX: a path with "." aka current dir, is ignored by rust!

    #-l:libucrt.a"
    #export RUSTFLAGS="-L native=/nix/store/k4qm1y42z1nrl6gcqhmlnk5kcsi967zs-mingw-w64-x86_64-w64-mingw32-11.0.1/lib -L native=${nixpkgs.pkgsCross.mingwW64.windows.pthreads}/lib -lucrt -lucrtapp -lucrtbase -lmsvcrt -lmsvcrt-os -lntdllcrt"
    #export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS="-L native=/nix/store/k4qm1y42z1nrl6gcqhmlnk5kcsi967zs-mingw-w64-x86_64-w64-mingw32-11.0.1/lib -L native=${nixpkgs.pkgsCross.mingwW64.windows.mingw_w64_pthreads}/lib -lucrt"
    #-l:libucrt.a -l:libucrtbase.a -l:libmsvcrt.a"

    #i don't trust this:
    #export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS="$CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS -L native=/nix/store/k4qm1y42z1nrl6gcqhmlnk5kcsi967zs-mingw-w64-x86_64-w64-mingw32-11.0.1/lib"
    export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS="-L native=${nixpkgs.pkgsCross.mingwW64.windows.mingw_w64_pthreads}/lib"
  '';
  buildInputs = [
    #cross.buildPackages.gcc
    nixpkgs.pkgsCross.mingwW64.stdenv.cc
    #these 2 are incompatible:
    #nixpkgs.pkgsCross.mingw32.windows.pthreads
    #nixpkgs.pkgsCross.mingw32.windows.mingw_w64_pthreads
    #these 2 seem good but fail to link:
    nixpkgs.pkgsCross.mingwW64.windows.pthreads
    #nixpkgs.pkgsCross.mingwW64.windows.mingw_w64_pthreads

#    #these don't get rid of -l:libpthread.a error:
#    nixpkgs.pkgsCross.mingwW64.windows.crossThreadsStdenv
#    nixpkgs.pkgsCross.mingwW64.windows.mingw_w64_headers
#    #nixpkgs.pkgsCross.mingwW64.windows.w32api
#    nixpkgs.pkgsCross.mingwW64.stdenv.all
#    nixpkgs.pkgsCross.mingwW64.windows.libgnurx
#    #nixpkgs.pkgsCross.mingwW64.windows.mingw_runtime
#    #nixpkgs.pkgsCross.mingwW64.windows.mingwrt
#    nixpkgs.pkgsCross.mingwW64.windows.mingw_w64

    #nixpkgs.pkgsCross.mingwW64.windows.mcfgthreads
     # cross.buildPackages.pthreads  
     #    (cross.pkgs.pthreadsFor "${cross.crossSystem.system}")
      # Add pthreads library, gets rid of this build error:   = note: /nix/store/c5yh1hyi182b81a5h8bndv34qjvgbv97-x86_64-w64-mingw32-binutils-2.41/bin/x86_64-w64-mingw32-ld: cannot find -l:libpthread.a: No such file or directory

  ];
}

#You’re evaluating twice <nixpkgs>, once for your current architecture, once for your target architecture.
#Then, in buildInputs, you’re adding the compiler which can build (“buildPackages”) for your target architecture.
#src: https://discourse.nixos.org/t/adding-a-crosscompiler-into-nix-shell-environment/16324

