#[doc = "Register `SWREG_51` reader"]
pub type R = crate::R<Swreg51Spec>;
#[doc = "Register `SWREG_51` writer"]
pub type W = crate::W<Swreg51Spec>;
#[doc = "Field `STRM_HEADER_LEFT_HBITS` reader - the high 32 bit of stram header be left\n\nthe high 32 bit of stram header be left"]
pub type StrmHeaderLeftHbitsR = crate::FieldReader<u32>;
#[doc = "Field `STRM_HEADER_LEFT_HBITS` writer - the high 32 bit of stram header be left\n\nthe high 32 bit of stram header be left"]
pub type StrmHeaderLeftHbitsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the high 32 bit of stram header be left\n\nthe high 32 bit of stram header be left"]
    #[inline(always)]
    pub fn strm_header_left_hbits(&self) -> StrmHeaderLeftHbitsR {
        StrmHeaderLeftHbitsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the high 32 bit of stram header be left\n\nthe high 32 bit of stram header be left"]
    #[inline(always)]
    #[must_use]
    pub fn strm_header_left_hbits(&mut self) -> StrmHeaderLeftHbitsW<Swreg51Spec> {
        StrmHeaderLeftHbitsW::new(self, 0)
    }
}
#[doc = "stream header bits left register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_51::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_51::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg51Spec;
impl crate::RegisterSpec for Swreg51Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_51::R`](R) reader structure"]
impl crate::Readable for Swreg51Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_51::W`](W) writer structure"]
impl crate::Writable for Swreg51Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_51 to value 0"]
impl crate::Resettable for Swreg51Spec {
    const RESET_VALUE: u32 = 0;
}
