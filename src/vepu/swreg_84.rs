#[doc = "Register `SWREG_84` reader"]
pub type R = crate::R<Swreg84Spec>;
#[doc = "Register `SWREG_84` writer"]
pub type W = crate::W<Swreg84Spec>;
#[doc = "Field `STAB_MATRIX1` reader - the 1st output of Stabilization matrix\n\n(position@ up-left)"]
pub type StabMatrix1R = crate::FieldReader<u32>;
#[doc = "Field `STAB_MATRIX1` writer - the 1st output of Stabilization matrix\n\n(position@ up-left)"]
pub type StabMatrix1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - the 1st output of Stabilization matrix\n\n(position@ up-left)"]
    #[inline(always)]
    pub fn stab_matrix1(&self) -> StabMatrix1R {
        StabMatrix1R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - the 1st output of Stabilization matrix\n\n(position@ up-left)"]
    #[inline(always)]
    #[must_use]
    pub fn stab_matrix1(&mut self) -> StabMatrix1W<Swreg84Spec> {
        StabMatrix1W::new(self, 0)
    }
}
#[doc = "Stabilization matrix\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_84::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_84::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg84Spec;
impl crate::RegisterSpec for Swreg84Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_84::R`](R) reader structure"]
impl crate::Readable for Swreg84Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_84::W`](W) writer structure"]
impl crate::Writable for Swreg84Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_84 to value 0"]
impl crate::Resettable for Swreg84Spec {
    const RESET_VALUE: u32 = 0;
}
