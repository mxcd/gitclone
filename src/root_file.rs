extern crate yaml_rust;
extern crate linked_hash_map;

#[path = "util.rs"] mod util;
use std::path::{PathBuf};
use std::env;
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};
use linked_hash_map::LinkedHashMap;
use std::io::{Write, BufReader};
use std::io::prelude::*;
use std::fs::File;
use log::{info, debug};

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

pub struct RootFile {
  file_path: PathBuf,
  git_provider_base_url: String,
  git_provider_basicauth_credentials: String,
  repositories: Vec<String>,
}

impl RootFile {
  pub fn new(path: PathBuf) -> RootFile {
    RootFile {
      file_path: path,
      git_provider_base_url: String::new(),
      git_provider_basicauth_credentials: String::new(),
      repositories: Vec::new(),
    }
  }

  pub fn get_file_path(&self) -> &PathBuf {
    &self.file_path
  }

  pub fn set_git_provider_base_url(&mut self, url: &String) {
    self.git_provider_base_url = url.to_string();
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

  // write .gitclone_root.yml file
  pub fn write(&mut self) {
    info!("Writing .gitclone_root.yml file");
    // create yaml object and store provider url and credentials
    let mut map = LinkedHashMap::new();
    map.insert(Yaml::from_str("provider_url"), Yaml::from_str(self.git_provider_base_url.as_str()));
    if self.git_provider_basicauth_credentials != "" {
      map.insert(Yaml::from_str("provider_basicauth_credentials"), Yaml::from_str(self.git_provider_basicauth_credentials.as_str()));
    }

    if self.repositories.len() > 0 {
      // create yaml object and store repositories
      let mut repositories_array = Vec::new();
      for repository in &self.repositories {
        repositories_array.push(Yaml::from_str(repository.as_str()));
      }
      map.insert(Yaml::from_str("repositories"), Yaml::Array(repositories_array));
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

    let provider_url_key:Yaml = Yaml::from_str("provider_url");
    let provider_basicauth_credentials_key:Yaml = Yaml::from_str("provider_basicauth_credentials");
    let repositories_key:Yaml = Yaml::from_str("repositories");

    if let Some(provider_url) = map.get(&provider_url_key) {
      self.git_provider_base_url = provider_url.as_str().unwrap().to_string();
    }

    if let Some(provider_basicauth_credentials) = map.get(&provider_basicauth_credentials_key) {
      self.git_provider_basicauth_credentials = provider_basicauth_credentials.as_str().unwrap().to_string();
    }

    if let Some(repositories) = map.get(&repositories_key) {
      let repositories_array = repositories.as_vec().unwrap();
      for repository in repositories_array {
        self.repositories.push(repository.as_str().unwrap().to_string());
      }
    }
  }
}