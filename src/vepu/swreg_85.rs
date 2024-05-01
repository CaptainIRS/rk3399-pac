#[doc = "Register `SWREG_85` reader"]
pub type R = crate::R<Swreg85Spec>;
#[doc = "Register `SWREG_85` writer"]
pub type W = crate::W<Swreg85Spec>;
#[doc = "Field `STAB_MATRIX2` reader - the 2st output of Stabilization matrix\n\n(position @ up)"]
pub type StabMatrix2R = crate::FieldReader<u32>;
#[doc = "Field `STAB_MATRIX2` writer - the 2st output of Stabilization matrix\n\n(position @ up)"]
pub type StabMatrix2W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - the 2st output of Stabilization matrix\n\n(position @ up)"]
    #[inline(always)]
    pub fn stab_matrix2(&self) -> StabMatrix2R {
        StabMatrix2R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - the 2st output of Stabilization matrix\n\n(position @ up)"]
    #[inline(always)]
    #[must_use]
    pub fn stab_matrix2(&mut self) -> StabMatrix2W<Swreg85Spec> {
        StabMatrix2W::new(self, 0)
    }
}
#[doc = "Stabilization matrix\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_85::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_85::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg85Spec;
impl crate::RegisterSpec for Swreg85Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_85::R`](R) reader structure"]
impl crate::Readable for Swreg85Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_85::W`](W) writer structure"]
impl crate::Writable for Swreg85Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_85 to value 0"]
impl crate::Resettable for Swreg85Spec {
    const RESET_VALUE: u32 = 0;
}
