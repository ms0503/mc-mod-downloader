{ inputs, ... }:
{
  imports = [
    inputs.treefmt-nix.flakeModule
  ];
  perSystem =
    {
      inputs',
      lib,
      pkgs,
      ...
    }:
    {
      treefmt = {
        programs = {
          jsonfmt.enable = true;
          mdformat = {
            enable = true;
            plugins =
              ps: with ps; [
                mdformat-gfm
              ];
            settings = {
              end-of-line = "lf";
              number = true;
              wrap = 80;
            };
          };
          nixfmt.enable = true;
          rustfmt = {
            edition = "2024";
            enable = true;
            package = inputs'.fenix.packages.latest.rustfmt;
          };
          taplo.enable = true;
        };
        settings.formatter.jsonfmt =
          let
            jsonfmt = pkgs.callPackage ./jsonfmt.nix { };
          in
          {
            command = "${jsonfmt}/bin/jf";
            options = lib.mkForce [ ];
            package = jsonfmt;
          };
      };
    };
}
