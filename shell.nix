with import <nixpkgs> { };
mkShell {
  buildInputs = [
    glib
    libsoup_3
    openssl
    openssl.dev
    cargo-tarpaulin
  ];

  PKG_CONFIG_PATH = "${glib.dev}/lib/pkgconfig:${libsoup_3.dev}/lib/pkgconfig:${openssl.dev}/lib/pkgconfig";
}
