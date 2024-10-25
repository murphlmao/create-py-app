// libraries
use askama::Template;


// modules


#[derive(Template)]
#[template(path = "./scripts/cd/archive.sh", escape = "none")]
pub struct ArchiveScript {}

#[derive(Template)]
#[template(path = "./scripts/ci/build_application.sh", escape = "none")]
pub struct BuildAppScript {}

#[derive(Template)]
#[template(path = "./scripts/ci/run_lint.sh", escape = "none")]
pub struct RunLintScript {}

#[derive(Template)]
#[template(path = "./scripts/ci/run_unit_tests.sh", escape = "none")]
pub struct RunUnitTestsScript {}

#[derive(Template)]
#[template(path = "./scripts/dev/setup.sh", escape = "none")]
pub struct LinuxSetupScript {}

#[derive(Template)]
#[template(path = "./scripts/dev/setup.ps1", escape = "none")]
pub struct WindowsSetupScript {}