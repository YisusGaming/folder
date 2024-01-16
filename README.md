[![build](https://github.com/YisusGaming/folder/actions/workflows/rust.yml/badge.svg)](https://github.com/YisusGaming/folder/actions/workflows/rust.yml)

<div align="center">
<h1>Folder</h1>

CLI Program to manage folders written in Rust.

</div>

**Folder** is just a simple wrapper around `mkdir` and `rm` that allows you create and delete folders.

Creating a folder looks like this:

```sh
folder new dirname
```

that'll run `mkdir -v dirname` for you.

Deleting folders looks like this:

```sh
folder del dirname
```

that'll run `rm -v -r -I dirname` for you.

**Keep in mind!** This is not meant to be complete CLI to manage folders, it's just simply a convenience.

# License

Licensed under the MIT License.
See [LICENSE](LICENSE) for details.
