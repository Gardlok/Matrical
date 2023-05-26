use crate::*;
extern crate test;
use test::Bencher;

#[cfg(test)]
mod bench {

    use super::*;
    use crate::*;

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
/*

#[bench]
fn bench_bitwise_operations(b: &mut Bencher) {
    let matrix = AtomicFlagMatrix::new((1000, 1000));

    let set_operation = SetOperation {
        index: (0, 0),
        value: true,
    };

    let and_operation = AndOperation {
        index: (0, 0),
        other: true,
    };

    let or_operation = OrOperation {
        index: (0, 0),
        other: true,
    };

    let xor_operation = XorOperation {
        index: (0, 0),
        other: true,
    };

    let not_operation = NotOperation { index: (0, 0) };

    b.iter(|| {
        for i in 0..1000 {
            for j in 0..1000 {
                set_operation.index = (i, j);
                and_operation.index = (i, j);
                or_operation.index = (i, j);
                xor_operation.index = (i, j);
                not_operation.index = (i, j);

                matrix
                    .handle_operation(Box::new(set_operation.clone()))
                    .unwrap();

                matrix
                    .handle_operation(Box::new(and_operation.clone()))
                    .unwrap();

                matrix
                    .handle_operation(Box::new(or_operation.clone()))
                    .unwrap();

                matrix
                    .handle_operation(Box::new(xor_operation.clone()))
                    .unwrap();

                matrix
                    .handle_operation(Box::new(not_operation.clone()))
                    .unwrap();
            }
        }
    });
}
*/
