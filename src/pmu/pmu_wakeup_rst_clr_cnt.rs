#[doc = "Register `PMU_WAKEUP_RST_CLR_CNT` reader"]
pub type R = crate::R<PmuWakeupRstClrCntSpec>;
#[doc = "Register `PMU_WAKEUP_RST_CLR_CNT` writer"]
pub type W = crate::W<PmuWakeupRstClrCntSpec>;
#[doc = "Field `PMU_WAKEUP_RST_CNT` reader - pmu wakeup reset counter value"]
pub type PmuWakeupRstCntR = crate::FieldReader<u32>;
#[doc = "Field `PMU_WAKEUP_RST_CNT` writer - pmu wakeup reset counter value"]
pub type PmuWakeupRstCntW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - pmu wakeup reset counter value"]
    #[inline(always)]
    pub fn pmu_wakeup_rst_cnt(&self) -> PmuWakeupRstCntR {
        PmuWakeupRstCntR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - pmu wakeup reset counter value"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_wakeup_rst_cnt(&mut self) -> PmuWakeupRstCntW<PmuWakeupRstClrCntSpec> {
        PmuWakeupRstCntW::new(self, 0)
    }
}
#[doc = "pmu wakeup reset clear count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_wakeup_rst_clr_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_wakeup_rst_clr_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuWakeupRstClrCntSpec;
impl crate::RegisterSpec for PmuWakeupRstClrCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_wakeup_rst_clr_cnt::R`](R) reader structure"]
impl crate::Readable for PmuWakeupRstClrCntSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_wakeup_rst_clr_cnt::W`](W) writer structure"]
impl crate::Writable for PmuWakeupRstClrCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_WAKEUP_RST_CLR_CNT to value 0"]
impl crate::Resettable for PmuWakeupRstClrCntSpec {
    const RESET_VALUE: u32 = 0;
}
