use crate::helpers::check_dependency;
use crate::helpers::fetch;
use crate::helpers::makepkg;
use crate::helpers::AUR_URL;
use std::process::Command;

#[derive(Debug, Default, Clone)]
pub struct Package {
    name: String,
    version: String,
    description: String,
}

impl Package {
    pub fn new(name: String, description: String, version: String) -> Package {
        Package {
            name,
            description,
            version,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub async fn check_for_updates(&self) -> Result<(bool, String), Box<dyn std::error::Error>> {
        let url = format!("{}/packages/{}", AUR_URL, &self.name);
        let res = fetch(&url).await.unwrap();

        let re = regex::Regex::new(r"<h2>Package Details: [^<]+ (.+)</h2>").unwrap();

        if let Some(captures) = re.captures(&res) {
            if let Some(version) = captures.get(1) {
                Ok((
                    version.as_str() != &self.version,
                    version.as_str().to_owned(),
                ))
            } else {
                Err("No version found".into())
            }
        } else {
            Err("Couldn't get most recent version".into())
        }
    }

    pub fn check_if_package_in_cache(&self) -> bool {
        let cache_path: String =
            format!("{}/{}", home::home_dir().unwrap().display(), ".cache/aur");
        let package_path: String = format!("{}/{}", cache_path, &self.name);

        std::path::Path::new(package_path.as_str()).exists()
    }

    pub fn pull_cached_package(&self) -> Result<(), Box<dyn std::error::Error>> {
        let package_path: String = format!(
            "{}/{}/{}",
            home::home_dir().unwrap().display(),
            ".cache/aur",
            &self.name
        );

        check_dependency("git");

        // cd into package, pull changes
        let exit_status = Command::new("git")
            .arg("pull")
            .arg("origin")
            .arg("master")
            .current_dir(package_path)
            .output()
            .unwrap();

        if exit_status.status.code().unwrap() != 0 {
            Err(String::from_utf8_lossy(&exit_status.stderr).into())
        } else {
            match makepkg(&self.name) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
    }
}
