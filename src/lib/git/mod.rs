use std::{
    fmt::Display,
    io,
    process::{Command, Stdio},
};

#[derive(thiserror::Error, Debug)]
pub enum GitError {
    BinNotFound,
    RepoNotFound,
    NoStagedFiles,
    GenericError { source: io::Error },
}

impl Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitError::BinNotFound => write!(f, "Git binary not found"),
            GitError::RepoNotFound => write!(f, "Not in a Git repository."),
            GitError::NoStagedFiles => write!(f, "No files staged."),
            GitError::GenericError { source: _ } => write!(f, "Sorry, something went wrong."),
        }
    }
}

impl From<io::Error> for GitError {
    fn from(io_error: io::Error) -> Self {
        match io_error.kind() {
            io::ErrorKind::NotFound => GitError::RepoNotFound,
            _ => GitError::GenericError { source: io_error },
        }
    }
}

pub fn check_ready() -> Result<(), GitError> {
    check_repo()?;
    check_staged_files()?;

    Ok(())
}

fn check_repo() -> Result<(), GitError> {
    let exit_code = Command::new("git")
        .args(["status", "--porcelain"])
        .output()?
        .status
        .code()
        .expect("Unexpected error: Git command terminated by signal.");

    match exit_code {
        0 => Ok(()),
        _ => Err(GitError::RepoNotFound),
    }
}

fn check_staged_files() -> Result<(), GitError> {
    let output = Command::new("git")
        .args(["diff", "--staged", "--quiet"])
        .output()?;

    let code = output
        .status
        .code()
        .expect("Unexpected error: Git command terminated by signal.");

    //println!("{:?}", code);

    match code {
        0 => Err(GitError::NoStagedFiles),
        _ => Ok(()),
    }
}

pub fn commit(message: String) {
    Command::new("git")
        .args(["commit", "-m", &message])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();
}

// fn repo(&self) -> &git2::Repository {
//     &self.repo
// }
