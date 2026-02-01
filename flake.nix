{
  description = "Monobook";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url  = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
  flake-utils.lib.eachDefaultSystem (system: let
    pkgs = nixpkgs.legacyPackages.${system};
    libDeps = with pkgs; [
    ] ++ [
      fenix.packages.x86_64-linux.complete.toolchain
    ];
    # libPath = pkgs.lib.makeLibraryPath libDeps;
  in {
    # packages.${system}.default = fenix.packages.x86_64-linux.latest.toolchain;
    devShells.default = pkgs.mkShell {
      buildInputs = libDeps;

      shellHook = ''
      # export RUSTFLAGS="-Clink-arg=-z -Clink-arg=nostart-stop-gc"
      export PATH="$HOME/.local/share/cargo/bin:$PATH"
      '';
    };
  }
  );

}
