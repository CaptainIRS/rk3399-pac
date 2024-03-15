#[doc = "Register `PMU_GPU_PWRDN_CNT` reader"]
pub type R = crate::R<PmuGpuPwrdnCntSpec>;
#[doc = "Register `PMU_GPU_PWRDN_CNT` writer"]
pub type W = crate::W<PmuGpuPwrdnCntSpec>;
#[doc = "Field `PMU_GPU_PWRDN_CNT` reader - pmu gpu power down counter value"]
pub type PmuGpuPwrdnCntR = crate::FieldReader<u32>;
#[doc = "Field `PMU_GPU_PWRDN_CNT` writer - pmu gpu power down counter value"]
pub type PmuGpuPwrdnCntW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - pmu gpu power down counter value"]
    #[inline(always)]
    pub fn pmu_gpu_pwrdn_cnt(&self) -> PmuGpuPwrdnCntR {
        PmuGpuPwrdnCntR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - pmu gpu power down counter value"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_gpu_pwrdn_cnt(&mut self) -> PmuGpuPwrdnCntW<PmuGpuPwrdnCntSpec> {
        PmuGpuPwrdnCntW::new(self, 0)
    }
}
#[doc = "pmu gpu power down count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_gpu_pwrdn_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_gpu_pwrdn_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuGpuPwrdnCntSpec;
impl crate::RegisterSpec for PmuGpuPwrdnCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_gpu_pwrdn_cnt::R`](R) reader structure"]
impl crate::Readable for PmuGpuPwrdnCntSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_gpu_pwrdn_cnt::W`](W) writer structure"]
impl crate::Writable for PmuGpuPwrdnCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_GPU_PWRDN_CNT to value 0x5dc0"]
impl crate::Resettable for PmuGpuPwrdnCntSpec {
    const RESET_VALUE: u32 = 0x5dc0;
}
