#[doc = "Register `HDCP_REVOC_SIZE_0` reader"]
pub type R = crate::R<HdcpRevocSize0Spec>;
#[doc = "Register `HDCP_REVOC_SIZE_0` writer"]
pub type W = crate::W<HdcpRevocSize0Spec>;
#[doc = "Field `HDCP_REVOC_SIZE_0` reader - Register containing the LSB of KSV list size\n\n(ksv_list_size\\[7:0\\]). If Attr has not been granted\n\n(see register a_ksvmemctrl), the value read is 8'hff."]
pub type HdcpRevocSize0R = crate::FieldReader;
#[doc = "Field `HDCP_REVOC_SIZE_0` writer - Register containing the LSB of KSV list size\n\n(ksv_list_size\\[7:0\\]). If Attr has not been granted\n\n(see register a_ksvmemctrl), the value read is 8'hff."]
pub type HdcpRevocSize0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Register containing the LSB of KSV list size\n\n(ksv_list_size\\[7:0\\]). If Attr has not been granted\n\n(see register a_ksvmemctrl), the value read is 8'hff."]
    #[inline(always)]
    pub fn hdcp_revoc_size_0(&self) -> HdcpRevocSize0R {
        HdcpRevocSize0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Register containing the LSB of KSV list size\n\n(ksv_list_size\\[7:0\\]). If Attr has not been granted\n\n(see register a_ksvmemctrl), the value read is 8'hff."]
    #[inline(always)]
    #[must_use]
    pub fn hdcp_revoc_size_0(&mut self) -> HdcpRevocSize0W<HdcpRevocSize0Spec> {
        HdcpRevocSize0W::new(self, 0)
    }
}
#[doc = "HDCP Revocation KSV List Size Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp_revoc_size_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp_revoc_size_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpRevocSize0Spec;
impl crate::RegisterSpec for HdcpRevocSize0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcp_revoc_size_0::R`](R) reader structure"]
impl crate::Readable for HdcpRevocSize0Spec {}
#[doc = "`write(|w| ..)` method takes [`hdcp_revoc_size_0::W`](W) writer structure"]
impl crate::Writable for HdcpRevocSize0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCP_REVOC_SIZE_0 to value 0xff"]
impl crate::Resettable for HdcpRevocSize0Spec {
    const RESET_VALUE: u8 = 0xff;
}
