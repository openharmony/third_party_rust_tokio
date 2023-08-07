use std::fs::{Metadata, Permissions};
use std::io;
use std::io::{Error, SeekFrom};
use std::path::Path;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::fs::File as TokioFile;
use tokio::io::ReadBuf;

use crate::io::{AsyncRead, AsyncSeek, AsyncWrite};

/// An asynchronous wrapping of [`std::fs::File`]. Provides async read/write methods.
pub struct File(pub(crate) TokioFile);

impl File {
    pub fn new(file: std::fs::File) -> File {
        File(TokioFile::from_std(file))
    }

    /// Attempts to open a file in read-only mode asynchronously.
    ///
    /// # Errors
    /// This function will return an error if `path` does not already exist
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_runtime::fs::async_file::File;
    ///  async fn open() -> std::io::Result<()> {
    ///     let mut f = File::open("foo.txt").await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn open<P: AsRef<Path>>(path: P) -> io::Result<File> {
        Ok(File(TokioFile::open(path).await?))
    }

    /// Opens a file in write-only mode asynchronously.
    ///
    /// This function will create a file if it does not exist
    /// and truncate it if it does.
    ///
    /// # Examples
    /// ```no_run
    /// use ylong_runtime::fs::async_file::File;
    ///  async fn create() -> std::io::Result<()> {
    ///     let mut f = File::create("foo.txt").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn create<P: AsRef<Path>>(path: P) -> io::Result<File> {
        Ok(File(TokioFile::create(path).await?))
    }

    /// Changes the permissions on the underlying file asynchronously.
    ///
    /// # Errors
    /// This function will return an error if the user lacks permission change
    /// attributes on the underlying file. It may also return an error in other
    /// os-specific unspecified cases.
    ///
    /// # Examples
    /// ```no_run
    /// use ylong_runtime::fs::File;
    ///
    /// async fn set_permissions() -> std::io::Result<()> {
    ///     let file = File::open("foo.txt").await?;
    ///     let mut perms = file.metadata().await?.permissions();
    ///     perms.set_readonly(true);
    ///     file.set_permissions(perms).await?;
    ///     Ok(())
    /// }
    ///
    /// ```
    ///
    /// Note that this method alters the permissions of the underlying file,
    /// even though it takes ``&self` rahter than `&mut self`.
    pub async fn set_permissions(&self, perm: Permissions) -> io::Result<()> {
        self.0.set_permissions(perm).await
    }

    /// Attempts to sync all OS-internal metadata to disk asynchronously.
    ///
    /// This function will attempt to ensure that all in-memory data reaches the
    /// filesystem before returning.
    ///
    /// This can be used to handle errors that would otherwise only be caught
    /// when the `File` is closed. Dropping a file will ignore errors in
    /// synchronizing this in-memory data.
    ///
    /// # Examples
    /// ```no_run
    ///
    /// use ylong_runtime::io::AsyncWriteExt;
    /// use ylong_runtime::fs::File;
    ///  async fn sync_all() -> std::io::Result<()> {
    ///     let mut f = File::create("foo.txt").await?;
    ///     f.write_all(b"Hello, world!").await?;
    ///     f.sync_all().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn sync_all(&self) -> io::Result<()> {
        self.0.sync_all().await
    }

    /// This function is similar to [`File::sync_all`], except that it might not
    /// synchronize file metadata to the filesystem.
    ///
    /// This is intended for use cases that must synchronize content, but don't
    /// need the metadata on disk. The goal of this method is to reduce disk
    /// operations.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_runtime::io::AsyncWriteExt;
    /// use ylong_runtime::fs::File;
    ///  async fn sync_data() -> std::io::Result<()> {
    ///     let mut f = File::create("foo.txt").await?;
    ///     f.write_all(b"Hello, word!").await?;
    ///     f.sync_data().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn sync_data(&self) -> io::Result<()> {
        self.0.sync_data().await
    }

    /// Queries metadata about hte underlying file asynchronously.
    ///
    /// # Exmaples
    /// ```no_run
    /// use ylong_runtime::fs::File;
    ///  async fn metadata() -> std::io::Result<()> {
    ///     let mut f = File::open("foo.txt").await?;
    ///     let metadata = f.metadata().await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn metadata(&self) -> io::Result<Metadata> {
        self.0.metadata().await
    }

    /// Truncates or extends the underlying file, updating the size of this file to become size.
    ///
    /// If the size is less than the current file's size, then the file will be
    /// shrunk. If it is greater than the current file's size, then the file
    /// will be extended to size and have all of the intermediate data filled in
    /// with 0s.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_runtime::fs::File;
    /// use ylong_runtime::io::AsyncWriteExt;
    ///
    /// async fn set_length() -> std::io::Result<()> {
    ///    let mut file = File::create("foo.txt").await?;
    ///    file.write_all(b"Hello World!").await?;
    ///    file.set_len(10).await?;
    ///    Ok(())
    /// }
    /// ```
    pub async fn set_len(&self, size: u64) -> io::Result<()> {
        self.0.set_len(size).await
    }
}

impl AsyncSeek for File {
    fn start_seek(self: Pin<&mut Self>, position: SeekFrom) -> io::Result<()> {
        Pin::new(&mut self.get_mut().0).start_seek(position)
    }

    fn poll_complete(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<u64>> {
        Pin::new(&mut self.get_mut().0).poll_complete(cx)
    }
}

impl AsyncRead for File {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().0).poll_read(cx, buf)
    }
}

impl AsyncWrite for File {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        Pin::new(&mut self.get_mut().0).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        Pin::new(&mut self.get_mut().0).poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        Pin::new(&mut self.get_mut().0).poll_shutdown(cx)
    }
}
