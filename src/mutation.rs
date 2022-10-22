use crate::genes;
use crate::genes::Genome;
use crate::util;

pub enum Mutation {
    Swap,
    Share,
    Delete,
    Duplicate,
    Move,
}

// This seems very dumb
impl Clone for Mutation {
    fn clone(&self) -> Self {
        match self {
            Mutation::Swap => Mutation::Swap,
            Mutation::Share => Mutation::Share,
            Mutation::Delete => Mutation::Delete,
            Mutation::Duplicate => Mutation::Duplicate,
            Mutation::Move => Mutation::Move,
        }
    }
}

pub type Population<'a> = Vec<&'a mut Genome>;

pub fn count_unique_terms(g: &Genome) -> usize {
    let mut unique_terms = Vec::new();

    for gene in g {
        for term in gene.split_whitespace() {
            if !unique_terms.contains(&term) {
                unique_terms.push(term);
            }
        }
    }

    unique_terms.len()
}

pub fn run_iteration(p: &mut Population) {
    let m = util::choose(&vec![
        Mutation::Swap,
        Mutation::Share,
        Mutation::Delete,
        Mutation::Duplicate,
        Mutation::Move,
    ]);
    
    let count = p.len();
    let index = util::random_index(count).max(1).min(count - 1);
    println!("split at index {}, for pop {}", index, count);
    let (a_ref, b_ref) = p.split_at_mut(index);
    let a = a_ref.first_mut().unwrap();
    let b = b_ref.first_mut().unwrap();

    match m {
        Mutation::Swap => genes::swap_random_gene(a, b),
        Mutation::Share => genes::share_random_gene(a, b),
        Mutation::Delete => genes::delete_random_gene(a),
        Mutation::Duplicate => genes::duplicate_random_gene(a),
        Mutation::Move => genes::move_gene_location(a),
    }
}

pub fn format_genome(g: &Genome) -> String {
    let s = g.join(". ");
    format!("{}.", s)
}

fn length_factor(g: &Genome) -> f64 {
    1.0 - (format_genome(g).len() as f64 - 280.0).abs() / 280.0
}

fn sort_population_by_fitness(p: &mut Population) {
    p.sort_by(|a, b| {
        let a_fitness = length_factor(a) * count_unique_terms(a) as f64;
        let b_fitness = length_factor(b) * count_unique_terms(b) as f64;

        b_fitness.partial_cmp(&a_fitness).unwrap()
    });
}

/*

export function sortPopulationByFitness(population: Genome[]) {
  population.sort(
    (a: Genome, b: Genome) => {
      const aTerms = uniqueInterestingTerms(formatGenome(a));
      const bTerms = uniqueInterestingTerms(formatGenome(b));
      return (bTerms.size - aTerms.size)
    })
}

export function uniqueInterestingTerms(note: string) {
  return new Set(
    (nlp(note).normalize("heavy") as any)
      .match("#Noun")
      .sort("freq")
      .out("json")
      .map(extractPlainTextTerms)
      .filter(isNotBanned)
  );
}

const PUNCTUATION = ["."]
function randomPunctuationMark() {
  return choose(PUNCTUATION)
}

export function formatGenome(genome: Genome) {
  return genome.map(g => g + randomPunctuationMark() + " ").join("")
}


function killLeastInteresting(population: Genome[]) {
  population.sort(
    (a: Genome, b: Genome) => {
      const aTerms = uniqueInterestingTerms(formatGenome(a));
      const bTerms = uniqueInterestingTerms(formatGenome(b));
      return (aTerms.size - bTerms.size)
    })
  population.splice(0, population.length / 5)

  return population
}

function killTooLongOrShot(population: Genome[]) {
  population.sort(
    (a: Genome, b: Genome) => {
      return lengthFactor(a) - lengthFactor(b)
    })
  population.splice(0, population.length / 5)
  return population.filter(g => g.length > 0)
}

function reproduce(population: Genome[], size: number) {
  while (population.length < size) {
    population.push(choose(population).slice())
  }
}

const REFILL = 32

export function run(population: Genome[], setPopulation: (p: Genome[], generation: number) => void) {

  let count = 0;
  function loop() {
    console.log("GENERATION " + count)
    for (let j = 0; j < MUTATIONS_PER_GENERATION; j++) {
      runIteration(population);
    }
    const size = population.length
    reproduce(population, REFILL);
    population = killTooLongOrShot(population)
    killLeastInteresting(population);
    reproduce(population, REFILL);
    sortPopulationByFitness(population)
    setPopulation(population, count)
    count++

    if (count < GENERATIONS) {
      setTimeout(loop, 100)
    }
  }

  loop()
}

 */
