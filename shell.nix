with import <nixpkgs> { };
mkShell {
  buildInputs = [
    glib
    libsoup_3
    openssl
    openssl.dev
    cargo-audit
    cargo-tarpaulin
    clippy
  ];

  PKG_CONFIG_PATH = "${glib.dev}/lib/pkgconfig:${libsoup_3.dev}/lib/pkgconfig:${openssl.dev}/lib/pkgconfig";
  LD_LIBRARY_PATH = lib.makeLibraryPath [ openssl ];
  DATABASE_URL = "postgres://postgres:passwordxxi0n@localhost:5432/xxi0ndb";
}

