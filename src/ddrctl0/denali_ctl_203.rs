#[doc = "Register `DENALI_CTL_203` reader"]
pub type R = crate::R<DenaliCtl203Spec>;
#[doc = "Field `INT_STATUS` reader - Status of interrupt features in the controller. READ-ONLY"]
pub type IntStatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Status of interrupt features in the controller. READ-ONLY"]
    #[inline(always)]
    pub fn int_status(&self) -> IntStatusR {
        IntStatusR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_203::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl203Spec;
impl crate::RegisterSpec for DenaliCtl203Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_203::R`](R) reader structure"]
impl crate::Readable for DenaliCtl203Spec {}
#[doc = "`reset()` method sets DENALI_CTL_203 to value 0"]
impl crate::Resettable for DenaliCtl203Spec {
    const RESET_VALUE: u32 = 0;
}
