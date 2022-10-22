use crate::mutation::Population;

mod util;
mod genes;
mod mutation;

fn main() {

    let mut a = genes::extract_genes(&String::from("Hello. My name is. Bob."));
    let mut b = genes::extract_genes(&String::from("Hello. My name is. Bob."));
    println!("{:?}", a);
    println!("{:?}", b);

    let mut p: Population = vec![&mut a,&mut b];
    mutation::run_iteration(&mut p);
    mutation::run_iteration(&mut p);
    mutation::run_iteration(&mut p);

    println!("{:?}", mutation::format_genome(&p[0]));
    println!("{:?}", mutation::format_genome(&p[1]));
}
