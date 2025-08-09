{
  description = "framework for R2Northstar plugins";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
    flake-utils = { url = "github:numtide/flake-utils"; };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }: 
     flake-utils.lib.eachDefaultSystem (system:
      let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
            crossSystem = {
              config = "x86_64-w64-mingw32";
              libc = "msvcrt";
            };
          };

          toolchain = (pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
      in
      {
        devShell = pkgs.mkShell rec {
          nativeBuildInputs = with pkgs; [ 
            toolchain
            pkg-config
          ];

          buildInputs = with pkgs; [ 
            windows.mingw_w64_headers 
            windows.mcfgthreads
            windows.mingw_w64_pthreads
          ];

          LD_LIBRARY_PATH = nixpkgs.lib.makeLibraryPath buildInputs;
          PATH = nixpkgs.lib.makeLibraryPath buildInputs;
        };
      });
}
