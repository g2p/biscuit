#[derive(Debug, Eq, PartialEq, Clone, Copy)]
/// Defines whether a claim is validated or not
///
/// The generic type T is used as the "options" for validating claims and is
/// specific to each claim being validated. Refer to [`biscuit::ValidationOptions`]
/// for the specifics of each claim.
pub enum Validation<T> {
    /// This claim is not validated
    Ignored,

    /// Validate this claim with type T.
    /// Refer to [`biscuit::ValidationOptions`] for the specifics of each claim.
    Validate(T),
}

impl<T> Default for Validation<T> {
    fn default() -> Self {
        Validation::Ignored
    }
}

impl<T> Validation<T> {
    /// Map the value to another validation requirement, similar to how .map works on iter()
    pub fn map<U, F>(self, f: F) -> Validation<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Validation::Ignored => Validation::Ignored,
            Validation::Validate(t) => Validation::Validate(f(t)),
        }
    }
}
