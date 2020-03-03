with import (builtins.fetchTarball "channel:nixos-20.03") {};
mkShell {
  buildInputs = [
    cargo
    rustfmt
  ];
}
