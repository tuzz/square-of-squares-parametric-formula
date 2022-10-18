use num_bigint::BigUint;
use std::io::Write;

fn main() {
    let num_threads = num_cpus::get();

    let mut threads = (1..=num_threads).map(|i| {
        std::thread::spawn(move || {
            let stride   = BigUint::new(vec![num_threads as u32]);
            let thread   = BigUint::new(vec![i as u32]);
            let interval = BigUint::new(vec![100_000_000]);

            let zero     = BigUint::new(vec![0]);
            let one      = BigUint::new(vec![1]);
            let two      = BigUint::new(vec![2]);
            let three    = BigUint::new(vec![3]);
            let four     = BigUint::new(vec![4]);
            let six      = BigUint::new(vec![6]);

            let mut n = read_progress_file() + thread;
            if i == 1 { println!("Starting from n={}", n); }

            loop {
                let w = &six * &n * &n + &six * &n + &two;
                let x = &two * &n + &one;
                let y = &three * &n * &n + &two * &n;
                let z = &three * &n * &n + &four * &n + &one;

                let xx = &x * &x;
                let yy = &y * &y;
                let zz = &z * &z;
                let ww = &w * &w;
                let wy = &w * &y;
                let xz = &x * &z;

                let xxyy = &xx * &yy;
                let wwzz = &ww * &zz;

                let common_term = &two * (&xxyy + &wwzz);
                let wy_plus_xz = &wy + &xz;
                let wy_minus_xz = &wy - &xz;

                let term1 = &common_term - &wy_plus_xz * &wy_plus_xz;
                let term2 = &common_term - &wy_minus_xz * &wy_minus_xz;
                let term3 = (&two * &yy - &zz) * &xx + (&two * &zz - &yy) * &ww;

                let sqrt1 = term1.sqrt();
                let sqrt2 = term2.sqrt();
                let sqrt3 = term3.sqrt();

                if &sqrt1 * &sqrt1 == term1 { print_solution_and_exit(1, &term1, &sqrt1, &n, &w, &x, &y, &z); }
                if &sqrt2 * &sqrt2 == term2 { print_solution_and_exit(2, &term2, &sqrt2, &n, &w, &x, &y, &z); }
                if &sqrt3 * &sqrt3 == term3 { print_solution_and_exit(3, &term3, &sqrt3, &n, &w, &x, &y, &z); }

                if &n % &interval == zero { write_progress_file(&n); }

                n += &stride;
            }
        })
    }).collect::<Vec<_>>();

    for thread in threads.drain(..) {
        thread.join().unwrap();
    }
}

fn print_solution_and_exit(position: u8, term: &BigUint, sqrt: &BigUint, n: &BigUint, w: &BigUint, x: &BigUint, y: &BigUint, z: &BigUint) {
    println!("Found a square:\nterm{}={}\nsqrt={}\nn={}\nw={}\nx={}\ny={}\nz={}", position, term, sqrt, n, w, x, y, z);

    std::io::stdout().flush().unwrap();
    std::process::exit(0);
}

fn read_progress_file() -> BigUint {
    let contents = std::fs::read_to_string("progress.txt").unwrap_or("1".to_string());

    contents.parse::<BigUint>().unwrap()
}

fn write_progress_file(n: &BigUint) {
    println!("Checked up to n={}", n);

    std::fs::write("progress.txt", format!("{}", n)).unwrap();
}
