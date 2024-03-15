#[doc = "Register `SDMMC_RESP0` reader"]
pub type R = crate::R<SdmmcResp0Spec>;
#[doc = "Field `RESPONSE0` reader - Bit\\[31:0\\]
of response"]
pub type Response0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bit\\[31:0\\]
of response"]
    #[inline(always)]
    pub fn response0(&self) -> Response0R {
        Response0R::new(self.bits)
    }
}
#[doc = "Response-0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_resp0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcResp0Spec;
impl crate::RegisterSpec for SdmmcResp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_resp0::R`](R) reader structure"]
impl crate::Readable for SdmmcResp0Spec {}
#[doc = "`reset()` method sets SDMMC_RESP0 to value 0"]
impl crate::Resettable for SdmmcResp0Spec {
    const RESET_VALUE: u32 = 0;
}
