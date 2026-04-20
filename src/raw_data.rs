use core::mem::MaybeUninit;

use crate::stp::SizedTP;

#[repr(C, packed)]
pub union RawData<const BYTE_CAP: usize>
where
    [(); BYTE_CAP / <*const ()>::SIZE * <*const ()>::SIZE]:,
{
    // access the data through here. writing to here will preserve pointer
    // provenance (hopefully).
    storing_vec: [MaybeUninit<*const ()>; {
        BYTE_CAP / <*const ()>::SIZE * <*const ()>::SIZE
    }],
    // only for setting the byte size of the backing buffer.
    storing_inline: [MaybeUninit<u8>; BYTE_CAP],
}

impl<const BYTE_CAP: usize> RawData<BYTE_CAP>
where
    [(); BYTE_CAP / <*const ()>::SIZE * <*const ()>::SIZE]:,
{
    const fn well_formedness_check() {
        // due to provenance access reasons, BYTE_CAP must be a multiple of the
        // pointer size.
        assert!(
            BYTE_CAP.is_multiple_of(<*const ()>::SIZE),
            "BYTE_CAP must be a multiple of the pointer size"
        )
    }

    const _ASSERT_WELL_FORMED: () = const { Self::well_formedness_check() };

    #[inline]
    pub(crate) const fn new() -> Self {
        const { Self::_ASSERT_WELL_FORMED };
        RawData {
            // SAFETY: within initialization, we don't have any pointers to keep
            // the provenance of, so this is okay.
            storing_inline: [const { MaybeUninit::uninit() }; BYTE_CAP],
        }
    }

    #[inline]
    pub(crate) const unsafe fn provenance_access(&self) -> *const u8 {
        // SAFETY: conceptually, the `data` field holds the pointer provenance,
        // while the actual byte data is stored in the other field of the union.
        // hopefully MIRI doesn't see that we're taking a raw reference to a ZST
        // and prevent us from writing to the other field (dubious).
        (&raw const self.storing_vec) as *const u8
    }

    #[inline]
    pub(crate) const unsafe fn provenance_access_mut(&mut self) -> *mut u8 {
        // SAFETY: conceptually, the `data` field holds the pointer provenance,
        // while the actual byte data is stored in the other field of the union.
        // hopefully MIRI doesn't see that we're taking a raw reference to a ZST
        // and prevent us from writing to the other field (dubious).
        (&raw mut self.storing_vec) as *mut u8
    }
    #[inline]
    pub(crate) const unsafe fn inline_access(&self) -> *const u8 {
        (&raw const self.storing_inline) as *const u8
    }

    #[inline]
    pub(crate) const unsafe fn inline_access_mut(&mut self) -> *mut u8 {
        (&raw mut self.storing_inline) as *mut u8
    }

    #[inline]
    pub(crate) const unsafe fn provenance_get_ptr<T>(
        &self, add: usize,
    ) -> *const T {
        unsafe { self.provenance_access().byte_add(add).cast::<T>() }
    }

    #[inline]
    pub(crate) const unsafe fn provenance_get_mut_ptr<T>(
        &mut self, add: usize,
    ) -> *mut T {
        unsafe { self.provenance_access_mut().byte_add(add).cast::<T>() }
    }

    #[inline]
    pub(crate) const unsafe fn get_ptr<T>(&self, add: usize) -> *const T {
        unsafe { self.inline_access().byte_add(add).cast::<T>() }
    }

    #[inline]
    pub(crate) const unsafe fn get_mut_ptr<T>(&mut self, add: usize) -> *mut T {
        unsafe { self.inline_access_mut().byte_add(add).cast::<T>() }
    }
}
