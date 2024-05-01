#[doc = "Register `SWREG_62` reader"]
pub type R = crate::R<Swreg62Spec>;
#[doc = "Register `SWREG_62` writer"]
pub type W = crate::W<Swreg62Spec>;
#[doc = "Field `RLC_SUM` reader - rlc_sum\n\nrlc_sum"]
pub type RlcSumR = crate::FieldReader<u32>;
#[doc = "Field `RLC_SUM` writer - rlc_sum\n\nrlc_sum"]
pub type RlcSumW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - rlc_sum\n\nrlc_sum"]
    #[inline(always)]
    pub fn rlc_sum(&self) -> RlcSumR {
        RlcSumR::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - rlc_sum\n\nrlc_sum"]
    #[inline(always)]
    #[must_use]
    pub fn rlc_sum(&mut self) -> RlcSumW<Swreg62Spec> {
        RlcSumW::new(self, 0)
    }
}
#[doc = "rlc_sum\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_62::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_62::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg62Spec;
impl crate::RegisterSpec for Swreg62Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_62::R`](R) reader structure"]
impl crate::Readable for Swreg62Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_62::W`](W) writer structure"]
impl crate::Writable for Swreg62Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_62 to value 0"]
impl crate::Resettable for Swreg62Spec {
    const RESET_VALUE: u32 = 0;
}
