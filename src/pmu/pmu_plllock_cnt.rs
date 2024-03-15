#[doc = "Register `PMU_PLLLOCK_CNT` reader"]
pub type R = crate::R<PmuPlllockCntSpec>;
#[doc = "Register `PMU_PLLLOCK_CNT` writer"]
pub type W = crate::W<PmuPlllockCntSpec>;
#[doc = "Field `PMU_PLLLOCK_CNT` reader - pmu pll lock counter value"]
pub type PmuPlllockCntR = crate::FieldReader<u32>;
#[doc = "Field `PMU_PLLLOCK_CNT` writer - pmu pll lock counter value"]
pub type PmuPlllockCntW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - pmu pll lock counter value"]
    #[inline(always)]
    pub fn pmu_plllock_cnt(&self) -> PmuPlllockCntR {
        PmuPlllockCntR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - pmu pll lock counter value"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_plllock_cnt(&mut self) -> PmuPlllockCntW<PmuPlllockCntSpec> {
        PmuPlllockCntW::new(self, 0)
    }
}
#[doc = "pmu pll lock count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_plllock_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_plllock_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuPlllockCntSpec;
impl crate::RegisterSpec for PmuPlllockCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_plllock_cnt::R`](R) reader structure"]
impl crate::Readable for PmuPlllockCntSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_plllock_cnt::W`](W) writer structure"]
impl crate::Writable for PmuPlllockCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_PLLLOCK_CNT to value 0"]
impl crate::Resettable for PmuPlllockCntSpec {
    const RESET_VALUE: u32 = 0;
}
