mod bench {

    use super::*;
    use crate::*;
    extern crate test;
    use test::Bencher;

    #[cfg(test)]

    mod bench {

        use super::*;

        use test::Bencher;

        // #[bench]

        // fn bench_get_set(b: &mut Bencher) {
        //     let matrix = AtomicFlagMatrix::new((1000, 1000));

        //     let set_operation = SetOperation {
        //         index: (0, 0),
        //         value: true,
        //     };

        //     let get_operation = GetOperation { index: (0, 0) };
        //     b.iter(|| {
        //         for i in 0..1000 {
        //             for j in 0..1000 {
        //                 set_operation.index = (i, j);
        //                 get_operation.index = (i, j);
        //                 matrix
        //                     .execute_operation(Box::new(set_operation.clone()))
        //                     .unwrap();

        //                 matrix
        //                     .handle_operation(Box::new(get_operation.clone()))
        //                     .unwrap();
        //             }
        //         }
        //     });
        // }

        // // Add more benchmarks for other operations here...
    }
}

// #[bench]
// fn bench_bitwise_and_operation(b: &mut Bencher) {
//     let matrix = AtomicFlagMatrix::new((1000, 1000));

//     b.iter(|| {
//         for i in 0..1000 {
//             for j in 0..1000 {
//                 matrix.bitwise_and_element((i, j), false).unwrap();
//             }
//         }
//     });
// }

// #[bench]
// fn bench_bitwise_or_operation(b: &mut Bencher) {
//     let matrix = AtomicFlagMatrix::new((1000, 1000));

//     b.iter(|| {
//         for i in 0..1000 {
//             for j in 0..1000 {
//                 matrix.bitwise_or_element((i, j), true).unwrap();
//             }
//         }
//     });
// }

// #[bench]
// fn bench_bitwise_xor_operation(b: &mut Bencher) {
//     let matrix = AtomicFlagMatrix::new((1000, 1000));

//     b.iter(|| {
//         for i in 0..1000 {
//             for j in 0..1000 {
//                 matrix.bitwise_xor_element((i, j), true).unwrap();
//             }
//         }
//     });
// }

// #[bench]
// fn bench_bitwise_not_operation(b: &mut Bencher) {
//     let matrix = AtomicFlagMatrix::new((1000, 1000));

//     b.iter(|| {
//         for i in 0..1000 {
//             for j in 0..1000 {
//                 matrix.bitwise_not_element((i, j)).unwrap();
//             }
//         }
//     });
// }

// use criterion::{black_box, criterion_group, criterion_main, Criterion};

// #[cfg(test)]
// mod benches {

//     use super::*;
//     use crate::*;

//     // use test::Bencher;

//     fn bench_bitwise_and_operation(b: &mut Criterion) {
//         let matrix = AtomicFlagMatrix::new((1000, 1000));
//         b.iter(|| {
//             for i in 0..1000 {
//                 for j in 0..1000 {
//                     matrix.bitwise_and_element((i, j), false).unwrap();
//                 }
//             }
//         });
//     }

//     fn bench_bitwise_or_operation(b: &mut Criterion) {
//         let matrix = AtomicFlagMatrix::new((1000, 1000));

//         b.iter(|| {
//             for i in 0..1000 {
//                 for j in 0..1000 {
//                     matrix.bitwise_or_element((i, j), true).unwrap();
//                 }
//             }
//         });
//     }

//     fn bench_bitwise_xor_operation(b: &mut Criterion) {
//         let matrix = AtomicFlagMatrix::new((1000, 1000));
//         b.iter(|| {
//             for i in 0..1000 {
//                 for j in 0..1000 {
//                     matrix.bitwise_xor_element((i, j), true).unwrap();
//                 }
//             }
//         });
//     }

//     fn bench_bitwise_not_operation(b: &mut Bencher) {
//         let matrix = AtomicFlagMatrix::new((1000, 1000));
//         b.iter(|| {
//             for i in 0..1000 {
//                 for j in 0..1000 {
//                     matrix.bitwise_not_element((i, j)).unwrap();
//                 }
//             }
//         });
//     }
// }
// fn criterion_benchmark(c: &mut Criterion) {
//     // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
//     c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
// }

// #[cfg(test)]
// mod bench {

//     use super::*;
//     use crate::*;

//     fn bench_bitwise_operations(b: &mut Bencher) {
//         let matrix = AtomicFlagMatrix::new((1000, 1000));

//         b.iter(|| {
//             for i in 0..1000 {
//                 for j in 0..1000 {
//                     matrix.set((i, j), true).unwrap();
//                     matrix.bitwise_and_element((i, j), false).unwrap();
//                     matrix.bitwise_or_element((i, j), true).unwrap();
//                     matrix.bitwise_xor_element((i, j), true).unwrap();
//                     matrix.bitwise_not_element((i, j)).unwrap();
//                 }
//             }
//         });
//     }
// }
// criterion_group!(benches, bench_function);
// criterion_main!(benches);
