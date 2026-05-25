# nix/modules/darwin.nix — auto-generated from lava-prelude.caixa.lisp
{ config, lib, pkgs, ... }:
let cfg = config.services.lava-prelude; in {
  options.services.lava-prelude = {
    enable = lib.mkEnableOption "lava-prelude";
    package = lib.mkOption { type = lib.types.package; default = pkgs.lava-prelude or null; };
  };
  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];
  };
}
