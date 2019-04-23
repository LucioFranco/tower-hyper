//! Tower <-> hyper body utilities

use futures::Poll;
use hyper::body::Payload;
use tower_http::Body as HttpBody;

/// Specialized Body that takes a `hyper::Body` and implements `tower_http::Body`.
#[derive(Debug)]
pub struct Body {
    inner: hyper::Body,
}

/// Lifts a body to support `Payload`
#[derive(Debug)]
pub struct LiftBody<T> {
    inner: T,
}

impl<T: HttpBody> From<T> for LiftBody<T> {
    fn from(inner: T) -> Self {
        LiftBody { inner }
    }
}

impl<T> Payload for LiftBody<T>
where
    T: HttpBody + Send + 'static,
    T::Item: Send,
    T::Error: Into<crate::Error>,
{
    type Data = T::Item;
    type Error = T::Error;

    fn poll_data(&mut self) -> Poll<Option<Self::Data>, Self::Error> {
        self.inner.poll_buf()
    }

    fn poll_trailers(&mut self) -> Poll<Option<hyper::HeaderMap>, Self::Error> {
        self.inner.poll_trailers()
    }
    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }
}

// impl Body {
//     /// Lifts the inner `T`
//     pub fn new(inner: T) -> Self {
//         Body { inner }
//     }
// }

impl From<hyper::Body> for Body {
    fn from(inner: hyper::Body) -> Self {
        Body { inner }
    }
}

impl Body {
    /// Get the inner wrapped payload
    pub fn into_inner(self) -> hyper::Body {
        self.inner
    }
}

impl HttpBody for Body {
    type Item = hyper::Chunk;
    type Error = hyper::Error;

    fn poll_buf(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.inner.poll_data()
    }

    fn poll_trailers(&mut self) -> Poll<Option<hyper::HeaderMap>, Self::Error> {
        self.inner.poll_trailers()
    }

    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }
}

impl Payload for Body {
    type Data = hyper::Chunk;
    type Error = hyper::Error;

    fn poll_data(&mut self) -> Poll<Option<Self::Data>, Self::Error> {
        self.inner.poll_data()
    }

    fn poll_trailers(&mut self) -> Poll<Option<hyper::HeaderMap>, Self::Error> {
        self.inner.poll_trailers()
    }

    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }
}
