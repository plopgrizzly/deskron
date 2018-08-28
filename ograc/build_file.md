# Build File
Ograc build files are stored in a file called `build.nr`.

## Format
The file is a list of setter functions, to set each property of the app.
Both name and version are required, like Cargo.
```
// Set the name of the project to "My Project" (Unicode)
name My Project
// Set the semver version (Versioning is the same as crates.io)
version 0.1.0
// Set the URL or path to the app icon.
icon icon.png
// Set the license to Boost 1.0
license BSL-1.0
// Add a copyright holder.  You may call this several times, if theres multiple.
copyright Jeron Lau 2018 <jeron.lau@plopgrizzly.com>
// Set the website for this project.  URL or Path to Markdown File.
website https://plopgrizzly.com
// Set the repository URL for this project
repository https://github.com/plopgrizzly/ograc
// Set the description for this project (Unicode)
description Cala Physics Engine Tutorial
// Add a dependency for this project (path, URL or version)
dependency cala ../../
// Add a keyword, can be called many times.
keyword
// Add a category, can be called many times.  Options are:
// `AudioVideo`, `Audio`, `Video`, `Development`, `Education`, `Game`,
// `Graphics`, `Network`, `Office`, `Science`, `Settings`, `System`, `Utility`.
// From: https://specifications.freedesktop.org/menu-spec/latest/apa.html#main-category-registry
category AudioVideo
// Add a contributor by real name or user name.  Can call multiple times.
contributor Jeron Lau
contributor OxyDeadbeef
```
