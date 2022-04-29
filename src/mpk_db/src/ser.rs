use rkyv::{with::AsStringError,
	   ser::Serializer,
	   Fallible};

#[derive(Debug, Clone)]
pub struct NodeSerializer<S> {
    inner: S,
}

impl<S> NodeSerializer<S> {
    pub fn into_inner(self) -> S {
      self.inner
    }
}

impl<S: Fallible> Fallible for NodeSerializer<S> {
    type Error = SerializerError<S::Error>;
}

impl<S: Serializer> Serializer for NodeSerializer<S> {
    #[inline]
    fn pos(&self) -> usize {
        self.inner.pos()
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.inner.write(bytes).map_err(SerializerError::Inner)
    }
}

impl<S: Default> Default for NodeSerializer<S> {
    fn default() -> Self {
        Self {
            inner: S::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EdgeSerializer<S> {
    inner: S,
}

impl<S> EdgeSerializer<S> {
    pub fn into_inner(self) -> S {
      self.inner
    }
}

impl<S: Fallible> Fallible for EdgeSerializer<S> {
    type Error = SerializerError<S::Error>;
}

impl<S: Serializer> Serializer for EdgeSerializer<S> {
    #[inline]
    fn pos(&self) -> usize {
        self.inner.pos()
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.inner.write(bytes).map_err(SerializerError::Inner)
    }
}

impl<S: Default> Default for EdgeSerializer<S> {
    fn default() -> Self {
        Self {
            inner: S::default(),
        }
    }
}

#[derive(Debug)]
pub enum SerializerError<E> {
    Inner(E),
    AsStringError,
}

impl<E> From<AsStringError> for SerializerError<E> {
    fn from(_: AsStringError) -> Self {
        Self::AsStringError
    }
}
