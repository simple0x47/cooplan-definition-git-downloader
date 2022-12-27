use git2::{Oid, Repository};

use crate::{
    clone::git_clone,
    error::{Error, ErrorKind},
    git_config::GitConfig,
    pull::git_pull,
};

const MAIN_BRANCH: &str = "main";

pub struct Downloader {
    repository_url: String,
    repository_local_dir: String,
    remote_name: String,
    remote_branch: String,
}

impl Downloader {
    pub fn new(git_config: GitConfig) -> Downloader {
        Downloader {
            repository_url: git_config.repository_url,
            repository_local_dir: git_config.repository_local_dir,
            remote_name: git_config.remote_name,
            remote_branch: git_config.remote_branch,
        }
    }

    fn clone_repository(&self) -> Result<Repository, Error> {
        match git_clone(
            self.repository_url.as_str(),
            self.repository_local_dir.as_str(),
            self.remote_branch.as_str(),
        ) {
            Ok(repository) => Ok(repository),
            Err(error) => Err(error),
        }
    }

    fn get_repository(&self) -> Result<Repository, Error> {
        match Repository::open(self.repository_local_dir.as_str()) {
            Ok(repository) => Ok(repository),
            Err(_) => self.clone_repository(),
        }
    }

    pub fn download(&self) -> Result<(), Error> {
        match self.get_repository() {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    pub fn update(&self) -> Result<(), Error> {
        match self.get_repository() {
            Ok(repository) => {
                match git_pull(
                    &repository,
                    self.remote_name.as_str(),
                    self.remote_branch.as_str(),
                ) {
                    Ok(_) => Ok(()),
                    Err(error) => Err(Error::new(
                        ErrorKind::FailedToUpdateDefinitions,
                        format!("failed to update definitions: {}", error).as_str(),
                    )),
                }
            }
            Err(error) => Err(error),
        }
    }

    pub fn set_version(&self, hash: String) -> Result<(), Error> {
        let repository = match self.get_repository() {
            Ok(repository) => repository,
            Err(error) => return Err(error),
        };

        let version = match Oid::from_str(hash.as_str()) {
            Ok(version) => version,
            Err(error) => {
                return Err(Error::new(
                    ErrorKind::VersionSetFailure,
                    format!("failed to set version: {}", error).as_str(),
                ))
            }
        };

        match repository.set_head_detached(version) {
            Ok(_) => (),
            Err(error) => {
                return Err(Error::new(
                    ErrorKind::VersionSetFailure,
                    format!("failed to set version: {}", error).as_str(),
                ))
            }
        }

        match repository.checkout_head(None) {
            Ok(_) => (),
            Err(error) => {
                return Err(Error::new(
                    ErrorKind::VersionSetFailure,
                    format!("failed to set version: {}", error).as_str(),
                ))
            }
        }

        Ok(())
    }

    pub fn set_version_to_latest(&self) -> Result<(), Error> {
        let repository = self.get_repository()?;

        let (_, reference) = match repository.revparse_ext(MAIN_BRANCH) {
            Ok(result) => match result.1 {
                Some(reference) => (result.0, reference),
                None => {
                    return Err(Error::new(
                        ErrorKind::VersionSetFailure,
                        "found no reference for main branch",
                    ))
                }
            },
            Err(error) => {
                return Err(Error::new(
                    ErrorKind::VersionSetFailure,
                    format!("failed to set version: {}", error),
                ))
            }
        };

        match reference.name() {
            Some(name) => match repository.set_head(name) {
                Ok(_) => (),
                Err(error) => {
                    return Err(Error::new(
                        ErrorKind::VersionSetFailure,
                        format!("failed to set head: {}", error),
                    ))
                }
            },
            None => {
                return Err(Error::new(
                    ErrorKind::VersionSetFailure,
                    "found no name for main branch reference",
                ))
            }
        }

        match repository.checkout_head(None) {
            Ok(_) => (),
            Err(error) => {
                return Err(Error::new(
                    ErrorKind::VersionSetFailure,
                    format!("failed to checkout head: {}", error),
                ))
            }
        }

        Ok(())
    }
}
