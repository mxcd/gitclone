extern crate yaml_rust;
extern crate linked_hash_map;

use std::io::Write;
use std::{env, process};
use yaml_rust::{Yaml, YamlEmitter};
use linked_hash_map::LinkedHashMap;

const GITCLONE_ROOT_FILE_NAME: &str = ".gitclone_root.yml";

pub fn do_init_action(verbose: bool) {
  let current_dir = env::current_dir().unwrap();
  if verbose { println!("pwd is '{}'", current_dir.display())}
  
  // determine .gitclone_root.yml file
  let gitclone_root_file_path = current_dir.as_path().join(GITCLONE_ROOT_FILE_NAME);
  if verbose { println!("gitclone_root_file_path is '{}'", gitclone_root_file_path.display()); }
  
  // abort init if .gitclone_root.yml already exists
  if gitclone_root_file_path.exists() {
    println!("{} file already exists in current directory.", GITCLONE_ROOT_FILE_NAME);
    process::exit(1);
  }

  // promt for git provider url
  println!("git provider base url: ");
  let mut git_provider_basicauth_credentials = String::new();
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
  let git_provider_base_url = line.trim().to_string();
  if verbose { println!("git_provider_base_url is '{}'", git_provider_base_url); }
  
  // in case of http or https, prompt for basicauth credentials
  if git_provider_base_url.starts_with("http://") || git_provider_base_url.starts_with("https://") {
    println!("input http(s) basic auth credentials. leave blank if not required.");
    git_provider_basicauth_credentials = rpassword::prompt_password("BasicAuth credentials: ").unwrap();
    if verbose { println!("git_provider_basicauth_credentials is '{}'", git_provider_basicauth_credentials); }
  }

  // write .gitclone_root.yml file
  // create yaml object and store provider url and credentials
  let mut map = LinkedHashMap::new();
  map.insert(Yaml::from_str("provider_url"), Yaml::from_str(git_provider_base_url.as_str()));
  if git_provider_basicauth_credentials != "" {
    map.insert(Yaml::from_str("provider_basicauth_credentials"), Yaml::from_str(git_provider_basicauth_credentials.as_str()));
  }
  
  let doc = Yaml::Hash(map);
  let mut output_string = String::new();
  let mut emitter = YamlEmitter::new(&mut output_string);
  emitter.dump(&doc).unwrap();

  // write .gitclone_root.yml file
  let mut file = std::fs::File::create(gitclone_root_file_path).expect("Error creating .gitclone_root.yml file");
  file.write_all(output_string.as_bytes()).expect("Error writing to .gitclone_root.yml file");

}