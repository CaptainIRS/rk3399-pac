#[doc = "Register `DDR_DENALI_CTL_204` reader"]
pub type R = crate::R<DdrDenaliCtl204Spec>;
#[doc = "Field `INT_STATUS` reader - Status of interrupt features in the controller. READ-ONLY"]
pub type IntStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Status of interrupt features in the controller. READ-ONLY"]
    #[inline(always)]
    pub fn int_status(&self) -> IntStatusR {
        IntStatusR::new((self.bits & 3) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_204::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl204Spec;
impl crate::RegisterSpec for DdrDenaliCtl204Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_204::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl204Spec {}
#[doc = "`reset()` method sets DDR_DENALI_CTL_204 to value 0"]
impl crate::Resettable for DdrDenaliCtl204Spec {
    const RESET_VALUE: u32 = 0;
}
