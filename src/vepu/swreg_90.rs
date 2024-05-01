#[doc = "Register `SWREG_90` reader"]
pub type R = crate::R<Swreg90Spec>;
#[doc = "Register `SWREG_90` writer"]
pub type W = crate::W<Swreg90Spec>;
#[doc = "Field `STAB_MATRIX7` reader - the 7st output of Stabilization matrix\n\n(position@down-left)"]
pub type StabMatrix7R = crate::FieldReader<u32>;
#[doc = "Field `STAB_MATRIX7` writer - the 7st output of Stabilization matrix\n\n(position@down-left)"]
pub type StabMatrix7W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - the 7st output of Stabilization matrix\n\n(position@down-left)"]
    #[inline(always)]
    pub fn stab_matrix7(&self) -> StabMatrix7R {
        StabMatrix7R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - the 7st output of Stabilization matrix\n\n(position@down-left)"]
    #[inline(always)]
    #[must_use]
    pub fn stab_matrix7(&mut self) -> StabMatrix7W<Swreg90Spec> {
        StabMatrix7W::new(self, 0)
    }
}
#[doc = "Stabilization matrix\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_90::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_90::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg90Spec;
impl crate::RegisterSpec for Swreg90Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_90::R`](R) reader structure"]
impl crate::Readable for Swreg90Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_90::W`](W) writer structure"]
impl crate::Writable for Swreg90Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_90 to value 0"]
impl crate::Resettable for Swreg90Spec {
    const RESET_VALUE: u32 = 0;
}
