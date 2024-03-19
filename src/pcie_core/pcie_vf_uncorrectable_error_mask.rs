#[doc = "Register `PCIE_VF_UNCORRECTABLE_ERROR_MASK` reader"]
pub type R = crate::R<PcieVfUncorrectableErrorMaskSpec>;
#[doc = "Field `R4` reader - Reserved \\[R4\\]\n\n(no description)"]
pub type R4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved \\[R4\\]\n\n(no description)"]
    #[inline(always)]
    pub fn r4(&self) -> R4R {
        R4R::new(self.bits)
    }
}
#[doc = "Uncorrectable Error Mask Register\n\n(no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_uncorrectable_error_mask::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfUncorrectableErrorMaskSpec;
impl crate::RegisterSpec for PcieVfUncorrectableErrorMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_uncorrectable_error_mask::R`](R) reader structure"]
impl crate::Readable for PcieVfUncorrectableErrorMaskSpec {}
#[doc = "`reset()` method sets PCIE_VF_UNCORRECTABLE_ERROR_MASK to value 0"]
impl crate::Resettable for PcieVfUncorrectableErrorMaskSpec {
    const RESET_VALUE: u32 = 0;
}
