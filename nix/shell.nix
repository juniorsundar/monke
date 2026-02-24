{ pkgs, rustToolchain }:
pkgs.mkShell {
  name = "rust-dev-shell";

  packages = with pkgs; [
    rustToolchain
    uv
    python3
    just
  ];

  shellHook = ''
    VENV_DIR=".venv"
    if [ ! -d "$VENV_DIR" ]; then
      echo "Creating Python virtual environment..."
      uv venv $VENV_DIR -p ${pkgs.python3}/bin/python
    fi
    source "$VENV_DIR/bin/activate"

    # Pre-commit setup
    uv pip install pre-commit
    pre-commit install
  '';
}
