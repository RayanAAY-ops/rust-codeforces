use std::io;
use std::process;
fn exit_func() -> () {
    process::exit(1);
}
fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("enter a valid string");


    let  line: i16 = line.trim() //delete whitespace ,(    4\n) => (4)
        .parse()//The parse method on strings parses a string into some kind of number.
        .expect("impossible to transform string to integer");
    if line >= 1 && line <= 1000 {
        for _i in 0..line {
            let mut taille = String::new();
            io::stdin()
                .read_line(&mut taille)
                .expect("enter a valid string");

            let taille :i64 = taille.trim().parse().expect("enter a number");




            let mut result:i8 = 0;
            let mut mon_vecteur = Vec::new();



            let mut view = String::new();

            io::stdin()
                .read_line(&mut view)
                .expect("enter a valid string");
            let nums_item = view.trim().split(' ').flat_map(str::parse::<i32>).collect::<Vec<_>>();
            if nums_item.len() != taille as usize {
                exit_func();

            }
            else {

                'outer: for mut i in 0..nums_item.len() {
                    for j in (i+1)..nums_item.len(){
                        for k in (j+1)..nums_item.len() {

                            if  nums_item[i] != nums_item[j] &&

                                nums_item[i] + nums_item[j] > nums_item[k] &&
                                nums_item[i] + nums_item[k] > nums_item[j] &&
                                nums_item[j] + nums_item[k] > nums_item[i]
                            {
                                result +=1 ;
                                mon_vecteur.push(i);
                                mon_vecteur.push(j);
                                mon_vecteur.push(k);
                                break 'outer;
                            }

                        }
                    }
                }
                if result  >=1 {
                    println!("{} {} {}", mon_vecteur[0],mon_vecteur[1],mon_vecteur[2]); // indices of the three elements forming a non degenerative triangle
                }
                else {
                    println!("-1");
                }
            }}
    }}