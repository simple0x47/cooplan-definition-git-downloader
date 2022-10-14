use git2::Repository;

use crate::error::{Error, ErrorKind};

pub struct VersionDetector {
    repository_local_dir: String,
}

impl VersionDetector {
    pub fn new(repository_local_dir: String) -> VersionDetector {
        VersionDetector {
            repository_local_dir,
        }
    }

    pub fn read_version(&self) -> Result<String, Error> {
        match Repository::open(self.repository_local_dir.as_str()) {
            Ok(repository) => {
                match repository.head() {
                    Ok(head) => match head.target() {
                        Some(oid) => Ok(oid.to_string()),
                        None => Err(Error::new(
                            ErrorKind::VersionReadFailure,
                            "failed to read version of the definition: head does not reference a commit",
                        )),
                    },
                    Err(error) => Err(Error::new(
                        ErrorKind::VersionReadFailure,
                        format!("failed to read version of the definition: {}", error).as_str(),
                    )),
                }
            }
            Err(error) => Err(Error::new(ErrorKind::FailedToOpenRepository, format!("failed to open repository: {}", error).as_str()))
        }
    }
}
