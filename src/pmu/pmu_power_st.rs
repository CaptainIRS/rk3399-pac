#[doc = "Register `PMU_POWER_ST` reader"]
pub type R = crate::R<PmuPowerStSpec>;
#[doc = "Register `PMU_POWER_ST` writer"]
pub type W = crate::W<PmuPowerStSpec>;
#[doc = "Field `POWER_STATE` reader - power state of pmu FSM"]
pub type PowerStateR = crate::FieldReader;
#[doc = "Field `POWER_STATE` writer - power state of pmu FSM"]
pub type PowerStateW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - power state of pmu FSM"]
    #[inline(always)]
    pub fn power_state(&self) -> PowerStateR {
        PowerStateR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - power state of pmu FSM"]
    #[inline(always)]
    #[must_use]
    pub fn power_state(&mut self) -> PowerStateW<PmuPowerStSpec> {
        PowerStateW::new(self, 0)
    }
}
#[doc = "pmu power status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_power_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_power_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuPowerStSpec;
impl crate::RegisterSpec for PmuPowerStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_power_st::R`](R) reader structure"]
impl crate::Readable for PmuPowerStSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_power_st::W`](W) writer structure"]
impl crate::Writable for PmuPowerStSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_POWER_ST to value 0"]
impl crate::Resettable for PmuPowerStSpec {
    const RESET_VALUE: u32 = 0;
}
