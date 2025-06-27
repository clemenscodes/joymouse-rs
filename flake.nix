{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
    };
    crane = {
      url = "github:ipetkov/crane";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
      };
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
      };
    };
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-parts,
    crane,
    fenix,
    rust-overlay,
    advisory-db,
    ...
  } @ inputs: let
    system = "x86_64-linux";

    pkgs = import nixpkgs {
      inherit system;
      overlays = [(import rust-overlay)];
    };

    inherit (pkgs) lib;

    rustToolchain = with fenix.packages.${system};
      combine [
        (fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-KUm16pHj+cRedf8vxs/Hd2YWxpOrWZ7UOrwhILdSJBU=";
        })
        targets.x86_64-pc-windows-gnu.latest.rust-std
      ];

    craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

    src = lib.fileset.toSource {
      root = ./.;
      fileset = lib.fileset.unions [(craneLib.fileset.commonCargoSources ./.)];
    };

    inherit (craneLib.crateNameFromCargoToml {inherit src;}) pname version;

    args = {
      inherit pname version src cargoArtifacts;
      strictDeps = true;
      doCheck = false;
      buildInputs = with pkgs; [
        openssl
        libevdev
        udev
        dbus
        sdl3.lib
        sdl3.dev
      ];
      nativeBuildInputs = with pkgs; [pkg-config];
    };

    cargoArtifacts = craneLib.buildDepsOnly args;

    joymouse-glibc = craneLib.buildPackage (args
      // {
        CARGO_BUILD_RUSTFLAGS = "-C strip=symbols";
      });

    joymouse-musl-static = craneLib.buildPackage (args
      // {
        CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
        CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static -C link-args=-static -C relocation-model=static -C strip=symbols";
      });
  in
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [inputs.flake-parts.flakeModules.easyOverlay];
      systems = [system];
      perSystem = {
        config,
        system,
        ...
      }: let
        apps = {inherit joymouse-glibc joymouse-musl-static;};
      in {
        formatter = pkgs.alejandra;

        checks = {
          doc = craneLib.cargoDoc (args // {inherit cargoArtifacts;});

          fmt = craneLib.cargoFmt {inherit src;};

          audit = craneLib.cargoAudit {inherit src advisory-db;};

          coverage = craneLib.cargoLlvmCov (args // {inherit cargoArtifacts;});

          clippy = craneLib.cargoClippy (args
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });

          toml-fmt = craneLib.taploFmt {
            src = pkgs.lib.sources.sourceFilesBySuffices src [".toml"];
            taploExtraArgs = "--config ./taplo.toml";
          };

          nextest = craneLib.cargoNextest (args
            // {
              inherit cargoArtifacts;
              partitions = 1;
              partitionType = "count";
            });
        };

        overlayAttrs = apps;

        packages = apps // {default = self.packages.${system}.joymouse-musl-static;};

        devShells = {
          default = craneLib.devShell {
            checks = self.checks.${system};
            buildInputs = with pkgs; [
              openssl
              libevdev
              udev
              dbus
              sdl3.lib
              sdl3.dev
            ];
            nativeBuildInputs = with pkgs; [
              pkg-config
              rust-analyzer
              cargo-watch
              cargo-audit
              cargo-deny
              cargo-llvm-cov
              cargo-tarpaulin
              cargo-nextest
              libinput
              evtest
              interception-tools
            ];
            RUST_SRC_PATH = "${craneLib.rustc}/lib/rustlib/src/rust/library";
            RUST_BACKTRACE = 1;
          };
        };
      };
    };

  nixConfig = {
    extra-substituters = [
      "https://nix-community.cachix.org"
      "https://clemenscodes.cachix.org"
    ];
    extra-trusted-public-keys = [
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
      "clemenscodes.cachix.org-1:yEwW1YgttL2xdsyfFDz/vv8zZRhRGMeDQsKKmtV1N18="
    ];
  };
}
