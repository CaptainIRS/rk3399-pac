#[doc = "Register `STABLE_CNT` reader"]
pub type R = crate::R<StableCntSpec>;
#[doc = "Register `STABLE_CNT` writer"]
pub type W = crate::W<StableCntSpec>;
#[doc = "Field `PMU_STABLE_CNT` reader - pmu PMIC stable counter value"]
pub type PmuStableCntR = crate::FieldReader<u32>;
#[doc = "Field `PMU_STABLE_CNT` writer - pmu PMIC stable counter value"]
pub type PmuStableCntW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - pmu PMIC stable counter value"]
    #[inline(always)]
    pub fn pmu_stable_cnt(&self) -> PmuStableCntR {
        PmuStableCntR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - pmu PMIC stable counter value"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_stable_cnt(&mut self) -> PmuStableCntW<StableCntSpec> {
        PmuStableCntW::new(self, 0)
    }
}
#[doc = "pmu power stable count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stable_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stable_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StableCntSpec;
impl crate::RegisterSpec for StableCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stable_cnt::R`](R) reader structure"]
impl crate::Readable for StableCntSpec {}
#[doc = "`write(|w| ..)` method takes [`stable_cnt::W`](W) writer structure"]
impl crate::Writable for StableCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STABLE_CNT to value 0"]
impl crate::Resettable for StableCntSpec {
    const RESET_VALUE: u32 = 0;
}
