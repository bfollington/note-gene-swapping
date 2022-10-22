use crate::util;
use regex::Regex;

pub type Gene = String;
pub type Genome = Vec<Gene>;

pub fn extract_genes(input: &String) -> Vec<Gene> {
    let seperator = Regex::new(r"[\.\n]+").expect("Invalid regex");
    seperator
      .split(input)
      .filter(|s| s.len() > 0)
      .map(|s| s.trim().to_string())
      .collect()
}

pub fn swap_random_gene(a: &mut &mut Genome, b: &mut &mut Genome) {
    let index = util::random_index(a.len());

    let temp = a[index].clone();
    a[index] = b[index].clone();
    b[index] = temp;
}

pub fn share_random_gene(a: &mut &mut Genome, b: &mut &mut Genome) {
    let index = util::random_index(a.len());
    let temp = a[index].clone();
    b.insert(index, temp);
}

pub fn delete_random_gene(g: &mut &mut Genome) {
    let index = util::random_index(g.len());
    g.remove(index);
}

pub fn duplicate_random_gene(g: &mut &mut Genome) {
    let index = util::random_index(g.len());
    let temp = g[index].clone();
    g.insert(index, temp);
}

pub fn move_gene_location(g: &mut &mut Genome) {
    let index = util::random_index(g.len());
    let destination = util::random_index(g.len());
    let temp = g[index].clone();
    g.remove(index);
    g.insert(destination, temp);
}
