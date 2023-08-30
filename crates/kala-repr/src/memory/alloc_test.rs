#[cfg(test)]
mod tests {
    use crate::memory::alloc::Ref;

    #[test]
    fn alloc_test() {
        let value: u64 = 1234567890;
        let r = Ref::new(value);
        assert_eq!(*r, value)
    }

    #[derive(Clone, Copy)]
    struct LargeData {
        data: [u8; 1000]
    }

    #[test]
    fn alloc_many_data_test() {
        let mut refs: Vec<Ref<LargeData>> = Vec::with_capacity(10);
        for i in 0..=9 {
            refs.push(Ref::new(LargeData { data: [i;1000] }));
        }
        for (i, r) in refs.into_iter().enumerate() {
            assert_eq!(r.data[0], i as u8);
            assert_eq!(r.data[999], i as u8); 
        }
    }
}