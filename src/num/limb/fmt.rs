use crate::num::limb::Limb;
impl Limb {
    #[inline(always)]
    pub(crate) fn to_binary_string(self, trim_leading_zero: bool) -> String {
        if trim_leading_zero {
            format!("{:b}", self.0)
        } else {
            format!("{:0width$b}", self.0, width = Self::BITS)
        }
    }
}
