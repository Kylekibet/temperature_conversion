fn main() {
    let degree_sign = '\u{00B0}';
    let f = f_to_c(80.0);
    let c = c_to_f(f);
    println!("{f:.2}{}C", degree_sign);
    println!("{c:.2}{degree_sign}F");
}


fn f_to_c (num :f64) -> f64 {
    (num - 32.0) / 1.8
}

fn c_to_f(num :f64) -> f64 {
    (num * 1.8) + 32.0
}