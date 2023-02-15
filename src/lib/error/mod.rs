use super::git;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    GitError {
        #[from]
        source: git::GitError,
    },
}
