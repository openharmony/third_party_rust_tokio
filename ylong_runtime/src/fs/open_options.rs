use crate::fs::async_file::File;
use std::io;
use std::path::Path;
use tokio::fs::OpenOptions as TokioOptions;

/// An asynchronous version of the [`std::fs::OpenOptions`].
///
/// Options and flags which can be used to configure how a file is opened.
#[derive(Clone, Debug)]
pub struct OpenOptions(TokioOptions);

impl OpenOptions {
    /// Creates a blank new set of options ready for configuration when opening a file.
    ///
    /// All options are initially set to `false` just like [`std::fs::OpenOptions::new`]
    pub fn new() -> OpenOptions {
        OpenOptions(TokioOptions::new())
    }

    /// Sets the option for file read access.
    ///
    /// This option, when true, will inidicate that the file should be
    /// `read`-able if opened
    pub fn read(&mut self, read: bool) -> &mut OpenOptions {
        self.0.read(read);
        self
    }

    /// Sets the option for file write access.
    ///
    /// This option, when true, will indicate that file should be
    /// `write`-able if opened.
    pub fn write(&mut self, write: bool) -> &mut OpenOptions {
        self.0.write(write);
        self
    }

    /// Sets the option for the file append mode.
    ///
    /// This option, when true, means that writes will append to a file instead of
    /// overwriting previous contents.
    pub fn append(&mut self, append: bool) -> &mut OpenOptions {
        self.0.append(append);
        self
    }

    /// Sets the option for truncating a file's previous content.
    ///
    /// If a file is successfully opened with this option set, it will truncate
    /// the file to 0 length if it already exists. Any already-existed content
    /// in this file will be dropped.
    pub fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.0.truncate(truncate);
        self
    }

    /// Sets the option to create a new file if it does not already exist, or simply
    /// open it if it does exist.
    ///
    /// In order for the file to be created, [`OpenOptions::write`] or
    /// [`OpenOptions::append`] access must be set to true.
    pub fn create(&mut self, create: bool) -> &mut Self {
        self.0.create(create);
        self
    }

    /// Sets the option to create a new file.
    ///
    /// If the file alreadys exists, opening the file with the option set will cause an error.
    pub fn create_new(&mut self, create_new: bool) -> &mut Self {
        self.0.create_new(create_new);
        self
    }

    /// Asynchronously opens a file at `path` with the options specified by `self`.
    ///
    /// # Errors
    /// Check std::file::OpenOptions
    pub async fn open<P: AsRef<Path>>(&self, path: P) -> io::Result<File> {
        Ok(File(self.0.open(path).await?))
    }
}
