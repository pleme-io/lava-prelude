(defcaixa
  :name
  "lava-prelude"
  :kind
  :Biblioteca
  :ecosystem
  :rust-single-crate
  :package
  {:name "lava-prelude"
   :version "0.1.0"
   :description "Single-import facade over the lava typed surface. use lava_prelude::*; pulls Architecture / Resource / Type / Interface / Synthesizer / EmbeddedRuntime / NetworkResult / LavaRuntime / TerraformJsonRuntime + every layer. Pattern: std::prelude. Reduces 9-crate dependency wall to one import."
   :license "MIT"
   :repository "https://github.com/pleme-io/lava-prelude"}
  :ci-config
  {:bump {:default-type "patch"}
   :publish {:no-verify true}}
  :workflows
  [:auto-release :pre-merge-gate :security-gate])
