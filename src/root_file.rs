extern crate yaml_rust;
extern crate linked_hash_map;
use lazy_static::lazy_static;
use std::fmt;

#[path = "util.rs"] mod util;
use std::path::{PathBuf};
use std::env;
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};
use linked_hash_map::LinkedHashMap;
use std::io::{Write, BufReader};
use std::io::prelude::*;
use std::fs::File;
use log::{info, debug};
use regex::Regex;

const GITCLONE_ROOT_FILE_NAME: &str = ".gitclone_root.yml";


pub fn get_pwd_file_path() -> PathBuf {
  let current_dir = env::current_dir().unwrap();
  debug!("pwd is '{}'", current_dir.display());
  // determine .gitclone_root.yml file
  let gitclone_root_file_path = current_dir.as_path().join(GITCLONE_ROOT_FILE_NAME);
  debug!("gitclone_root_file_path is '{}'", gitclone_root_file_path.display());
  gitclone_root_file_path
}

pub fn exists_in_pwd() -> bool {
  let file_path = get_pwd_file_path();
  file_path.exists()
}

pub fn find_root_file_path() -> Result<PathBuf, bool> {
  if exists_in_pwd() {
    return Ok(get_pwd_file_path());
  }
  else {
    return Err(false);
  }
}

pub enum GitProviderConnectionMode {
  HTTP,
  HTTPS,
  SSH,
}

impl fmt::Display for GitProviderConnectionMode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      GitProviderConnectionMode::HTTP => write!(f, "HTTP"),
      GitProviderConnectionMode::HTTPS => write!(f, "HTTPS"),
      GitProviderConnectionMode::SSH => write!(f, "SSH"),
    }
  }
}

pub fn resolve_git_provider_connection_mode(s: String) -> GitProviderConnectionMode {
  match s.as_str() {
    "HTTP" => { GitProviderConnectionMode::HTTP },
    "HTTPS" => { GitProviderConnectionMode::HTTPS },
    "SSH" => { GitProviderConnectionMode::SSH },
    _ => { GitProviderConnectionMode::HTTP }
  }
}

pub struct RootFile {
  file_path: PathBuf,
  git_provider_base_url: String,
  git_provider_connection_mode: GitProviderConnectionMode,
  git_provider_basicauth_credentials: String,
  repositories: Vec<String>,

  git_provider_base_url_key: String,
  git_provider_connection_mode_key: String,
  git_provider_basicauth_credentials_key: String,
  repositories_key: String,
}

impl RootFile {
  pub fn new(path: PathBuf) -> RootFile {
    RootFile {
      file_path: path,
      git_provider_base_url: String::new(),
      git_provider_connection_mode: GitProviderConnectionMode::HTTP,
      git_provider_basicauth_credentials: String::new(),
      repositories: Vec::new(),

      git_provider_base_url_key: String::from("git_provider_base_url"),
      git_provider_connection_mode_key: String::from("provider_connection_mode"),
      git_provider_basicauth_credentials_key: String::from("git_provider_basicauth_credentials"),
      repositories_key: String::from("repositories"),
    }
  }

  pub fn get_file_path(&self) -> &PathBuf {
    &self.file_path
  }

  pub fn set_git_provider_base_url(&mut self, url: &String) {
    let mut provider_url = url.clone();
    lazy_static! {
      static ref SSH_CONNECTION_REGEX: Regex = Regex::new(r"^(ssh://)?.*?@").unwrap();
      static ref HTTP_CONNECTION_REGEX: Regex = Regex::new(r"^http://").unwrap();
      static ref HTTPS_CONNECTION_REGEX: Regex = Regex::new(r"^https://").unwrap();  
    }
    // remove tailing slash if present
    if provider_url.ends_with("/") {
      provider_url.pop();
    }
    if SSH_CONNECTION_REGEX.is_match(&provider_url) {
      self.git_provider_connection_mode = GitProviderConnectionMode::SSH;
      self.git_provider_base_url = provider_url.replace("ssh://", "");
    }
    else if HTTP_CONNECTION_REGEX.is_match(&provider_url) {
      self.git_provider_connection_mode = GitProviderConnectionMode::HTTP;
      self.git_provider_base_url = provider_url.replace("http://", "");
    }
    else if HTTPS_CONNECTION_REGEX.is_match(&provider_url) {
      self.git_provider_connection_mode = GitProviderConnectionMode::HTTPS;
      self.git_provider_base_url = provider_url.replace("https://", "");
    }
    else {
      self.git_provider_connection_mode = GitProviderConnectionMode::HTTPS;
      self.git_provider_base_url = provider_url.to_string();
    }
  }

  pub fn set_git_provider_basicauth_credentials(&mut self, credentials: &String) {
    self.git_provider_basicauth_credentials = credentials.to_string();
  }

  pub fn get_git_provider_base_url(&self) -> &String {
    &self.git_provider_base_url
  }

  pub fn get_git_provider_basicauth_credentials(&self) -> &String {
    &self.git_provider_basicauth_credentials
  }

  pub fn get_git_provider_connection_mode(&self) -> &GitProviderConnectionMode {
    &self.git_provider_connection_mode
  }

  // write .gitclone_root.yml file
  pub fn write(&mut self) {
    info!("Writing .gitclone_root.yml file");
    // create yaml object and store provider url and credentials
    let mut map = LinkedHashMap::new();
    map.insert(Yaml::from_str(&self.git_provider_base_url_key.to_string()), Yaml::from_str(self.git_provider_base_url.as_str()));
    if self.git_provider_basicauth_credentials != "" {
      map.insert(Yaml::from_str(&self.git_provider_basicauth_credentials_key.to_string()), Yaml::from_str(self.git_provider_basicauth_credentials.as_str()));
    }

    map.insert(Yaml::from_str(&self.git_provider_connection_mode_key.to_string()), Yaml::from_str(self.git_provider_connection_mode.to_string().as_str()));

    if self.repositories.len() > 0 {
      // create yaml object and store repositories
      let mut repositories_array = Vec::new();
      for repository in &self.repositories {
        repositories_array.push(Yaml::from_str(repository.as_str()));
      }
      map.insert(Yaml::from_str(&self.repositories_key.to_string()), Yaml::Array(repositories_array));
    }
  
    let doc = Yaml::Hash(map);
    let mut output_string = String::new();
    let mut emitter = YamlEmitter::new(&mut output_string);
    emitter.dump(&doc).unwrap();

    // write .gitclone_root.yml file
    let mut file = std::fs::File::create(self.get_file_path()).expect("Error creating .gitclone_root.yml file");
    file.write_all(output_string.as_bytes()).expect("Error writing to .gitclone_root.yml file");
    info!("File '{}' written", self.get_file_path().display());
  }

  pub fn read(&mut self) {
    info!("Reading .gitclone_root.yml file from '{}'", self.get_file_path().display());
    let file = File::open(self.get_file_path()).expect("Error opening .gitclone_root.yml file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Error reading .gitclone_root.yml file");

    debug!("\n{}\n", contents);

    let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();
    let doc = &docs[0];
    let map = doc.as_hash().unwrap();

    if let Some(provider_url) = map.get(&Yaml::from_str(&self.git_provider_base_url_key.to_string())) {
      self.git_provider_base_url = provider_url.as_str().unwrap().to_string();
    }

    if let Some(provider_basicauth_credentials) = map.get(&Yaml::from_str(&self.git_provider_basicauth_credentials_key.to_string())) {
      self.git_provider_basicauth_credentials = provider_basicauth_credentials.as_str().unwrap().to_string();
    }

    if let Some(provider_connection_mode) = map.get(&Yaml::from_str(&self.git_provider_connection_mode_key.to_string())) {
      self.git_provider_connection_mode = resolve_git_provider_connection_mode(provider_connection_mode.as_str().unwrap().to_string());
    }

    if let Some(repositories) = map.get(&Yaml::from_str(&self.repositories_key.to_string())) {
      let repositories_array = repositories.as_vec().unwrap();
      for repository in repositories_array {
        self.repositories.push(repository.as_str().unwrap().to_string());
      }
    }
  }
}