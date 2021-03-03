//! Signed big integer.

use crate::{
    sign::Sign::{self, *},
    ubig::UBig,
};

/// Signed big integer.
///
/// Arbitrarily large signed integer.
///
/// # Examples
///
/// ```
/// # use ibig::{prelude::*, ParseError};
/// let a = ibig!(a2a123bbb127779cccc123123ccc base 32);
/// let b = ibig!(-0x1231abcd4134);
/// let c = IBig::from_str_radix("a2a123bbb127779cccc123123ccc", 32)?;
/// let d = IBig::from_str_radix("-1231abcd4134", 16)?;
/// assert_eq!(a, c);
/// assert_eq!(b, d);
/// # Ok::<(), ParseError>(())
/// ```
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct IBig {
    sign: Sign,
    magnitude: UBig,
}

impl IBig {
    pub(crate) fn from_sign_magnitude(mut sign: Sign, magnitude: UBig) -> IBig {
        if magnitude.is_zero() {
            sign = Positive;
        }
        IBig { sign, magnitude }
    }

    pub(crate) fn sign(&self) -> Sign {
        self.sign
    }

    pub(crate) fn magnitude(&self) -> &UBig {
        &self.magnitude
    }

    pub(crate) fn into_sign_magnitude(self) -> (Sign, UBig) {
        (self.sign, self.magnitude)
    }
}
