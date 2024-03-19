#[doc = "Register `HDCPREG_AN3` reader"]
pub type R = crate::R<HdcpregAn3Spec>;
#[doc = "Register `HDCPREG_AN3` writer"]
pub type W = crate::W<HdcpregAn3Spec>;
#[doc = "Field `HDCPREG_AN3` reader - Contains the value of AN\\[31:24\\]"]
pub type HdcpregAn3R = crate::FieldReader;
#[doc = "Field `HDCPREG_AN3` writer - Contains the value of AN\\[31:24\\]"]
pub type HdcpregAn3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Contains the value of AN\\[31:24\\]"]
    #[inline(always)]
    pub fn hdcpreg_an3(&self) -> HdcpregAn3R {
        HdcpregAn3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains the value of AN\\[31:24\\]"]
    #[inline(always)]
    #[must_use]
    pub fn hdcpreg_an3(&mut self) -> HdcpregAn3W<HdcpregAn3Spec> {
        HdcpregAn3W::new(self, 0)
    }
}
#[doc = "HDCP Forced AN Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_an3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_an3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregAn3Spec;
impl crate::RegisterSpec for HdcpregAn3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcpreg_an3::R`](R) reader structure"]
impl crate::Readable for HdcpregAn3Spec {}
#[doc = "`write(|w| ..)` method takes [`hdcpreg_an3::W`](W) writer structure"]
impl crate::Writable for HdcpregAn3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCPREG_AN3 to value 0"]
impl crate::Resettable for HdcpregAn3Spec {
    const RESET_VALUE: u8 = 0;
}
