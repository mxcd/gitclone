#[path = "root_file.rs"] mod root_file;
use log::{debug, info};

pub fn clone(relative_path: String) {
  info!("executing clone action with relative repository path '{}'", relative_path);

  let mut root_file = root_file::RootFile::new(root_file::find_root_file_path().unwrap());
  root_file.read();

  let git_provider_base_url = root_file.get_git_provider_base_url();
  let git_provider_basicauth_credentials = root_file.get_git_provider_basicauth_credentials();
  debug!("git_provider_base_url is '{}'", git_provider_base_url);
  debug!("git_provider_basicauth_credentials is '{}'", git_provider_basicauth_credentials);
}