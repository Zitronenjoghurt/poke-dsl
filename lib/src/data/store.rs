use crate::data::dex::{ability, item, poke_move, poke_type, species, type_chart, Dex};
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::Arc;
use string_interner::DefaultSymbol;

pub struct Key<T>(DefaultSymbol, PhantomData<T>);

impl<T> Key<T> {
    pub fn new(symbol: DefaultSymbol) -> Self {
        Key(symbol, PhantomData)
    }

    pub fn symbol(&self) -> DefaultSymbol {
        self.0
    }
}

impl<T> std::fmt::Debug for Key<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> PartialEq for Key<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Key<T> {}

impl<T> Hash for Key<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> Clone for Key<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Key<T> {}

#[derive(Debug)]
pub struct Store<T> {
    entries: HashMap<Key<T>, Arc<T>>,
}

impl<T> Default for Store<T> {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}

impl<T> Store<T> {
    pub(crate) fn new() -> Self {
        Store {
            entries: HashMap::new(),
        }
    }

    pub(crate) fn insert_with_key(&mut self, key: Key<T>, value: T) {
        self.entries.insert(key, Arc::new(value));
    }

    pub fn get(&self, key: Key<T>) -> &Arc<T> {
        self.entries.get(&key).expect(
            "dangling key, this is a bug as long as data associated to the key cannot be dropped",
        )
    }
}

pub trait Stored: Sized {
    fn store(dex: &Dex) -> &Store<Self>;
    fn store_mut(dex: &mut Dex) -> &mut Store<Self>;
}

impl Stored for ability::AbilityData {
    fn store(dex: &Dex) -> &Store<Self> {
        &dex.abilities
    }

    fn store_mut(dex: &mut Dex) -> &mut Store<Self> {
        &mut dex.abilities
    }
}

impl Stored for item::ItemData {
    fn store(dex: &Dex) -> &Store<Self> {
        &dex.items
    }

    fn store_mut(dex: &mut Dex) -> &mut Store<Self> {
        &mut dex.items
    }
}

impl Stored for poke_move::MoveData {
    fn store(dex: &Dex) -> &Store<Self> {
        &dex.moves
    }

    fn store_mut(dex: &mut Dex) -> &mut Store<Self> {
        &mut dex.moves
    }
}

impl Stored for species::SpeciesData {
    fn store(dex: &Dex) -> &Store<Self> {
        &dex.species
    }

    fn store_mut(dex: &mut Dex) -> &mut Store<Self> {
        &mut dex.species
    }
}

impl Stored for poke_type::TypeData {
    fn store(dex: &Dex) -> &Store<Self> {
        &dex.types
    }

    fn store_mut(dex: &mut Dex) -> &mut Store<Self> {
        &mut dex.types
    }
}

impl Stored for type_chart::TypeChartData {
    fn store(dex: &Dex) -> &Store<Self> {
        &dex.type_charts
    }

    fn store_mut(dex: &mut Dex) -> &mut Store<Self> {
        &mut dex.type_charts
    }
}
