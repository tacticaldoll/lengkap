//! Executable architectural governance for the Lengkap workspace.

#![forbid(unsafe_code)]

use std::{env, process::ExitCode};

use tianheng::prelude::*;

const CONTRACT_DEPENDENCY_REASON: &str = concat!(
    "lengkap-contract is the portable no_std mechanism root: its model and ",
    "adjudication need no dependency, so it must declare none."
);
const FACADE_DEPENDENCY_REASON: &str = concat!(
    "lengkap is the curated public entrypoint: its surface is the complete ",
    "lengkap-contract re-export, so it may depend only on lengkap-contract."
);
const GOVERNOR_DEPENDENCY_REASON: &str = concat!(
    "the governance gate stays independent of the workspace it judges: ",
    "lengkap-governance may depend only on tianheng."
);
const NO_IO_REASON: &str = concat!(
    "evidence adjudication is an in-memory mechanism: lengkap-contract must ",
    "not call std::io, std::fs, std::net, or std::process."
);
const SANS_IO_REASON: &str = concat!(
    "evidence adjudication is deterministic and caller-driven: ",
    "lengkap-contract reads no ambient clock and exposes no public async API."
);
const NO_SERDE_REASON: &str = concat!(
    "lengkap-contract owns transient generic mechanism, not a wire format: ",
    "Serialize and Deserialize remain the caller's responsibility."
);

fn with_contract_dependency_boundary(constitution: Constitution) -> Constitution {
    constitution.boundary(
        CrateBoundary::crate_("lengkap-contract")
            .restrict_dependencies_to(Vec::<&str>::new())
            .because(CONTRACT_DEPENDENCY_REASON),
    )
}

fn with_facade_dependency_boundary(constitution: Constitution) -> Constitution {
    constitution.boundary(
        CrateBoundary::crate_("lengkap")
            .restrict_dependencies_to(["lengkap-contract"])
            .because(FACADE_DEPENDENCY_REASON),
    )
}

fn with_governor_dependency_boundary(constitution: Constitution) -> Constitution {
    constitution.boundary(
        CrateBoundary::crate_("lengkap-governance")
            .restrict_dependencies_to(["tianheng"])
            .because(GOVERNOR_DEPENDENCY_REASON),
    )
}

fn with_no_io_boundaries(mut constitution: Constitution) -> Constitution {
    for path in ["std::io", "std::fs", "std::net", "std::process"] {
        constitution = constitution.boundary(
            ModuleBoundary::in_crate("lengkap-contract")
                .module("crate")
                .must_not_call_inline(path)
                .because(NO_IO_REASON),
        );
    }
    constitution
}

fn with_sans_io_boundary(constitution: Constitution) -> Constitution {
    constitution.sans_io_pure(
        SansIoPure::in_crate("lengkap-contract")
            .module("crate")
            .reading_clock_via("std::time", ["now"])
            .because(SANS_IO_REASON),
    )
}

fn with_no_serde_boundary(constitution: Constitution) -> Constitution {
    constitution.forbidden_marker_boundary(
        ForbiddenMarkerBoundary::in_crate("lengkap-contract")
            .module("crate")
            .must_not_acquire("serde::Serialize")
            .and_not_acquire("serde::Deserialize")
            .because(NO_SERDE_REASON),
    )
}

fn constitution() -> Constitution {
    let constitution = Constitution::new("lengkap");
    let constitution = with_contract_dependency_boundary(constitution);
    let constitution = with_facade_dependency_boundary(constitution);
    let constitution = with_governor_dependency_boundary(constitution);
    let constitution = with_no_io_boundaries(constitution);
    let constitution = with_sans_io_boundary(constitution);
    with_no_serde_boundary(constitution)
}

fn main() -> ExitCode {
    tianheng::run(&constitution(), env::args())
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
        sync::atomic::{AtomicUsize, Ordering},
    };

    use super::*;

    static NEXT_FIXTURE: AtomicUsize = AtomicUsize::new(0);

    struct Fixture {
        root: PathBuf,
    }

    impl Fixture {
        fn package(package: &str, source: &str) -> Self {
            let fixture = Self::empty();
            fs::create_dir_all(fixture.root.join("src")).unwrap();
            fs::write(
                fixture.root.join("Cargo.toml"),
                format!(
                    "[package]\nname = \"{package}\"\nversion = \"0.0.0\"\n\
                     edition = \"2024\"\n"
                ),
            )
            .unwrap();
            fs::write(fixture.root.join("src/lib.rs"), source).unwrap();
            fixture
        }

        fn package_with_unwanted_dependency(package: &str) -> Self {
            let fixture = Self::empty();
            fs::create_dir_all(fixture.root.join("subject/src")).unwrap();
            fs::create_dir_all(fixture.root.join("unwanted/src")).unwrap();
            fs::write(
                fixture.root.join("Cargo.toml"),
                "[workspace]\nresolver = \"2\"\nmembers = [\"subject\", \"unwanted\"]\n",
            )
            .unwrap();
            fs::write(
                fixture.root.join("subject/Cargo.toml"),
                format!(
                    "[package]\nname = \"{package}\"\nversion = \"0.0.0\"\n\
                     edition = \"2024\"\n\n[dependencies]\n\
                     unwanted = {{ path = \"../unwanted\" }}\n"
                ),
            )
            .unwrap();
            fs::write(fixture.root.join("subject/src/lib.rs"), "").unwrap();
            fs::write(
                fixture.root.join("unwanted/Cargo.toml"),
                "[package]\nname = \"unwanted\"\nversion = \"0.0.0\"\n\
                 edition = \"2024\"\n",
            )
            .unwrap();
            fs::write(fixture.root.join("unwanted/src/lib.rs"), "").unwrap();
            fixture
        }

        fn clean_workspace() -> Self {
            let fixture = Self::empty();
            fs::write(
                fixture.root.join("Cargo.toml"),
                "[workspace]\nresolver = \"2\"\n\
                 members = [\"contract\", \"facade\", \"governor\", \"tianheng\"]\n",
            )
            .unwrap();
            fixture.member("contract", "lengkap-contract", "", "#![no_std]\n");
            fixture.member(
                "facade",
                "lengkap",
                "lengkap-contract = { path = \"../contract\" }\n",
                "#![no_std]\npub use lengkap_contract::*;\n",
            );
            fixture.member(
                "governor",
                "lengkap-governance",
                "tianheng = { path = \"../tianheng\" }\n",
                "",
            );
            fixture.member("tianheng", "tianheng", "", "");
            fixture
        }

        fn empty() -> Self {
            let id = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
            let root = std::env::temp_dir()
                .join(format!("lengkap-governance-{}-{id}", std::process::id()));
            let _ = fs::remove_dir_all(&root);
            fs::create_dir_all(&root).unwrap();
            Self { root }
        }

        fn member(&self, directory: &str, package: &str, dependencies: &str, source: &str) {
            let root = self.root.join(directory);
            fs::create_dir_all(root.join("src")).unwrap();
            fs::write(
                root.join("Cargo.toml"),
                format!(
                    "[package]\nname = \"{package}\"\nversion = \"0.0.0\"\n\
                     edition = \"2024\"\n\n[dependencies]\n{dependencies}"
                ),
            )
            .unwrap();
            fs::write(root.join("src/lib.rs"), source).unwrap();
        }

        fn manifest(&self) -> &Path {
            self.root.as_path()
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    fn assert_reacts(
        constitution: Constitution,
        fixture: &Fixture,
        reason: &str,
        target: &str,
        rule_type: &str,
        fact_type: &str,
    ) {
        let outcome = tianheng::check_constitution(&constitution, &fixture.root.join("Cargo.toml"));
        assert_eq!(
            outcome.exit_code(),
            1,
            "expected violation, got {outcome:?}"
        );
        let Outcome::Violations(report) = outcome else {
            panic!("exit class 1 must carry violations");
        };
        assert!(
            report.violations.iter().any(|violation| {
                violation.reason == reason
                    && violation.target() == target
                    && violation.rule_key().rule_type() == rule_type
                    && violation.fact().fact_type() == fact_type
            }),
            "the intended boundary did not react: {report:?}"
        );
    }

    #[test]
    fn repository_is_clean_and_every_member_is_governed() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .unwrap();
        GovernanceTest::for_constitution(constitution())
            .with_manifest_dir(root)
            .assert_clean()
            .assert_all_workspace_members_covered();
    }

    #[test]
    fn clean_control_passes_the_complete_constitution() {
        let fixture = Fixture::clean_workspace();
        GovernanceTest::for_constitution(constitution())
            .with_manifest_dir(fixture.manifest())
            .assert_clean();
    }

    #[test]
    fn contract_dependency_boundary_reacts() {
        let fixture = Fixture::package_with_unwanted_dependency("lengkap-contract");
        assert_reacts(
            with_contract_dependency_boundary(Constitution::new("fixture")),
            &fixture,
            CONTRACT_DEPENDENCY_REASON,
            "lengkap-contract",
            "tianheng.rule/guibiao/restrict-dependencies-to",
            "tianheng.fact/guibiao/dependency",
        );
    }

    #[test]
    fn facade_dependency_boundary_reacts() {
        let fixture = Fixture::package_with_unwanted_dependency("lengkap");
        assert_reacts(
            with_facade_dependency_boundary(Constitution::new("fixture")),
            &fixture,
            FACADE_DEPENDENCY_REASON,
            "lengkap",
            "tianheng.rule/guibiao/restrict-dependencies-to",
            "tianheng.fact/guibiao/dependency",
        );
    }

    #[test]
    fn governor_dependency_boundary_reacts() {
        let fixture = Fixture::package_with_unwanted_dependency("lengkap-governance");
        assert_reacts(
            with_governor_dependency_boundary(Constitution::new("fixture")),
            &fixture,
            GOVERNOR_DEPENDENCY_REASON,
            "lengkap-governance",
            "tianheng.rule/guibiao/restrict-dependencies-to",
            "tianheng.fact/guibiao/dependency",
        );
    }

    #[test]
    fn each_io_boundary_reacts_independently() {
        for (path, source) in [
            ("std::io", "pub fn f() { let _ = std::io::empty(); }\n"),
            ("std::fs", "pub fn f() { let _ = std::fs::read(\"x\"); }\n"),
            (
                "std::net",
                "pub fn f() { let _ = std::net::TcpStream::connect(\"x\"); }\n",
            ),
            (
                "std::process",
                "pub fn f() { let _ = std::process::Command::new(\"x\"); }\n",
            ),
        ] {
            let fixture = Fixture::package("lengkap-contract", source);
            let boundary = Constitution::new("fixture").boundary(
                ModuleBoundary::in_crate("lengkap-contract")
                    .module("crate")
                    .must_not_call_inline(path)
                    .because(NO_IO_REASON),
            );
            assert_reacts(
                boundary,
                &fixture,
                NO_IO_REASON,
                path,
                "tianheng.rule/guibiao/confine-inline-symbol-path",
                "tianheng.fact/guibiao/inline-path",
            );
        }
    }

    #[test]
    fn ambient_clock_boundary_reacts() {
        let fixture = Fixture::package(
            "lengkap-contract",
            "pub fn f() { let _ = std::time::SystemTime::now(); }\n",
        );
        assert_reacts(
            with_sans_io_boundary(Constitution::new("fixture")),
            &fixture,
            SANS_IO_REASON,
            "std::time",
            "tianheng.rule/guibiao/confine-inline-symbol-path",
            "tianheng.fact/guibiao/inline-path",
        );
    }

    #[test]
    fn public_async_boundary_reacts() {
        let fixture = Fixture::package("lengkap-contract", "pub async fn wait() {}\n");
        assert_reacts(
            with_sans_io_boundary(Constitution::new("fixture")),
            &fixture,
            SANS_IO_REASON,
            "crate",
            "tianheng.rule/hunyi/async-exposure",
            "tianheng.fact/hunyi/async-exposure",
        );
    }

    #[test]
    fn serialization_marker_boundary_reacts() {
        let fixture = Fixture::package(
            "lengkap-contract",
            "#[derive(serde::Serialize)]\npub struct Wire;\n",
        );
        assert_reacts(
            with_no_serde_boundary(Constitution::new("fixture")),
            &fixture,
            NO_SERDE_REASON,
            "crate",
            "tianheng.rule/hunyi/forbidden-marker",
            "tianheng.fact/hunyi/forbidden-marker-acquisition",
        );
    }

    #[test]
    fn constitution_projection_is_fresh() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .unwrap();
        GovernanceTest::for_constitution(constitution())
            .with_manifest_dir(root)
            .assert_projection_fresh("docs/architecture/tianheng-law.md");
    }

    #[test]
    fn contract_declares_no_std_unconditionally() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .unwrap();
        let source = fs::read_to_string(root.join("crates/lengkap-contract/src/lib.rs")).unwrap();
        assert!(
            source.lines().any(|line| line.trim() == "#![no_std]"),
            "lengkap-contract must declare #![no_std] unconditionally"
        );
    }
}
