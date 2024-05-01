#[doc = "Register `SWREG_52` reader"]
pub type R = crate::R<Swreg52Spec>;
#[doc = "Register `SWREG_52` writer"]
pub type W = crate::W<Swreg52Spec>;
#[doc = "Field `STRM_HEADER_LEFT_LBITS` reader - the low 32 bit of stram header be left\n\nthe low 32 bit of stram header be left"]
pub type StrmHeaderLeftLbitsR = crate::FieldReader<u32>;
#[doc = "Field `STRM_HEADER_LEFT_LBITS` writer - the low 32 bit of stram header be left\n\nthe low 32 bit of stram header be left"]
pub type StrmHeaderLeftLbitsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the low 32 bit of stram header be left\n\nthe low 32 bit of stram header be left"]
    #[inline(always)]
    pub fn strm_header_left_lbits(&self) -> StrmHeaderLeftLbitsR {
        StrmHeaderLeftLbitsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the low 32 bit of stram header be left\n\nthe low 32 bit of stram header be left"]
    #[inline(always)]
    #[must_use]
    pub fn strm_header_left_lbits(&mut self) -> StrmHeaderLeftLbitsW<Swreg52Spec> {
        StrmHeaderLeftLbitsW::new(self, 0)
    }
}
#[doc = "stream header bits left register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_52::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_52::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg52Spec;
impl crate::RegisterSpec for Swreg52Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_52::R`](R) reader structure"]
impl crate::Readable for Swreg52Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_52::W`](W) writer structure"]
impl crate::Writable for Swreg52Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_52 to value 0"]
impl crate::Resettable for Swreg52Spec {
    const RESET_VALUE: u32 = 0;
}
