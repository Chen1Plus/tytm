# Quick Start

```bash
./tytm update
```

## Usage:
```bash
./tytm.exe update
./tytm.exe add <THEME_ID>
./tytm.exe add <THEME_ID> --sub <SUB_THEME_ID>
```
<THEME_ID> will be theme's name but use lowercase and use dash to replace whitespace. Ex: "GitHub Night" has id "github-night"
```bash
./tytm.exe rm <THEME_ID>
./tytm.exe rm <THEME_ID> --sub <SUB_THEME_ID>
```

## Build
1. install rust (https://www.rust-lang.org/tools/install)
2. `cargo build --release`

## Contribute: 

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
"excludes" means files in "content" but unnecessary, such as readme or license. You may also refer to `manifest/`.
