with import <nixpkgs> {};

mkShell {
  buildInputs = [
    atkmm
    duckdb
    libiconv
    openssl
    rustup
  ] ++ lib.optional stdenv.isDarwin darwin.apple_sdk.frameworks.SystemConfiguration;

  # nativeBuildInputs = with pkgs; [
  #     pkg-config
  # ];

  # dbus = pkgs.dbus;

  shellHook = ''
    export DUCKDB_LIB_DIR="${duckdb}/bin"
  '';

}
