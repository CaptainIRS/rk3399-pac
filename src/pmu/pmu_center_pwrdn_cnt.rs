#[doc = "Register `PMU_CENTER_PWRDN_CNT` reader"]
pub type R = crate::R<PmuCenterPwrdnCntSpec>;
#[doc = "Register `PMU_CENTER_PWRDN_CNT` writer"]
pub type W = crate::W<PmuCenterPwrdnCntSpec>;
#[doc = "Field `PMU_CENTER_PWRDN_CNT` reader - pmu center power down counter value"]
pub type PmuCenterPwrdnCntR = crate::FieldReader<u32>;
#[doc = "Field `PMU_CENTER_PWRDN_CNT` writer - pmu center power down counter value"]
pub type PmuCenterPwrdnCntW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - pmu center power down counter value"]
    #[inline(always)]
    pub fn pmu_center_pwrdn_cnt(&self) -> PmuCenterPwrdnCntR {
        PmuCenterPwrdnCntR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - pmu center power down counter value"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_center_pwrdn_cnt(&mut self) -> PmuCenterPwrdnCntW<PmuCenterPwrdnCntSpec> {
        PmuCenterPwrdnCntW::new(self, 0)
    }
}
#[doc = "pmu center power down count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_center_pwrdn_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_center_pwrdn_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuCenterPwrdnCntSpec;
impl crate::RegisterSpec for PmuCenterPwrdnCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_center_pwrdn_cnt::R`](R) reader structure"]
impl crate::Readable for PmuCenterPwrdnCntSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_center_pwrdn_cnt::W`](W) writer structure"]
impl crate::Writable for PmuCenterPwrdnCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_CENTER_PWRDN_CNT to value 0x5dc0"]
impl crate::Resettable for PmuCenterPwrdnCntSpec {
    const RESET_VALUE: u32 = 0x5dc0;
}
