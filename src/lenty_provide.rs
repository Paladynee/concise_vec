use core::fmt::Debug;
use core::hash::Hash;
use core::mem::size_of;

pub(crate) mod private {
    use crate::stp::SizedTP;

    pub(crate) trait Sealed {}
    macro_rules! impl_sealed {
        ($($Ty:ty),* $(,)?) => {
            $( impl Sealed for $Ty {} )*
        };
    }
    impl_sealed!(u8, u16, u32, u64, u128, usize);
}

pub const trait ProvideLenTy:
    const ConstNumOps
    + Clone
    + Copy
    + Debug
    + Hash
    + SizedTP
    + private::Sealed
    + 'static
{
    const BIT_SIZE: usize;
    const LEN_FIELD_MASK: Self;
    const HEAP_FIELD_MASK: Self = !Self::LEN_FIELD_MASK;
    const MAX_SAFE_LEN: Self = Self::LEN_FIELD_MASK;

    const MAX: Self;
    const ONE: Self;
    const ZERO: Self;

    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn to_usize_lossy(self) -> usize;
    fn from_usize_lossy(val: usize) -> Self;
}

macro_rules! impl_provide_lenty {
    ($( $Type: ty ),* $(,)?) => {
        $(
            impl const ProvideLenTy for $Type {
                const BIT_SIZE: usize = size_of::<Self>() * 8;
                const LEN_FIELD_MASK: Self = Self::MAX >> 1;
                const ONE: Self = 1;
                const ZERO: Self = 0;
                const MAX: Self = Self::MAX;
                #[inline]
                fn checked_add(self, rhs: Self) -> Option<Self> {
                    self.checked_add(rhs)
                }
                #[inline]
                fn to_usize_lossy(self) -> usize {
                    self as usize
                }
                #[inline]
                fn from_usize_lossy(val: usize) -> Self {
                    val as Self
                }
            }
        )*
    };
}

impl_provide_lenty!(u8, u16, u32, u64, u128, usize);

mod const_num_ops {
    use core::marker::Destruct;
    use core::ops::*;

    pub const trait ConstPrimOps:
        Sized
        + const Add<Output = Self>
        + const Sub<Output = Self>
        + const Mul<Output = Self>
        + const Div<Output = Self>
        + const Rem<Output = Self>
    {
    }

    pub const trait ConstPrimAssignOps:
        Sized
        + const AddAssign<Self>
        + const SubAssign<Self>
        + const MulAssign<Self>
        + const DivAssign<Self>
        + const RemAssign<Self>
    {
    }

    pub const trait ConstBitOps:
        Sized
        + const BitAnd<Output = Self>
        + const BitOr<Output = Self>
        + const BitXor<Output = Self>
        + const Not<Output = Self>
        + const Shl<Output = Self>
        + const Shr<Output = Self>
    {
    }

    pub const trait ConstBitAssignOps:
        Sized
        + const BitAndAssign<Self>
        + const BitOrAssign<Self>
        + const BitXorAssign<Self>
        + const ShlAssign<Self>
        + const ShrAssign<Self>
    {
    }

    use core::cmp::PartialEq;
    use core::cmp::PartialOrd;

    pub const trait ConstNumCmpOps:
        const PartialEq
        + const Eq
        + const PartialOrd
        + const Ord
        + const Destruct
    {
    }

    pub const trait ConstNumOps:
        Sized
        + ConstPrimOps
        + ConstPrimAssignOps
        + ConstBitOps
        + ConstBitAssignOps
        + ConstNumCmpOps
    {
    }

    macro_rules! impl_const_num_ops {
        ($( $Type: ty ),* $(,)?) => {
            $(
                impl const ConstPrimOps for $Type {}
                impl const ConstPrimAssignOps for $Type {}
                impl const ConstBitOps for $Type {}
                impl const ConstBitAssignOps for $Type {}
                impl const ConstNumCmpOps for $Type {}
                impl const ConstNumOps for $Type {}
             )*
        };
    }

    impl_const_num_ops!(u8, u16, u32, u64, u128, usize);
}

pub use const_num_ops::ConstNumOps;

use crate::stp::SizedTP;
