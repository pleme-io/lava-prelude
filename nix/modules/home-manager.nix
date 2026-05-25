# nix/modules/home-manager.nix — auto-generated from lava-prelude.caixa.lisp
{ config, lib, pkgs, ... }:
let cfg = config.programs.lava-prelude; in {
  options.programs.lava-prelude = {
    enable = lib.mkEnableOption "lava-prelude";
    package = lib.mkOption { type = lib.types.package; default = pkgs.lava-prelude or null; };
  };
  config = lib.mkIf cfg.enable { home.packages = [ cfg.package ]; };
}
