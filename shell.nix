with import <nixpkgs> {};

mkShell {
  buildInputs = [
    rustup
    duckdb
    libiconv
    atkmm
    darwin.apple_sdk.frameworks.SystemConfiguration
  ];
}
