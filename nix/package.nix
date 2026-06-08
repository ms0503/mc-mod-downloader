{
  lib,
  mold,
  rustPlatform,
}:
let
  inherit (lib) cleanSource cleanSourceWith;
  cargoToml = ../Cargo.toml |> builtins.readFile |> builtins.fromTOML;
in
rustPlatform.buildRustPackage {
  inherit (cargoToml.package) version;
  RUSTFLAGS = "-Clink-arg=-fuse-ld=mold";
  cargoLock.lockFile = ../Cargo.lock;
  meta = {
    description = "A tool that download mods from multiple sources in bulk via the command line";
    license = lib.licenses.mit;
    mainProgram = "mc-mod-downloader";
    sourceProvenance = with lib.sourceTypes; [
      fromSource
    ];
  };
  nativeBuildInputs = [
    mold
  ];
  pname = cargoToml.package.name;
  src =
    let
      isNotNixDirectory = name: type: !(type == "directory" && builtins.baseNameOf name == "nix");
      isNotNixFiles =
        name: type:
        !(type == "file" && (lib.hasSuffix ".nix" name || builtins.baseNameOf name == "flake.lock"));
    in
    cleanSourceWith {
      filter = isNotNixDirectory;
      src = cleanSourceWith {
        filter = isNotNixFiles;
        src = cleanSource ../.;
      };
    };
}
