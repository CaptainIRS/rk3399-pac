#[doc = "Register `DDR_DENALI_CTL_209` reader"]
pub type R = crate::R<DdrDenaliCtl209Spec>;
#[doc = "Field `OUT_OF_RANGE_ADDR` reader - Address of command that caused an out-of-range interrupt. READ- ONLY"]
pub type OutOfRangeAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address of command that caused an out-of-range interrupt. READ- ONLY"]
    #[inline(always)]
    pub fn out_of_range_addr(&self) -> OutOfRangeAddrR {
        OutOfRangeAddrR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_209::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl209Spec;
impl crate::RegisterSpec for DdrDenaliCtl209Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_209::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl209Spec {}
#[doc = "`reset()` method sets DDR_DENALI_CTL_209 to value 0"]
impl crate::Resettable for DdrDenaliCtl209Spec {
    const RESET_VALUE: u32 = 0;
}
