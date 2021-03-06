use anyhow::Result;
use assert_cmd::prelude::*;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn render_config_example() -> Result<()> {
    let mut cmd = Command::cargo_bin("helm-templexer")?;

    cmd.current_dir("tests/data")
        .arg("render")
        .arg("config_example.toml");

    cmd.assert().success();

    // assert that all the deployment directories exist
    assert_eq!(PathBuf::from("tests/data/manifests").exists(), true);
    assert_eq!(
        PathBuf::from("tests/data/manifests/edge-eu-w4").exists(),
        true
    );
    assert_eq!(
        PathBuf::from("tests/data/manifests/stage-eu-w4").exists(),
        true
    );
    assert_eq!(
        PathBuf::from("tests/data/manifests/prod-eu-w4").exists(),
        true
    );
    assert_eq!(
        PathBuf::from("tests/data/manifests/next-edge-eu-w4").exists(),
        false
    );

    // asert that the release name override for prod-eu-e4 worked
    assert_eq!(
        PathBuf::from("tests/data/manifests/prod-eu-w4/my-app-prod-eu-w4").exists(),
        true
    );

    // dig deep into some of the rendered manifest files
    assert_eq!(
        PathBuf::from("tests/data/manifests/edge-eu-w4/my-app/nginx-chart/templates").exists(),
        true
    );

    let mut edge_deployment_yaml = std::fs::File::open(
        "tests/data/manifests/edge-eu-w4/my-app/nginx-chart/templates/deployment.yaml",
    )?;
    let mut contents = "".to_string();
    edge_deployment_yaml.read_to_string(&mut contents)?;
    assert_eq!(contents.contains("image: \"nginx:latest\""), true);

    // todo extend assertions here while changing the chart under test
    // todo this test could also benefit from some utility functions/macros to make it less verbose

    // clean up file wrote to disk
    std::fs::remove_dir_all("tests/data/manifests")?;

    Ok(())
}
