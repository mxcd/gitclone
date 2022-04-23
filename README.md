# `> gitclone`
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

## Usage
Firstly, the root directory of the git provider needs to be initialized.  
The base URL of the git provider will be prompted.
```
$> gitclone init
   git provider base url: 
   github.com
   gitclone provider root registered
$> 
```

```
gitclone <relative path>
```

### Examples