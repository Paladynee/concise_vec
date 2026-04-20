use core::alloc::Layout;
use core::mem;

pub trait SizedTP {
    const SIZE: usize;
    const ALIGN: usize;
    const STRIDE: usize;
    const IS_ZST: bool;
    const LAYOUT: Layout;
}
impl<T> SizedTP for T {
    const ALIGN: usize = mem::align_of::<T>();
    const IS_ZST: bool = Self::SIZE == 0;
    const LAYOUT: Layout =
        match Layout::from_size_align(Self::SIZE, Self::ALIGN) {
            Ok(layout) => layout,
            Err(_) => panic!(),
        };
    const SIZE: usize = mem::size_of::<T>();
    const STRIDE: usize = Self::SIZE.max(Self::ALIGN);
}
