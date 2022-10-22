use rand::Rng;

pub fn random_index(len: usize) -> usize {
    let mut rng = rand::thread_rng();

    rng.gen_range(0..len)
}

pub fn choose(g: &Vec<String>) -> String {
    let index = random_index(g.len());

    g[index].clone()
}