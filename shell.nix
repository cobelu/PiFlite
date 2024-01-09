with import <nixpkgs> {};

mkShell {
  buildInputs = [
    rustup
    duckdb
    libiconv
    atkmm
  ] ++ lib.optional stdenv.isDarwin darwin.apple_sdk.frameworks.SystemConfiguration;
}
