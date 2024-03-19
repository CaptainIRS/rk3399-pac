#[doc = "Register `HDCP_M0[%s]` reader"]
pub type R = crate::R<HdcpM0Spec>;
#[doc = "Register `HDCP_M0[%s]` writer"]
pub type W = crate::W<HdcpM0Spec>;
#[doc = "Field `M0` reader - HDCP M0\\[32:0\\]. If Attr has not been granted (see\n\nregister a_ksvmemctrl) , the value read will be\n\n8'hff. These values are only available on a\n\nconfiguration that has the SHA1 calculation by\n\nsoftware."]
pub type M0R = crate::FieldReader;
#[doc = "Field `M0` writer - HDCP M0\\[32:0\\]. If Attr has not been granted (see\n\nregister a_ksvmemctrl) , the value read will be\n\n8'hff. These values are only available on a\n\nconfiguration that has the SHA1 calculation by\n\nsoftware."]
pub type M0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - HDCP M0\\[32:0\\]. If Attr has not been granted (see\n\nregister a_ksvmemctrl) , the value read will be\n\n8'hff. These values are only available on a\n\nconfiguration that has the SHA1 calculation by\n\nsoftware."]
    #[inline(always)]
    pub fn m0(&self) -> M0R {
        M0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - HDCP M0\\[32:0\\]. If Attr has not been granted (see\n\nregister a_ksvmemctrl) , the value read will be\n\n8'hff. These values are only available on a\n\nconfiguration that has the SHA1 calculation by\n\nsoftware."]
    #[inline(always)]
    #[must_use]
    pub fn m0(&mut self) -> M0W<HdcpM0Spec> {
        M0W::new(self, 0)
    }
}
#[doc = "HDCP M0 Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp_m0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp_m0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpM0Spec;
impl crate::RegisterSpec for HdcpM0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcp_m0::R`](R) reader structure"]
impl crate::Readable for HdcpM0Spec {}
#[doc = "`write(|w| ..)` method takes [`hdcp_m0::W`](W) writer structure"]
impl crate::Writable for HdcpM0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCP_M0[%s]
to value 0xff"]
impl crate::Resettable for HdcpM0Spec {
    const RESET_VALUE: u8 = 0xff;
}
