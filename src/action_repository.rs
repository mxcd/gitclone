#[path = "root_file.rs"] mod root_file;
use log::{debug, info};
use std::process::Command;
use std::process;

pub fn clone(relative_path: String) {

  info!("executing clone action with relative repository path '{}'", relative_path);

  let mut root_file = root_file::RootFile::new(root_file::find_root_file_path().unwrap());
  root_file.read();

  let git_provider_base_url = root_file.get_git_provider_base_url().clone();
  let git_provider_connection_mode = root_file.get_git_provider_connection_mode().clone();
  let git_provider_basicauth_credentials = root_file.get_git_provider_basicauth_credentials().clone();

  debug!("git_provider_base_url is '{}'", git_provider_base_url);
  debug!("git_provider_basicauth_credentials is '{}'", git_provider_basicauth_credentials);
  debug!("git_provider_connection_mode is '{}'", git_provider_connection_mode);

  let mut absolute_repository_path = relative_path.clone();
  if absolute_repository_path.starts_with("/") {
    absolute_repository_path.remove(0);
  }

  let mut url = String::new();
  match git_provider_connection_mode {
    root_file::GitProviderConnectionMode::HTTP => {
      url.push_str("http://");
      if git_provider_basicauth_credentials != "" {
        url.push_str(&git_provider_basicauth_credentials);
        url.push_str("@");
      }
      url.push_str(&git_provider_base_url);
      url.push_str("/");
      url.push_str(&absolute_repository_path);
    },
    root_file::GitProviderConnectionMode::HTTPS => {
      url.push_str("https://");
      if git_provider_basicauth_credentials != "" {
        url.push_str(&git_provider_basicauth_credentials);
        url.push_str("@");
      }
      url.push_str(&git_provider_base_url);
      url.push_str("/");
      url.push_str(&absolute_repository_path);
    },
    root_file::GitProviderConnectionMode::SSH => {
      url.push_str("ssh://");
      url.push_str(&git_provider_base_url);
      url.push_str("/");
      url.push_str(&absolute_repository_path);
    },
  }

  debug!("url is '{}'", url);
  println!("cloning '{}'", url);
  let output = Command::new("git").arg("clone").arg(&url).output().expect("Error: git clone failed");
  debug!("status: {}", output.status.code().unwrap());
  debug!("stdout: {}", String::from_utf8_lossy(&output.stdout));
  debug!("stderr: {}", String::from_utf8_lossy(&output.stderr));
  let message = String::from_utf8_lossy(&output.stderr);
  println!("{}", message);
  process::exit(output.status.code().unwrap());
}