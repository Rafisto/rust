use std::fmt;
use std::fs;
use std::io;

pub struct FileListing {
    name: String,
    filetype: String,
    size: u64,
}

impl fmt::Display for FileListing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.filetype == "dir" {
            return write!(f, "/{}", self.name);
        } else {
            return write!(f, "{} {}B", self.name, self.size);
        }
    }
}

fn file_type_string(file_type: &fs::FileType) -> &str {
    if file_type.is_dir() {
        return "dir";
    } else {
        return "file";
    }
}

pub fn list_storage(path: &str) -> Result<Vec<FileListing>, io::Error> {
    let mut files = vec![];
    let mut dirs = vec![];
    if let Ok(listing) = fs::read_dir(path) {
        for entry in listing {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    let file = FileListing {
                        name: entry.file_name().to_string_lossy().to_string(),
                        filetype: file_type_string(&metadata.file_type()).to_string(),
                        size: metadata.len(),
                    };
                    if metadata.file_type().is_dir() {
                        dirs.push(file)
                    } else {
                        files.push(file)
                    }
                }
            }
        }
    }
    dirs.append(&mut files);
    return Ok(dirs);
}
