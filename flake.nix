{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      forAllSystems =
        function:
        nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (
          system: function nixpkgs.legacyPackages.${system}
        );
    in
    {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          buildInputs = with pkgs; [
			marp-cli
			(writeShellScriptBin "rebuild" "${watchexec}/bin/watchexec -r -e md -- ${marp-cli}/bin/marp presi.md --pdf")
			(writeShellScriptBin "thing" "${pkgs.polkit_gnome}/libexec/polkit-agent-1")
          ];
        };
      });
    };
}
