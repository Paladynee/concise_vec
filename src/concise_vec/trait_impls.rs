use core::ops::Deref;
use core::ops::DerefMut;
use core::ptr;
use core::slice;

use crate::ConciseVec;
use crate::aligner::AlignTyProvider;
use crate::concise_vec::push_provider::PushProvider;
use crate::heap_strategy_provider::StrategyProvider;
use crate::lenty_provide::ProvideLenTy;
use crate::stp::SizedTP;

impl<
    T,
    LenTy: const ProvideLenTy,
    const BYTE_CAP: usize,
    const HEAP_ALLOWED: bool,
> Deref for ConciseVec<T, LenTy, BYTE_CAP, HEAP_ALLOWED>
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
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<
    T,
    LenTy: const ProvideLenTy,
    const BYTE_CAP: usize,
    const HEAP_ALLOWED: bool,
> DerefMut for ConciseVec<T, LenTy, BYTE_CAP, HEAP_ALLOWED>
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
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<
    T,
    LenTy: const ProvideLenTy,
    const BYTE_CAP: usize,
    const HEAP_ALLOWED: bool,
> Drop for ConciseVec<T, LenTy, BYTE_CAP, HEAP_ALLOWED>
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
    fn drop(&mut self) {
        if HEAP_ALLOWED {
            panic!("todo: check is_heap, provide drop impl for heap")
        } else {
            unsafe {
                ptr::drop_in_place(self.as_mut_slice());
            }
        }
    }
}

impl<
    's,
    T,
    LenTy: const ProvideLenTy,
    const BYTE_CAP: usize,
    const HEAP_ALLOWED: bool,
> IntoIterator for &'s ConciseVec<T, LenTy, BYTE_CAP, HEAP_ALLOWED>
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
    type Item = &'s T;
    type IntoIter = slice::Iter<'s, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<
    's,
    T,
    LenTy: const ProvideLenTy,
    const BYTE_CAP: usize,
    const HEAP_ALLOWED: bool,
> IntoIterator for &'s mut ConciseVec<T, LenTy, BYTE_CAP, HEAP_ALLOWED>
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
    type Item = &'s mut T;
    type IntoIter = slice::IterMut<'s, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
