{
  description = "framework for R2Northstar plugins";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-26.05";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      inherit (nixpkgs.lib)
        hasSuffix
        hasPrefix
        filesystem
        genAttrs
        path
        removeSuffix
        ;
      inherit (builtins)
        concatMap
        isPath
        filter
        readFileType
        ;

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      eachSystem = genAttrs systems;

      perSystem = eachSystem (system: rec {
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
          config = {
            # TODO: use win-sdk
            # allowUnfreePredicate =
            #   pkg:
            #   builtins.elem (nixpkgs.lib.getName pkg) [
            #     "win-sdk"
            #     "xwin-fetch-msvc"
            #   ];
            # microsoftVisualStudioLicenseAccepted = true;
          };
        };

        toolchain = (pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
      });
    in

    {
      formatter = eachSystem (
        system: perSystem.${system}.pkgs.nixfmt-tree

      );

      devShells = eachSystem (
        system: with perSystem.${system}; {
          default = pkgs.pkgsCross.mingwW64.mkShell rec {
            nativeBuildInputs = with pkgs; [
              toolchain
              pkg-config
            ];

            buildInputs =
              let
                inherit (pkgs.pkgsCross.mingwW64) windows;
              in
              [
                windows.mingw_w64_headers
                windows.mcfgthreads
                windows.pthreads
              ];
          };
        }
      );

      checks = eachSystem (
        system:
        with perSystem.${system};
        # TODO: make this smarter
        genAttrs [ "async_engine" "cvar_example" "squirrel_example" ] (
          example:
          (pkgs.pkgsCross.mingwW64.makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          }).buildRustPackage
            {
              pname = example;
              name = example;

              src = ./.;

              rustToolchain = toolchain;

              cargoBuildFlags = [
                "--example"
                example
                "--features=async_engine"
              ];

              cargoLock.lockFile = ./Cargo.lock;
            }
        )
      );
    };
}
