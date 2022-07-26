use num_bigint::BigUint;
use std::io::Write;

fn main() {
    let num_threads = num_cpus::get();

    // Comment in this code to continue checking the leftmost term:
    // 2(x²y² + w²z²) - (wy + xz)²
    //
    // let mut threads = (1..=num_threads).map(|start_number| {
    //     std::thread::spawn(move || {
    //         let zero    = BigUint::new(vec![0]);
    //         let one     = BigUint::new(vec![1]);
    //         let two     = BigUint::new(vec![2]);
    //         let three   = BigUint::new(vec![3]);
    //         let four    = BigUint::new(vec![4]);
    //         let six     = BigUint::new(vec![6]);
    //         let million = BigUint::new(vec![1_000_000]);

    //         let mut n = BigUint::new(vec![start_number as u32]);
    //         let increment = BigUint::new(vec![num_threads as u32]);

    //         // Skip numbers that have already been checked.
    //         n += BigUint::new(vec![100_000]) * BigUint::new(vec![100_000_000]);

    //         loop {
    //             let w = &six * &n * &n + &six * &n + &two;
    //             let x = &two * &n + &one;
    //             let y = &three * &n * &n + &two * &n;
    //             let z = &three * &n * &n + &four * &n + &one;

    //             let term1 = &x * &x * &y * &y;
    //             let term2 = &w * &w * &z * &z;
    //             let term3 = &two * (&term1 + &term2);

    //             let term4 = &w * &y + &x * &z;
    //             let term5 = &term4 * &term4;

    //             // Calculate 2(x²y² + w²z²) - (wy + xz)².
    //             let partial = &term3 - &term5;
    //             let sqrt = partial.sqrt();

    //             if &sqrt * &sqrt == partial {
    //                 println!(">>>>>>>>>>> {}, {}", n, partial);
    //                 break;
    //             }

    //             if &n % &million == zero {
    //                 println!("{}", &n / &million);
    //                 std::io::stdout().flush().unwrap();
    //             }

    //             n += &increment;
    //         }
    //     })
    // }).collect::<Vec<_>>();

    // let mut threads = (1..=num_threads).map(|start_number| {
    //     std::thread::spawn(move || {
    //         let zero    = BigUint::new(vec![0]);
    //         let one     = BigUint::new(vec![1]);
    //         let two     = BigUint::new(vec![2]);
    //         let three   = BigUint::new(vec![3]);
    //         let four    = BigUint::new(vec![4]);
    //         let six     = BigUint::new(vec![6]);
    //         let million = BigUint::new(vec![1_000_000]);

    //         let mut n = BigUint::new(vec![start_number as u32]);
    //         let increment = BigUint::new(vec![num_threads as u32]);

    //         // Skip numbers that have already been checked.
    //         n += BigUint::new(vec![100_000]) * BigUint::new(vec![100_000_000]);

    //         loop {
    //             let w = &six * &n * &n + &six * &n + &two;
    //             let x = &two * &n + &one;
    //             let y = &three * &n * &n + &two * &n;
    //             let z = &three * &n * &n + &four * &n + &one;

    //             let term1 = &x * &x * &y * &y;
    //             let term2 = &w * &w * &z * &z;
    //             let term3 = &two * (&term1 + &term2);

    //             let term4 = &w * &y - &x * &z;
    //             let term5 = &term4 * &term4;

    //             // Calculate 2(x²y² + w²z²) - (wy - xz)².
    //             let partial = &term3 - &term5;
    //             let sqrt = partial.sqrt();

    //             if &sqrt * &sqrt == partial {
    //                 println!(">>>>>>>>>>> {}, {}", n, partial);
    //                 break;
    //             }

    //             if &n % &million == zero {
    //                 println!("{}", &n / &million);
    //                 std::io::stdout().flush().unwrap();
    //             }

    //             n += &increment;
    //         }
    //     })
    // }).collect::<Vec<_>>();

    let mut threads = (1..=num_threads).map(|start_number| {
        std::thread::spawn(move || {
            let zero    = BigUint::new(vec![0]);
            let one     = BigUint::new(vec![1]);
            let two     = BigUint::new(vec![2]);
            let three   = BigUint::new(vec![3]);
            let four    = BigUint::new(vec![4]);
            let six     = BigUint::new(vec![6]);
            let million = BigUint::new(vec![1_000_000]);

            let mut n = BigUint::new(vec![start_number as u32]);
            let increment = BigUint::new(vec![num_threads as u32]);

            // Skip numbers that have already been checked.
            n += BigUint::new(vec![100_000]) * BigUint::new(vec![100_000]);

            loop {
                let w = &six * &n * &n + &six * &n + &two;
                let x = &two * &n + &one;
                let y = &three * &n * &n + &two * &n;
                let z = &three * &n * &n + &four * &n + &one;

                let y2 = &y * &y;
                let z2 = &z * &z;

                let term1 = (&two * &y2 - &z2) * &x * &x;
                let term2 = (&two * &z2 - &y2) * &w * &w;

                // Calculate (2y² - z²)x² + (2z² - y²)w²
                let partial = &term1 + &term2;
                let sqrt = partial.sqrt();

                if &sqrt * &sqrt == partial {
                    println!(">>>>>>>>>>> {}, {}", n, partial);
                    break;
                }

                if &n % &million == zero {
                    println!("{}", &n / &million);
                    std::io::stdout().flush().unwrap();
                }

                n += &increment;
            }
        })
    }).collect::<Vec<_>>();

    for thread in threads.drain(..) {
        thread.join().unwrap();
    }
}
