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
#[template(path = ".gitlab-ci.yml", escape = "none")]
pub struct GitLabCiYML {}

#[derive(Template)]
#[template(path = ".gitlab/issue_templates/custom.md", escape = "none")]
pub struct GitLabIssueTemplateCustom {}

#[derive(Template)]
#[template(path = ".gitlab/issue_templates/bug_report.md", escape = "none")]
pub struct GitLabIssueTemplateBugReport {}

#[derive(Template)]
#[template(path = ".gitlab/issue_templates/feature_request.md", escape = "none")]
pub struct GitLabIssueTemplateFeatureRequest {}

#[derive(Template)]
#[template(path = ".gitlab/merge_request_templates/default.md", escape = "none")]
pub struct GitLabPRTemplateDefault {}