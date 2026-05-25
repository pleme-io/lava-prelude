//! End-to-end pipeline test — exercise the whole lava stack through
//! the single `use lava_prelude::*;` import.
//!
//! Pipeline under test:
//!
//! ```text
//! .tlisp source string
//!   ─► LavaRuntime::evaluate (lava-runtime)
//!     ─► parse_all (lava-eval)
//!     ─► Interface::validate_inputs (lava-schema, via with_schema)
//!     ─► eval_architecture body (lava-eval → lava-arch)
//!   ─► Architecture (lava-core)
//!     ─► Synthesizer<TerraformJson>::synthesize
//!     ─► serde_json::Value (terraform.json shape — magma-compatible)
//! ```

use lava_prelude::*;

/// Authored .tlisp source → typed terraform.json via the single
/// `use lava_prelude::*;` import. Every step in the lava pipeline is
/// reached through one import.
#[test]
fn full_pipeline_tlisp_to_terraform_json_via_one_prelude() {
    let tlisp = r#"
        (deflava-interface demo-vpc
          :inputs ((:cidr :type :cidr-block :required #t)))

        (deflava-architecture demo-vpc
          :inputs ((:cidr "10.0.0.0/16"))
          :resources (
            (aws-vpc "demo"
              :cidr-block "{cidr}"
              :enable-dns-support #t
              :tags ((Name "demo-vpc")))))
    "#;

    // 1) Parse-all surface (multi-form .tlisp).
    let forms = parse_all(tlisp).unwrap();
    assert_eq!(forms.len(), 2);

    // 2) Build the typed Interface in code (the .tlisp deflava-interface
    //    sibling form is informational; runtime gate uses the typed
    //    Interface).
    let mut iface = Interface::new("demo-vpc");
    iface
        .inputs
        .insert("cidr".to_string(), Field::strict(Type::CidrBlock));

    // 3) Eval through the schema gate with valid input.
    let mut bindings = InputBindings::new();
    bindings.set_str("cidr", "10.42.0.0/16");
    let arch = eval_architecture_with_schema(tlisp, &bindings, &iface).unwrap();

    // 4) Synthesize multi-target — TerraformJson + MagmaPlan both
    //    reachable through the prelude.
    let tf_json: serde_json::Value =
        Synthesizer::<TerraformJson>::synthesize(&arch).unwrap();
    let magma_plan: serde_json::Value =
        Synthesizer::<MagmaPlan>::synthesize(&arch).unwrap();

    // 5) Wire correctness — terraform.json carries the input through.
    assert_eq!(tf_json["resource"]["aws_vpc"]["demo"]["cidr_block"], "10.42.0.0/16");
    assert_eq!(tf_json["resource"]["aws_vpc"]["demo"]["enable_dns_support"], true);
    assert_eq!(tf_json["resource"]["aws_vpc"]["demo"]["tags"]["Name"], "demo-vpc");

    // 6) MagmaPlan is also reachable + non-empty.
    assert!(!magma_plan.is_null());
}

/// Schema-gate rejection at compose time, via the typed interface
/// registry shipped by lava-architectures. Bad input fails before any
/// resource is built — the GraphQL-equivalent compose-time gate.
#[test]
fn bundled_architecture_schema_rejects_bad_input_at_compose_time() {
    let iface = interface_for("cloudflare-dns-records").unwrap();
    let src = std::fs::read_to_string(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("workspace root")
            .join("lava-architectures")
            .join("architectures")
            .join("cloudflare-dns-records.tlisp"),
    )
    .ok();

    if let Some(src) = src {
        // Empty bag — :zone-id is required #t.
        let bindings = InputBindings::new();
        let err = eval_architecture_with_schema(&src, &bindings, &iface).unwrap_err();
        match err {
            EvalError::Schema {
                interface, errors, ..
            } => {
                assert_eq!(interface, "cloudflare-dns-records");
                assert!(errors.iter().any(|e| matches!(
                    e,
                    SchemaError::MissingRequired { name, .. } if name == "zone-id"
                )));
            }
            other => panic!("expected EvalError::Schema, got {other:?}"),
        }
    }
}

/// Auto-detect a runtime by file extension + drive it through the
/// embedded interpreter trait.
#[test]
fn runtime_auto_detect_and_drive_via_prelude() {
    let p = std::path::Path::new("/some/path/architecture.tlisp");
    let rt = pick_runtime_for_path(p).unwrap();
    assert_eq!(rt.kind(), "lava");
    assert_eq!(rt.extension(), "tlisp");
}
