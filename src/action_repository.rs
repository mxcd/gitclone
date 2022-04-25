use std::path::PathBuf;
use super::root_file;
use log::{debug, info};
use std::process::Command;
use std::process;
use crate::util;

pub fn clone(relative_path_string: String) {

  util::print_logo();

  info!("executing clone action with relative repository path '{}'", relative_path_string);

  let root_file_option = root_file::find_root_file_path();
  if root_file_option.is_none() {
    println!("No root file found. Use 'gitclone init' to initialize a root directory.");
    process::exit(1);
  }

  let mut root_file = root_file::RootFile::new(root_file_option.unwrap());
  root_file.read();

  let git_provider_base_url = root_file.get_git_provider_base_url().clone();
  let git_provider_connection_mode = root_file.get_git_provider_connection_mode().clone();
  let git_provider_basicauth_credentials = root_file.get_git_provider_basicauth_credentials().clone();

  debug!("git_provider_base_url is '{}'", git_provider_base_url);
  debug!("git_provider_basicauth_credentials is '{}'", git_provider_basicauth_credentials);
  debug!("git_provider_connection_mode is '{}'", git_provider_connection_mode);

  // create PathBuf from relative path string ignoring tailing slashes
  let relative_path = PathBuf::from(if relative_path_string.starts_with("/") {
    &relative_path_string[1..]
  } else {
    &relative_path_string
  });

  let path_diff = root_file.get_path_diff();
  debug!("path_diff is '{}'", path_diff.to_str().unwrap());
  let absolute_repository_path = path_diff.join(&relative_path);
  debug!("absolute_repository_path is '{}'", absolute_repository_path.to_str().unwrap());

  // for every path diff component, create the directory if it doesnt exist
  let relative_path_components = relative_path.components().collect::<Vec<_>>();
  let mut current_dir = std::env::current_dir().unwrap();

  for relative_path_component in relative_path_components {
    let create_dir = current_dir.join(relative_path_component);
    if !create_dir.exists() {
      debug!("creating directory '{}'", create_dir.to_str().unwrap());
      std::fs::create_dir_all(&create_dir).unwrap();
    }
    current_dir = create_dir;
  }

  let mut url = String::new();
  let mut safe_url = String::new();
  match git_provider_connection_mode {
    root_file::GitProviderConnectionMode::HTTP => {
      url.push_str("http://");
      safe_url.push_str("http://");
      if git_provider_basicauth_credentials != "" {
        url.push_str(&git_provider_basicauth_credentials);
        url.push_str("@");
      }
      url.push_str(&git_provider_base_url);
      safe_url.push_str(&git_provider_base_url);
      url.push_str("/");
      safe_url.push_str("/");
      url.push_str(&absolute_repository_path.to_str().unwrap());
      safe_url.push_str(&absolute_repository_path.to_str().unwrap());
    },
    root_file::GitProviderConnectionMode::HTTPS => {
      url.push_str("https://");
      safe_url.push_str("https://");
      if git_provider_basicauth_credentials != "" {
        url.push_str(&git_provider_basicauth_credentials);
        url.push_str("@");
      }
      url.push_str(&git_provider_base_url);
      safe_url.push_str(&git_provider_base_url);
      url.push_str("/");
      safe_url.push_str("/");
      url.push_str(&absolute_repository_path.to_str().unwrap());
      safe_url.push_str(&absolute_repository_path.to_str().unwrap());
    },
    root_file::GitProviderConnectionMode::SSH => {
      url.push_str("ssh://");
      url.push_str(&git_provider_base_url);
      url.push_str("/");
      url.push_str(&absolute_repository_path.to_str().unwrap());
      safe_url.push_str(&url);
    },
  }

  debug!("url is '{}'", url);
  println!("Cloning '{}'", safe_url);
  let output = Command::new("git").arg("clone").arg(&url).arg(relative_path.to_str().unwrap()).output().expect("Error: git clone failed");
  let status_code = output.status.code().unwrap();
  debug!("status: {}", status_code);
  debug!("stdout: {}", String::from_utf8_lossy(&output.stdout));
  debug!("stderr: {}", String::from_utf8_lossy(&output.stderr));
  let message = String::from_utf8_lossy(&output.stderr);

  if status_code == 0 {
    debug!("Cloning successful.");
    root_file.add_repository(absolute_repository_path.to_str().unwrap());
    root_file.write();
  }

  println!("{}", message);
  process::exit(status_code);
}