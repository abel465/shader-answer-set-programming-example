{
  description = "shader-answer-set-programming-example";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix/3b89d5df39afc6ef3a8575fa92d8fa10ec68c95f";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-parts,
    fenix,
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];
      perSystem = {
        pkgs,
        system,
        ...
      }: let
        rustPkg = fenix.packages.${system}.latest.withComponents [
          "rust-src"
          "rustc-dev"
          "llvm-tools-preview"
          "cargo"
          "clippy"
          "rustc"
          "rustfmt"
          "rust-analyzer"
        ];
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustPkg;
          rustc = rustPkg;
        };
        buildInputs = with pkgs; [
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          vulkan-loader
          vulkan-tools
          wayland
          libxkbcommon
          libgcc.lib
          clingo
        ];
        shadersCompilePath = "$HOME/.cache/rust-gpu-shaders";
        shader-answer-set-programming-example = rustPlatform.buildRustPackage {
          pname = "shader-answer-set-programming-example";
          version = "0.0.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          cargoLock.outputHashes = {
            "rustc_codegen_spirv-0.9.0" = "sha256-Yu/126ByKt+S0cmyeqeKyIfx4Xb616LnrL/6loYjJzs=";
          };
          dontCargoSetupPostUnpack = true;
          postUnpack = ''
            mkdir -p .cargo
            cat "$cargoDeps"/.cargo/config.toml | sed "s|cargo-vendor-dir|$cargoDeps|" >> .cargo/config.toml
            # HACK(eddyb) bypass cargoSetupPostPatchHook.
            export cargoDepsCopy="$cargoDeps"
          '';
          nativeBuildInputs = [pkgs.makeWrapper];
          configurePhase = ''
            export SHADERS_DIR="$out/repo/shaders"
            export SHADERS_TARGET_DIR=${shadersCompilePath}
            export CLINGO_LIBRARY_PATH="${pkgs.clingo}/lib";
          '';
          fixupPhase = ''
            cp -r . $out/repo
            wrapProgram $out/bin/runner \
              --set LD_LIBRARY_PATH $LD_LIBRARY_PATH:$out/lib:${nixpkgs.lib.makeLibraryPath buildInputs} \
              --set PATH $PATH:${nixpkgs.lib.makeBinPath [rustPkg]}
          '';
        };
      in rec {
        packages.default = pkgs.writeShellScriptBin "shader-answer-set-programming-example" ''
          export CARGO_TARGET_DIR="${shadersCompilePath}"
          exec -a "$0" "${shader-answer-set-programming-example}/bin/runner" "$@"
        '';
        apps.default = {
          type = "app";
          program = "${packages.default}/bin/shader-answer-set-programming-example";
        };
        devShells.default = with pkgs;
          mkShell {
            nativeBuildInputs = [rustPkg swi-prolog];
            LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
            CLINGO_LIBRARY_PATH = "${clingo}/lib";
          };
      };
    };
}
