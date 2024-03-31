#[doc = "Register `DENALI_CTL_209` reader"]
pub type R = crate::R<DenaliCtl209Spec>;
#[doc = "Field `OUT_OF_RANGE_ADDR` reader - Address of command that caused an out-of-range interrupt."]
pub type OutOfRangeAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address of command that caused an out-of-range interrupt."]
    #[inline(always)]
    pub fn out_of_range_addr(&self) -> OutOfRangeAddrR {
        OutOfRangeAddrR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_209::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl209Spec;
impl crate::RegisterSpec for DenaliCtl209Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_209::R`](R) reader structure"]
impl crate::Readable for DenaliCtl209Spec {}
#[doc = "`reset()` method sets DENALI_CTL_209 to value 0"]
impl crate::Resettable for DenaliCtl209Spec {
    const RESET_VALUE: u32 = 0;
}
