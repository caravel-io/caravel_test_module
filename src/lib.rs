use caravel_macro::{caravel_resource};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug)]
enum FileState {
    Present,
    Absent,
}

#[derive(Serialize, Deserialize, Debug)]
enum FileType {
    File,
    Directory,
}



// #[caravel_resource_methods]
impl File {
    pub fn test_method(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("fancy export test {}", self.path.display());
        Ok(())
    }
}

impl File {
    fn apply(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.file_type {
            FileType::File => match &self.state {
                FileState::Absent => {
                    fs::remove_file(&self.path);
                }
                FileState::Present => {
                    // let fs_file = fs::File::create(&self.path).unwrap();
                    match &self.content {
                        Some(content) => {
                            // if file exists, check if content is different
                            match fs::exists(&self.path).unwrap() {
                                true => {
                                    let mut content_hasher = Sha256::new();
                                    content_hasher.update(content);
                                    let content_hash = content_hasher.finalize();

                                    let mut file_hasher = Sha256::new();
                                    let mut file = fs::File::open(&self.path).unwrap();
                                    std::io::copy(&mut file, &mut file_hasher).unwrap();
                                    let file_hash = file_hasher.finalize();

                                    if content_hash != file_hash {
                                        println!("Content is different, updating file \"{}\"", self.path.display());
                                        fs::write(&self.path, content);
                                    }
                                }
                                false => {
                                    fs::write(&self.path, content);
                                }
                            }
                        }
                        None => {
                            match fs::exists(&self.path).unwrap() {
                                true => {
                                    // do nothing
                                }
                                false => {
                                    let mut _file = fs::File::create(&self.path).unwrap();
                                }
                            }
                        }
                    }
                }
            },
            FileType::Directory => {
                fs::DirBuilder::new().create(&self.path);
            }
        }
        Ok(())
    }
}



#[caravel_resource]
#[derive(Serialize, Deserialize, Debug)]
struct File {
    path: PathBuf,
    state: FileState,
    file_type: FileType,
    uid: Option<u32>,
    gid: Option<u32>,
    content: Option<String>,
}

#[caravel_resource]
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String 
}

impl Person {
    fn apply(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Hello {}", self.name);
        Ok(())
    }
}