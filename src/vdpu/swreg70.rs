#[doc = "Register `SWREG70` reader"]
pub type R = crate::R<Swreg70Spec>;
#[doc = "Field `SW_YCOMP_MV_SUM` reader - sum of the decoded motion vector y-components"]
pub type SwYcompMvSumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - sum of the decoded motion vector y-components"]
    #[inline(always)]
    pub fn sw_ycomp_mv_sum(&self) -> SwYcompMvSumR {
        SwYcompMvSumR::new(self.bits & 0x003f_ffff)
    }
}
#[doc = "sum of the decoded motion vector y-components(read only)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg70::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg70Spec;
impl crate::RegisterSpec for Swreg70Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg70::R`](R) reader structure"]
impl crate::Readable for Swreg70Spec {}
#[doc = "`reset()` method sets SWREG70 to value 0"]
impl crate::Resettable for Swreg70Spec {
    const RESET_VALUE: u32 = 0;
}
