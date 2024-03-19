#[doc = "Register `DDR_DENALI_CTL_203` reader"]
pub type R = crate::R<DdrDenaliCtl203Spec>;
#[doc = "Field `INT_STATUS` reader - Status of interrupt features in the controller. READ-ONLY"]
pub type IntStatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Status of interrupt features in the controller. READ-ONLY"]
    #[inline(always)]
    pub fn int_status(&self) -> IntStatusR {
        IntStatusR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_203::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl203Spec;
impl crate::RegisterSpec for DdrDenaliCtl203Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_203::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl203Spec {}
#[doc = "`reset()` method sets DDR_DENALI_CTL_203 to value 0"]
impl crate::Resettable for DdrDenaliCtl203Spec {
    const RESET_VALUE: u32 = 0;
}
