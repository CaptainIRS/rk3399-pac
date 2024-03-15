#[doc = "Register `PMU_OSC_CNT` reader"]
pub type R = crate::R<PmuOscCntSpec>;
#[doc = "Register `PMU_OSC_CNT` writer"]
pub type W = crate::W<PmuOscCntSpec>;
#[doc = "Field `PMU_OSC_CNT` reader - pmu osc stable counter value"]
pub type PmuOscCntR = crate::FieldReader<u32>;
#[doc = "Field `PMU_OSC_CNT` writer - pmu osc stable counter value"]
pub type PmuOscCntW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - pmu osc stable counter value"]
    #[inline(always)]
    pub fn pmu_osc_cnt(&self) -> PmuOscCntR {
        PmuOscCntR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - pmu osc stable counter value"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_osc_cnt(&mut self) -> PmuOscCntW<PmuOscCntSpec> {
        PmuOscCntW::new(self, 0)
    }
}
#[doc = "pmu osc count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_osc_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_osc_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuOscCntSpec;
impl crate::RegisterSpec for PmuOscCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_osc_cnt::R`](R) reader structure"]
impl crate::Readable for PmuOscCntSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_osc_cnt::W`](W) writer structure"]
impl crate::Writable for PmuOscCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_OSC_CNT to value 0"]
impl crate::Resettable for PmuOscCntSpec {
    const RESET_VALUE: u32 = 0;
}
