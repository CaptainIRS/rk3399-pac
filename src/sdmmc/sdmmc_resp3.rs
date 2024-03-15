#[doc = "Register `SDMMC_RESP3` reader"]
pub type R = crate::R<SdmmcResp3Spec>;
#[doc = "Field `RESPONSE3` reader - Bit\\[127:96\\]
of long response"]
pub type Response3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bit\\[127:96\\]
of long response"]
    #[inline(always)]
    pub fn response3(&self) -> Response3R {
        Response3R::new(self.bits)
    }
}
#[doc = "Response-3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_resp3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcResp3Spec;
impl crate::RegisterSpec for SdmmcResp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_resp3::R`](R) reader structure"]
impl crate::Readable for SdmmcResp3Spec {}
#[doc = "`reset()` method sets SDMMC_RESP3 to value 0"]
impl crate::Resettable for SdmmcResp3Spec {
    const RESET_VALUE: u32 = 0;
}
