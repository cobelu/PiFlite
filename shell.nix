with import <nixpkgs> {};

mkShell {
  buildInputs = [
    rustup
    duckdb
    libiconv
    darwin.apple_sdk.frameworks.SystemConfiguration
  ];
}
