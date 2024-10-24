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
    pub fn new(base_path: &str) -> Self {
        fn prefix_paths(base: &str, paths: Vec<&str>) -> Vec<String> {
            paths.into_iter()
                .map(|p| format!("{}/{}", base.trim_end_matches('/'), p.trim_start_matches("./")))
                .collect()
        }

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
            ]),
            docker: prefix_paths(base_path, vec![
                "./docker/dev",
                "./docker/prod",
            ]),
            git: prefix_paths(base_path, vec![
                "./.gitlab",
                "./.gitlab/issue_templates/",
                "./.gitlab/merge_request_templates/",
            ]),
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