use num_bigint::BigUint;
use std::io::Write;

fn main() {
    let (sender, receiver) = crossbeam_channel::bounded::<(u64, BigUint, BigUint, BigUint)>(1024);

    let mut threads = (0..num_cpus::get()).map(|thread| {
        let receiver = receiver.clone();

        let one = BigUint::new(vec![1]);
        let two = BigUint::new(vec![2]);

        std::thread::spawn(move || {
            loop {
                let (n, x, y, z) = receiver.recv().unwrap();

                let xx = &x * &x;
                let yy = &y * &y;
                let zz = &z * &z;

                let top_right = &xx + &yy * &zz;
                let sqrt = top_right.sqrt();
                if top_right == &sqrt * &sqrt { print_solution_and_exit(0, &top_right, &sqrt, n, &x, &y, &z); }

                let bot_left = &xx * &zz + &yy;
                let sqrt = bot_left.sqrt();
                if bot_left == &sqrt * &sqrt { print_solution_and_exit(1, &bot_left, &sqrt, n, &x, &y, &z); }

                let mid_mid = (xx + yy) * (zz + &one) / &two;
                let sqrt = mid_mid.sqrt();
                if mid_mid == &sqrt * &sqrt { print_solution_and_exit(2, &mid_mid, &sqrt, n, &x, &y, &z); }

                write_progress_file(n, thread);
            }
        })
    }).collect::<Vec<_>>();

    let mut x = BigUint::new(vec![3]);
    let mut y = BigUint::new(vec![1]);
    let mut z = BigUint::new(vec![1]);

    let two = BigUint::new(vec![2]);
    let three = BigUint::new(vec![3]);

    let start_n = lowest_unchecked_n();

    for n in 0.. {
        let x_next = &three * &x + &two * &z;
        let y_next = x.clone();
        let z_next = &x + &z;

        if n % 2 == 1 && n >= start_n {
            sender.send((n, x, y, z)).unwrap();
        }

        x = x_next;
        y = y_next;
        z = z_next;
    }

    for thread in threads.drain(..) {
        let _ = thread.join();
    }
}

fn print_solution_and_exit(position: u8, term: &BigUint, sqrt: &BigUint, n: u64, x: &BigUint, y: &BigUint, z: &BigUint) {
    println!("Found a square:\nterm{}={}\nsqrt={}\nn={}\nx={}\ny={}\nz={}", position, term, sqrt, n, x, y, z);

    std::io::stdout().flush().unwrap();
    std::process::exit(0);
}

fn read_progress_file(thread: usize) -> u64 {
    let filename = format!("progress-v3-thread-{}.txt", thread);

    if let Ok(s) = std::fs::read_to_string(filename) {
        s.parse().unwrap()
    } else {
        thread as u64
    }
}

fn write_progress_file(n: u64, thread: usize) {
    println!("Checked n={} on thread {}", n, thread);

    let filename = format!("progress-v3-thread-{}.txt", thread);
    std::fs::write(filename, format!("{}", n)).unwrap();
}

fn lowest_unchecked_n() -> u64 {
    let mut checked = (0..num_cpus::get()).map(|i| read_progress_file(i)).collect::<Vec<_>>();
    checked.sort();

    if checked.is_empty() { return 0; }

    for pairs in checked.windows(2) {
        let expected = pairs[0] + 2;
        let actual = pairs[1];

        if expected != actual {
            return expected;
        }
    }

    return checked.last().unwrap() + 1;
}
