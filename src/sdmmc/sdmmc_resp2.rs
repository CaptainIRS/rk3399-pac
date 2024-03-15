#[doc = "Register `SDMMC_RESP2` reader"]
pub type R = crate::R<SdmmcResp2Spec>;
#[doc = "Field `RESPONSE2` reader - Bit\\[95:64\\]
of long response"]
pub type Response2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bit\\[95:64\\]
of long response"]
    #[inline(always)]
    pub fn response2(&self) -> Response2R {
        Response2R::new(self.bits)
    }
}
#[doc = "Response-2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_resp2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcResp2Spec;
impl crate::RegisterSpec for SdmmcResp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_resp2::R`](R) reader structure"]
impl crate::Readable for SdmmcResp2Spec {}
#[doc = "`reset()` method sets SDMMC_RESP2 to value 0"]
impl crate::Resettable for SdmmcResp2Spec {
    const RESET_VALUE: u32 = 0;
}
