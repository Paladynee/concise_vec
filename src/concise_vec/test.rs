#[test]
fn concise_vec() {
    use core::ops::Deref;
    use core::ops::DerefMut;

    use super::ConciseVec;

    #[repr(align(64))]
    struct CacheAlign<T>(T);

    impl<T> Deref for CacheAlign<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> DerefMut for CacheAlign<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    let mut cache_vec = CacheAlign(ConciseVec::<u32, u8, 64, false>::new());
    eprintln!(
        "Len: {}, Capacity: {}",
        cache_vec.len(),
        cache_vec.capacity()
    );
    let mut i = 0;
    while let Ok(x) = cache_vec.push(i) {
        println!("Pushed: {x}");
        i += 1;
    }

    eprintln!(
        "Len: {}, Capacity: {}",
        cache_vec.len(),
        cache_vec.capacity()
    );

    for item in cache_vec.iter() {
        println!("Item: {item}");
    }

    for item in cache_vec.iter_mut() {
        *item += 1;
    }

    // reverse
    cache_vec.sort_by(|a, b| b.cmp(a));

    for item in cache_vec.iter() {
        println!("Item: {item}");
    }
    // while let Some(x) = cache_vec.pop() {
    //     println!("Popped: {x}");
    // }
}
