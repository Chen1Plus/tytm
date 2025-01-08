# TyTM (Typora Theme Manager)

A theme package manager for Typora.

Current features:
- [x] add a theme
- [ ] remove a theme
- [ ] list installed themes

## Installation

> [!Warning]
>
> Only use the pre-built binary if you trust me; otherwise, consider building it from source.

### Windows

Download the `tytm.exe` from the release page.

### Mac OS

1. Download the `tytm` from the release page.
2. Remove the file attribute to bypass your Mac's restriction and grant execute permission.

```sh
xattr -dr com.apple.quarantine ./tytm
chmod +x ./tytm
```

## Usage

```sh
./tytm add <URL>
```

which the url points to the git/zip file of the theme.
