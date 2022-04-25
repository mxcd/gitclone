use std::{process};

use crate::root_file;
use crate::util;
use log::{debug};

pub fn do_init_action() {

  util::print_logo();
  println!("=> Initializing gitclone root");

  // abort init if .gitclone_root.yml already exists
  if root_file::exists_in_pwd() {
    println!(".gitclone_root.yml already exists in current directory.");
    process::exit(1);
  }
  
  // promt for git provider url
  println!();
  println!("please enter git provider base url including protocol");
  println!("for https connections: e.g. https://github.com");
  println!("for ssh connections: e.g. ssh://git@github.com");
  println!();
  println!("git provider base url:");
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
  let git_provider_base_url = line.trim().to_string();
  debug!("git_provider_base_url is '{}'", git_provider_base_url);
  
  let mut root_file = root_file::RootFile::new(root_file::get_pwd_file_path());
  root_file.set_git_provider_base_url(&git_provider_base_url);
  
  // in case of http or https, prompt for basicauth credentials
  match root_file.get_git_provider_connection_mode() {
    root_file::GitProviderConnectionMode::HTTP | root_file::GitProviderConnectionMode::HTTPS => {
      println!();
      println!("input http(s) basic auth credentials. leave blank if not required.");
      println!("WARNING: credentials will be stored in plain text in the root file. If you are uncomfortable with this please consider using ssh instead.");
      println!();
      let git_provider_basicauth_credentials = rpassword::prompt_password("BasicAuth credentials: ").unwrap();
      root_file.set_git_provider_basicauth_credentials(&git_provider_basicauth_credentials);
      debug!("git_provider_basicauth_credentials is '{}'", git_provider_basicauth_credentials);
    },
    _ => {
      debug!("no basicauth credentials required");
    }
  }

  root_file.write();
}