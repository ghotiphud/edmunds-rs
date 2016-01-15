use std::io::{self, Read, Write, Cursor};
use std::net::{SocketAddr, Shutdown};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::cell::Cell;

use hyper;
use hyper::net::{NetworkStream, NetworkConnector};

#[derive(Clone, Debug)]
pub struct MockStream {
    pub read: Cursor<Vec<u8>>,
    next_reads: Vec<Vec<u8>>,
    pub write: Vec<u8>,
    pub is_closed: bool,
    pub error_on_write: bool,
    pub error_on_read: bool,
    pub read_timeout: Cell<Option<Duration>>,
    pub write_timeout: Cell<Option<Duration>>,
}

impl PartialEq for MockStream {
    fn eq(&self, other: &MockStream) -> bool {
        self.read.get_ref() == other.read.get_ref() && self.write == other.write
    }
}

impl MockStream {
    pub fn new() -> MockStream {
        MockStream::with_input(b"")
    }

    pub fn with_input(input: &[u8]) -> MockStream {
        MockStream::with_responses(vec![input])
    }

    pub fn with_responses(mut responses: Vec<&[u8]>) -> MockStream {
        MockStream {
            read: Cursor::new(responses.remove(0).to_vec()),
            next_reads: responses.into_iter().map(|arr| arr.to_vec()).collect(),
            write: vec![],
            is_closed: false,
            error_on_write: false,
            error_on_read: false,
            read_timeout: Cell::new(None),
            write_timeout: Cell::new(None),
        }
    }
}

impl Read for MockStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.error_on_read {
            Err(io::Error::new(io::ErrorKind::Other, "mock error"))
        } else {
            match self.read.read(buf) {
                Ok(n) => {
                    if self.read.position() as usize == self.read.get_ref().len() {
                        if self.next_reads.len() > 0 {
                            self.read = Cursor::new(self.next_reads.remove(0));
                        }
                    }
                    Ok(n)
                },
                r => r
            }
        }
    }
}

impl Write for MockStream {
    fn write(&mut self, msg: &[u8]) -> io::Result<usize> {
        if self.error_on_write {
            Err(io::Error::new(io::ErrorKind::Other, "mock error"))
        } else {
            Write::write(&mut self.write, msg)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl NetworkStream for MockStream {
    fn peer_addr(&mut self) -> io::Result<SocketAddr> {
        Ok("127.0.0.1:1337".parse().unwrap())
    }

    fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.read_timeout.set(dur);
        Ok(())
    }

    fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.write_timeout.set(dur);
        Ok(())
    }

    fn close(&mut self, _how: Shutdown) -> io::Result<()> {
        self.is_closed = true;
        Ok(())
    }
}

/// A wrapper around a `MockStream` that allows one to clone it and keep an independent copy to the
/// same underlying stream.
#[derive(Clone)]
pub struct CloneableMockStream {
    pub inner: Arc<Mutex<MockStream>>,
}

impl Write for CloneableMockStream {
    fn write(&mut self, msg: &[u8]) -> io::Result<usize> {
        self.inner.lock().unwrap().write(msg)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.lock().unwrap().flush()
    }
}

impl Read for CloneableMockStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.lock().unwrap().read(buf)
    }
}

impl NetworkStream for CloneableMockStream {
    fn peer_addr(&mut self) -> io::Result<SocketAddr> {
        self.inner.lock().unwrap().peer_addr()
    }

    fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.inner.lock().unwrap().set_read_timeout(dur)
    }

    fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.inner.lock().unwrap().set_write_timeout(dur)
    }

    fn close(&mut self, how: Shutdown) -> io::Result<()> {
        NetworkStream::close(&mut *self.inner.lock().unwrap(), how)
    }
}

impl CloneableMockStream {
    pub fn with_stream(stream: MockStream) -> CloneableMockStream {
        CloneableMockStream {
            inner: Arc::new(Mutex::new(stream)),
        }
    }
}

pub struct MockConnector;

impl NetworkConnector for MockConnector {
    type Stream = MockStream;

    fn connect(&self, _host: &str, _port: u16, _scheme: &str) -> hyper::Result<MockStream> {
        Ok(MockStream::new())
    }
}

/// new connectors must be created if you wish to intercept requests.
macro_rules! mock_connector (
    ($name:ident {
        $($url:expr => $res:expr)*
    }) => (

        struct $name;

        impl hyper::net::NetworkConnector for $name {
            type Stream = $crate::hyper_mock::MockStream;
            fn connect(&self, host: &str, port: u16, scheme: &str)
                    -> hyper::Result<$crate::hyper_mock::MockStream> {
                use std::collections::HashMap;
                debug!("MockStream::connect({:?}, {:?}, {:?})", host, port, scheme);
                let mut map = HashMap::new();
                $(map.insert($url, $res);)*


                let key = format!("{}://{}", scheme, host);
                // ignore port for now
                match map.get(&*key) {
                    Some(&res) => Ok($crate::hyper_mock::MockStream::with_input(res.as_bytes())),
                    None => panic!("{:?} doesn't know url {}", stringify!($name), key)
                }
            }
        }

    );

    ($name:ident { $($response:expr),+ }) => (
        struct $name;

        impl hyper::net::NetworkConnector for $name {
            type Stream = $crate::hyper_mock::MockStream;
            fn connect(&self, _: &str, _: u16, _: &str)
                    -> $crate::Result<$crate::hyper_mock::MockStream> {
                Ok($crate::hyper_mock::MockStream::with_responses(vec![
                    $($response),+
                ]))
            }
        }
    );
);