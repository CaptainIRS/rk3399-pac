#[doc = "Register `PCIE_VF_UNCORRECTABLE_ERROR_SEVERITY` reader"]
pub type R = crate::R<PcieVfUncorrectableErrorSeveritySpec>;
#[doc = "Field `R8` reader - Reserved \\[R8\\]\n\n(no description)"]
pub type R8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved \\[R8\\]\n\n(no description)"]
    #[inline(always)]
    pub fn r8(&self) -> R8R {
        R8R::new(self.bits)
    }
}
#[doc = "Uncorrectable Error Severity Register\n\n(no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_uncorrectable_error_severity::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfUncorrectableErrorSeveritySpec;
impl crate::RegisterSpec for PcieVfUncorrectableErrorSeveritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_uncorrectable_error_severity::R`](R) reader structure"]
impl crate::Readable for PcieVfUncorrectableErrorSeveritySpec {}
#[doc = "`reset()` method sets PCIE_VF_UNCORRECTABLE_ERROR_SEVERITY to value 0"]
impl crate::Resettable for PcieVfUncorrectableErrorSeveritySpec {
    const RESET_VALUE: u32 = 0;
}
