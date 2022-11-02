use std::{collections::HashMap, hash::Hash};

pub struct Memoire<I: Hash + Eq + Clone, O: Clone> {
    func: fn(I, &mut Memoire<I, O>) -> O,
    cache: HashMap<I, O>,
}

impl<I: Hash + Eq + Clone, O: Clone> Memoire<I, O> {
    pub fn new(func: fn(I, &mut Memoire<I, O>) -> O) -> Self {
        Self {
            func,
            cache: HashMap::new(),
        }
    }

    pub fn run(&mut self, input: I) -> O {
        match self.cache.get(&input) {
            Some(output) => output.clone(),
            None => {
                let output = (self.func)(input.clone(), self);
                self.cache.insert(input, output.clone());
                output
            }
        }
    }
}
