#[doc = "Register `HDCPREG_AN4` reader"]
pub type R = crate::R<HdcpregAn4Spec>;
#[doc = "Register `HDCPREG_AN4` writer"]
pub type W = crate::W<HdcpregAn4Spec>;
#[doc = "Field `HDCPREG_AN4` reader - Contains the value of AN\\[39:32\\]"]
pub type HdcpregAn4R = crate::FieldReader;
#[doc = "Field `HDCPREG_AN4` writer - Contains the value of AN\\[39:32\\]"]
pub type HdcpregAn4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Contains the value of AN\\[39:32\\]"]
    #[inline(always)]
    pub fn hdcpreg_an4(&self) -> HdcpregAn4R {
        HdcpregAn4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains the value of AN\\[39:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn hdcpreg_an4(&mut self) -> HdcpregAn4W<HdcpregAn4Spec> {
        HdcpregAn4W::new(self, 0)
    }
}
#[doc = "HDCP Forced AN Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_an4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_an4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregAn4Spec;
impl crate::RegisterSpec for HdcpregAn4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcpreg_an4::R`](R) reader structure"]
impl crate::Readable for HdcpregAn4Spec {}
#[doc = "`write(|w| ..)` method takes [`hdcpreg_an4::W`](W) writer structure"]
impl crate::Writable for HdcpregAn4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCPREG_AN4 to value 0"]
impl crate::Resettable for HdcpregAn4Spec {
    const RESET_VALUE: u8 = 0;
}
