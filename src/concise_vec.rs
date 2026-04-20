mod r#impl;
mod push_provider;
mod trait_impls;

use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::slice;

use push_provider::PushProvider;

use crate::aligner::AlignTyProvider;
use crate::aligner::Aligner;
use crate::heap_strategy_provider::StrategyProvider;
use crate::into_iter::IntoIter;
use crate::lenfield::LenField;
use crate::lenty_provide::ProvideLenTy;
use crate::raw_data::RawData;
use crate::stp::SizedTP;

#[repr(C)]
pub struct ConciseVec<
    T,
    LenTy: const ProvideLenTy,
    const BYTE_CAP: usize,
    const HEAP_ALLOWED: bool,
> where
    // this is proof that the Aligner can work for all bools.
    (): const AlignTyProvider<HEAP_ALLOWED>,
    // this is proof that the StrategyProvider can work for all bools.
    (): const StrategyProvider<LenTy, HEAP_ALLOWED>,
    // this is proof that the PushProvider can work for all bools.
    (): PushProvider<T, LenTy, BYTE_CAP, HEAP_ALLOWED>,
    // this is proof that BYTE_CAP does not exceed a usize (??? rust???)
    [(); BYTE_CAP / <*const ()>::SIZE * <*const ()>::SIZE]:,
{
    // a provenance-preserving arbitrary byte buffer. sounds kinda cool when
    // you say it out loud.
    data: RawData<BYTE_CAP>,
    // this ZST aligns the ConciseVec to whatever minimum of the following is
    // supposed to be aligned to:
    // - T
    // - LenTy
    // - if HEAP_ALLOWED is enabled, a usize
    // - if HEAP_ALLOWED is enabled, a thin pointer
    // this does NOT natively align to a cache line. for that, you can wrap the
    // ConciseVec in a general purpose cache aligner for your architecture.
    _aligner: Aligner<T, LenTy, HEAP_ALLOWED>,
    _marker: PhantomData<T>,
    _len_ty: PhantomData<LenTy>,
}

impl<
    T,
    LenTy: const ProvideLenTy,
    const BYTE_CAP: usize,
    const HEAP_ALLOWED: bool,
> const Default for ConciseVec<T, LenTy, BYTE_CAP, HEAP_ALLOWED>
where
    // this is proof that the Aligner can work for all bools.
    (): const AlignTyProvider<HEAP_ALLOWED>,
    // this is proof that the StrategyProvider can work for all bools.
    (): const StrategyProvider<LenTy, HEAP_ALLOWED>,
    // this is proof that the PushProvider can work for all bools.
    (): PushProvider<T, LenTy, BYTE_CAP, HEAP_ALLOWED>,
    // this is proof that BYTE_CAP does not exceed a usize (??? rust???)
    [(); BYTE_CAP / <*const ()>::SIZE * <*const ()>::SIZE]:,
{
    #[inline]
    fn default() -> Self {
        ConciseVec::new()
    }
}

impl<
    T,
    LenTy: const ProvideLenTy,
    const BYTE_CAP: usize,
    const HEAP_ALLOWED: bool,
> ConciseVec<T, LenTy, BYTE_CAP, HEAP_ALLOWED>
where
    // this is proof that the Aligner can work for all bools.
    (): const AlignTyProvider<HEAP_ALLOWED>,
    // this is proof that the StrategyProvider can work for all bools.
    (): const StrategyProvider<LenTy, HEAP_ALLOWED>,
    // this is proof that the PushProvider can work for all bools.
    (): PushProvider<T, LenTy, BYTE_CAP, HEAP_ALLOWED>,
    // this is proof that BYTE_CAP does not exceed a usize (??? rust???)
    [(); BYTE_CAP / <*const ()>::SIZE * <*const ()>::SIZE]:,
{
    #[inline]
    const fn well_formedness_check() {
        // the LenTy must fit within.
        assert!(
            BYTE_CAP >= LenTy::SIZE,
            "ConciseVec BYTE_CAP must be able to contain a LenTy"
        );

        // total byte storage of T must not exceed isize::MAX
        assert!(
            Self::T_BYTE_CAP <= isize::MAX as usize,
            "ConciseVec storage can not exceed system allocation \
            limits of isize::MAX"
        );

        if HEAP_ALLOWED {
            // if heap is enabled, "std" or "alloc" features must not be
            // disabled
            assert!(
                cfg!(not(any(feature = "std", feature = "alloc"))),
                "heap is enabled: ConciseVec requires std or alloc features"
            );

            // the LenTy, and a (pointer, len, cap) tuple must fit within.
            assert!(
                BYTE_CAP >= <*const ()>::SIZE + usize::SIZE + usize::SIZE,
                "heap is enabled: ConciseVec's BYTE_CAP must be able to \
                contain a LenTy and a (pointer, len, cap) tuple"
            );
        }
    }
    const _ASSERT_WELL_FORMED: () = const { Self::well_formedness_check() };

    pub const CAPACITY: usize = Self::REMAINING_STORAGE / Self::T_STRIDE;

    #[inline]
    pub const fn new() -> Self {
        const { Self::_ASSERT_WELL_FORMED };
        let mut data = RawData::new();
        unsafe {
            (*data.get_mut_ptr::<MaybeUninit<LenTy>>(Self::LENFIELD_OFFSET))
                .write(LenTy::ZERO)
        };
        ConciseVec {
            data,
            _aligner: Aligner::new(),
            _marker: PhantomData,
            _len_ty: PhantomData,
        }
    }

    /// ## If `HEAP_ALLOWED`
    ///
    /// If stored in-line, the amount of T's stored in-line. If stored in-heap,
    /// amount of T's stored in-heap.
    ///
    /// ## If NOT `HEAP_ALLOWED`
    ///
    /// The amount of T's stored in-line.
    #[inline]
    pub const fn len(&self) -> LenTy {
        if HEAP_ALLOWED {
            panic!("todo: check is_heap, provide LenField.get_len or vec::len")
        } else {
            self.get_len_field().get_len()
        }
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == LenTy::ZERO
    }

    #[inline]
    pub const fn capacity(&self) -> usize {
        if HEAP_ALLOWED {
            panic!(
                "todo: check is_heap, provide Self::CAPACITY or vec::capacity"
            )
        } else {
            Self::CAPACITY
        }
    }

    /// ## If `HEAP_ALLOWED`
    ///
    /// If in-line, attempts storing the value. On failure, offloads the data to
    /// a heap allocated vector and stores the value.
    ///
    /// May panic (rare) if memory allocation fails.
    ///
    /// Returns `&mut T` to the pushed value.
    ///
    /// ## If NOT `HEAP_ALLOWED`
    ///
    /// Attempts storing the value. On failure, returns an `Err(T)`.
    ///
    /// Returns `Ok(&mut T)` to the pushed value on success.
    #[inline]
    pub fn push(
        &mut self, value: T,
    ) -> <() as PushProvider<T, LenTy, BYTE_CAP, HEAP_ALLOWED>>::PushResult<'_, T>
    {
        <() as PushProvider<T, LenTy, BYTE_CAP, HEAP_ALLOWED>>::push(
            self, value,
        )
    }

    /// Removes the last element from a vector and returns it, or [`None`] if it
    /// is empty.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        let temp_ref = &*self;

        if HEAP_ALLOWED {
            panic!("todo: check is_heap, provide vec::pop")
        } else {
            let len = temp_ref.len();
            let new_len = len - LenTy::ONE;
            let lossy_new_len = new_len.to_usize_lossy();

            // explicit reborrow fence to help the optimizer realize that the
            // len field is not written to until this point.
            let cv = &mut *self;

            let ptr =
                unsafe { cv.data.get_ptr::<T>(lossy_new_len * Self::T_STRIDE) };
            let val = unsafe { ptr.read() };

            unsafe { cv.get_len_field_mut().set_len(new_len) };

            Some(val)
        }
    }

    /// Sets the amount of elements stored in the vector.
    ///
    /// # Safety
    ///
    /// This value must not exceed `ConciseVec::CAPACITY` nor
    /// `LenTy::MAX_SAFE_LEN`.
    ///
    /// ## If `HEAP_ALLOWED`
    ///
    /// Sets the amount of elements stored in the vector.
    ///
    /// If already allocated, sets the length for the allocated vector.
    ///
    /// # Safety
    ///
    /// This value must not exceed `ConciseVec::CAPACITY` nor
    /// `LenTy::MAX_SAFE_LEN`.
    #[inline]
    pub const unsafe fn set_len(&mut self, new_len: LenTy) {
        if HEAP_ALLOWED {
            panic!(
                "todo: check is_heap, provide LenField.set_len or vec::set_len"
            )
        } else {
            unsafe { self.get_len_field_mut().set_len(new_len) };
        }
    }

    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        if HEAP_ALLOWED {
            panic!("todo: check is_heap, provide vec::as_slice")
        } else {
            unsafe {
                slice::from_raw_parts(
                    self.data.get_ptr::<T>(Self::T_STORAGE_START_OFFSET),
                    // this conversion is safe because if you have a valid
                    // ConciseVec in memory already (as exemplified by &self
                    // above), it is safe to cast the length field to a usize.
                    self.len().to_usize_lossy(),
                )
            }
        }
    }

    #[inline]
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        if HEAP_ALLOWED {
            panic!("todo: check is_heap, provide vec::as_slice")
        } else {
            unsafe {
                slice::from_raw_parts_mut(
                    self.data.get_mut_ptr::<T>(Self::T_STORAGE_START_OFFSET),
                    // this conversion is safe because if you have a valid
                    // ConciseVec in memory already (as exemplified by &self
                    // above), it is safe to cast the length field to a usize.
                    self.len().to_usize_lossy(),
                )
            }
        }
    }

    #[inline]
    pub const fn iter(&self) -> slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    #[inline]
    pub const fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }

    /// Yields all the elements in the vector by-value sequentially, clearing
    /// the backing array. Does not take ownership of the backing array.
    ///
    /// ## If `HEAP_ALLOWED`
    ///
    /// If it already allocated, does not deallocate the heap vector. See
    /// [`ConciseVec::shrink_to_fit`] for deallocation.
    #[inline]
    pub fn into_iter(&mut self) -> IntoIter<'_, T, LenTy> {
        let _len = if HEAP_ALLOWED {
            panic!("todo: check is_heap, provide vec::into_iter-ish")
        } else {
            let val = self.len();
            unsafe { self.set_len(LenTy::ZERO) };
            val
        };

        IntoIter {
            _marker: PhantomData,
        }
    }

    /// Clears the vector, removing all values.
    ///
    /// ## If `HEAP_ALLOWED`
    ///
    /// If it allocated, does not deallocate the heap vector. See
    /// [`ConciseVec::shrink_to_fit`] for deallocation.
    #[inline]
    pub fn clear(&mut self) {
        if HEAP_ALLOWED {
            panic!("todo: check is_heap, provide vec::clear")
        } else {
            let len = self.len().to_usize_lossy();
            if len > 0 {
                let slice = core::ptr::slice_from_raw_parts_mut(
                    unsafe {
                        self.data.get_mut_ptr::<T>(Self::T_STORAGE_START_OFFSET)
                    },
                    len,
                );
                unsafe {
                    self.get_len_field_mut().set_len(LenTy::ZERO);
                    core::ptr::drop_in_place(slice);
                }
            }
        }
    }

    /// Shortens the vector, keeping the first `len` elements and dropping
    /// the rest.
    ///
    /// ## If `HEAP_ALLOWED`
    ///
    /// If it already allocated, does not deallocate the heap vector. See
    /// [`ConciseVec::shrink_to_fit`] for deallocation.
    #[inline]
    pub fn truncate(&mut self, len: usize) {
        if HEAP_ALLOWED {
            panic!("todo: check is_heap, provide vec::truncate")
        } else {
            let current_len = self.len().to_usize_lossy();
            if len >= current_len {
                return;
            }

            unsafe {
                let remaining_len = current_len - len;
                let ptr = self.data.get_mut_ptr::<T>(len * Self::T_STRIDE);
                let slice =
                    core::ptr::slice_from_raw_parts_mut(ptr, remaining_len);

                self.get_len_field_mut()
                    .set_len(LenTy::from_usize_lossy(len));
                core::ptr::drop_in_place(slice);
            }
        }
    }
}

mod test;
