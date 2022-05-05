use std::fs;
use rand::Rng;
mod suffixes;

// https://github.com/sayamqazi/planet-name-generator/blob/master/script.py

const PLANETS_FILE: &str = "./planets.txt";

pub struct PlanetName {
    pub filepath: String,
}

impl PlanetName {
    pub fn new (filepath: String) -> Self {
        PlanetName {
            filepath,
        }
    }

    fn get_planets(&self) -> Vec<String> {
        let contents = fs::read_to_string(PLANETS_FILE.to_string())
            .expect("Something went wrong reading the file");

        contents.split("\n").collect::<Vec<&str>>().iter().map(|x| x.to_string()).collect()
    }

    fn get_planets_syllables(&self) -> Vec<String> {
        let planets: Vec<String> = self.get_planets();
        let mut syllables: Vec<String> = Vec::new();

        for planet in planets {
            let lex: Vec<&str> = planet.split("-").collect::<Vec<&str>>();

            for syllable in lex {
                if !syllables.contains(&syllable.to_string()) {
                    syllables.push(syllable.to_string());
                }
            }
        }

        syllables
    }

    fn get_frequency(&self, syllables: Vec<String>) -> Vec<Vec<usize>> {
        let size = syllables.len() + 1;
        let mut freq = vec![vec![0; size]; size];

        let planets: Vec<String> = self.get_planets();

        for planet in planets {
            let lex: Vec<&str> = planet.split("-").collect::<Vec<&str>>();
            let mut i = 0;


            while i < lex.len() - 1 {
                let first_index = syllables.iter().position(|x| x.eq(lex[i])).unwrap();
                let second_index = syllables.iter().position(|x| x.eq(lex[i + 1])).unwrap();
                freq[first_index][second_index] += 1;
                i += 1;
            }

            let first_index = syllables.iter().position(|x| x == lex[lex.len() - 1]).unwrap();
            freq[first_index][size - 1] += 1;
        }

        freq
    }

    pub fn generate(&self) -> String {
        let syllables = self.get_planets_syllables();

        let size = syllables.len() + 1;
        let freq = self.get_frequency(syllables.clone());

        let mut rng = rand::thread_rng();
        let mut length: u16 = rng.gen_range(2..3);
        let mut initial: usize = rng.gen_range(0..size - 2);
        let mut planet_name: String = "".to_string();

        while length > 0 {
            while !freq[initial].contains(&1) {
                let mut rng = rand::thread_rng();
                initial = rng.gen_range(0..size - 2);
            }

            planet_name += &syllables[initial];
            initial = rng.gen_range(0..size - 2);
            length -= 1;
        }

        let suffixes = suffixes::get_suffixes();

        let mut rng = rand::thread_rng();
        let suffix_index = rng.gen_range(0..suffixes.len() - 1);

        planet_name += &" ";

        planet_name += &suffixes[suffix_index];

        planet_name
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        for n in 0..100 {
            PlanetName::new("./planetes-example.txt".to_string()).generate();
        }
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
