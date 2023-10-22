{
  pkgs ? import <nixpkgs> {}
}:
  pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
      cargo
      rustc
      rust-analyzer
<<<<<<< HEAD
      nbd
=======
      libnbd
>>>>>>> 7df3500 (chore(shell.nix): add libnbd)
    ];
  }
