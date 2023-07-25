
use std::any::Any;
use std::marker::PhantomData;


#[cfg(test)]
mod tests {

    use super::*;




    // #[test]
    // fn test_set_operation() {
    //     let mut matrix = AtomicFlagMatrix::new((5, 5));
    //     let set_operation = SetOperation;
    //     let strategy = MatrixStrategy::new(Box::new(set_operation));

    //     // Set a value in the matrix
    //     strategy.execute(&mut matrix, (2, 2), Some(true)).unwrap();

    //     // Check if the value was set correctly
    //     assert_eq!(matrix.get((2, 2)).unwrap(), true);
    // }

    // #[test]

    // fn test_get_operation() {
    //     let mut matrix = AtomicFlagMatrix::new((5, 5));
    //     let set_operation = SetOperation;
    //     let get_operation = GetOperation;
    //     let set_strategy = MatrixStrategy::new(Box::new(set_operation));
    //     let get_strategy = MatrixStrategy::new(Box::new(get_operation));

    //     // Set a value in the matrix
    //     set_strategy
    //         .execute(&mut matrix, (2, 2), Some(true))
    //         .unwrap();

    //     // Get the value from the matrix
    //     let value = get_strategy.execute(&mut matrix, (2, 2), None).unwrap();

    //     // Check if the value was retrieved correctly
    //     assert_eq!(value, true);
    // }

    // #[test]

    // fn test_toggle_operation() {
    //     let mut matrix = AtomicFlagMatrix::new((5, 5));
    //     let set_operation = SetOperation;
    //     let toggle_operation = ToggleOperation;
    //     let set_strategy = MatrixStrategy::new(Box::new(set_operation));
    //     let toggle_strategy = MatrixStrategy::new(Box::new(toggle_operation));

    //     // Set a value in the matrix
    //     set_strategy
    //         .execute(&mut matrix, (2, 2), Some(true))
    //         .unwrap();

    //     // Toggle the value in the matrix
    //     toggle_strategy.execute(&mut matrix, (2, 2), None).unwrap();

    //     // Check if the value was toggled correctly
    //     assert_eq!(matrix.get((2, 2)).unwrap(), false);
    // }

    // #[test]
    // fn test_view_operation() {
    //     let mut matrix = AtomicFlagMatrix::new((5, 5));   
    //     let set_operation = SetOperation;
    //     let view_operation = ViewOperation::new(1, 1, 3, 3);
    //     let set_strategy = MatrixStrategy::new(Box::new(set_operation));
    //     let view_strategy = MatrixStrategy::new(Box::new(view_operation));

    //     // Set a value in the matrix
    //     set_strategy
    //         .execute(&mut matrix, (2, 2), Some(true))
    //         .unwrap();

    //     // Get the view from the matrix
    //     let view = view_strategy.execute(&mut matrix, (0, 0), None).unwrap();

    //     // Check if the view was retrieved correctly
    //     assert_eq!(
    //         view,
    //         vec![
    //             vec![false, false, false],
    //             vec![false, true, false],
    //             vec![false, false, false]
    //         ]
    //     );
    // }
  

    // #[test]
    // fn test_bitwise_and_operation() {
    //     let matrix = AtomicFlagMatrix::new((2, 2));
    //     let strategy = BitwiseAndOperation;
    //     let context = MatrixContext {
    //         matrix: &matrix,
    //         index: (0, 0),
    //         other: Some(true),
    //     };

    //     strategy.execute(&context).unwrap();
    //     assert_eq!(matrix.get((0, 0)).unwrap(), false);
    // }

    // #[test]
    // fn test_bitwise_or_operation() {
    //     let matrix = AtomicFlagMatrix::new((2, 2));
    //     let strategy = BitwiseOrOperation;
    //     let context = MatrixContext {
    //         matrix: &matrix,
    //         index: (0, 0),
    //         other: Some(true),
    //     };

    //     strategy.execute(&context).unwrap();
    //     assert_eq!(matrix.get((0, 0)).unwrap(), true);
    // }

    // #[test]
    // fn test_bitwise_xor_operation() {
    //     let matrix = AtomicFlagMatrix::new((2, 2));
    //     let strategy = BitwiseXorOperation;
    //     let context = MatrixContext {
    //         matrix: &matrix,
    //         index: (0, 0),
    //         other: Some(true),
    //     };

    //     strategy.execute(&context).unwrap();
    //     assert_eq!(matrix.get((0, 0)).unwrap(), true);
    // }

    // #[test]

    // fn test_bitwise_not_operation() {
    //     let matrix = AtomicFlagMatrix::new((2, 2));
    //     let strategy = BitwiseNotOperation;
    //     let context = MatrixContext {
    //         matrix: &matrix,
    //         index: (0, 0),
    //         other: None,
    //     };

    //     strategy.execute(&context).unwrap();
    //     assert_eq!(matrix.get((0, 0)).unwrap(), true);
    // }
}
