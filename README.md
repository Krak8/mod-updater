# Mod Updater
This program updates all your mods to a newer/later version.

## To use:
1. Create a file named `config.toml`
2. Create a folder named `mods`;
3. Add the following to the file:
```toml
[minecraft]
version = "1.18.1" # Or minecraft version you wanted

[fabric]
mods = [
    "krypton", # add the mod id / slug from modrinth here
    "multiconnect"
]
```
3. Profit