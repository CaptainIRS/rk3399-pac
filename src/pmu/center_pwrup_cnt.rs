#[doc = "Register `CENTER_PWRUP_CNT` reader"]
pub type R = crate::R<CenterPwrupCntSpec>;
#[doc = "Register `CENTER_PWRUP_CNT` writer"]
pub type W = crate::W<CenterPwrupCntSpec>;
#[doc = "Field `PMU_CENTER_PWRUP_CNT` reader - pmu center power up counter value"]
pub type PmuCenterPwrupCntR = crate::FieldReader<u32>;
#[doc = "Field `PMU_CENTER_PWRUP_CNT` writer - pmu center power up counter value"]
pub type PmuCenterPwrupCntW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - pmu center power up counter value"]
    #[inline(always)]
    pub fn pmu_center_pwrup_cnt(&self) -> PmuCenterPwrupCntR {
        PmuCenterPwrupCntR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - pmu center power up counter value"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_center_pwrup_cnt(&mut self) -> PmuCenterPwrupCntW<CenterPwrupCntSpec> {
        PmuCenterPwrupCntW::new(self, 0)
    }
}
#[doc = "pmu center power up count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`center_pwrup_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`center_pwrup_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CenterPwrupCntSpec;
impl crate::RegisterSpec for CenterPwrupCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`center_pwrup_cnt::R`](R) reader structure"]
impl crate::Readable for CenterPwrupCntSpec {}
#[doc = "`write(|w| ..)` method takes [`center_pwrup_cnt::W`](W) writer structure"]
impl crate::Writable for CenterPwrupCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CENTER_PWRUP_CNT to value 0x5dc0"]
impl crate::Resettable for CenterPwrupCntSpec {
    const RESET_VALUE: u32 = 0x5dc0;
}
