use crate::io::ReadBuf;
use std::io;
use std::net::SocketAddr;
use std::task::{Context, Poll};
use tokio::net::UdpSocket as TokioUdp;

/// Asynchronous UdpSockets.
pub struct UdpSocket(TokioUdp);

/// A connected asynchronous UdpSocket.
pub struct ConnectedUdpSocket(TokioUdp);

impl UdpSocket {
    /// Creates a new UDP socket and attempts to bind it to the address provided,
    ///
    /// # Examples
    /// ```
    /// use std::io;
    /// use ylong_runtime::net::UdpSocket;
    /// async fn io_func() -> io::Result<()> {
    ///     let addr = "127.0.0.1:8080".parse().unwrap();
    ///     let mut sock = UdpSocket::bind(addr).await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn bind(addr: SocketAddr) -> io::Result<Self> {
        Ok(UdpSocket(TokioUdp::bind(addr).await?))
    }

    /// Sets the default address for the UdpSocket and limits packets to those are
    /// read via recv from the specific address.
    ///
    /// Returns the connected UdpSocket if succeeds.
    pub async fn connect(self, addr: SocketAddr) -> io::Result<ConnectedUdpSocket> {
        self.0.connect(addr).await?;
        Ok(ConnectedUdpSocket(self.0))
    }

    /// Returns the local address that this socket is bound to.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// use ylong_runtime::net::UdpSocket;
    /// async fn io_func() -> io::Result<()> {
    ///     let addr = "127.0.0.1:8080".parse().unwrap();
    ///     let mut sock = UdpSocket::bind(addr).await?;
    ///     let local_addr = sock.local_addr()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.0.local_addr()
    }

    /// Sends the buffer to the given address. On success, returns the number of bytes written.
    /// This will return an error when the IP version of the local socket does not
    /// match the one returned from SocketAddr.
    ///
    /// # Exampels
    /// ```
    /// use std::io;
    /// use ylong_runtime::net::UdpSocket;
    /// async fn io_func() -> io::Result<()> {
    ///     let local_addr = "127.0.0.1:8080".parse().unwrap();
    ///     let sock = UdpSocket::bind(local_addr).await?;
    ///     let remote_addr = "127.0.0.1:8081".parse().unwrap();
    ///     let len = sock.send_to(b"hello world", remote_addr).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn send_to(&self, buf: &[u8], target: SocketAddr) -> io::Result<usize> {
        self.0.send_to(buf, target).await
    }

    /// Attempts to send the buffer to a given address.
    pub fn poll_send_to(
        &self,
        cx: &mut Context<'_>,
        buf: &[u8],
        target: SocketAddr,
    ) -> Poll<io::Result<usize>> {
        self.0.poll_send_to(cx, buf, target)
    }

    /// Receives a single datagram message on the socket. On success, returns the number of
    /// bytes read and the origin. The function must be called with valid type array buf of
    /// sufficient size of hold the message bytes. If a message is too long to fit in the
    /// supplied buffer, excess bytes may be discarded.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// use ylong_runtime::net::UdpSocket;
    /// async fn io_func() -> io::Result<()> {
    ///     let local_addr = "127.0.0.1:8080".parse().unwrap();
    ///     let sock = UdpSocket::bind(local_addr).await?;
    ///     let mut recv_buf = [0u8; 12];
    ///     let (len, addr) = sock.recv_from(&mut recv_buf).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.0.recv_from(buf).await
    }

    /// Attempts to receive a single datagram on the socket.
    pub fn poll_recv_from(
        &self,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<SocketAddr>> {
        self.0.poll_recv_from(cx, buf)
    }

    /// Sets the value of the `SO_BROADCAST` option for this socket.
    /// When enabled, this socket is allowed to send packets to a broadcast address.
    ///
    /// # Examples
    /// ```
    /// use std::io;
    /// use ylong_runtime::net::UdpSocket;
    /// async fn io_func() -> io::Result<()> {
    ///     let local_addr = "127.0.0.1:8080".parse().unwrap();
    ///     let sock = UdpSocket::bind(local_addr).await?;
    ///     if sock.broadcast()? == false {
    ///         sock.set_broadcast(true)?;
    ///     }
    ///     assert_eq!(sock.broadcast()?, true);
    ///     Ok(())
    /// }
    /// ```
    pub fn set_broadcast(&self, on: bool) -> io::Result<()> {
        self.0.set_broadcast(on)
    }

    /// Gets the value of the `SO_BROADCAST` option for this socket.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// use ylong_runtime::net::UdpSocket;
    /// async fn io_func() -> io::Result<()> {
    ///     let local_addr = "127.0.0.1:8080".parse().unwrap();
    ///     let sock = UdpSocket::bind(local_addr).await?;
    ///     assert_eq!(sock.broadcast()?, false);
    ///     Ok(())
    /// }
    /// ```
    pub fn broadcast(&self) -> io::Result<bool> {
        self.0.broadcast()
    }
}

impl ConnectedUdpSocket {
    /// Returns the local address that this socket is bound to.
    ///
    /// # Examples
    /// ```
    /// use std::io;
    /// use ylong_runtime::net::UdpSocket;
    /// async fn io_func() -> io::Result<()> {
    ///     let local_addr = "127.0.0.1:8080".parse().unwrap();
    ///     let sock = UdpSocket::bind(local_addr).await?;
    ///     let remote_addr = "127.0.0.1:8081".parse().unwrap();
    ///     let connected_sock = match sock.connect(remote_addr).await {
    ///         Ok(socket) => socket,
    ///         Err(e) => return Err(e),
    ///     };
    ///     let local_addr = connected_sock.local_addr()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.0.local_addr()
    }

    /// Returns the socket address of the remote peer this socket was connected to.
    ///
    /// # Examples
    /// ```
    /// use std::io;
    /// use ylong_runtime::net::UdpSocket;
    /// async fn io_func() -> io::Result<()> {
    ///     let local_addr = "127.0.0.1:8080".parse().unwrap();
    ///     let sock = UdpSocket::bind(local_addr).await?;
    ///     let remote_addr = "127.0.0.1:8081".parse().unwrap();
    ///     let connected_sock = match sock.connect(remote_addr).await {
    ///         Ok(socket) => socket,
    ///         Err(e) => return Err(e),
    ///     };
    ///     assert_eq!(connected_sock.peer_addr()?, remote_addr);
    ///     Ok(())
    /// }
    /// ```
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.0.peer_addr()
    }

    /// Sends data on the socket to the remote address that the socket is connected to.
    /// The connect method will connect this socket to a remote address.
    /// This method will fail if the socket is not connected.
    ///
    /// # Return value
    /// On success, the number of bytes sent is returned, otherwise, the encountered error
    /// is returned.
    ///
    /// # Examples
    /// ```
    /// use std::io;
    /// use ylong_runtime::net::UdpSocket;
    /// async fn io_func() -> io::Result<()> {
    ///     let local_addr = "127.0.0.1:8080".parse().unwrap();
    ///     let sock = UdpSocket::bind(local_addr).await?;
    ///     let remote_addr = "127.0.0.1:8081".parse().unwrap();
    ///     let connected_sock = match sock.connect(remote_addr).await {
    ///         Ok(socket) => socket,
    ///         Err(e) => return Err(e),
    ///     };
    ///     connected_sock.send(b"hello").await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.0.send(buf).await
    }

    /// Attempts to send data on the socket to the remote address to which is was
    /// previously connected.
    pub fn poll_send(&self, cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        self.0.poll_send(cx, buf)
    }

    /// Receives a single datagram message on the socket from the remote address to which it is
    /// connected. On success, returns the number of bytes read.
    /// The function must be called with valid bytes array buf of sufficient size to hold
    /// the message bytes.
    /// If a message is too long to fit in the supplied buffer, excess bytes may be discarded.
    /// The connect method will connect this socket to a remote address.
    /// This method will fail if the socket is not connected.
    ///
    /// # Examples
    /// ```
    /// use std::io;
    /// use ylong_runtime::net::UdpSocket;
    /// async fn io_func() -> io::Result<()> {
    ///     let local_addr = "127.0.0.1:8080".parse().unwrap();
    ///     let sock = UdpSocket::bind(local_addr).await?;
    ///     let remote_addr = "127.0.0.1:8081".parse().unwrap();
    ///     let connected_sock = match sock.connect(remote_addr).await {
    ///         Ok(socket) => socket,
    ///         Err(e) => return Err(e),
    ///     };
    ///     let mut recv_buf = [0u8; 12];
    ///     let n = connected_sock.recv(&mut recv_buf[..]).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.recv(buf).await
    }

    /// Attempts to receive a single datagram message on the socket from the remote address to
    /// which it is connected.
    pub fn poll_recv(&self, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<io::Result<()>> {
        self.0.poll_recv(cx, buf)
    }
}
