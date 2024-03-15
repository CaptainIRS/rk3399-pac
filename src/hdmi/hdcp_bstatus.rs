#[doc = "Register `HDCP_BSTATUS[%s]` reader"]
pub type R = crate::R<HdcpBstatusSpec>;
#[doc = "Register `HDCP_BSTATUS[%s]` writer"]
pub type W = crate::W<HdcpBstatusSpec>;
#[doc = "Field `BSTATUS` reader - HDCP BSTATUS\\[15:0\\]. If Attr has not been granted (see register a_ksvmemctrl), the value read will be 8'hff."]
pub type BstatusR = crate::FieldReader;
#[doc = "Field `BSTATUS` writer - HDCP BSTATUS\\[15:0\\]. If Attr has not been granted (see register a_ksvmemctrl), the value read will be 8'hff."]
pub type BstatusW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - HDCP BSTATUS\\[15:0\\]. If Attr has not been granted (see register a_ksvmemctrl), the value read will be 8'hff."]
    #[inline(always)]
    pub fn bstatus(&self) -> BstatusR {
        BstatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - HDCP BSTATUS\\[15:0\\]. If Attr has not been granted (see register a_ksvmemctrl), the value read will be 8'hff."]
    #[inline(always)]
    #[must_use]
    pub fn bstatus(&mut self) -> BstatusW<HdcpBstatusSpec> {
        BstatusW::new(self, 0)
    }
}
#[doc = "HDCP BSTATUS\\[15:0\\]. If Attr has not been granted (see register a_ksvmemctrl), the value read will be 8'hff.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp_bstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp_bstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpBstatusSpec;
impl crate::RegisterSpec for HdcpBstatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcp_bstatus::R`](R) reader structure"]
impl crate::Readable for HdcpBstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`hdcp_bstatus::W`](W) writer structure"]
impl crate::Writable for HdcpBstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCP_BSTATUS[%s]
to value 0xff"]
impl crate::Resettable for HdcpBstatusSpec {
    const RESET_VALUE: u8 = 0xff;
}
