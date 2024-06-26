use crate::num::uint::Uint;
use std::cmp::Ordering;

// Impl Cmp
impl<const LIMBS: usize> PartialEq for Uint<LIMBS> {
    fn eq(&self, other: &Self) -> bool {
        self.limbs.eq(&other.limbs)
    }
}

impl<const LIMBS: usize> Ord for Uint<LIMBS> {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in (0..LIMBS).rev() {
            match self.limbs[i].0.cmp(&other.limbs[i].0) {
                Ordering::Less => {
                    return Ordering::Less;
                }
                Ordering::Greater => return Ordering::Greater,
                Ordering::Equal => {}
            }
        }
        Ordering::Equal
    }
}

impl<const LIMBS: usize> PartialOrd for Uint<LIMBS> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
