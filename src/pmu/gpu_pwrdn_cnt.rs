#[doc = "Register `GPU_PWRDN_CNT` reader"]
pub type R = crate::R<GpuPwrdnCntSpec>;
#[doc = "Register `GPU_PWRDN_CNT` writer"]
pub type W = crate::W<GpuPwrdnCntSpec>;
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
    pub fn pmu_gpu_pwrdn_cnt(&mut self) -> PmuGpuPwrdnCntW<GpuPwrdnCntSpec> {
        PmuGpuPwrdnCntW::new(self, 0)
    }
}
#[doc = "pmu gpu power down count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpu_pwrdn_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpu_pwrdn_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpuPwrdnCntSpec;
impl crate::RegisterSpec for GpuPwrdnCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpu_pwrdn_cnt::R`](R) reader structure"]
impl crate::Readable for GpuPwrdnCntSpec {}
#[doc = "`write(|w| ..)` method takes [`gpu_pwrdn_cnt::W`](W) writer structure"]
impl crate::Writable for GpuPwrdnCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPU_PWRDN_CNT to value 0x5dc0"]
impl crate::Resettable for GpuPwrdnCntSpec {
    const RESET_VALUE: u32 = 0x5dc0;
}
