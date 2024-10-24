// libraries
use askama::Template;


// modules


#[derive(Template)]
#[template(path = "./docker/dev/docker-compose-dev.yml", escape = "none")]
pub struct DockerComposeDev {}


#[derive(Template)]
#[template(path = "./docker/dev/Dockerfile.dev", escape = "none")]
pub struct DockerFileDev {}

#[derive(Template)]
#[template(path = "./docker/prod/docker-compose-prod.yml", escape = "none")]
pub struct DockerComposeProd {}

#[derive(Template)]
#[template(path = "./docker/prod/Dockerfile.prod", escape = "none")]
pub struct DockerFileProd {}