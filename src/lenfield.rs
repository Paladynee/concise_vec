use crate::heap_strategy_provider::LenStrategy;
use crate::heap_strategy_provider::StrategyProvider;
use crate::lenty_provide::ProvideLenTy;

/// The field of a ConciseVec that stores the amount of items currently stored
/// in-line, and whether the contents were offloaded to the heap in a single
/// packed integer.
///
/// Note that when stored in heap, the amount of items in-line does not make
/// sense.
#[repr(transparent)]
pub struct LenField<LenTy: const ProvideLenTy, const HEAP: bool> {
    lenty: LenTy,
}

impl<LenTy: const ProvideLenTy, const HEAP: bool> LenField<LenTy, HEAP>
where
    // proof that StrategyProvider has been implemented for all bools.
    (): const StrategyProvider<LenTy, HEAP>,
{
    /// A zero initialized LenField. `is_heap` is false, and the length is 0.
    #[inline]
    pub const fn new() -> Self {
        LenField {
            lenty: LenTy::ZERO,
        }
    }

    /// ## If `HEAP` is `true`:
    ///
    /// For HeapEnabled strategy, the highest bit is reserved for
    /// storing `is_heap`. Therefore, the maximum value of the return value
    /// of this function is `LenTy::MAX >> 1`.
    ///
    /// ## If `HEAP` is `false`:
    ///
    /// For HeapDisabled strategy, the full LenTy is available for storing the
    /// length.
    #[inline]
    pub const fn get_len(&self) -> LenTy {
        <() as StrategyProvider<LenTy, HEAP>>::Strategy::get_len(self.lenty)
    }

    /// ## If `HEAP` is `true`:
    ///
    /// For HeapEnabled strategy, the highest bit is reserved for storing
    /// `is_heap`. Therefore, the maximum value that can be stored is
    /// `LenTy::MAX >> 1`.
    ///
    /// It is up to the caller to ensure that they don't rely on safety
    /// contracts based on this number being losslessly stored in the field.
    /// Therefore, it must always be checked for `LenTy::MAX_SAFE_LEN` before
    /// calling this function.
    ///
    /// ## If `HEAP` is `false`:
    ///
    /// For HeapDisabled strategy, the full LenTy is available for storing the
    /// length.
    #[inline]
    pub const unsafe fn set_len(&mut self, new_len: LenTy) {
        <() as StrategyProvider<LenTy, HEAP>>::Strategy::set_len(
            &mut self.lenty,
            new_len,
        );
    }

    /// ## If `HEAP` is `true`:
    ///
    /// Returns whether the inline storage is storing a `(ptr, len, cap)` tuple.
    ///
    /// ## If `HEAP` is `false`:
    ///
    /// For HeapDisabled strategy, the `is_heap` flag is always false.
    #[inline]
    pub const fn get_is_heap(&self) -> bool {
        <() as StrategyProvider<LenTy, HEAP>>::Strategy::get_is_heap(self.lenty)
    }

    /// ## If `HEAP` is `true`:
    ///
    /// Sets the `is_heap` flag. When `is_heap` is true, the inline storage is
    /// said to be storing a `(ptr, len, cap)` tuple.
    ///
    /// ## If `HEAP` is `false`:
    ///
    /// For HeapDisabled strategy, the `is_heap` flag can not be set. This
    /// function always immediately panics.
    #[inline]
    pub const unsafe fn set_is_heap(&mut self, is_heap: bool) {
        <() as StrategyProvider<LenTy, HEAP>>::Strategy::set_is_heap(
            &mut self.lenty,
            is_heap,
        );
    }
}
