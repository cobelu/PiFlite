# PiFlite

## General Rust Advice

* Upgrade Rust
  * `rustup update`
* Rust-analyzer

## VSC Advice

* Rust-analyzer is stuck while collecting metadata
  * `cargo check --workspace --manifest-path Cargo.toml --all-targets`
* VSC has issues loading workspaces, so see `help/settings.json` for an example root-level settings file to go in a root-level `.vscode` directory

## Nix Advice

* [Install instructions here](https://nixos.org/download)
* [Nix environment selector for VSC](https://marketplace.visualstudio.com/items?itemName=arrterian.nix-env-selector)
* Not using the nix environment selector? Run `nix-shell` to start a nix-shell instance with everything you need.
