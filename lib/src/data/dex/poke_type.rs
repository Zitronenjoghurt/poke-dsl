use crate::data::dex::Dex;
use crate::data::dex::Resolvable;
use crate::data::dex::ResolveError;
use crate::data::store::Key;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawTypeData {
    id: String,
}

#[derive(Debug)]
pub struct TypeData {
    id: Key<TypeData>,
}

impl Resolvable for RawTypeData {
    type Output = TypeData;

    fn id(&self) -> &str {
        &self.id
    }

    fn resolve(self, id: Key<Self::Output>, _: &Dex) -> Result<Self::Output, ResolveError> {
        Ok(TypeData { id })
    }
}
