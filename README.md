```
__             _ __       __               
\ \     ____ _(_) /______/ /___  ____  ___ 
 \ \   / __ `/ / __/ ___/ / __ \/ __ \/ _ \
 / /  / /_/ / / /_/ /__/ / /_/ / / / /  __/
/_/   \__, /_/\__/\___/_/\____/_/ /_/\___/ 
     /____/                                
```
cli tool to clone git repositories in the folder structure implied by the repository url

## Concept
There are approximately two types of people:  
Those who clone every repository they work on in a directory called `git` and those who like to keep things organized neatly in a folder structure matching the actual url of the repository. For GitHub.com other GitHub Enterprise instances, this is fairly simple since there is only one layer of depth to the directories. All GitHub repositories are either under a personal account or under a organization account.  

-----------------

Folder structure **without** `gitclone`:
```
git
├── gitclone
├── crossbeam
├── angular
├── vue
├── vuex
└── api-template
```  
-----------------
Folder structure **with** `gitclone`:
```
git
└── github.com
    ├── mxcd
    │   ├── api-template
    │   └── gitclone
    ├── vue
    │   ├── vue
    │   └── vuex
    ├── crossbeam-rs
    │   └── crossbeam
    └── angular
        └── angular
  
```
-----------------
The benefits become even more apparent, when we have to work with multiple GitHub (Enterprise) or GitLab instances. Even more so because GitLab allows the use of nested groups.
## Installation
### Pre-built binaries
pre-built binaries are available for Linux, MacOS and Windows.
#### Linux
```
sudo wget https://github.com/mxcd/gitclone/releases/latest/download/gitclone-linux-amd64 -O /usr/local/bin/gitclone && sudo chmod +x /usr/local/bin/gitclone
```
#### MacOS 11 and newer (amd64)
```
sudo wget https://github.com/mxcd/gitclone/releases/latest/download/gitclone-macos-11-amd64 -O /usr/local/bin/gitclone && sudo chmod +x /usr/local/bin/gitclone
```
#### MacOS 11 and newer (apple silicon)
```
sudo wget https://github.com/mxcd/gitclone/releases/latest/download/gitclone-macos-11-apple -O /usr/local/bin/gitclone && sudo chmod +x /usr/local/bin/gitclone
```
#### MacOS 10.15 (amd64)
```
sudo wget https://github.com/mxcd/gitclone/releases/latest/download/gitclone-macos-10.15-amd64 -O /usr/local/bin/gitclone && sudo chmod +x /usr/local/bin/gitclone
```
#### Windows
Go to the latest release page and download your binary:  
https://github.com/mxcd/gitclone/releases/latest  
Place it in `C:\Windows\System32\gitclone.exe`

### Build from source
#### Prequisites

Make sure you have the latest version of rust installed:  
https://www.rust-lang.org/tools/install  
#### Clone & Build
```
git clone https://github.com/mxcd/gitclone
cd gitclone
cargo build --release
sudo cp target/release/gitclone /usr/local/bin/gitclone
sudo chmod u+x /usr/local/bin/gitclone
```


## Usage
### Initialization
Firstly, the root directory of the git provider needs to be initialized.  
Create a directory that will act as the root of your git provider.  
We encourage you to name the directory after the provider (e.g. `./github.com`)
The base URL of the git provider will be prompted.
```
$ mkdir github.com
$ cd github.com
$ gitclone init
__             _ __       __               
\ \     ____ _(_) /______/ /___  ____  ___ 
 \ \   / __ `/ / __/ ___/ / __ \/ __ \/ _ \
 / /  / /_/ / / /_/ /__/ / /_/ / / / /  __/
/_/   \__, /_/\__/\___/_/\____/_/ /_/\___/ 
     /____/                                

=> Initializing gitclone root

please enter git provider base url including protocol
for https connections: e.g. https://github.com
for ssh connections: e.g. ssh://git@github.com

git provider base url:
https://github.com

input http(s) basic auth credentials. leave blank if not required.
WARNING: credentials will be stored in plain text in the root file. If you are uncomfortable with this please consider using ssh instead.

BasicAuth credentials: username:access-token-or-api-key

$ 
```
This will create a .gitclone_root.yml file in the directory marking it as root directory for the git provider.  

### Cloning
```
gitclone <relative path>
```
This will clone the repository with respect to the relative path you have to the root directory.

### Examples
#### Cloning from the root directory
```
$ pwd
/home/user/github.com
$ gitclone mxcd/api-template
```
This will create the `mxcd` directory and clone the repository `api-template` into it.
#### Cloning from a subdirectory
```
$ pwd
/home/user/github.com/mxcd
$ gitclone api-template
```
This will clone the repository `api-template` from `mxcd/api-template` into the current directory since you already are in the `mxcd` directory.

#### Cloning from within nested subdirectories
```
$ pwd
/home/user/gitlab.com/some/nested/group
$ gitclone subgroup/my-repo
```
This will clone the repository `my-repo` from `gitlab.com/some/nested/group/subgroup/my-repo` into the `./subgroup/my-repo` directory.

## License
Licensed under MIT License  
Copyright (c) 2022 Max Partenfelder
