#[doc = "Register `HDCPREG_AN5` reader"]
pub type R = crate::R<HdcpregAn5Spec>;
#[doc = "Register `HDCPREG_AN5` writer"]
pub type W = crate::W<HdcpregAn5Spec>;
#[doc = "Field `HDCPREG_AN5` reader - Contains the value of AN\\[47:40\\]"]
pub type HdcpregAn5R = crate::FieldReader;
#[doc = "Field `HDCPREG_AN5` writer - Contains the value of AN\\[47:40\\]"]
pub type HdcpregAn5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Contains the value of AN\\[47:40\\]"]
    #[inline(always)]
    pub fn hdcpreg_an5(&self) -> HdcpregAn5R {
        HdcpregAn5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains the value of AN\\[47:40\\]"]
    #[inline(always)]
    #[must_use]
    pub fn hdcpreg_an5(&mut self) -> HdcpregAn5W<HdcpregAn5Spec> {
        HdcpregAn5W::new(self, 0)
    }
}
#[doc = "Contains the value of AN\\[47:40\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_an5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_an5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregAn5Spec;
impl crate::RegisterSpec for HdcpregAn5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcpreg_an5::R`](R) reader structure"]
impl crate::Readable for HdcpregAn5Spec {}
#[doc = "`write(|w| ..)` method takes [`hdcpreg_an5::W`](W) writer structure"]
impl crate::Writable for HdcpregAn5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCPREG_AN5 to value 0"]
impl crate::Resettable for HdcpregAn5Spec {
    const RESET_VALUE: u8 = 0;
}
