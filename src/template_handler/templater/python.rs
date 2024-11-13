// libraries
use askama::Template;


// modules


#[derive(Template)]
#[template(path = "./src/requirements.txt", escape = "none")]
pub struct RequirementsTXT {}

#[derive(Template)]
#[template(path = ".python-version", escape = "none")]
pub struct PythonVersion {
    pub python_version: String,
}

#[derive(Template)]
#[template(path = "./src/pyproject.toml", escape = "none")]
pub struct PyProjectTOML {
    pub name: String,
    pub python_version: String,
}

#[derive(Template)]
#[template(path = "./src/setup.py", escape = "none")]
pub struct SetupPy {
    pub name: String,
}

#[derive(Template)]
#[template(path = "./src/project/__init__.py", escape = "none")]
pub struct BaseInitPy {}

#[derive(Template)]
#[template(path = "./src/project/config/__init__.py", escape = "none")]
pub struct ConfigInitPy {}



#[derive(Template)]
#[template(path = "./src/project/main.py", escape = "none")]
pub struct MainPy {
    pub name: String,
}

#[derive(Template)]
#[template(path = "./src/project/const.py", escape = "none")]
pub struct ConstPy {
    pub name: String,
}

#[derive(Template)]
#[template(path = "./src/project/filesystem.py", escape = "none")]
pub struct FilesystemPy {}

#[derive(Template)]
#[template(path = "./src/project/metadata.py", escape = "none")]
pub struct MetadataPy {
    pub name: String,
    pub python_version: String,
}

#[derive(Template)]
#[template(path = "./src/project/config/configuration.py", escape = "none")]
pub struct ConfigurationPy {
    pub name: String,
}

#[derive(Template)]
#[template(path = "./src/project/logger.py", escape = "none")]
pub struct LoggerPy {
    pub name: String,
}

#[derive(Template)]
#[template(path = "./src/project/exceptions/__init__.py", escape = "none")]
pub struct ExceptionInitPy {}

#[derive(Template)]
#[template(path = "./src/project/exceptions/base_exceptions.py", escape = "none")]
pub struct BaseExceptionsPy {}

