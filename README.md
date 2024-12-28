This repository contains the templates available on the LighthouseMC Minecraft server.

They may be used for any purpose.
 When used on the LighthouseMC Minecraft server, make sure to follow the server rules.
 We, LighthouseMC and its developers/moderators), reserve the right to penalise misuse <ins>on the LighthouseMC Minecraft server</ins>.
 We are not responsible or liable for use outside of the server.


**Contributing**:

To add support for another language, fork this repository.

Each template must be in its own directory, which contains a `template.toml` and `build.sh` file.

For a starting point, see the `blank` template.

The `template.toml` file follows this format:
```toml
[template]
name        = "Name of the template"
description = "A short description of the template. This could be the slogan of the language being supported."
icon        = "Base64 Minecraft skin texture value" # You can generate one using https://mineskin.org.
homepage    = "https://example.com" # OPTIONAL: A link to the homepage of the language being supported.
advanced    = true # OPTIONAL: This indicates that this is a template intended for more advanced users.

exclude = [ # OPTIONAL: A list of files or directories to exclude from the template.
    "file_or_dir_name",
    "icon.png"
]
```
