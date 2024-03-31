#[doc = "Register `AUTO_GATING` reader"]
pub type R = crate::R<AutoGatingSpec>;
#[doc = "Register `AUTO_GATING` writer"]
pub type W = crate::W<AutoGatingSpec>;
#[doc = "Field `MMU_ATUO_GATING` reader - mmu clock auto gating\n\nwhen it is 1, the mmu will auto gating itself"]
pub type MmuAtuoGatingR = crate::BitReader;
#[doc = "Field `MMU_ATUO_GATING` writer - mmu clock auto gating\n\nwhen it is 1, the mmu will auto gating itself"]
pub type MmuAtuoGatingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - mmu clock auto gating\n\nwhen it is 1, the mmu will auto gating itself"]
    #[inline(always)]
    pub fn mmu_atuo_gating(&self) -> MmuAtuoGatingR {
        MmuAtuoGatingR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - mmu clock auto gating\n\nwhen it is 1, the mmu will auto gating itself"]
    #[inline(always)]
    #[must_use]
    pub fn mmu_atuo_gating(&mut self) -> MmuAtuoGatingW<AutoGatingSpec> {
        MmuAtuoGatingW::new(self, 0)
    }
}
#[doc = "clock atuo gating register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_gating::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auto_gating::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AutoGatingSpec;
impl crate::RegisterSpec for AutoGatingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auto_gating::R`](R) reader structure"]
impl crate::Readable for AutoGatingSpec {}
#[doc = "`write(|w| ..)` method takes [`auto_gating::W`](W) writer structure"]
impl crate::Writable for AutoGatingSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUTO_GATING to value 0"]
impl crate::Resettable for AutoGatingSpec {
    const RESET_VALUE: u32 = 0;
}
