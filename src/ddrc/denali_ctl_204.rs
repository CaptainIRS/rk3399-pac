#[doc = "Register `DENALI_CTL_204` reader"]
pub type R = crate::R<DenaliCtl204Spec>;
#[doc = "Field `INT_STATUS` reader - Status of interrupt features in the controller."]
pub type IntStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Status of interrupt features in the controller."]
    #[inline(always)]
    pub fn int_status(&self) -> IntStatusR {
        IntStatusR::new((self.bits & 3) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_204::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl204Spec;
impl crate::RegisterSpec for DenaliCtl204Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_204::R`](R) reader structure"]
impl crate::Readable for DenaliCtl204Spec {}
#[doc = "`reset()` method sets DENALI_CTL_204 to value 0"]
impl crate::Resettable for DenaliCtl204Spec {
    const RESET_VALUE: u32 = 0;
}
