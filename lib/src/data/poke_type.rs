use crate::data::{Key, Resolvable};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawTypeData {
    id: String,
}

pub struct TypeData {
    id: Key<TypeData>,
}

impl Resolvable for RawTypeData {
    type Output = TypeData;

    fn id(&self) -> &str {
        &self.id
    }

    fn resolve(
        self,
        id: Key<Self::Output>,
        _: &crate::data::Dex,
    ) -> Result<Self::Output, crate::data::ResolveError> {
        Ok(TypeData { id })
    }
}
