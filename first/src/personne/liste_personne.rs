extern crate rand;

use std::i8;

use rand::thread_rng;
use rand::Rng;
use random_string::generate;

use crate::personne::personne::Personne;

#[derive(Clone)]
pub struct ListePersonne {
    liste: Vec<Personne>,
}

impl ListePersonne {
    ///Constructeur
    pub fn new() -> Self {
        Self { liste: Vec::new() }
    }
    
    ///Getteur
    pub fn get_list(&self) -> Vec<Personne> {
        return self.liste.clone();
    }
    
    pub fn add(&mut self, p: Personne) {
        self.liste.push(p);
    }

    pub fn to_string(&mut self) {
        for p in self.liste.iter() {
            p.to_string()
        }
    }

    pub fn remove_with_name(&mut self, str: String) {
        let mut i = 0;
        while i < self.liste.len() {
            if String::eq(&self.liste[i].get_name(), &str) {
                self.liste.remove(i);
                continue;
            }
            i += 1;
        }
    }

    pub fn remove_with_age(&mut self, age: i8) {
        let mut i = 0;
        while i < self.liste.len() {
            if self.liste[i].get_age() == age {
                self.liste.remove(i);
                continue;
            }
            i += 1;
        }
    }

    pub fn remove_with_personne(&mut self, p: Personne) {
        let mut i = 0;
        while i < self.liste.len() {
            if self.liste[i].eq(&p) {
                self.liste.remove(i);
                continue;
            }
            i += 1;
        }
    }

    pub fn viellir_all(&mut self) {
        for p in self.liste.iter_mut() {
            p.viellir();
        }
    }

    /// >= 80
    pub fn is_dead(&mut self) {
        let mut i = 0;
        let mut nbr_dead = 0;
        while i < self.liste.len() {
            if self.liste[i].get_age() >= 80 {
                self.liste.remove(i);
                nbr_dead += 1;
                continue;
            }
            i += 1;
        }
        println!("Nombre de mort {}", nbr_dead);
    }

    pub fn random_personne(&mut self, nbr_person: i8) {
        let str = "abcdefghijkmnopqrstuvwxyz";
        let mut name: String;
        let mut name_len: usize;
        let mut age: i8;
        let mut i = 0;

        let mut rng = thread_rng();

        while i < nbr_person {
            name_len = rng.gen_range(3, 8);
            name = Self::capitalize_first_letter(&generate(name_len, str));
            age = rng.gen_range(0, 90);
            self.add(Personne::new(name, age));
            i += 1;
        }
    }

    pub fn trie_age(&mut self) {
        let mut trie = false;
        let mut i = 0;
        while !trie {
            i = 0;
            trie = true;
            while i < self.liste.len() - 1 {
                if self.liste[i].get_age() > self.liste[i + 1].get_age() {
                    let tmp = self.liste[i + 1].clone();
                    self.liste[i + 1] = self.liste[i].clone();
                    self.liste[i] = tmp;
                    trie = false;
                }
                i += 1;
            }
        }
    }

    fn capitalize_first_letter(s: &str) -> String {
        s[0..1].to_uppercase() + &s[1..]
    }
}
