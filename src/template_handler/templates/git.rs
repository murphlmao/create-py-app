// libraries
use askama::Template;


// modules


#[derive(Template)]
#[template(path = ".gitignore", escape = "none")]
pub struct GitIgnore {}

#[derive(Template)]
#[template(path = ".gitattributes", escape = "none")]
pub struct GitAttributes {}

#[derive(Template)]
#[template(path = ".gitattributes", escape = "none")]
pub struct GitAttributes {}

#[derive(Template)]
#[template(path = ".gitlab-ci.yml", escape = "none")]
pub struct GitLabCiYML {}