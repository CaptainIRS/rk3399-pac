#[doc = "Register `HDCPREG_AN7` reader"]
pub type R = crate::R<HdcpregAn7Spec>;
#[doc = "Register `HDCPREG_AN7` writer"]
pub type W = crate::W<HdcpregAn7Spec>;
#[doc = "Field `HDCPREG_AN7` reader - Contains the value of BKSV\\[63:56\\]"]
pub type HdcpregAn7R = crate::FieldReader;
#[doc = "Field `HDCPREG_AN7` writer - Contains the value of BKSV\\[63:56\\]"]
pub type HdcpregAn7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Contains the value of BKSV\\[63:56\\]"]
    #[inline(always)]
    pub fn hdcpreg_an7(&self) -> HdcpregAn7R {
        HdcpregAn7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains the value of BKSV\\[63:56\\]"]
    #[inline(always)]
    #[must_use]
    pub fn hdcpreg_an7(&mut self) -> HdcpregAn7W<HdcpregAn7Spec> {
        HdcpregAn7W::new(self, 0)
    }
}
#[doc = "HDCP Forced AN Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_an7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_an7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregAn7Spec;
impl crate::RegisterSpec for HdcpregAn7Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcpreg_an7::R`](R) reader structure"]
impl crate::Readable for HdcpregAn7Spec {}
#[doc = "`write(|w| ..)` method takes [`hdcpreg_an7::W`](W) writer structure"]
impl crate::Writable for HdcpregAn7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCPREG_AN7 to value 0"]
impl crate::Resettable for HdcpregAn7Spec {
    const RESET_VALUE: u8 = 0;
}
