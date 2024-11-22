// libraries
use std::{fs::File, io::Write};
use askama::Template;


// modules
use crate::template_handler::templater::*;

trait WritableTemplate {
    fn write(&self, prefix: &str, path: &str);
}

// convenience trait for any type that implements `askama::Template`
impl<T: Template> WritableTemplate for T {
    fn write(&self, prefix: &str, path: &str) {
        let mut content = self.render().expect("Failed to render file.");
        content.push('\n');

        let mut f = File::create(format!("{}/{}", prefix, path)).expect("Could not create file");
        f.write_all(content.as_bytes()).expect("Could not write to file");
    }
}

fn render_git(path_prefix: &str, vcs_platform: &str) {
    git::GitIgnore{}.write(path_prefix, ".gitignore");
    git::GitAttributes{}.write(path_prefix, ".gitattributes");

    if vcs_platform == "github" {
        git::GitHubIssueTemplateCustom{}.write(path_prefix, ".github/ISSUE_TEMPLATE/custom.md");
        git::GitHubIssueTemplateBugReport{}.write(path_prefix, ".github/ISSUE_TEMPLATE/bug_report.md");
        git::GitHubIssueTemplateFeatureRequest{}.write(path_prefix, ".github/ISSUE_TEMPLATE/feature_request.md");
        git::GitHubPRTemplateDefault{}.write(path_prefix, ".github/PULL_REQUEST_TEMPLATE/default.md");
        git::GitHubWorkflowsRelease{}.write(path_prefix, ".github/workflows/release.yml");
    }
    else {
        git::GitLabCiYML{}.write(path_prefix, ".gitlab-ci.yml");
        git::GitLabIssueTemplateCustom{}.write(path_prefix, ".gitlab/issue_templates/custom.md");
        git::GitLabIssueTemplateBugReport{}.write(path_prefix, ".gitlab/issue_templates/bug_report.md");
        git::GitLabIssueTemplateFeatureRequest{}.write(path_prefix, ".gitlab/issue_templates/feature_request.md");
        git::GitLabPRTemplateDefault{}.write(path_prefix, ".gitlab/merge_request_templates/default.md");
    }
}

fn render_docker(path_prefix: &str) {
    docker::DockerComposeDev{}.write(path_prefix, "./docker/dev/docker-compose-dev.yml");
    docker::DockerFileDev{}.write(path_prefix, "./docker/dev/Dockerfile.dev");
    docker::DockerComposeProd{}.write(path_prefix, "./docker/prod/docker-compose-prod.yml");
    docker::DockerFileProd{}.write(path_prefix, "./docker/prod/Dockerfile.prod");
}

fn render_docs(name: &str, python_version: &str, path_prefix: &str) {
    docs::CdREADME{}.write(path_prefix, "./scripts/cd/README.md");
    docs::CiREADME{}.write(path_prefix, "./scripts/ci/README.md");
    docs::DeploymentMD{}.write(path_prefix, "./docs/Deployment.md");
    docs::DevelopmentMD{}.write(path_prefix, "./docs/Development.md");
    docs::UsageREADME{}.write(path_prefix, "./docs/Usage/README.md");
    docs::BaseREADME{
        name: name.to_string(),
        python_version: python_version.to_string()
    }.write(path_prefix, "README.md");
}

fn render_scripts(path_prefix: &str) {
    scripts::ArchiveScript{}.write(path_prefix, "./scripts/cd/archive.sh");
    scripts::BuildAppScript{}.write(path_prefix, "./scripts/ci/build_application.sh");
    scripts::RunLintScript{}.write(path_prefix, "./scripts/ci/run_lint.sh");
    scripts::RunUnitTestsScript{}.write(path_prefix, "./scripts/ci/run_unit_tests.sh");
    scripts::LinuxSetupScript{}.write(path_prefix, "./scripts/dev/setup.sh");
    scripts::WindowsSetupScript{}.write(path_prefix, "./scripts/dev/setup.ps1");
}

fn render_python(name: &str, python_version: &str, path_prefix: &str) {
    // ./*
    python::PythonVersion{
        python_version: python_version.to_string()
    }.write(path_prefix, ".python-version");


    // ./src/*
    python::RequirementsTXT{}.write(path_prefix, "./src/requirements.txt");
    python::RequirementsDevTXT{}.write(path_prefix, "./src/requirements-dev.txt");

    python::PyProjectTOML{
        name: name.to_string(),
        python_version: python_version.to_string()
    }.write(path_prefix, "./src/pyproject.toml");

    python::SetupPy{
        name: name.to_string()
    }.write(path_prefix, "./src/setup.py");

    // ./src/tests/*
    python::TestConftestPy{}.write(path_prefix, "./src/tests/conftest.py");

    python::TestUnitLoggerPy{
        name: name.to_string()
    }.write(path_prefix, "./src/tests/unit/test_logger.py");


    // ./src/project/*
    python::BaseInitPy{}.write(path_prefix, "./src/project/__init__.py");
    python::FilesystemPy{}.write(path_prefix, "./src/project/filesystem.py");

    python::MainPy{
        name: name.to_string()
    }.write(path_prefix, "./src/project/main.py");

    python::ConstPy{
        name: name.to_string()
    }.write(path_prefix, "./src/project/const.py");

    python::EnumsPy{}.write(path_prefix, "./src/project/enums.py");


    python::MetadataPy{
        name: name.to_string(),
        python_version: python_version.to_string()
    }.write(path_prefix, "./src/project/metadata.py");


    // ./src/project/config/*
    python::ConfigurationPy{
        name: name.to_string(),
    }.write(path_prefix, "./src/project/config/configuration.py");

    python::ConfigInitPy{}.write(path_prefix, "./src/project/config/__init__.py");
    python::LoggerPy{
        name: name.to_string()
    }.write(path_prefix, "./src/project/logger.py");


    // ./src/project/exceptions/*
    python::ExceptionInitPy{}.write(path_prefix, "./src/project/exceptions/__init__.py");
    python::BaseExceptionsPy{}.write(path_prefix, "./src/project/exceptions/base_exceptions.py");

}

pub fn render_all(name: &str, python_version: &str, vcs_platform: &str) -> String {
    let prefix_path: String = format!("./{}", name);

    render_docker(&prefix_path);
    render_docs(name, python_version, &prefix_path);
    render_git(&prefix_path, vcs_platform);
    render_python(name, python_version, &prefix_path);
    render_scripts(&prefix_path);

    // rename ./src/project to ./src/{name}
    let src_path = format!("{}/src/project", prefix_path);
    let new_src_path = format!("{}/src/{}", prefix_path, name);
    std::fs::rename(src_path, new_src_path).expect("Failed to rename project directory");

    let message = format!("Created project with name: {}", name);
    return message;
}