{ inputs, ... }:
{
  imports = [
    inputs.git-hooks.flakeModule
  ];
  perSystem =
    { config, ... }:
    {
      pre-commit = {
        check.enable = true;
        settings = {
          hooks = {
            check-json.enable = true;
            check-toml.enable = true;
            editorconfig-checker = {
              enable = true;
              excludes = [
                "flake.lock"
              ];
            };
            markdownlint = {
              enable = true;
              settings.configuration = {
                MD013 = false;
                MD024 = false;
                MD026 = false;
                MD033 = false;
              };
            };
            treefmt = {
              enable = true;
              package = config.treefmt.build.wrapper;
            };
          };
          src = ../.;
        };
      };
    };
}
