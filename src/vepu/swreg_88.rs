#[doc = "Register `SWREG_88` reader"]
pub type R = crate::R<Swreg88Spec>;
#[doc = "Register `SWREG_88` writer"]
pub type W = crate::W<Swreg88Spec>;
#[doc = "Field `STAB_MATRIX5` reader - the 5st output of Stabilization matrix\n\n(position @GMV )"]
pub type StabMatrix5R = crate::FieldReader<u32>;
#[doc = "Field `STAB_MATRIX5` writer - the 5st output of Stabilization matrix\n\n(position @GMV )"]
pub type StabMatrix5W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - the 5st output of Stabilization matrix\n\n(position @GMV )"]
    #[inline(always)]
    pub fn stab_matrix5(&self) -> StabMatrix5R {
        StabMatrix5R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - the 5st output of Stabilization matrix\n\n(position @GMV )"]
    #[inline(always)]
    #[must_use]
    pub fn stab_matrix5(&mut self) -> StabMatrix5W<Swreg88Spec> {
        StabMatrix5W::new(self, 0)
    }
}
#[doc = "Stabilization matrix\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_88::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_88::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg88Spec;
impl crate::RegisterSpec for Swreg88Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_88::R`](R) reader structure"]
impl crate::Readable for Swreg88Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_88::W`](W) writer structure"]
impl crate::Writable for Swreg88Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_88 to value 0"]
impl crate::Resettable for Swreg88Spec {
    const RESET_VALUE: u32 = 0;
}
