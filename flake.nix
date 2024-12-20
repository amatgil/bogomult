{
  description = "Default Rust flake (casenc)";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
  };
  outputs =
    { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgs = (import nixpkgs { system = "x86_64-linux"; });
    in
    {
      packages = forAllSystems (system: {
        default = pkgs.callPackage ./default.nix { inherit pkgs; };
      });
      devShells = forAllSystems (system: {
        default = pkgs.callPackage ./shell.nix { inherit pkgs; };
      });
    };
}
