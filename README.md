# Mod Updater
This program updates all your mods to a newer/later version.

## To use:
### Creating the config file
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
3. Run the program, and it will download all the mods for you listed in the config file.
___
### Generate the config file
1. Place the program inside your mods folder
2. Run `mod_updater --scan`
3. Fill in the missing mods by yourself, refer to the example config file
4. Profit
