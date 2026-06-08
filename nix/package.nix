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
  src = cleanSourceWith {
    filter =
      name: type:
      let
        isNixDirectory = type == "directory" && builtins.baseNameOf name == "nix";
        isNixFiles =
          type == "file" && (lib.hasSuffix ".nix" name || builtins.baseNameOf name == "flake.lock");
      in
      !isNixDirectory && !isNixFiles;
    src = cleanSource ../.;
  };
}
