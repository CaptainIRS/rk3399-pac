#[doc = "Register `PMU_PLLRST_CNT` reader"]
pub type R = crate::R<PmuPllrstCntSpec>;
#[doc = "Register `PMU_PLLRST_CNT` writer"]
pub type W = crate::W<PmuPllrstCntSpec>;
#[doc = "Field `PMU_PLLRST_CNT` reader - pmu pll reset counter value"]
pub type PmuPllrstCntR = crate::FieldReader<u32>;
#[doc = "Field `PMU_PLLRST_CNT` writer - pmu pll reset counter value"]
pub type PmuPllrstCntW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - pmu pll reset counter value"]
    #[inline(always)]
    pub fn pmu_pllrst_cnt(&self) -> PmuPllrstCntR {
        PmuPllrstCntR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - pmu pll reset counter value"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_pllrst_cnt(&mut self) -> PmuPllrstCntW<PmuPllrstCntSpec> {
        PmuPllrstCntW::new(self, 0)
    }
}
#[doc = "pmu pll reset count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_pllrst_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_pllrst_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuPllrstCntSpec;
impl crate::RegisterSpec for PmuPllrstCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_pllrst_cnt::R`](R) reader structure"]
impl crate::Readable for PmuPllrstCntSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_pllrst_cnt::W`](W) writer structure"]
impl crate::Writable for PmuPllrstCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_PLLRST_CNT to value 0"]
impl crate::Resettable for PmuPllrstCntSpec {
    const RESET_VALUE: u32 = 0;
}
