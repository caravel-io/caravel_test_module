use caravel_macro::caravel_resource;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

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

impl File {
    fn apply(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.file_type {
            FileType::File => match &self.state {
                FileState::Absent => {
                    fs::remove_file(&self.path);
                }
                FileState::Present => {
                    let fs_file = fs::File::create(&self.path).unwrap();
                    match &self.content {
                        Some(content) => {
                            fs::write(&self.path, content);
                        }
                        None => {}
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
