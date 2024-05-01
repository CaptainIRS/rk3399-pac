#[doc = "Register `TRNG_DOUT_2` reader"]
pub type R = crate::R<TrngDout2Spec>;
#[doc = "Register `TRNG_DOUT_2` writer"]
pub type W = crate::W<TrngDout2Spec>;
#[doc = "Field `TRNG_OUTPUT` reader - TRNG output"]
pub type TrngOutputR = crate::FieldReader<u32>;
#[doc = "Field `TRNG_OUTPUT` writer - TRNG output"]
pub type TrngOutputW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRNG output"]
    #[inline(always)]
    pub fn trng_output(&self) -> TrngOutputR {
        TrngOutputR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRNG output"]
    #[inline(always)]
    #[must_use]
    pub fn trng_output(&mut self) -> TrngOutputW<TrngDout2Spec> {
        TrngOutputW::new(self, 0)
    }
}
#[doc = "TRNG output register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trng_dout_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trng_dout_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngDout2Spec;
impl crate::RegisterSpec for TrngDout2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trng_dout_2::R`](R) reader structure"]
impl crate::Readable for TrngDout2Spec {}
#[doc = "`write(|w| ..)` method takes [`trng_dout_2::W`](W) writer structure"]
impl crate::Writable for TrngDout2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRNG_DOUT_2 to value 0"]
impl crate::Resettable for TrngDout2Spec {
    const RESET_VALUE: u32 = 0;
}
