// libraries

use askama::Template;


// modules

#[derive(Template)]
#[template(path = "./retype.yml", escape = "none")]
pub struct RetypeConfig {}