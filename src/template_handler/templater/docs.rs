// libraries
use askama::Template;


// modules


#[derive(Template)]
#[template(path = "./scripts/cd/README.md", escape = "none")]
pub struct CdREADME {}

#[derive(Template)]
#[template(path = "./scripts/ci/README.md", escape = "none")]
pub struct CiREADME {}

#[derive(Template)]
#[template(path = "./docs/Deployment.md", escape = "none")]
pub struct DeploymentMD {}

#[derive(Template)]
#[template(path = "./docs/Development.md", escape = "none")]
pub struct DevelopmentMD {}

#[derive(Template)]
#[template(path = "./docs/Usage/README.md", escape = "none")]
pub struct UsageREADME {}

#[derive(Template)]
#[template(path = "README.md", escape = "none")]
pub struct BaseREADME {
    pub name: String,
    pub python_version: String,
}