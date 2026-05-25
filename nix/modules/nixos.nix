# nix/modules/nixos.nix — auto-generated from lava-prelude.caixa.lisp
# description: "Single-import facade over the lava typed surface. use lava_prelude::*; pulls Architecture / Resource / Type / Interface / Synthesizer / EmbeddedRuntime / NetworkResult / LavaRuntime / TerraformJsonRuntime + every layer. Pattern: std::prelude. Reduces 9-crate dependency wall to one import."
{ config, lib, pkgs, ... }:
let
  cfg = config.services.lava-prelude;
in {
  options.services.lava-prelude = {
    enable = lib.mkEnableOption "lava-prelude";
    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.lava-prelude or null;
    };
  };
  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];
  };
}
