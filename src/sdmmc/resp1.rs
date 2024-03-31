#[doc = "Register `RESP1` reader"]
pub type R = crate::R<Resp1Spec>;
#[doc = "Field `RESPONSE` reader - Register represents bit\\[63:32\\]
of long response.\n\nWhen CIU sends auto-stop command, then response is saved in\n\nregister. Response for previous command sent by host is still\n\npreserved in Response 0 register. Additional auto-stop issued\n\nonly for data transfer commands, and response type is always\n\n'short' for them."]
pub type ResponseR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register represents bit\\[63:32\\]
of long response.\n\nWhen CIU sends auto-stop command, then response is saved in\n\nregister. Response for previous command sent by host is still\n\npreserved in Response 0 register. Additional auto-stop issued\n\nonly for data transfer commands, and response type is always\n\n'short' for them."]
    #[inline(always)]
    pub fn response(&self) -> ResponseR {
        ResponseR::new(self.bits)
    }
}
#[doc = "Response-1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp1Spec;
impl crate::RegisterSpec for Resp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp1::R`](R) reader structure"]
impl crate::Readable for Resp1Spec {}
#[doc = "`reset()` method sets RESP1 to value 0"]
impl crate::Resettable for Resp1Spec {
    const RESET_VALUE: u32 = 0;
}
