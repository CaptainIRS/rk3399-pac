#[doc = "Register `HDCPREG_AN0` reader"]
pub type R = crate::R<HdcpregAn0Spec>;
#[doc = "Register `HDCPREG_AN0` writer"]
pub type W = crate::W<HdcpregAn0Spec>;
#[doc = "Field `HDCPREG_AN0` reader - Contains the value of AN\\[7:0\\]"]
pub type HdcpregAn0R = crate::FieldReader;
#[doc = "Field `HDCPREG_AN0` writer - Contains the value of AN\\[7:0\\]"]
pub type HdcpregAn0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Contains the value of AN\\[7:0\\]"]
    #[inline(always)]
    pub fn hdcpreg_an0(&self) -> HdcpregAn0R {
        HdcpregAn0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains the value of AN\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn hdcpreg_an0(&mut self) -> HdcpregAn0W<HdcpregAn0Spec> {
        HdcpregAn0W::new(self, 0)
    }
}
#[doc = "HDCP Forced AN Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_an0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_an0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregAn0Spec;
impl crate::RegisterSpec for HdcpregAn0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcpreg_an0::R`](R) reader structure"]
impl crate::Readable for HdcpregAn0Spec {}
#[doc = "`write(|w| ..)` method takes [`hdcpreg_an0::W`](W) writer structure"]
impl crate::Writable for HdcpregAn0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCPREG_AN0 to value 0"]
impl crate::Resettable for HdcpregAn0Spec {
    const RESET_VALUE: u8 = 0;
}
