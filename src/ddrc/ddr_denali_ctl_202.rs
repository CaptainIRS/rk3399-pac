#[doc = "Register `DDR_DENALI_CTL_202` reader"]
pub type R = crate::R<DdrDenaliCtl202Spec>;
#[doc = "Field `DFI_ERROR_INFO` reader - Holds the encoded DFI error type associated with the DFI_ERROR parameter assertion."]
pub type DfiErrorInfoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - Holds the encoded DFI error type associated with the DFI_ERROR parameter assertion."]
    #[inline(always)]
    pub fn dfi_error_info(&self) -> DfiErrorInfoR {
        DfiErrorInfoR::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_202::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl202Spec;
impl crate::RegisterSpec for DdrDenaliCtl202Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_202::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl202Spec {}
#[doc = "`reset()` method sets DDR_DENALI_CTL_202 to value 0"]
impl crate::Resettable for DdrDenaliCtl202Spec {
    const RESET_VALUE: u32 = 0;
}
