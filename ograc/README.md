# Ograc
A cross-platform software manager / launcher (crates, but for apps).

When Ograc is complete, it will have the functionality of:
* Container Security (not using containers, but a simpler method)
* Any App Store / Command Line Installer
* Steam
* Cargo

## Ideas
There needs to be something like Cargo, but for apps.  https://crates.io is a great website, but it's not built for sharing you Rust applications.  There also needs to be something like Steam, but for apps.  Also containers are insane these days.  Instead of using containers, Ograc apps will be required to only use specific Rust crates that do not violate the rules of the system (the apps may not break these rules either):
* Don't use any unsafe code
* Don't use any of Rust's file IO: because the app could delete a bunch of files or create malicious ones.
* Don't Execute other processes: that could get bad.

Good (secure) modules:
* `any` / `arch` / `ascii`
* `borrow` / `boxed`
* `cell` / `char` / `clone` / `cmp` / `collections` / `convert`
* `default`
* `error`
* `f32` / `f64` / `fmt`
* `hash` / `hint`
* `i8` / `i16` / `i32` / `i64` / `i128` / `isize` / `io` / `iter`
* `marker` / `mem`
* `net` / `num`
* `ops` / `option`
* `panic`
* `rc`
* `result`
* `slice` / `str` / `string` / `sync`
* `thread` / `time`
* `u8` / `u16` / `u32` / `u64` / `u128` / `usize`
* `vec`

Bad (unsecure) modules:
* `env` - because it can set environment variables, even if the user doesn't want to set them - no need.
* `ffi` - because ffi is unsafe & insecure - let a trusted crate deal with all of that.
* `fs` - creating and deleting files maliciously - use ADI's `storage` module.
* `os` - filesystem access
* `path` - while it doesn't actually manipulate the filesystem - that's it's only use.
* `process` - starting random programs, not a good sign - just don't.
* `ptr` - don't use raw pointers - they're unsafe

Some basic things to make an app require unsafe, so the ADI crate and it's dependencies are whitelisted.

## Install
`cargo install ograc`

In the future there will be other ways to install:
* MacOS (file ograc.dmg)
* Windows (file ogracinstaller.exe)
* Android (Ograc on the PlayStore)
* Linux (ograc.tar.gz - containing ograc.desktop and ograc)

In the future there will be something like SteamOS, but it just runs Ograc - it'll be built on top of the Raspberry Pi image, both the ARM and Desktop versions.

## Running Apps
A normal Ograc application will be written in Rust.  Instead of directly using Cargo, the programmer will use Ograc to build their project.

## Porting To Ograc
pyc(python), java compiled files, C files will be able to be converted through transcoding.
APKs, ELF files, EXEs may have to run in an emulator.
