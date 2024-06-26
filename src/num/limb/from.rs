use crate::num::limb::Limb;
impl From<usize> for Limb {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}
