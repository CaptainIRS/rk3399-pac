#[doc = "Register `HDCPREG_AN6` reader"]
pub type R = crate::R<HdcpregAn6Spec>;
#[doc = "Register `HDCPREG_AN6` writer"]
pub type W = crate::W<HdcpregAn6Spec>;
#[doc = "Field `HDCPREG_AN6` reader - Contains the value of AN\\[55:48\\]"]
pub type HdcpregAn6R = crate::FieldReader;
#[doc = "Field `HDCPREG_AN6` writer - Contains the value of AN\\[55:48\\]"]
pub type HdcpregAn6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Contains the value of AN\\[55:48\\]"]
    #[inline(always)]
    pub fn hdcpreg_an6(&self) -> HdcpregAn6R {
        HdcpregAn6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains the value of AN\\[55:48\\]"]
    #[inline(always)]
    #[must_use]
    pub fn hdcpreg_an6(&mut self) -> HdcpregAn6W<HdcpregAn6Spec> {
        HdcpregAn6W::new(self, 0)
    }
}
#[doc = "HDCP Forced AN Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_an6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_an6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregAn6Spec;
impl crate::RegisterSpec for HdcpregAn6Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcpreg_an6::R`](R) reader structure"]
impl crate::Readable for HdcpregAn6Spec {}
#[doc = "`write(|w| ..)` method takes [`hdcpreg_an6::W`](W) writer structure"]
impl crate::Writable for HdcpregAn6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCPREG_AN6 to value 0"]
impl crate::Resettable for HdcpregAn6Spec {
    const RESET_VALUE: u8 = 0;
}
