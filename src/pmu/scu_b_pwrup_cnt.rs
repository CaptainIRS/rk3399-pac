#[doc = "Register `SCU_B_PWRUP_CNT` reader"]
pub type R = crate::R<ScuBPwrupCntSpec>;
#[doc = "Register `SCU_B_PWRUP_CNT` writer"]
pub type W = crate::W<ScuBPwrupCntSpec>;
#[doc = "Field `PMU_SCU_B_PWRUP_CNT` reader - pmu scu_b power up counter value"]
pub type PmuScuBPwrupCntR = crate::FieldReader<u32>;
#[doc = "Field `PMU_SCU_B_PWRUP_CNT` writer - pmu scu_b power up counter value"]
pub type PmuScuBPwrupCntW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - pmu scu_b power up counter value"]
    #[inline(always)]
    pub fn pmu_scu_b_pwrup_cnt(&self) -> PmuScuBPwrupCntR {
        PmuScuBPwrupCntR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - pmu scu_b power up counter value"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_scu_b_pwrup_cnt(&mut self) -> PmuScuBPwrupCntW<ScuBPwrupCntSpec> {
        PmuScuBPwrupCntW::new(self, 0)
    }
}
#[doc = "pmu scu_b power up count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scu_b_pwrup_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scu_b_pwrup_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScuBPwrupCntSpec;
impl crate::RegisterSpec for ScuBPwrupCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu_b_pwrup_cnt::R`](R) reader structure"]
impl crate::Readable for ScuBPwrupCntSpec {}
#[doc = "`write(|w| ..)` method takes [`scu_b_pwrup_cnt::W`](W) writer structure"]
impl crate::Writable for ScuBPwrupCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCU_B_PWRUP_CNT to value 0x5dc0"]
impl crate::Resettable for ScuBPwrupCntSpec {
    const RESET_VALUE: u32 = 0x5dc0;
}
