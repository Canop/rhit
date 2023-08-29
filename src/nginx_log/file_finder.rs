use {
    crate::*,
    std::{
        io,
        path::{Path, PathBuf},
    },
};

/// from a root path, which may be a file or directory,
/// find the list of files
fn find_files(
    path: PathBuf,
    files: &mut Vec<PathBuf>,
    check_name: bool,
    check_deeper_names: bool,
) -> Result<(), io::Error> {
    if path.is_dir() {
        for entry in path.read_dir()? {
            find_files(entry?.path(), files, check_deeper_names, check_deeper_names)?;
        }
    } else if !check_name || is_access_log_path(&path) {
        files.push(path);
    }
    Ok(())
}


pub struct FileFinder<'p> {
    roots: &'p [PathBuf],
    check_names: bool,
}

impl<'p> FileFinder<'p> {
    pub fn new(
        roots: &'p [PathBuf],
        check_names: bool,
    ) -> Self {
        Self {
            roots,
            check_names,
        }
    }
    /// return tuples (date, path), sorted, the date being
    /// the one of the first line in file
    pub fn dated_files(self) -> Result<Vec<(Date, PathBuf)>, RhitError> {
        let mut files = Vec::new();
        for root in self.roots {
            find_files(root.clone(), &mut files, false, self.check_names)?;
        }
        let mut dated_files = Vec::new();
        for path in files.drain(..) {
            if let Some(date) = get_file_first_date(&path)? {
                dated_files.push((date, path));
            } else {
                debug!("no date found in {:?}", path);
            }
        }
        dated_files.sort_unstable_by_key(|t| t.0);
        Ok(dated_files)
    }
}

pub fn is_access_log_path(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map_or(false, |name| {
            name.contains("access.log")
        })
}
