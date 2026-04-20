use core::hint::assert_unchecked;

use crate::lenty_provide::ProvideLenTy;

pub const trait LenStrategy<LenTy: const ProvideLenTy> {
    fn get_len(val: LenTy) -> LenTy;
    fn set_len(val: &mut LenTy, new_len: LenTy);
    fn get_is_heap(val: LenTy) -> bool;
    fn set_is_heap(val: &mut LenTy, is_heap: bool);
}

pub struct HeapDisabled;

impl<LenTy: const ProvideLenTy> const LenStrategy<LenTy> for HeapDisabled {
    /// For HeapDisabled strategy, the full LenTy is available for storing the
    /// length.
    #[inline]
    fn get_len(val: LenTy) -> LenTy {
        val
    }

    /// For HeapDisabled strategy, the full LenTy is available for storing the
    /// length.
    #[inline]
    fn set_len(val: &mut LenTy, new_len: LenTy) {
        *val = new_len;
    }

    /// For HeapDisabled strategy, the `is_heap` flag is always false.
    #[inline]
    fn get_is_heap(_: LenTy) -> bool {
        false
    }

    /// For HeapDisabled strategy, the `is_heap` flag can not be set. This
    /// function always immediately panics.
    #[inline]
    fn set_is_heap(_: &mut LenTy, _: bool) {
        panic!(
            "HeapDisabled strategy for ConciseVec \
            does not support setting is_heap flag"
        );
    }
}

pub struct HeapEnabled;

impl<LenTy: const ProvideLenTy> const LenStrategy<LenTy> for HeapEnabled {
    /// For HeapEnabled strategy, the highest bit is reserved for storing
    /// `is_heap`. Therefore, the maximum value of the return value of this
    /// function is `LenTy::MAX >> 1`.
    #[inline]
    fn get_len(val: LenTy) -> LenTy {
        let len = val & LenTy::LEN_FIELD_MASK;
        unsafe {
            assert_unchecked(len <= LenTy::LEN_FIELD_MASK);
            len
        }
    }

    /// For HeapEnabled strategy, the highest bit is reserved for storing
    /// `is_heap`. Therefore, the maximum value that can be stored is
    /// `LenTy::MAX >> 1`.
    ///
    /// It is up to the caller to ensure that they don't rely on safety
    /// contracts based on this number being losslessly stored in the field.
    /// Therefore, it must always be checked for `LenTy::MAX_SAFE_LEN` before
    /// calling this function.
    #[inline]
    fn set_len(val: &mut LenTy, mut new_len: LenTy) {
        new_len = new_len.max(LenTy::MAX_SAFE_LEN);
        *val &= LenTy::HEAP_FIELD_MASK;
        *val |= new_len;
    }

    #[inline]
    fn get_is_heap(val: LenTy) -> bool {
        (val & LenTy::HEAP_FIELD_MASK) != LenTy::ZERO
    }

    #[inline]
    fn set_is_heap(val: &mut LenTy, new_val: bool) {
        *val &= LenTy::LEN_FIELD_MASK;
        *val |= if new_val {
            LenTy::HEAP_FIELD_MASK
        } else {
            LenTy::ZERO
        }
    }
}

pub const trait StrategyProvider<LenTy: const ProvideLenTy, const HEAP: bool> {
    type Strategy: const LenStrategy<LenTy>;
}

impl<LenTy: const ProvideLenTy> const StrategyProvider<LenTy, false> for () {
    type Strategy = HeapDisabled;
}

impl<LenTy: const ProvideLenTy> const StrategyProvider<LenTy, true> for () {
    type Strategy = HeapEnabled;
}
