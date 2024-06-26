#[doc = "Register `GPU_PWRUP_CNT` reader"]
pub type R = crate::R<GpuPwrupCntSpec>;
#[doc = "Register `GPU_PWRUP_CNT` writer"]
pub type W = crate::W<GpuPwrupCntSpec>;
#[doc = "Field `PMU_GPU_PWRUP_CNT` reader - pmu gpu power up counter value"]
pub type PmuGpuPwrupCntR = crate::FieldReader<u32>;
#[doc = "Field `PMU_GPU_PWRUP_CNT` writer - pmu gpu power up counter value"]
pub type PmuGpuPwrupCntW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - pmu gpu power up counter value"]
    #[inline(always)]
    pub fn pmu_gpu_pwrup_cnt(&self) -> PmuGpuPwrupCntR {
        PmuGpuPwrupCntR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - pmu gpu power up counter value"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_gpu_pwrup_cnt(&mut self) -> PmuGpuPwrupCntW<GpuPwrupCntSpec> {
        PmuGpuPwrupCntW::new(self, 0)
    }
}
#[doc = "pmu gpu power up count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpu_pwrup_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpu_pwrup_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpuPwrupCntSpec;
impl crate::RegisterSpec for GpuPwrupCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpu_pwrup_cnt::R`](R) reader structure"]
impl crate::Readable for GpuPwrupCntSpec {}
#[doc = "`write(|w| ..)` method takes [`gpu_pwrup_cnt::W`](W) writer structure"]
impl crate::Writable for GpuPwrupCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPU_PWRUP_CNT to value 0x5dc0"]
impl crate::Resettable for GpuPwrupCntSpec {
    const RESET_VALUE: u32 = 0x5dc0;
}
