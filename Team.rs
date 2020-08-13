use std::io;
use std::process;
fn main() {
    let mut line = String::new();

    let mut result = 0;
    io::stdin()
        .read_line(&mut line)
        .expect("enter a valid string");

    // Transform the input into an integer i16

    let  line: i16 = line.trim() //delete whitespace ,(    4\n) => (4)
        .parse()//The parse method on strings parses a string into some kind of number.
        .expect("impossible to transform string to integer");
    if line >= 1 && line <= 1000 {
        for _i in 0..line {
            let mut sum :i8;
            let mut vect_holder = Vec::new();


            let mut  view = String::new();
            io::stdin()
                .read_line(&mut view)
                .expect("enter a valid string");
            let view_without_whitespace = remove_whitespace(&mut view);
            for c in view_without_whitespace.chars() {
                vect_holder.push(c);
            }
            let mut sum = 0;
            for  item in &vect_holder{
                let i = *item as i8 -48 ;
                if i > 1 {
                    exit_func()
                }
                //if we remove the * we're trying to treat a reference (which is what as_bytes results in)
                // as a regular value, which doesn't work.
                sum +=i;
            }
            //println!("the sum is {}",sum);
            if sum >=2 {
                result+=1;
            }

            //println!("{:?}",vect_holder);
            //println!("{}", view_trim.chars().map(|c| c.to_digit(RADIX).unwrap()).sum::<u32>());




            //println!("{}",sum);

        }
        println!("{}",result);
    }
    else {
        println!("exit");
        exit_func()
    }
}
fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}
fn exit_func() -> () {
        process::exit(1);
    }





