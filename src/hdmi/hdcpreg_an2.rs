#[doc = "Register `HDCPREG_AN2` reader"]
pub type R = crate::R<HdcpregAn2Spec>;
#[doc = "Register `HDCPREG_AN2` writer"]
pub type W = crate::W<HdcpregAn2Spec>;
#[doc = "Field `HDCPREG_AN2` reader - Contains the value of AN\\[23:16\\]"]
pub type HdcpregAn2R = crate::FieldReader;
#[doc = "Field `HDCPREG_AN2` writer - Contains the value of AN\\[23:16\\]"]
pub type HdcpregAn2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Contains the value of AN\\[23:16\\]"]
    #[inline(always)]
    pub fn hdcpreg_an2(&self) -> HdcpregAn2R {
        HdcpregAn2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains the value of AN\\[23:16\\]"]
    #[inline(always)]
    #[must_use]
    pub fn hdcpreg_an2(&mut self) -> HdcpregAn2W<HdcpregAn2Spec> {
        HdcpregAn2W::new(self, 0)
    }
}
#[doc = "HDCP forced AN Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_an2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_an2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregAn2Spec;
impl crate::RegisterSpec for HdcpregAn2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcpreg_an2::R`](R) reader structure"]
impl crate::Readable for HdcpregAn2Spec {}
#[doc = "`write(|w| ..)` method takes [`hdcpreg_an2::W`](W) writer structure"]
impl crate::Writable for HdcpregAn2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCPREG_AN2 to value 0"]
impl crate::Resettable for HdcpregAn2Spec {
    const RESET_VALUE: u8 = 0;
}
