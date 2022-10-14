use std::path::Path;

use git2::{
    build::RepoBuilder, Cred, CredentialType, Error, ErrorClass, ErrorCode, FetchOptions,
    RemoteCallbacks, Repository,
};

const ENV_GIT_USERNAME: &str = "GIT_USERNAME";
const ENV_GIT_PASSWORD: &str = "GIT_PASSWORD";

pub fn git_credentials_callback(
    _: &str,
    _: Option<&str>,
    allowed_types: CredentialType,
) -> Result<Cred, Error> {
    if !allowed_types.contains(CredentialType::USER_PASS_PLAINTEXT) {
        return Err(Error::new(
            ErrorCode::Auth,
            ErrorClass::Repository,
            "unsupported authentication credential types requested from repository's host",
        ));
    }

    match std::env::var(ENV_GIT_USERNAME) {
        Ok(username) => match std::env::var(ENV_GIT_PASSWORD) {
            Ok(password) => Cred::userpass_plaintext(username.as_str(), password.as_str()),
            Err(error) => Err(Error::new(
                ErrorCode::Auth,
                ErrorClass::Os,
                format!("failed to read environment variable: {}", error),
            )),
        },
        Err(error) => Err(Error::new(
            ErrorCode::Auth,
            ErrorClass::Os,
            format!("failed to read environment variable: {}", error),
        )),
    }
}

pub fn git_clone(
    repository_url: &str,
    repository_local_dir: &str,
    branch: &str,
) -> Result<Repository, crate::error::Error> {
    let mut callbacks = RemoteCallbacks::new();

    callbacks.credentials(git_credentials_callback);

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch_options);
    builder.branch(branch);

    let repository_local_path: &Path = Path::new(repository_local_dir);
    match builder.clone(repository_url, repository_local_path) {
        Ok(repository) => Ok(repository),
        Err(error) => Err(crate::error::Error::new(
            crate::error::ErrorKind::FailedToCloneRepository,
            format!("failed to clone repository: {}", error).as_str(),
        )),
    }
}
