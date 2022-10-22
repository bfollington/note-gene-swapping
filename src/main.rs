mod util;
mod genes;

fn main() {

    let mut genome = genes::extract_genes(&String::from("Hello. My name is. Bob."));
    println!("{:?}", genome);
    genes::duplicate_random_gene(&mut genome);

    let s = genome.join(". ");
    println!("{}.", s);
}
