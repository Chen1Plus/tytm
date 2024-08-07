# Installation

#### Windows

1. Download the `tytm.exe` from the [releases page](https://github.com/Chen1Plus/tytm/releases/latest).
2. Place the executable in your desired directory.

```powershell
./tytm update
```

<!-- #### MacOS

Currently, a pre-built executable is not available. See [build](#build) for more. -->

# Usage

#### Update manifests

TyTM uses manifests to obtain the necessary information for fetching themes.

```shell
./tytm update
```

#### Install a theme

You can install a theme using its unique ID. Browse [themes](themes/index) to see the available themes.

```shell
./tytm add <ID>
```

#### Remove a theme

You can simply use the same id to remove a theme.

```shell
./tytm remove <ID>
```

<!--

# Build

1. install rust (https://www.rust-lang.org/tools/install)
2. `cargo build --release`

# Contribute

### Add Package Manifest

Refer to `manifest/`. Readme may be outdated.  
An example manifest (OneDark). The file name is same as the id.

```json
{
    "id": "onedark",
    "name": "Blackout",
    "version": "1.0.7",
    "source": {
        "type": "Zip",
        "value": {
            "url": "https://github.com/obscurefreeman/typora_theme_blackout/releases/download/V1.0.7/blackout_theme.zip",
            "content": "./",
            "excludes": []
        }
    },
    "assets": [ ... ],
    "pkgs": [ ... ]
}
```
"id" will be theme's name but use lowercase and use dash to replace whitespace. Ex: "GitHub Night" has id "github-night"  
"type" can be "Zip" or "Git"
- "Zip": download a zip file from web
- "Git": clone a repository

"content" means the root folder of css and assets.  
"excludes" means files in "content" but unnecessary, such as readme or license. You may also refer to `manifest/`. -->
