use std::io;

fn main() {

    let mut tranch = String::new();

    io::stdin()
        .read_line(&mut tranch)
        .expect("Failed to read line");

    let tranch:u8 = tranch.trim().parse().expect("no");

        check_function(tranch);
}

fn check_function(x:u8){
    if x >=1 {
        if x<100 {
        if x%2 ==0 {
            println!("YES");

        }
        else{
            println!("NO");
        }}
    else {
        println!("NO");
    }}
        else{
            println!("NO");
        

    }
}


