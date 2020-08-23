use std::io;
fn pgcd(mut a:i8,mut b:i8) -> i8 {
    while (a != b){
        if a > b {
            a = a -b;

        }else{
            b = b - a;
        }}
    return a;
}
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("error");

    let line: String = line.trim().parse().expect("no");
    let inputs: Vec<i8> = line.split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    let mut pgcd_a;
    let mut pgcd_b;

    let (mut a, mut b, mut n) = (inputs[0], inputs[1], inputs[2]);
    loop {
          pgcd_a = pgcd(a, n); // 3,9 => 3
        //println!("{}",pgcd_a);
        n = n -pgcd_a;  // 9 - 3 = 6
        if pgcd_a > n {
            println!("0");
            break;
        }
         pgcd_b = pgcd(b,n); // 5,6 => 1
        //println!("{}",pgcd_b);
        n = n-pgcd_b;
        if pgcd_b > n {
            println!("1");
            break;
        }

    }
}









