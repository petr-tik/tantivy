use common::HasLen;
use stable_deref_trait::{CloneStableDeref, StableDeref};
use std::ops::Deref;
use std::sync::Arc;

pub type BoxedData = Box<Deref<Target = [u8]> + Send + Sync + 'static>;


const EMPTY_SLICE: [u8; 0] = [];

/// Read object that represents files in tantivy.
///
/// These read objects are only in charge to deliver
/// the data in the form of a constant read-only `&[u8]`.
/// Whatever happens to the directory file, the data
/// hold by this object should never be altered or destroyed.
<<<<<<< HEAD
pub enum ReadOnlySource {
    /// Mmap source of data
    #[cfg(feature = "mmap")]
    Mmap(MmapReadOnly),
    /// Wrapping a `Vec<u8>`
    Anonymous(SharedVecSlice),
    /// Wrapping a static slice
    Static(&'static [u8])
=======
pub struct ReadOnlySource {
    data: Arc<BoxedData>,
    start: usize,
    stop: usize,
>>>>>>> upstream/master
}

unsafe impl StableDeref for ReadOnlySource {}
unsafe impl CloneStableDeref for ReadOnlySource {}

impl Deref for ReadOnlySource {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl From<Arc<BoxedData>> for ReadOnlySource {
    fn from(data: Arc<BoxedData>) -> Self {
        let len = data.len();
        ReadOnlySource {
            data,
            start: 0,
            stop: len,
        }
    }
}

impl ReadOnlySource {
    pub(crate) fn new<D>(data: D) -> ReadOnlySource
    where
        D: Deref<Target = [u8]> + Send + Sync + 'static,
    {
        let len = data.len();
        ReadOnlySource {
            data: Arc::new(Box::new(data)),
            start: 0,
            stop: len,
        }
    }

    /// Creates an empty ReadOnlySource
    pub fn empty() -> ReadOnlySource {
<<<<<<< HEAD
        ReadOnlySource::Static(&EMPTY_SLICE)
=======
        ReadOnlySource::new(&[][..])
>>>>>>> upstream/master
    }

    /// Returns the data underlying the ReadOnlySource object.
    pub fn as_slice(&self) -> &[u8] {
<<<<<<< HEAD
        match *self {
            #[cfg(feature = "mmap")]
            ReadOnlySource::Mmap(ref mmap_read_only) => mmap_read_only.as_slice(),
            ReadOnlySource::Anonymous(ref shared_vec) => shared_vec.as_slice(),
            ReadOnlySource::Static(data) => data,
        }
=======
        &self.data[self.start..self.stop]
>>>>>>> upstream/master
    }

    /// Splits into 2 `ReadOnlySource`, at the offset given
    /// as an argument.
    pub fn split(self, addr: usize) -> (ReadOnlySource, ReadOnlySource) {
        let left = self.slice(0, addr);
        let right = self.slice_from(addr);
        (left, right)
    }

    /// Creates a ReadOnlySource that is just a
    /// view over a slice of the data.
    ///
    /// Keep in mind that any living slice extends
    /// the lifetime of the original ReadOnlySource,
    ///
    /// For instance, if `ReadOnlySource` wraps 500MB
    /// worth of data in anonymous memory, and only a
    /// 1KB slice is remaining, the whole `500MBs`
    /// are retained in memory.
    pub fn slice(&self, start: usize, stop: usize) -> ReadOnlySource {
        assert!(
            start <= stop,
            "Requested negative slice [{}..{}]",
            start,
            stop
        );
<<<<<<< HEAD
        match *self {
            #[cfg(feature = "mmap")]
            ReadOnlySource::Mmap(ref mmap_read_only) => {
                let sliced_mmap = mmap_read_only.range(from_offset, to_offset - from_offset);
                ReadOnlySource::Mmap(sliced_mmap)
            }
            ReadOnlySource::Anonymous(ref shared_vec) => {
                ReadOnlySource::Anonymous(shared_vec.slice(from_offset, to_offset))
            }
            ReadOnlySource::Static(data) => {
                ReadOnlySource::Static(&data[from_offset..to_offset])
            }
=======
        assert!(stop <= self.len());
        ReadOnlySource {
            data: self.data.clone(),
            start: self.start + start,
            stop: self.start + stop,
>>>>>>> upstream/master
        }
    }

    /// Like `.slice(...)` but enforcing only the `from`
    /// boundary.
    ///
    /// Equivalent to `.slice(from_offset, self.len())`
    pub fn slice_from(&self, from_offset: usize) -> ReadOnlySource {
        self.slice(from_offset, self.len())
    }

    /// Like `.slice(...)` but enforcing only the `to`
    /// boundary.
    ///
    /// Equivalent to `.slice(0, to_offset)`
    pub fn slice_to(&self, to_offset: usize) -> ReadOnlySource {
        self.slice(0, to_offset)
    }
}

impl HasLen for ReadOnlySource {
    fn len(&self) -> usize {
        self.stop - self.start
    }
}

impl Clone for ReadOnlySource {
    fn clone(&self) -> Self {
        self.slice_from(0)
    }
}

impl From<Vec<u8>> for ReadOnlySource {
    fn from(data: Vec<u8>) -> ReadOnlySource {
        ReadOnlySource::new(data)
    }
}

impl From<&'static [u8]> for ReadOnlySource {
    fn from(data: &'static [u8]) -> ReadOnlySource {
        ReadOnlySource::Static(data)
    }
}
