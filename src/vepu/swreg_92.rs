#[doc = "Register `SWREG_92` reader"]
pub type R = crate::R<Swreg92Spec>;
#[doc = "Register `SWREG_92` writer"]
pub type W = crate::W<Swreg92Spec>;
#[doc = "Field `STAB_MATRIX9` reader - the 9st output of Stabilization matrix\n\n(position@down- right )"]
pub type StabMatrix9R = crate::FieldReader<u32>;
#[doc = "Field `STAB_MATRIX9` writer - the 9st output of Stabilization matrix\n\n(position@down- right )"]
pub type StabMatrix9W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `STAB_GMV_VRTL` reader - the output of Stabilization GMV vertical\n\nsigned register\n\nrange : -16~16"]
pub type StabGmvVrtlR = crate::FieldReader;
#[doc = "Field `STAB_GMV_VRTL` writer - the output of Stabilization GMV vertical\n\nsigned register\n\nrange : -16~16"]
pub type StabGmvVrtlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:23 - the 9st output of Stabilization matrix\n\n(position@down- right )"]
    #[inline(always)]
    pub fn stab_matrix9(&self) -> StabMatrix9R {
        StabMatrix9R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 26:31 - the output of Stabilization GMV vertical\n\nsigned register\n\nrange : -16~16"]
    #[inline(always)]
    pub fn stab_gmv_vrtl(&self) -> StabGmvVrtlR {
        StabGmvVrtlR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - the 9st output of Stabilization matrix\n\n(position@down- right )"]
    #[inline(always)]
    #[must_use]
    pub fn stab_matrix9(&mut self) -> StabMatrix9W<Swreg92Spec> {
        StabMatrix9W::new(self, 0)
    }
    #[doc = "Bits 26:31 - the output of Stabilization GMV vertical\n\nsigned register\n\nrange : -16~16"]
    #[inline(always)]
    #[must_use]
    pub fn stab_gmv_vrtl(&mut self) -> StabGmvVrtlW<Swreg92Spec> {
        StabGmvVrtlW::new(self, 26)
    }
}
#[doc = "Stabilization matrix\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_92::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_92::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg92Spec;
impl crate::RegisterSpec for Swreg92Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_92::R`](R) reader structure"]
impl crate::Readable for Swreg92Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_92::W`](W) writer structure"]
impl crate::Writable for Swreg92Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_92 to value 0"]
impl crate::Resettable for Swreg92Spec {
    const RESET_VALUE: u32 = 0;
}
