use crate::ConciseVec;
use crate::aligner::AlignTyProvider;
use crate::heap_strategy_provider::StrategyProvider;
use crate::lenty_provide::ProvideLenTy;
use crate::stp::SizedTP;

pub trait PushProvider<
    T,
    LenTy: const ProvideLenTy,
    const BYTE_CAP: usize,
    const HEAP: bool,
> where
    // this is proof that the Aligner can work for all bools.
    (): const AlignTyProvider<HEAP>,
    // this is proof that the StrategyProvider can work for all bools.
    (): const StrategyProvider<LenTy, HEAP>,
    // this is proof that the PushProvider can work for all bools.
    (): PushProvider<T, LenTy, BYTE_CAP, HEAP>,
    // this is proof that BYTE_CAP does not exceed a usize (??? rust???)
    [(); BYTE_CAP / <*const ()>::SIZE * <*const ()>::SIZE]:,
{
    type PushResult<'lt, U>
    where
        U: 'lt,
        // todo: these bounds are currently required to ensure that impls have
        // maximum flexibility. Rust is soliciting feedback, see issue #87479
        // <https://github.com/rust-lang/rust/issues/87479> for more information
        LenTy: 'lt,
        T: 'lt;

    fn push<'lt>(
        cv: &'lt mut ConciseVec<T, LenTy, BYTE_CAP, HEAP>, val: T,
    ) -> Self::PushResult<'lt, T>;
}

impl<T, LenTy: const ProvideLenTy, const BYTE_CAP: usize>
    PushProvider<T, LenTy, BYTE_CAP, false> for ()
where
    // this is proof that the Aligner can work for false.
    (): const AlignTyProvider<false>,
    // this is proof that the StrategyProvider can work for false.
    (): const StrategyProvider<LenTy, false>,
    // this is proof that BYTE_CAP does not exceed a usize (??? rust???)
    [(); BYTE_CAP / <*const ()>::SIZE * <*const ()>::SIZE]:,
{
    type PushResult<'lt, U>
        = Result<&'lt mut U, U>
    where
        U: 'lt,
        // todo: these bounds are currently required to ensure that impls have
        // maximum flexibility. Rust is soliciting feedback, see issue #87479
        // <https://github.com/rust-lang/rust/issues/87479> for more information
        LenTy: 'lt,
        T: 'lt;

    /// Attempts storing the value. On failure, returns an `Err(T)`.
    ///
    /// Returns `Ok(&mut T)` to the pushed value on success.
    fn push<'lt>(
        cv: &'lt mut ConciseVec<T, LenTy, BYTE_CAP, false>, val: T,
    ) -> Self::PushResult<'lt, T> {
        let temp_ref = &*cv;
        let len = temp_ref.len();
        if temp_ref.is_next_write_within_inline_bounds()
            && let new_len = match len.checked_add(LenTy::ONE) {
                Some(new_len) => new_len,
                None => return Err(val),
            }
        {
            // this lossy conversion is safe because the next write is within
            // bounds, which accounts for that.
            let len = len.to_usize_lossy();

            // explicit reborrow fence to help the optimizer realize that
            // is_next_write_within_bounds and get_len and so on does not write
            // to the len field.
            let cv = &mut *cv;

            unsafe { cv.get_len_field_mut().set_len(new_len) };

            let ptr = unsafe {
                cv.data.get_mut_ptr::<T>(
                    len * ConciseVec::<T, LenTy, BYTE_CAP, false>::T_STRIDE,
                )
            };

            unsafe {
                ptr.write(val);
            }

            Ok(unsafe { &mut *ptr })
        } else {
            Err(val)
        }
    }
}
