#[doc = "Register `SDMMC_RESP1` reader"]
pub type R = crate::R<SdmmcResp1Spec>;
#[doc = "Field `RESPONSE` reader - Register represents bit\\[63:32\\]
of long response. When CIU sends auto-stop command, then response is saved in register. Response for previous command sent by host is still preserved in Response 0 register. Additional auto-stop issued only for data transfer commands, and response type is always \"short\" for them."]
pub type ResponseR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register represents bit\\[63:32\\]
of long response. When CIU sends auto-stop command, then response is saved in register. Response for previous command sent by host is still preserved in Response 0 register. Additional auto-stop issued only for data transfer commands, and response type is always \"short\" for them."]
    #[inline(always)]
    pub fn response(&self) -> ResponseR {
        ResponseR::new(self.bits)
    }
}
#[doc = "Response-1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_resp1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcResp1Spec;
impl crate::RegisterSpec for SdmmcResp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_resp1::R`](R) reader structure"]
impl crate::Readable for SdmmcResp1Spec {}
#[doc = "`reset()` method sets SDMMC_RESP1 to value 0"]
impl crate::Resettable for SdmmcResp1Spec {
    const RESET_VALUE: u32 = 0;
}
