#[doc = "Register `HDCPREG_AN1` reader"]
pub type R = crate::R<HdcpregAn1Spec>;
#[doc = "Register `HDCPREG_AN1` writer"]
pub type W = crate::W<HdcpregAn1Spec>;
#[doc = "Field `HDCPREG_AN1` reader - Contains the value of AN\\[15:8\\]"]
pub type HdcpregAn1R = crate::FieldReader;
#[doc = "Field `HDCPREG_AN1` writer - Contains the value of AN\\[15:8\\]"]
pub type HdcpregAn1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Contains the value of AN\\[15:8\\]"]
    #[inline(always)]
    pub fn hdcpreg_an1(&self) -> HdcpregAn1R {
        HdcpregAn1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains the value of AN\\[15:8\\]"]
    #[inline(always)]
    #[must_use]
    pub fn hdcpreg_an1(&mut self) -> HdcpregAn1W<HdcpregAn1Spec> {
        HdcpregAn1W::new(self, 0)
    }
}
#[doc = "HDCP Forced AN Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_an1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_an1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregAn1Spec;
impl crate::RegisterSpec for HdcpregAn1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcpreg_an1::R`](R) reader structure"]
impl crate::Readable for HdcpregAn1Spec {}
#[doc = "`write(|w| ..)` method takes [`hdcpreg_an1::W`](W) writer structure"]
impl crate::Writable for HdcpregAn1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCPREG_AN1 to value 0"]
impl crate::Resettable for HdcpregAn1Spec {
    const RESET_VALUE: u8 = 0;
}
