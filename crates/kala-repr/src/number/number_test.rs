#[cfg(test)]
mod tests {
    use std::mem::transmute;

    use crate::number::NumberSlot;

    #[test]
    fn slot_test() {
        let test_cases: &[i128] = &[
            0,
            1,
            2,
            -1,
            -2,
            0x7FFF_FFFF_FFFF_FFFF,
            0x8000_0000_0000_0000,
        ];

        for test_case in test_cases {  
            let slot = NumberSlot::new(*test_case);
            let slot_value: i128 = slot.into();

            assert_eq!(slot_value, *test_case, "converting slot to i128 and back should be identity, but got {} instead of {}. ", slot_value, *test_case);
        }
    }
}