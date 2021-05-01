{ pkgs ? (import <nixpkgs> {})}:

let
  rust-toolchain = pkgs.symlinkJoin {
    name = "rust-toolchain";
    paths = with pkgs.rustChannels.nightly; [
      rust rustc cargo rust-src
    ];
  };
in pkgs.mkShell {
    buildInputs = with pkgs; [
      rust-toolchain
      pkg-config
      openssl
      zlib
    ];

    shellHook = ''
      rm -rf .toolchain
      ln -sf ${rust-toolchain} .toolchain
    '';
 }
