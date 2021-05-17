with import <nixpkgs> {};

mkShell {
  buildInputs = [
    stdenv
    cargo
    pkg-config
  ];
}
