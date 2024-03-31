#[doc = "Register `CENTER_PWRDN_CNT` reader"]
pub type R = crate::R<CenterPwrdnCntSpec>;
#[doc = "Register `CENTER_PWRDN_CNT` writer"]
pub type W = crate::W<CenterPwrdnCntSpec>;
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
    pub fn pmu_center_pwrdn_cnt(&mut self) -> PmuCenterPwrdnCntW<CenterPwrdnCntSpec> {
        PmuCenterPwrdnCntW::new(self, 0)
    }
}
#[doc = "pmu center power down count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`center_pwrdn_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`center_pwrdn_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CenterPwrdnCntSpec;
impl crate::RegisterSpec for CenterPwrdnCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`center_pwrdn_cnt::R`](R) reader structure"]
impl crate::Readable for CenterPwrdnCntSpec {}
#[doc = "`write(|w| ..)` method takes [`center_pwrdn_cnt::W`](W) writer structure"]
impl crate::Writable for CenterPwrdnCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CENTER_PWRDN_CNT to value 0x5dc0"]
impl crate::Resettable for CenterPwrdnCntSpec {
    const RESET_VALUE: u32 = 0x5dc0;
}
