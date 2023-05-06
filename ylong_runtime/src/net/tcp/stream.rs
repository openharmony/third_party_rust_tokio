use std::io::Error;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream as TokioStream;

use crate::io::{AsyncRead, AsyncWrite, ReadBuf};

pub struct TcpStream(pub(crate) TokioStream);

impl TcpStream {
    pub async fn connect(addr: SocketAddr) -> io::Result<Self> {
        Ok(TcpStream(TokioStream::connect(addr).await?))
    }

    pub async fn shutdown(&mut self) -> io::Result<()> {
        self.0.shutdown().await
    }
}

impl AsyncRead for TcpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().0).poll_read(cx, buf)
    }
}

impl AsyncWrite for TcpStream {
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
