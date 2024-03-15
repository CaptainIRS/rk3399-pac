#[doc = "Register `GRF_DLL_STATUS3` reader"]
pub type R = crate::R<GrfDllStatus3Spec>;
#[doc = "Register `GRF_DLL_STATUS3` writer"]
pub type W = crate::W<GrfDllStatus3Spec>;
#[doc = "Field `PVTM_GPU_FREQ_CNT` reader - gpu pvtm frequency count"]
pub type PvtmGpuFreqCntR = crate::FieldReader<u32>;
#[doc = "Field `PVTM_GPU_FREQ_CNT` writer - gpu pvtm frequency count"]
pub type PvtmGpuFreqCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - gpu pvtm frequency count"]
    #[inline(always)]
    pub fn pvtm_gpu_freq_cnt(&self) -> PvtmGpuFreqCntR {
        PvtmGpuFreqCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - gpu pvtm frequency count"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_gpu_freq_cnt(&mut self) -> PvtmGpuFreqCntW<GrfDllStatus3Spec> {
        PvtmGpuFreqCntW::new(self, 0)
    }
}
#[doc = "pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_dll_status3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_dll_status3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfDllStatus3Spec;
impl crate::RegisterSpec for GrfDllStatus3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_dll_status3::R`](R) reader structure"]
impl crate::Readable for GrfDllStatus3Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_dll_status3::W`](W) writer structure"]
impl crate::Writable for GrfDllStatus3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_DLL_STATUS3 to value 0"]
impl crate::Resettable for GrfDllStatus3Spec {
    const RESET_VALUE: u32 = 0;
}
