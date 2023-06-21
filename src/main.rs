fn main() {

//  Generate all two-figure prime values whose reciprocal is less than 1/20
    let primes = sieve_of_eratothsenes(100).into_iter()
    .filter(|&z| z > 20)
    .collect::<Vec<usize>>();



//  The three primes in the solution are x, y, and z (x < y < z)
//  The corresponding numerators for the three gain stages are a, b, and c
//  For an effective gain (EG) to intrinsic gain (IG) ratio that can be expressed as the ratio of two two-figure integers
//  EG / IG = (1 - x/a).(1 - y/b).(1 - z/c) = (x - a).(y - b).(z - c) / xyz
//  since all of x, y, z, (x - a), (y - b) and (z - c) are two-figure integers, 
//  two of the numerators must be multiples of two of the denominators
//  since EG / IG < 1, the remaining numerator must be less than the remaining denominator
//  therefore (z - c) is a multiple of y and (y - b) is a multiple of x

//  For each possible value of x, generate values of y and z that satisfy this relationship, 
//  allowing that b/y < 1/20 and c/z < 1/20

    for x in primes.clone() {
        for y in next_p(x){

            if primes.contains(&y) // filter out the primes
            && (20.0 * (y % x) as f32 / y as f32) < 1.0 { //and filter out cases where the loss is more than 1/20

                    for z in next_p(y){
                    if primes.contains(&z) && (20.0 * (z % y) as f32 / z as f32) < 1.0 {

//  we now have possible x, y, z sets, from which we can infer b and c directly
                        let c = z % y;
                        let b = y % x;

//  finding a and checking the set satisfies the full puzzle parameters is slightly more involved

                        let a_mod = z / 10; // gives us the first digit of the intrinsic gain
                        let num_mod_x = (z - c) * (y - b) / x / y;  // effective gain is this multiple of (x - a)
                        if x > a_mod * 10 / num_mod_x { // eliminate case where a is not a positive integer
                            let a_max = x - a_mod * 10 / num_mod_x; // limit range of possible values for a
                            for a in 1..a_max + 1 {
                                if a * 20 > x {continue;} // remove cases where a / x > 1 / 20
                                if ((x - a) * num_mod_x) / 10 != a_mod {continue} // only take acceptable values for the first digit of effective gain

                                println!("The fractions in ascending order are:");
                                let mut result = vec![(x, a), (y, b), (z, c)].into_iter()
                                .map(|(n, d)| ((n / d, n % d), (n, d)))
                                .collect::<Vec<((usize, usize), (usize, usize))>>();
                                result.sort();
                                result.reverse();
                                for r in result {
                                    println!("{} / {}", r.1.1, r.1.0);
                                }
                            }
                        }
                    }
                }
            }
        }
    }


}

//  Generate any values (possible y) that have the correct relationship to a specific prime (x < y)
//  i.e. (y - b) = multiple * x
//  multiple is limited by y < 100
//  b is limited by b/y < 1/20 and y < 100, i.e. b < 5
//  for convenience, we pass back all possible values, and filter out primes in the main routine

fn next_p(x: usize)-> Vec<usize> {

    let mut nexts = Vec::<usize>::new();
    let cap = 100/x + 1; // because y < 100
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