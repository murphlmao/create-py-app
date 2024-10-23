// libraries
use std::{fs::{self, File}, io::Write};
use askama::Template;
extern crate regex;


// modules


trait CPATemplate {
    fn write(&self, prefix: &str, path: &str);
}

// Implement convenience trait for any type that implements `askama::Template`
impl<T: Template> CPATemplate for T {
    fn write(&self, prefix: &str, path: &str) {
        let mut content = self.render().expect("Failed to render file.");
        content.push('\n');
        let mut f = File::create(format!("{}/{}", prefix, path)).expect("Could not create file");
        f.write_all(content.as_bytes()).expect("Could not write to file");
    }
}

////////////////////////////////////
// COMMON
////////////////////////////////////
#[derive(Template)]
#[template(path = ".gitignore", escape = "none")]
pub struct GitIgnore {}


////////////////////////////////////
// PYTHON
////////////////////////////////////
#[derive(Template)]
#[template(path = "python/pyproject.toml", escape = "none")]
pub struct PyProject {
    pub name: String,
    pub python_ver: String,
}

pub fn common(name: &str) -> String {
    let prefix: String = format!("./{}", name);

    // Create needed dirs
    let _ = fs::create_dir_all(format!("{}/.ci", prefix));
    let _ = fs::create_dir_all(format!("{}/.vscode", prefix));
    //let _ = fs::create_dir_all(format!("{}/.github/workflows", prefix));

    // Render common files
    //GhCI {}.write(&prefix, ".github/workflows/ci.yaml");
    GitIgnore {}.write(&prefix, ".gitignore");
    prefix
}

pub fn python(name: &str, prefix: &str, python_version: &str) {
    let pyproj: PyProject = PyProject {
        name: name.to_string(),
        python_ver: python_version.to_string(),
    };
    pyproj.write(prefix, "pyproject.toml");
}