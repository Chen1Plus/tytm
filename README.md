# TyTM (Typora Theme Manager)

A theme package manager for Typora. It's currently in progress, lacking features and not that user friendly.

Current features:
- [x] add a theme
- [x] remove a theme
- [ ] list installed themes

## Usage:
```bash
./tytm.exe update
./tytm.exe add <THEME_ID>
```
<THEME_ID> will be theme's name but use lowercase and use dash to replace whitespace. Ex: "GitHub Night" has id "github-night"
```bash
./tytm.exe rm <THEME_ID>
```

## Contribute: 

### Add Package Manifest

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
    }
}
```
"id" will be theme's name but use lowercase and use dash to replace whitespace. Ex: "GitHub Night" has id "github-night"  
"type" can be "Zip" or "Git"
- "Zip": download a zip file from web
- "Git": clone a repository

"content" means the root folder of css and assets.  
"excludes" means files in "content" but unnecessary, such as readme or license. You may also refer to `manifest/`.
