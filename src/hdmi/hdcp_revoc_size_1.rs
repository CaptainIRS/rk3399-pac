#[doc = "Register `HDCP_REVOC_SIZE_1` reader"]
pub type R = crate::R<HdcpRevocSize1Spec>;
#[doc = "Register `HDCP_REVOC_SIZE_1` writer"]
pub type W = crate::W<HdcpRevocSize1Spec>;
#[doc = "Field `HDCP_REVOC_SIZE_1` reader - Register containing the MSB of KSV list size\n\n(ksv_list_size\\[15:8\\]). If Attr has not been granted\n\n(see register a_ksvmemctrl), the value read is 8'hff."]
pub type HdcpRevocSize1R = crate::FieldReader;
#[doc = "Field `HDCP_REVOC_SIZE_1` writer - Register containing the MSB of KSV list size\n\n(ksv_list_size\\[15:8\\]). If Attr has not been granted\n\n(see register a_ksvmemctrl), the value read is 8'hff."]
pub type HdcpRevocSize1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Register containing the MSB of KSV list size\n\n(ksv_list_size\\[15:8\\]). If Attr has not been granted\n\n(see register a_ksvmemctrl), the value read is 8'hff."]
    #[inline(always)]
    pub fn hdcp_revoc_size_1(&self) -> HdcpRevocSize1R {
        HdcpRevocSize1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Register containing the MSB of KSV list size\n\n(ksv_list_size\\[15:8\\]). If Attr has not been granted\n\n(see register a_ksvmemctrl), the value read is 8'hff."]
    #[inline(always)]
    #[must_use]
    pub fn hdcp_revoc_size_1(&mut self) -> HdcpRevocSize1W<HdcpRevocSize1Spec> {
        HdcpRevocSize1W::new(self, 0)
    }
}
#[doc = "HDCP Revocation KSV List Size Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp_revoc_size_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp_revoc_size_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpRevocSize1Spec;
impl crate::RegisterSpec for HdcpRevocSize1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcp_revoc_size_1::R`](R) reader structure"]
impl crate::Readable for HdcpRevocSize1Spec {}
#[doc = "`write(|w| ..)` method takes [`hdcp_revoc_size_1::W`](W) writer structure"]
impl crate::Writable for HdcpRevocSize1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCP_REVOC_SIZE_1 to value 0xff"]
impl crate::Resettable for HdcpRevocSize1Spec {
    const RESET_VALUE: u8 = 0xff;
}
