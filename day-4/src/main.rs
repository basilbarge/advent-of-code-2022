fn main() {
    println!("# of subsets: {}",
    include_str!("./input.txt").
        lines()
        .map(|l| {
            let (l, r) = l.split_once(",").unwrap();
            let ((a, b), (c, d)) = (l.split_once("-").unwrap(), r.split_once("-").unwrap());
            (
                a.parse::<u8>().unwrap(),
                b.parse::<u8>().unwrap(),
                c.parse::<u8>().unwrap(),
                d.parse::<u8>().unwrap(),
                )
        })
        .filter(|(a, b, c, d)| ((a >= c && b <= d) || (a <= c && b >= d)))
        .count());


    println!("# of overlapping sets: {}",
    include_str!("./input.txt").
        lines()
        .map(|l| {
            let (l, r) = l.split_once(",").unwrap();
            let ((a, b), (c, d)) = (l.split_once("-").unwrap(), r.split_once("-").unwrap());
            (
                a.parse::<u8>().unwrap(),
                b.parse::<u8>().unwrap(),
                c.parse::<u8>().unwrap(),
                d.parse::<u8>().unwrap(),
                )
        })
        .filter(|(a, b, c, d)| ((a >= c && a <= d) && (b >= c || b <= d)) || ((c<=b && c >=a) || ( d >= a && d <= b)))
        .count());
}
