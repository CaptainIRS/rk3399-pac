#[doc = "Register `SCU_B_PWRDN_CNT` reader"]
pub type R = crate::R<ScuBPwrdnCntSpec>;
#[doc = "Register `SCU_B_PWRDN_CNT` writer"]
pub type W = crate::W<ScuBPwrdnCntSpec>;
#[doc = "Field `PMU_SCU_B_PWRDN_CNT` reader - pmu scu_b power down counter value"]
pub type PmuScuBPwrdnCntR = crate::FieldReader<u32>;
#[doc = "Field `PMU_SCU_B_PWRDN_CNT` writer - pmu scu_b power down counter value"]
pub type PmuScuBPwrdnCntW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - pmu scu_b power down counter value"]
    #[inline(always)]
    pub fn pmu_scu_b_pwrdn_cnt(&self) -> PmuScuBPwrdnCntR {
        PmuScuBPwrdnCntR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - pmu scu_b power down counter value"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_scu_b_pwrdn_cnt(&mut self) -> PmuScuBPwrdnCntW<ScuBPwrdnCntSpec> {
        PmuScuBPwrdnCntW::new(self, 0)
    }
}
#[doc = "pmu scu_b power down count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scu_b_pwrdn_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scu_b_pwrdn_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScuBPwrdnCntSpec;
impl crate::RegisterSpec for ScuBPwrdnCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu_b_pwrdn_cnt::R`](R) reader structure"]
impl crate::Readable for ScuBPwrdnCntSpec {}
#[doc = "`write(|w| ..)` method takes [`scu_b_pwrdn_cnt::W`](W) writer structure"]
impl crate::Writable for ScuBPwrdnCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCU_B_PWRDN_CNT to value 0x5dc0"]
impl crate::Resettable for ScuBPwrdnCntSpec {
    const RESET_VALUE: u32 = 0x5dc0;
}
