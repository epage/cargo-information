use cargo_test_macro::cargo_test;
use cargo_test_support::{compare::assert_ui, curr_dir, Project};

use super::{cargo_info, init_registry_without_token};

#[cargo_test]
fn case() {
    init_registry_without_token();
    cargo_test_support::registry::Package::new("my-package", "0.1.0").publish();
    cargo_test_support::registry::Package::new("my-package", "0.2.0")
        .rust_version("1.0.0")
        .publish();
    cargo_test_support::registry::Package::new("my-package", "0.2.1")
        .rust_version("1.70.0")
        .publish();

    let project = Project::from_template(curr_dir!().join("in"));
    let project_root = project.root();
    let cwd = &project_root.join("crate1");

    cargo_info()
        .arg("my-package")
        .arg("--registry=dummy-registry")
        .current_dir(cwd)
        .assert()
        .success()
        .stdout_matches_path(curr_dir!().join("stdout.log"))
        .stderr_matches_path(curr_dir!().join("stderr.log"));

    assert_ui().subset_matches(curr_dir!().join("out"), &project_root);
}
