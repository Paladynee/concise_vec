use crate::ConciseVec;
use crate::aligner::AlignTyProvider;
use crate::concise_vec::push_provider::PushProvider;
use crate::heap_strategy_provider::StrategyProvider;
use crate::lenfield::LenField;
use crate::lenty_provide::ProvideLenTy;
use crate::stp::SizedTP;

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
    pub(crate) const LENFIELD_OFFSET: usize =
        { ((BYTE_CAP / LenTy::ALIGN) * LenTy::ALIGN) - LenTy::ALIGN };

    pub(crate) const T_STRIDE: usize = T::SIZE.max(T::ALIGN);

    pub(crate) const REMAINING_STORAGE: usize = Self::LENFIELD_OFFSET;
    pub(crate) const T_STORAGE_START_OFFSET: usize = 0;
    pub(crate) const T_BYTE_CAP: usize = { Self::CAPACITY * Self::T_STRIDE };

    #[inline]
    pub(crate) const fn get_len_field_mut(
        &mut self,
    ) -> &mut LenField<LenTy, HEAP_ALLOWED> {
        unsafe {
            &mut *self.data.get_mut_ptr::<LenField<LenTy, HEAP_ALLOWED>>(
                Self::LENFIELD_OFFSET,
            )
        }
    }

    #[inline]
    pub(crate) const fn get_len_field(&self) -> &LenField<LenTy, HEAP_ALLOWED> {
        unsafe {
            &*self
                .data
                .get_ptr::<LenField<LenTy, HEAP_ALLOWED>>(Self::LENFIELD_OFFSET)
        }
    }

    #[inline]
    pub(crate) const fn is_next_write_within_inline_bounds(&self) -> bool {
        if HEAP_ALLOWED {
            panic!(
                "todo: check is_heap, provide \
                is_next_write_within_inline_bounds"
            )
        } else {
            let next_offset = self.get_len_field().get_len();

            // todo: investigate unchecked mul opportunity here, if previous
            // write succeeded we can unchecked multiply by T's stride since
            // memory constraints force our lossy conversion to be smaller than
            // isize::MAX. might not be correct since lossy conversion to usize
            // can truncate a big length number such as u64(fixed) -> u32(usize
            // on 32 bit) to make it appear any random u32 value.
            let alien = next_offset.to_usize_lossy() * T::STRIDE;
            if alien >= Self::T_BYTE_CAP {
                return false;
            }

            if alien > isize::MAX as usize {
                return false;
            }

            // this case is only relevant on circumstances where the size of
            // LenTy is actually greater than a usize. while that data structure
            // is not necessarily useful, we have to check for it.
            let homeland = LenTy::from_usize_lossy(isize::MAX as usize);
            if next_offset > homeland {
                return false;
            }

            // homeland and T_BYTE_CAP comparison is handled statically as a
            // well-formedness check as both values are known at compile time
            // known.

            true
        }
    }
}
