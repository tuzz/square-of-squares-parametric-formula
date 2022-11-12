use num_bigint::BigUint;
use std::io::Write;

fn main() {
    let num_threads = num_cpus::get();

    let mut threads = (0..num_threads).map(|i| {
        std::thread::spawn(move || {
            let stride = BigUint::new(vec![num_threads as u32]);
            let thread   = BigUint::new(vec![i as u32]);
            let interval = BigUint::new(vec![100_000_000]);

            let zero = BigUint::new(vec![0]);
            let one = BigUint::new(vec![1]);
            let two = BigUint::new(vec![2]);
            let four = BigUint::new(vec![4]);
            let eight = BigUint::new(vec![8]);
            let twelve = BigUint::new(vec![12]);
            let thirteen = BigUint::new(vec![13]);
            let twenty_four = BigUint::new(vec![24]);
            let twenty_six = BigUint::new(vec![26]);
            let thirty_eight = BigUint::new(vec![38]);
            let forty_nine = BigUint::new(vec![49]);
            let fifty_one = BigUint::new(vec![51]);
            let one_o_one = BigUint::new(vec![101]);
            let one_o_two = BigUint::new(vec![102]);
            let one_fifty_one = BigUint::new(vec![151]);

            let mut z = read_progress_file() + &thread + &one;
            println!("Starting from {} on thread {}", &z, thread);

            loop {
                'block: {
                    let z4 = &z * &four;
                    let z8 = &z * &eight;
                    let zz = &z * &z;
                    let zz2 = &two * &zz;
                    let zz2_p1 = &zz2 + &one;
                    let zz2_p1_sq = &zz2_p1 * &zz2_p1;
                    let zz2_p1_sq_m49 = &forty_nine * &zz2_p1_sq;

                    let mut squares = 0;

                    let b = &zz2_p1_sq_m49 - &z4 * (&zz2_p1 + &one_o_one * &z);
                    let sqrt = b.sqrt();
                    if &sqrt * &sqrt == b { squares += 1; }

                    let c = &zz2_p1_sq + &z8 * (&twenty_six * &zz + &thirty_eight * &z + &thirteen);
                    let sqrt = c.sqrt();
                    if &sqrt * &sqrt == c { squares += 1; }

                    let g = &zz2_p1_sq_m49 + &z8 * (&twenty_four * &zz - &thirteen * &z + &twelve);
                    let sqrt = g.sqrt();
                    if &sqrt * &sqrt == g { squares += 1; }

                    if squares == 0 { break 'block; }

                    let h = &zz2_p1_sq + &z4 * (&one_o_two * &zz + &one_fifty_one * &z + &fifty_one);
                    let sqrt = h.sqrt();
                    if &sqrt * &sqrt == h { squares += 1; }

                    if squares == 1 { break 'block; }

                    println!("Found a square: z={}, b={}, c={}, g={}, h={}", z, b, c, g, h);
                    std::io::stdout().flush().unwrap();
                    std::process::exit(0);
                }

                if &z % &interval == zero {
                    write_progress_file(&z);
                }

                z += &stride;
            }
        })
    }).collect::<Vec<_>>();

    for thread in threads.drain(..) {
        thread.join().unwrap();
    }
}

fn read_progress_file() -> BigUint {
    if let Ok(s) = std::fs::read_to_string("progress-v3.txt") {
        s.parse().unwrap()
    } else {
        BigUint::new(vec![1])
    }
}

fn write_progress_file(z: &BigUint) {
    println!("Checked up to {}", z);

    std::fs::write("progress-v3.txt", format!("{}", z)).unwrap();
}
