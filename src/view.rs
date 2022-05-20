
use std::fmt;

pub enum ByteViewError {
    ImproperAlignment {
        required: usize,
        found: usize
    },
    ImproperSize {
        required: usize,
        found: usize
    }
}

impl fmt::Debug for ByteViewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImproperAlignment { required, found } => {
                write!(f, "type specified has alignment of {}, this field's location suggests an alignment of {}", required, found)
            },
            Self::ImproperSize { required, found } => {
                write!(f, "type specified requires a size that is a multiple of {}, this field's size is {}", required, found)
            }
        }
    }
}

#[repr(C)]
pub struct ByteView<const LENGTH: usize> {
    bytes: [u8; LENGTH]
}

impl<const LENGTH: usize> ByteView<LENGTH> {
    fn size_align_checks<T: Sized>(&self) -> Result<(), ByteViewError> {
        // Get the alignment of the pointer from the number of trailing zeros
        let self_align = 2usize.pow((self.bytes.as_ptr() as u64).trailing_zeros());
        let required_align = std::mem::align_of::<T>();

        // Compare the alignments and dip if it's not the right size
        if self_align < required_align {
            return Err(ByteViewError::ImproperAlignment { required: required_align, found: self_align });
        }    

        // Ensure that we can safely encapsulate the whole byte array as a slice of the specified type
        let required_size = std::mem::size_of::<T>();
        if LENGTH % required_size != 0 {
            return Err(ByteViewError::ImproperSize { required: required_size, found: LENGTH });
        }

        Ok(())
    }

    /// Creates a new byte view full of zeros
    pub fn zeros() -> Self {
        Self {
            bytes: [0; LENGTH]
        }
    }

    /// Creates a new byte view full of the specified value
    pub fn new(base: u8) -> Self {
        Self {
            bytes: [base; LENGTH]
        }
    }

    /// Gets the underlying byte array of the byte view
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Gets the underlying mutable byte array of the byte view
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }

    /// Attempts to get the byte view as a slice of the specified type
    /// 
    /// This performs alignment checks on the starting pointer of this range
    /// as well as size checks of the type compared to the number of bytes in the range
    /// 
    /// If the alignments don't match up, or the length of the byte view does not match
    /// a multiple of the size of the specified type, this function will return an error
    pub fn view_as<T: Sized>(&self) -> Result<&[T], ByteViewError> {
        self.size_align_checks::<T>()
            .map(|_| unsafe {
                std::slice::from_raw_parts(self.bytes.as_ptr() as *const T, LENGTH / std::mem::size_of::<T>())
            })
    }

    /// Attempts to get the byte view as a mutable slice of the specified type
    /// 
    /// This performs alignment checks on the starting pointer of this range
    /// as well as size checks of the type compared to the number of bytes in the range
    /// 
    /// If the alignments don't match up, or the length of the byte view does not match
    /// a multiple of the size of the specified type, this function will return an error
    pub fn view_as_mut<T: Sized>(&mut self) -> Result<&mut [T], ByteViewError> {
        self.size_align_checks::<T>()
            .map(|_| unsafe {
                std::slice::from_raw_parts_mut(self.bytes.as_ptr() as *mut T, LENGTH / std::mem::size_of::<T>())
            })
    }

    /// Gets the byte view as a slice of the specified type, without performing alignment or size checks.
    /// 
    /// This slice will never extend past the end of the byte view, even if the size of view is not a multiple
    /// of the size of the specified type
    pub unsafe fn view_as_unchecked<T: Sized>(&self) -> &[T] {
        std::slice::from_raw_parts(self.bytes.as_ptr() as *const T, LENGTH / std::mem::size_of::<T>())
    }

    /// Gets the byte view as a mutable slice of the specified type, without performing alignment or size checks.
    /// 
    /// This slice will never extend past the end of the byte view, even if the size of view is not a multiple
    /// of the size of the specified type
    pub unsafe fn view_as_mut_unchecked<T: Sized>(&mut self) -> &mut [T] {
        std::slice::from_raw_parts_mut(self.bytes.as_ptr() as *mut T, LENGTH / std::mem::size_of::<T>())
    }
}