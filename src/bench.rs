use crate::*;

use test::Bencher;

// #[cfg(test)]
// mod bench {
//     #[cfg(test)]

//     use super::*;
//     extern crate test;

//     use test::Bencher;

//     #[bench]
//     fn bench_get_set(b: &mut Bencher) {
//         let matrix = AtomicFlagMatrix::new((1000, 1000));

//         let set_operation = SetOperation::execute(&self, &mut matrix, index, other) {
//             let mut matrix = AtomicFlagMatrix::new((5, 5));
//             let set_operation = SetOperation;
//             let strategy = MatrixStrategy::new(Box::new(set_operation));
//             strategy.execute(&mut matrix, (2, 2), Some(true)).unwrap();
//             assert_eq!(matrix.get((2, 2)).unwrap(), true);
//         }
//         let get_operation = GetOperation { index: (0, 0) };
//         b.iter(|| {
//             for i in 0..1000 {
//                 for j in 0..1000 {
//                     set_operation.index = (i, j);
//                     get_operation.index = (i, j);
//                     matrix
//                         .handle_operation(Box::new(set_operation.clone()))
//                         .unwrap();
//                     AtomicFlagMatrix::new((1000, 1000))
//                         .handle_operation(Box::new(get_operation.clone()))
//                         .unwrap();
//                 }

//             }

//         });
//         b.bytes = 1000 * 1000 * 2;
//         b.iter(|| { });
//         b.bytes = 1000 * 1000 * 2;
//         b.iter(|| { });  
//     }  
// }
 