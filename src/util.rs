use rand::Rng;

pub fn random_index(len: usize) -> usize {
    let mut rng = rand::thread_rng();

    rng.gen_range(0..len)
}

pub fn choose<T>(g: &Vec<T>) -> T where T : Clone {
    let index = random_index(g.len());

    g[index].clone()
}
