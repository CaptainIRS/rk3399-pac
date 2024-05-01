#[doc = "Register `SWREG_91` reader"]
pub type R = crate::R<Swreg91Spec>;
#[doc = "Register `SWREG_91` writer"]
pub type W = crate::W<Swreg91Spec>;
#[doc = "Field `STAB_MATRIX8` reader - the 8st output of Stabilization matrix\n\n(position@down )"]
pub type StabMatrix8R = crate::FieldReader<u32>;
#[doc = "Field `STAB_MATRIX8` writer - the 8st output of Stabilization matrix\n\n(position@down )"]
pub type StabMatrix8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - the 8st output of Stabilization matrix\n\n(position@down )"]
    #[inline(always)]
    pub fn stab_matrix8(&self) -> StabMatrix8R {
        StabMatrix8R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - the 8st output of Stabilization matrix\n\n(position@down )"]
    #[inline(always)]
    #[must_use]
    pub fn stab_matrix8(&mut self) -> StabMatrix8W<Swreg91Spec> {
        StabMatrix8W::new(self, 0)
    }
}
#[doc = "Stabilization matrix\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_91::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_91::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg91Spec;
impl crate::RegisterSpec for Swreg91Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_91::R`](R) reader structure"]
impl crate::Readable for Swreg91Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_91::W`](W) writer structure"]
impl crate::Writable for Swreg91Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_91 to value 0"]
impl crate::Resettable for Swreg91Spec {
    const RESET_VALUE: u32 = 0;
}
