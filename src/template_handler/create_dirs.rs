// libraries

// modules


pub struct DirectoryManager {
    pub scripts: Vec<String>,
    pub docs: Vec<String>,
    pub project: Vec<String>,
    pub docker: Vec<String>,
    pub git: Vec<String>,
}

impl DirectoryManager {
    pub fn new(base_path: &str, vcs_platform: &str) -> Self {
        fn prefix_paths(base: &str, paths: Vec<&str>) -> Vec<String> {
            paths.into_iter()
                .map(|p| format!("{}/{}", base.trim_end_matches('/'), p.trim_start_matches("./")))
                .collect()
        }

        let git_dirs = if vcs_platform.to_lowercase() == "gitlab" {
            prefix_paths(base_path, vec![
                "./.gitlab",
                "./.gitlab/issue_templates/",
                "./.gitlab/merge_request_templates/",
            ])
        } else if vcs_platform.to_lowercase() == "github" {
            prefix_paths(base_path, vec![
                "./.github",
                "./.github/ISSUE_TEMPLATE/",
                "./.github/PULL_REQUEST_TEMPLATE/",
                "./.github/workflows/",
            ])
        } else {
            eprintln!("Unsupported VCS platform '{}'. Defaulting to GitHub structure.", vcs_platform);
            prefix_paths(base_path, vec![
                "./.gitlab",
                "./.gitlab/issue_templates/",
                "./.gitlab/merge_request_templates/",
            ])
        };

        DirectoryManager {
            scripts: prefix_paths(base_path, vec![
                "./scripts/",
                "./scripts/cd",
                "./scripts/ci",
                "./scripts/dev",
            ]),
            docs: prefix_paths(base_path, vec![
                "./docs",
                "./docs/Usage",
            ]),
            project: prefix_paths(base_path, vec![
                "./src/project",
                "./src/project/config",
                "./src/project/exceptions",
                "./src/tests",
                "./src/tests/unit",
            ]),
            docker: prefix_paths(base_path, vec![
                "./docker/dev",
                "./docker/prod",
            ]),
            git: git_dirs,
        }
    }

    fn directories_to_create(&self) -> Vec<String> {
        let mut all_dirs = Vec::new();

        all_dirs.extend(self.scripts.clone());
        all_dirs.extend(self.docs.clone());
        all_dirs.extend(self.project.clone());
        all_dirs.extend(self.docker.clone());
        all_dirs.extend(self.git.clone());

        return all_dirs
    }

    pub fn create(&self) -> () {
        let dirs = self.directories_to_create();
        for dir in dirs {
            std::fs::create_dir_all(dir).unwrap();
        }
    }
}