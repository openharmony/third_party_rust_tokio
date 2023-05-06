use crate::net::tcp::stream::TcpStream;
use std::io;
use std::net::SocketAddr;
use tokio::net::TcpListener as TokioListener;

/// An asynchronous version of [`std::net::TcpListener`]. Provides async bind/accept methods.
///
/// # Example
///
/// ```
/// use tokio::io;
/// use ylong_runtime::net::TcpListener;
///
/// async fn io_func() -> io::Result<()> {
///     let addr = "127.0.0.1:8080".parse().unwrap();
///     let server = TcpListener::bind(addr).await?;
///     let (stream, addr) = server.accept().await?;
///     Ok(())
/// }
/// ```
pub struct TcpListener(TokioListener);

impl TcpListener {
    /// A TCP socket server, asynchronously listening for connections.
    ///
    /// After creating a `TcpListener` by binding it to a socket address, it listens
    /// for incoming TCP connections asynchronously. These connections can be accepted
    /// by calling [`TcpListener::accept`]
    ///
    /// # Example
    /// ```
    /// use std::io;
    /// use ylong_runtime::net::TcpListener;
    ///  async fn io_func() -> io::Result<()> {
    ///     let addr = "127.0.0.1:8080".parse().unwrap();
    ///     let server = TcpListener::bind(addr).await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn bind(addr: SocketAddr) -> io::Result<TcpListener> {
        Ok(TcpListener(TokioListener::bind(addr).await?))
    }

    /// Asynchronously accepts a new incoming connection from this listener.
    ///
    /// When connection gets established, the corresponding [`TcpStream`] and the remote
    /// peer's address will be returned.
    ///
    /// # Example
    ///
    /// ```
    /// use tokio::io;
    /// use ylong_runtime::net::TcpListener;
    ///
    /// async fn io_func() -> io::Result<()> {
    ///     let addr = "127.0.0.1:8080".parse().unwrap();
    ///     let server = TcpListener::bind(addr).await?;
    ///     let (stream, addr) = server.accept().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        let (s, a) = self.0.accept().await?;
        Ok((TcpStream(s), a))
    }
}
