use crate::data::raw::RawData;
use crate::data::store::{Key, Store, Stored};
use std::path::Path;
use std::sync::Arc;
use string_interner::DefaultStringInterner;

pub mod ability;
pub mod item;
pub mod poke_move;
pub mod poke_type;
pub mod species;
pub mod type_chart;

#[derive(Default, Debug)]
pub struct Dex {
    interner: DefaultStringInterner,
    pub(crate) abilities: Store<ability::AbilityData>,
    pub(crate) items: Store<item::ItemData>,
    pub(crate) moves: Store<poke_move::MoveData>,
    pub(crate) species: Store<species::SpeciesData>,
    pub(crate) types: Store<poke_type::TypeData>,
    pub(crate) type_charts: Store<type_chart::TypeChartData>,
}

impl Dex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<T: Stored>(&self, key: Key<T>) -> &Arc<T> {
        T::store(self).get(key)
    }

    pub fn add<R: Resolvable>(&mut self, raw: R) -> Result<Key<R::Output>, ResolveError> {
        let key = Key::new(self.interner.get_or_intern(raw.id()));
        let resolved = raw.resolve(key, self)?;
        R::Output::store_mut(self).insert_with_key(key, resolved);
        Ok(key)
    }

    pub fn add_raw(&mut self, raw: RawData) -> Result<(), ResolveError> {
        match raw {
            RawData::Ability(r) => {
                self.add(r)?;
            }
            RawData::Item(r) => {
                self.add(r)?;
            }
            RawData::Move(r) => {
                self.add(r)?;
            }
            RawData::Type(r) => {
                self.add(r)?;
            }
            RawData::Species(r) => {
                self.add(r)?;
            }
            RawData::TypeChart(r) => {
                self.add(r)?;
            }
        }
        Ok(())
    }

    pub fn load_all(&mut self, mut raw: Vec<RawData>) -> Result<(), ResolveError> {
        raw.sort_by_key(|r| r.phase());
        for entry in raw {
            self.add_raw(entry)?;
        }
        Ok(())
    }

    #[cfg(all(feature = "serde", feature = "ron"))]
    fn parse_ron(contents: &str) -> Result<Vec<RawData>, ron::error::SpannedError> {
        let opts = ron::Options::default()
            .without_default_extension(ron::extensions::Extensions::EXPLICIT_STRUCT_NAMES);
        match opts.from_str::<Vec<RawData>>(contents) {
            Ok(v) => Ok(v),
            Err(vec_err) => match opts.from_str::<RawData>(contents) {
                Ok(single) => Ok(vec![single]),
                Err(_) => Err(vec_err),
            },
        }
    }

    #[cfg(all(feature = "serde", feature = "ron"))]
    pub fn load_ron_from_dir(&mut self, path: &Path) -> Result<(), ResolveError> {
        let mut data: Vec<RawData> = std::fs::read_dir(path)?
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                (path.extension()? == "ron").then_some(path)
            })
            .map(|path| {
                let contents = std::fs::read_to_string(&path)?;
                let raw = Self::parse_ron(&contents).map_err(|e| {
                    eprintln!("Failed to parse {}: {e}", path.display());
                    e
                })?;
                Ok(raw)
            })
            .collect::<Result<Vec<Vec<_>>, ResolveError>>()?
            .into_iter()
            .flatten()
            .collect();

        data.sort_by_key(|r| r.phase());
        for entry in data {
            self.add_raw(entry)?;
        }
        Ok(())
    }

    pub(crate) fn resolve_id<T>(&self, name: &str) -> Result<Key<T>, ResolveError> {
        self.interner
            .get(name)
            .map(Key::new)
            .ok_or_else(|| ResolveError::NotFound(name.to_string()))
    }

    pub(crate) fn resolve_ids<T: Stored>(
        &self,
        ids: &[String],
    ) -> Result<Vec<Key<T>>, ResolveError> {
        ids.iter().map(|id| self.resolve_id::<T>(id)).collect()
    }

    pub(crate) fn resolve_ref<T: Stored>(&self, name: &str) -> Result<Arc<T>, ResolveError> {
        let key = self.resolve_id::<T>(name)?;
        Ok(Arc::clone(self.get(key)))
    }

    pub(crate) fn resolve_refs<T: Stored>(
        &self,
        names: &[String],
    ) -> Result<Vec<Arc<T>>, ResolveError> {
        names.iter().map(|id| self.resolve_ref::<T>(id)).collect()
    }
}

pub trait Resolvable {
    type Output: Stored;

    fn id(&self) -> &str;
    fn resolve(self, id: Key<Self::Output>, dex: &Dex) -> Result<Self::Output, ResolveError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ResolveError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[cfg(feature = "ron")]
    #[error(transparent)]
    Ron(#[from] ron::error::SpannedError),
    #[error("Entry with id '{0}' was referenced but not previously resolved")]
    NotFound(String),
}
