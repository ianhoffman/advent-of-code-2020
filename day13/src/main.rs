use std::fs;

fn part1(content: &String) {
    let mut iter = content.splitn(2, '\n');
    let start_time = iter.next().unwrap().parse::<u32>().unwrap();
    let (bus_id, min_wait_time) = iter
        .next()
        .unwrap()
        .trim_end()
        .split(',')
        .filter(|bus_id| *bus_id != "x")
        .map(|bus_id| bus_id.parse::<u32>().unwrap())
        .map(|bus_id| (bus_id, (start_time / bus_id + 1) * bus_id - start_time))
        .min_by(|(_, min_wait_time), (_, curr_wait_time)| min_wait_time.cmp(curr_wait_time))
        .unwrap();
    println!("{}", min_wait_time * bus_id);
}

// Find x such that ax =~ 1 (mod m)
fn mod_inv(a: i64, b: i64) -> i64 {
    //
    // We are going to apply "Euclid's Extended Algorithm"
    // to compute the GCD of (a, b) (which we know to be 1,
    // since a and b must be coprime). In doing so, we will
    // compute an integer x s.t. ax * by = gcd(a, b). But
    // gcd(a, b) = 1, so:
    //      ax - 1 = (-y)b
    //      b | ax - 1
    //      ax =~ 1 (mod b)
    // Thus x is the modular multiplicative inverse of a mod b.

    let b_start = b;

    let mut a = a;
    let mut b = b;
    let mut x = 0;
    let mut x_last = 1;

    while b > 0 {
        let q = a / b;

        let t = a;
        a = b;
        b = t - q * b;

        let t = x;
        x = x_last - q * x;
        x_last = t;
    }

    if x_last < 0 {
        x_last += b_start;
    }

    x_last
}

fn part2(content: &String) {
    let num_to_remainder: Vec<(i64, i64)> = content
        .splitn(2, '\n')
        .skip(1)
        .next()
        .unwrap()
        .trim_end()
        .split(',')
        //
        // Enumerate the list since these indices
        // will be used to determine the target "distance"
        // from the initial departure timestamp. (See below.)
        //
        .enumerate()
        //
        // Filter out buses without a valid ID.
        // We don't care about them.
        //
        .filter(|(_, bus_id)| *bus_id != "x")
        //
        // `idx` is the offset from the timestamp.
        // The first bus leaves at the timestamp, so the offset is 0.
        // The next leaves at the timestamp + 1, etc.
        //
        .map(|(idx, bus_id)| (bus_id.parse::<i64>().unwrap(), idx as i64))
        //
        // If x + k % a = 0, then x % a = a - k.
        // So we're looking for a number which, when divided by a,
        // has a remainder of a - k.
        //
        .map(|(bus_id, idx)| (bus_id, bus_id - idx))
        .collect();
    //
    // I did a bit of a deep-dive on the Chinese remainder theorem.
    // This video was useful: https://www.youtube.com/watch?v=ru7mWZJlRQg,
    // as well as this GeeksForGeeks article:
    // https://www.geeksforgeeks.org/chinese-remainder-theorem-set-2-implementation
    //
    // In short, we need this sum because, by dividing it by each number
    // in the part below, we can ensure that each "part" of our result only contains
    // terms divisible by the number in question. That is to say, if we want to calculate
    // a number s such that s % a = 2, s % b = 3, and s % c = 4 we can construct a
    // sum s = bcx + acy + abz, so then s % a = bcx % a, s % b = acy % b, and
    // s % c = abz % c. Thus we need only consider each term in isolation,
    // which simplifies our work.
    //
    let product: i64 = num_to_remainder.iter().map(|(num, _)| *num).product();
    let result = num_to_remainder.iter().fold(0, |acc, (num, rem)| {
        // This is the number `bc`, for example, in the sum `s` described above.
        let prod_without_num = product / num;
        //
        // This is a number `x` such that ax % m ~= 1.
        // Here's (to my understanding) why we use `product_without_num` as the modulus:
        // the equation is now of the form s = bcx + acy + abz.
        // When we take s % a, we get s = bcx (modulo a) (all other terms go to 0).
        // Thus we seek x such that bcx % a = rem.
        // To simplify, we will assume rem = 1.
        // Thus we seek bcx % a = 1, so bcx ~= 1 (mod a).
        // So x is the modular inverse of bc.
        //
        let inv = mod_inv(prod_without_num /* a */, *num /* modulus */);
        //
        // Now that we have the modular inverse x of bc, we need to finagle it
        // such that bcx % a = rem. If rem is 1, we're done.
        // Otherwise, we multiply x by rem because
        //      bcx =~ 1 (modulo a)
        //   => bcx * rem =~ rem (modulo a)
        //   => bc(x * rem) =~ rem (modulo a)
        //
        // Thus our final result is of the form
        // result = bc * x1 * rem1 + ac * x2 * rem2 ...
        //
        acc + prod_without_num * inv * *rem
    }) % product;

    println!("{}", result);
}

fn main() {
    let content = fs::read_to_string("input/input.txt").unwrap();
    part1(&content);
    part2(&content);
}
