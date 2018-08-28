# Getting Started
## Install
`cargo install ograc`

## Make a new project
```bash
mkdir my_project
cd my_project
ograc init
```

## List of commands
```
ograc - Print the list of commands.
ograc upgrade - Run `cargo install grizzly --force`.
ograc version - Print the crate version of grizzly.
ograc gui - Open the Store GUI.
ograc run [app] - Run this app, or an app from the store.
ograc test - Run unit tests & Time them.
ograc android - Build & Deploy to Android.
ograc web - Build for the Web (wasm) & Open the HTML in a web browser.
ograc windows - Build Windows port.
ograc linux - Build Linux port.
ograc macos - Build MacOS port.
ograc release - Build all ports.
ograc html - Build the website for this project based on documentation and .md files.
ograc install [app_name] - Install this app, or an app from the store.
ograc update - Update all installed apps.
```

Generated files are placed in project/out/
