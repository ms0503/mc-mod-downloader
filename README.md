# Minecraft MOD Downloader

Minecraft MOD Downloader is a tool that download mods from multiple sources in
bulk via the command line.

## Supported Platforms

- CurseForge (require your api key)
- File URL
- Modrinth

## Usage

```shell
mc-mod-downloader [OPTIONS] <COMMAND>
```

### Common Options

| Short |   Long    |  Description  |
| :---: | :-------: | :-----------: |
|  -V   | --version | Print version |
|  -h   |  --help   |  Print help   |

### Download Subcommand

```shell
mc-mod-downloader download [OPTIONS] --side <SIDE>
```

| Short |        Long        |                                       Description                                        |
| :---: | :----------------: | :--------------------------------------------------------------------------------------: |
|  -c   |      --config      |                     Path to config file [Default value: "mods.json"]                     |
|  -d   |       --dir        |                      Path to mods directory [Default value: "mods"]                      |
|  -o   | --include-optional |                 Whether to download optional mods [Default value: false]                 |
|  -s   |       --side       |             Side of mod(s) to download [Possible values: "client", "server"]             |
|       |   --skip-source    | Mod sources to skip [Comma separated, possible values: "curseforge", "file", "modrinth"] |

### Generate Page Subcommand

```shell
mc-mod-downloader generate-page [OPTIONS]
```

| Short | Long  |                   Description                    |
| :---: | :---: | :----------------------------------------------: |
|  -o   | --out | Path to output file [Default value: "mods.html"] |

## Config File

Schema: [mods.schema.json](mods.schema.json)

Supported JSON, TOML and YAML.

```toml
# "mods" - List of mods

# CurseForge Example
[[mods]]
# File ID in end of URL
fileId = 3070947
# File Name in File Details
name = "AromaBackup-1.12.2-3.0.0.0.b135.jar"
# Project ID in Details
projectId = 225658
side = {
    # Server mod is not needed for client-side.
    client = "none",
    # Server can run even if this mod is not installed.
    server = "optional"
}
source = "curseforge"

# File URL Example
[[mods]]
# File name
name = "GVCReversion2__1.12.2__α.10.2.zip"
# A mod that adds some items is needed for both sides.
side = {
    client = "require",
    server = "require"
}
source = "file"
# URL to download
url = "https://www.dropbox.com/scl/fi/fgsdozwx61b09d3fosnm2/GVCReversion2__1.12.2__-.10.2.zip?rlkey=0hdkd4ijw938qnk0l8m1jo0w0&dl=1"

# Modrinth Example
[[mods]]
# File name in files (marked primary)
name = "appleskin-fabric-mc26.1-3.0.9.jar"
# Project ID (press kebab-button on right-top, then press Copy ID to copy)
projectId = "EsAfCjCV"
side = {
    # You can play even if this mod is not installed.
    client = "optional",
    # Client mod must not be installed on server-side.
    server = "none"
}
source = "modrinth"
# Version ID in Metadata
versionId = "HwaLJe3v"
```
