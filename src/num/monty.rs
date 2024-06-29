use crate::num::uint::Uint;

#[derive(Clone, Copy)]
pub struct MontyForm<const LIMBS: usize> {
    modular: Uint<LIMBS>,
    r: Uint<LIMBS>,
    r2: Uint<LIMBS>,
}
