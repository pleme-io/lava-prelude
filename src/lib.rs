//! lava-prelude — single-import facade over the lava typed surface.
//!
//! `use lava_prelude::*;` pulls every typed primitive across the
//! 8-crate suite. Pattern: std::prelude.
//!
//! ## What's exposed
//!
//! | Source crate | Exports |
//! |---|---|
//! | lava-core      | `Architecture`, `Resource`, `ResourceRef`, `Value`, `Multiplicity`, `ProviderRef`, `Stack`, `BackendRef`, `Synthesizer`, `TerraformJson`, `MagmaPlan`, `RenderError` |
//! | lava-arch      | `Builder`, `Library`, `ArchitectureSpec`, `ArchError` |
//! | lava-contracts | `ArchitectureResult`, `NetworkResult`, `IamResult`, `ClusterResult`, `SecretsResult`, `SecretRef`, `SecretBackendKind`, `DnsResult`, `DnsRecord`, `StorageResult`, `LoadBalancerResult`, `ObservabilityResult` |
//! | lava-stack     | `Stack as StackInstance`, `StackTag`, `StackConfig`, `StackError` |
//! | lava-types     | `Type`, `TypeError`, `MatchKind` |
//! | lava-schema    | `Interface`, `Field`, `InterfaceRegistry`, `SchemaError` |
//! | lava-eval      | `eval_architecture`, `parse`, `Atom`, `Sx`, `EvalError`, `InputBindings`, `ParseError` |
//! | lava-runtime   | `EmbeddedRuntime`, `ArtifactInput`, `ArtifactBinding`, `EvaluationResult`, `Diagnostic`, `DiagnosticLevel`, `LavaRuntime`, `TerraformJsonRuntime`, `pick_runtime_for_path`, `RuntimeError` |
//!
//! ## Naming notes
//!
//! - `Stack as StackInstance` — lava-stack's `Stack` differs from
//!   lava-core's `Stack` (deployment instance vs. typed stack value).
//!   The prelude renames the lava-stack one so consumers can `use
//!   lava_prelude::*` without name collision.
//!
//! ## Example
//!
//! ```ignore
//! use lava_prelude::*;
//!
//! let rt = LavaRuntime::new();
//! let result = rt.evaluate(&ArtifactInput {
//!     source: tlisp_source,
//!     bindings: bindings,
//!     name: Some("aws-vpc-network".to_string()),
//! })?;
//!
//! // Multi-target emission.
//! let tf_json: serde_json::Value =
//!     Synthesizer::<TerraformJson>::synthesize(&result.architecture)?;
//! ```

// ── re-exports ─────────────────────────────────────────────────────

pub use lava_core::{
    Architecture, BackendRef, MagmaPlan, Multiplicity, ProviderRef, RenderError, RenderTarget,
    Resource, ResourceRef, Stack, Synthesizer, TerraformJson, Value,
};

pub use lava_arch::{ArchError, ArchitectureSpec, Builder, Library};

pub use lava_contracts::{
    ArchitectureResult, ClusterResult, DnsRecord, DnsResult, IamResult, LoadBalancerResult,
    NetworkResult, ObservabilityResult, SecretBackendKind, SecretRef, SecretsResult,
    StorageResult,
};

pub use lava_stack::{Stack as StackInstance, StackConfig, StackError, StackTag};

pub use lava_types::{MatchKind, Type, TypeError};

pub use lava_schema::{Field, Interface, InterfaceRegistry, SchemaError};

pub use lava_eval::{
    eval_architecture, eval_architecture_with_schema, parse, parse_all, Atom, EvalError,
    InputBindings, ParseError, Sx,
};

pub use lava_architectures::{interface_for, load_bundled, ARCHITECTURE_DIR};

pub use lava_runtime::{
    pick_runtime_for_path, ArtifactBinding, ArtifactInput, Diagnostic, DiagnosticLevel,
    EmbeddedRuntime, EvaluationResult, LavaRuntime, RuntimeError, TerraformJsonRuntime,
};

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    /// The whole-stack smoke test: author tatara-lisp source, evaluate
    /// via LavaRuntime, synthesize to terraform.json via Synthesizer<T>.
    /// One import gave us everything.
    #[test]
    fn one_import_full_pipeline_works_end_to_end() {
        let tlisp = r#"
            (deflava-architecture demo-vpc
              :inputs ((:cidr "10.50.0.0/16"))
              :resources (
                (aws-vpc "demo"
                  :cidr-block "{cidr}"
                  :enable-dns-support #t)))
        "#;

        let rt = LavaRuntime::new();
        let input = ArtifactInput {
            source: tlisp.to_string(),
            bindings: IndexMap::new(),
            name: Some("demo".to_string()),
        };

        let result = rt.evaluate(&input).unwrap();
        let json: serde_json::Value =
            Synthesizer::<TerraformJson>::synthesize(&result.architecture).unwrap();

        assert_eq!(json["resource"]["aws_vpc"]["demo"]["cidr_block"], "10.50.0.0/16");
        assert_eq!(json["resource"]["aws_vpc"]["demo"]["enable_dns_support"], true);
    }

    /// Typed Interface validation + Architecture composition all
    /// reachable from the one `use lava_prelude::*;`.
    #[test]
    fn typed_interface_validation_via_prelude() {
        let mut iface = Interface::new("aws-vpc-network");
        iface.inputs.insert(
            "cidr".to_string(),
            Field::strict(Type::CidrBlock).with_doc("VPC CIDR block"),
        );
        iface.inputs.insert(
            "env".to_string(),
            Field::strict(Type::Enum {
                values: vec!["prod".into(), "staging".into(), "dev".into()],
            }),
        );

        let mut bag = IndexMap::new();
        bag.insert("cidr".to_string(), "10.0.0.0/16".to_string());
        bag.insert("env".to_string(), "prod".to_string());
        assert!(iface.validate_inputs(&bag).is_ok());

        // Wrong enum -> typed mismatch via SchemaError.
        bag.insert("env".to_string(), "preview".to_string());
        let errs = iface.validate_inputs(&bag).unwrap_err();
        assert!(matches!(errs[0], SchemaError::TypeMismatch { .. }));
    }

    /// Multi-target Synthesizer dispatch reachable through prelude.
    #[test]
    fn synthesizer_multi_target_reachable_via_prelude() {
        let arch = Architecture::new("empty");
        let _tf: serde_json::Value =
            Synthesizer::<TerraformJson>::synthesize(&arch).unwrap();
        let _plan: serde_json::Value = Synthesizer::<MagmaPlan>::synthesize(&arch).unwrap();
        // Both reachable; both produce typed values.
    }

    /// Auto-detect runtime resolution via prelude.
    #[test]
    fn auto_detect_runtime_via_prelude() {
        let p = std::path::Path::new("/some/path/architecture.tlisp");
        let rt = pick_runtime_for_path(p).unwrap();
        assert_eq!(rt.kind(), "lava");
    }
}
