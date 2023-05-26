#[cfg(test)]
mod tests {

    use super::*;
    use crate::*;

    #[cfg(test)]

    mod tests {

        use super::*;

        #[test]

        fn test_get_set() {
            let matrix = AtomicFlagMatrix::new((2, 2));

            let set_operation = SetOperation {
                index: (0, 0),
                value: true,
            };

            let get_operation = GetOperation { index: (0, 0) };

            assert_eq!(
                matrix.handle_operation(Box::new(get_operation)),
                Ok(OperationResult::Get(false))
            );

            assert_eq!(
                matrix.handle_operation(Box::new(set_operation)),
                Ok(OperationResult::Set)
            );

            assert_eq!(
                matrix.handle_operation(Box::new(get_operation)),
                Ok(OperationResult::Get(true))
            );
        }

        // #[test]

        // fn test_out_of_bounds() {
        //     let matrix = AtomicFlagMatrix::new((2, 2));
        //     let set_operation = SetOperation {Box::new()
        //         index: (2, 2),
        //         value: true,
        //     };
        //     let get_operation = GetOperation { index: (2, 2) };

        //     assert_eq!(
        //         matrix.handle_operation(Box::new(get_operation)),
        //         Err(AtomicFlagMatrixError::IndexOutOfBounds)
        //     );
        //     assert_eq!(
        //         matrix.handle_operation(Box::new(set_operation)),
        //         Err(AtomicFlagMatrixError::IndexOutOfBounds)
        //     );
        // }
    }

    // #[test]
    // fn test_get_set() {
    //     let matrix = AtomicFlagMatrix::new((2, 2));
    //     assert_eq!(matrix.get((0, 0)), Ok(false));
    //     assert_eq!(matrix..set((0, 0), true), Ok(()));
    //     assert_eq!(matrix.get((0, 0)), Ok(true));
    // }

    // #[test]
    // fn test_out_of_bounds() {
    //     let matrix = AtomicFlagMatrix::new((2, 2));
    //     assert_eq!(
    //         matrix.get((2, 2)),
    //         Err(AtomicFlagMatrixError::IndexOutOfBounds)
    //     );

    //     assert_eq!(
    //         matrix.set((2, 2), true),
    //         Err(AtomicFlagMatrixError::IndexOutOfBounds)
    //     );
    // }

    // #[test]
    // fn test_updates() {
    //     let matrix = AtomicFlagMatrix::new((2, 2));
    //     matrix
    //         .queue_update(Box::new(|data| {
    //             data[(0, 0)].store(true);
    //         }))
    //         .unwrap();

    //     assert_eq!(matrix.get((0, 0)), Ok(false));
    //     matrix.apply_next_update().unwrap();
    //     assert_eq!(matrix.get((0, 0)), Ok(true));
    // }

    // #[test]
    // fn test_bitwise_operations() {
    //     let matrix = AtomicFlagMatrix::new((2, 2));

    //     // Set a value to true

    //     assert_eq!(matrix.set((0, 0), true), Ok(()));

    //     // Bitwise AND with false should result in false

    //     assert_eq!(matrix.bitwise_and_element((0, 0), false), Ok(()));

    //     assert_eq!(matrix.get((0, 0)), Ok(false));

    //     // Bitwise OR with true should result in true

    //     assert_eq!(matrix.bitwise_or_element((0, 0), true), Ok(()));

    //     assert_eq!(matrix.get((0, 0)), Ok(true));

    //     // Bitwise XOR with true should result in false

    //     assert_eq!(matrix.bitwise_xor_element((0, 0), true), Ok(()));

    //     assert_eq!(matrix.get((0, 0)), Ok(false));

    //     // Bitwise NOT should result in true

    //     assert_eq!(matrix.bitwise_not_element((0, 0)), Ok(()));

    //     assert_eq!(matrix.get((0, 0)), Ok(true));
    // }
}
