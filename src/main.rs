use reikna::factor::perfect_square;
use std::io::Write;

fn main() {
    let num_threads = num_cpus::get();

    let mut threads = (0..num_threads).map(|thread| {
        std::thread::spawn(move || {
            let mut q = read_progress_file(thread) + 1;
            println!("Starting from {} choose 4 on thread {}", q, thread);

            loop {
                let qq = q * q;
                let q4 = 4 * q;

                for r in 1..q {
                    let rr = r * r;
                    let qqrr = qq * rr;
                    let qr4 = q4 * r;
                    let qqrr3 = 3 * qqrr;

                    for p in 1..r {
                        let pp = p * p;
                        let pprr = pp * rr;
                        let pqr4 = qr4 * p;
                        let pprr3 = 3 * pprr;

                        for s in 1..p {
                            let ss = s * s;
                            let ppss = pp * ss;
                            let qqss = qq * ss;
                            let pqrs4 = pqr4 * s;
                            let ppss3 = 3 * ppss;
                            let qqss3 = 3 * qqss;

                            let mut squares = 0;

                            // [(qr)² + (ps)² + (pr)² + (qs)²]/2
                            let mid_mid = (qqrr + ppss + pprr + qqss) / 2;

                            // > There are no more magic squares which use 7 or more squares which have a
                            // > non-square central cell up to 10^14, or a square central cell up to 10^28.
                            //
                            // https://benchaffin.com/magic-squares/magic-squares.html#results
                            //
                            // For this term to exceed 10^28, the variables would need to be > 8,408,964
                            // which would require checking ~2*10^26 squares which isn't feasible.
                            if perfect_square(mid_mid) { continue; }

                            // [(qr)² + (ps)² + (pr)² + (qs)²]/2 - 4pqrs
                            if pqrs4 > mid_mid { continue; } // Skip negatives.
                            let bot_mid = mid_mid - pqrs4;
                            if perfect_square(bot_mid) { squares += 1; }

                            // [3(pr)² + 3(qs)² - (qr)² - (ps)²]/2
                            let positive = pprr3 + qqss3;
                            let negative = qqrr + ppss;
                            if negative > positive { continue; } // Skip negatives.
                            let mid_left = (positive - negative) / 2;
                            if mid_left == bot_mid { continue; } // Skip duplicates.
                            if perfect_square(mid_left) { squares += 1; }

                            if squares == 0 { continue; } // Not enough squares.

                            // [3(qr)² + 3(ps)² - (pr)² - (qs)²]/2
                            let positive = qqrr3 + ppss3;
                            let negative = pprr + qqss;
                            if negative > positive { continue; } // Skip negatives.
                            let mid_right = (positive - negative) / 2;
                            if perfect_square(mid_right) { squares += 1; }

                            if squares == 1 { continue; } // Not enough squares.

                            // [(qr)² + (ps)² + (pr)² + (qs)²]/2 + 4pqrs
                            let top_mid = mid_mid + pqrs4;
                            if top_mid == mid_right { continue; } // Skip duplicates.
                            if perfect_square(top_mid) { squares += 1; }

                            if squares == 2 { continue; } // Not enough squares.

                            println!("\nFound a square:\np={}\nq={}\nr={}\ns={}\ntop_mid={}\nmid_left={}\nmid_mid={}\nmid_right={}\nbot_mid={}\n", p, q, r, s, top_mid, mid_left, mid_mid, mid_right, bot_mid);

                            std::io::stdout().flush().unwrap();
                            std::process::exit(0);
                        }
                    }
                }

                write_progress_file(q, thread);

                q += num_threads as u64;
            }
        })
    }).collect::<Vec<_>>();

    for thread in threads.drain(..) {
        thread.join().unwrap();
    }
}

fn read_progress_file(thread: usize) -> u64 {
    let filename = format!("progress-thread-{}.txt", thread);

    if let Ok(s) = std::fs::read_to_string(filename) {
        s.parse().unwrap()
    } else {
        thread as u64
    }
}

fn write_progress_file(q: u64, thread: usize) {
    println!("Checked {} choose 4 on thread {}", q, thread);

    let filename = format!("progress-thread-{}.txt", thread);
    std::fs::write(filename, format!("{}", q)).unwrap();
}
