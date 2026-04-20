use core::mem::ManuallyDrop;

pub const trait AlignTyProvider<const HEAP: bool> {
    type AlignTy1;
    type AlignTy2;
}

impl const AlignTyProvider<true> for () {
    type AlignTy1 = usize;
    type AlignTy2 = *const ();
}
impl const AlignTyProvider<false> for () {
    type AlignTy1 = ();
    type AlignTy2 = ();
}

pub union Aligner<T, LenTy, const IS_HEAP_ALLOWED: bool>
where
    // this is proof that AlignTyProvider has
    // been implemented for all bools.
    (): const AlignTyProvider<IS_HEAP_ALLOWED>,
{
    _align1: [ManuallyDrop<T>; 0],
    _align2: [ManuallyDrop<LenTy>; 0],
    // this either has the alignment of a usize or 1 depending on whether
    // HEAP_ALLOWED is true or false.
    _align3:
        [ManuallyDrop<<() as AlignTyProvider<IS_HEAP_ALLOWED>>::AlignTy1>; 0],
    // this either has the alignment of a *const () or 1 depending on whether
    // HEAP_ALLOWED is true or false.
    _align4:
        [ManuallyDrop<<() as AlignTyProvider<IS_HEAP_ALLOWED>>::AlignTy2>; 0],
    _align5: [ManuallyDrop<()>; 0],
}

impl<T, LenTy, const IS_HEAP_ALLOWED: bool> Aligner<T, LenTy, IS_HEAP_ALLOWED>
where
    // this is proof that AlignTyProvider has been implemented for all bools.
    (): const AlignTyProvider<IS_HEAP_ALLOWED>,
{
    #[inline]
    pub const fn new() -> Self {
        Aligner {
            _align5: [],
        }
    }
}
