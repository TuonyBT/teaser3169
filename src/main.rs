fn main() {

    let primes = sieve_of_eratothsenes(100).into_iter()
    .filter(|&z| z > 20)
    .collect::<Vec<usize>>();

    for x in primes.clone() {
        for y in next_p(x){
            if primes.contains(&y) && (20.0 * (y % x) as f32 / y as f32) < 1.0 {
                    for z in next_p(y){
                    if primes.contains(&z) && (20.0 * (z % y) as f32 / z as f32) < 1.0 {
                        let c = z % y;
                        let b = y % x;
                        let a_mod = z / 10;
                        let num_mod_x = (z - c) * (y - b) / x / y;
                        if ((x - 1) * num_mod_x) / 10 == a_mod {
                            let a_max = x - a_mod * 10 / num_mod_x;
                            for a in 1..a_max + 1 {
                                if a * 20 > x {continue;}
                                println!("x: {}, y: {}, z: {}, c: {}, b: {}, a_max: {}", x, y, z, c, b, a_max);
                            }
                        }
                    }
                }
            }
        }
    }


}

//  Generate any primes (possible y) that have the correct relationship to a specific prime (x < y)

fn next_p(x: usize)-> Vec<usize> {

    let mut nexts = Vec::<usize>::new();
    let cap = 100/x + 1;
    for multiple in 1..cap {
        let poss = (1usize..5).map(|z| x * multiple + z).collect::<Vec<usize>>();
        nexts.extend(poss);

    }
    nexts
}

//  This sieve tests only odd numbers
pub fn sieve_of_eratothsenes(x: usize) -> Vec<usize> {
    let mut sieve = vec![true; (x + 1) / 2];
    sieve[0] = false;

    let mut lp: usize = 3;
    while lp <= (x as f64).sqrt().floor() as usize {
        let fnp = lp.pow(2);
        for idx in (fnp..x + 1).step_by(lp * 2) {
            sieve[(idx - 1) / 2] = false;
        }
        lp = match sieve[(lp - 1) / 2 + 1..].iter().position(|z| z == &true) {
            Some(y) => (y + (lp - 1) / 2 + 1) * 2 + 1,
            None => x,
        };
    }
    let mut primes = sieve.iter().enumerate().filter(|z| z.1 == &true).map(|z| z.0 * 2 + 1).collect::<Vec<usize>>();
    primes.insert(0, 2);
    primes

}