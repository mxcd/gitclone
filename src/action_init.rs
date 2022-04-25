use std::{process};

#[path = "root_file.rs"] mod root_file;
use log::{debug};
pub fn do_init_action() {

  // abort init if .gitclone_root.yml already exists
  if root_file::exists_in_pwd() {
    println!(".gitclone_root.yml already exists in current directory.");
    process::exit(1);
  }
  
  // promt for git provider url
  println!("git provider base url: ");
  let mut git_provider_basicauth_credentials = String::new();
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
  let git_provider_base_url = line.trim().to_string();
  debug!("git_provider_base_url is '{}'", git_provider_base_url);
  
  // in case of http or https, prompt for basicauth credentials
  if git_provider_base_url.starts_with("http://") || git_provider_base_url.starts_with("https://") {
    println!("input http(s) basic auth credentials. leave blank if not required.");
    git_provider_basicauth_credentials = rpassword::prompt_password("BasicAuth credentials: ").unwrap();
    debug!("git_provider_basicauth_credentials is '{}'", git_provider_basicauth_credentials);
  }

  let mut root_file = root_file::RootFile::new(root_file::get_pwd_file_path());
  root_file.set_git_provider_base_url(&git_provider_base_url);
  root_file.set_git_provider_basicauth_credentials(&git_provider_basicauth_credentials);
  root_file.write();
}