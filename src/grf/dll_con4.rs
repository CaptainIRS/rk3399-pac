#[doc = "Register `DLL_CON4` reader"]
pub type R = crate::R<DllCon4Spec>;
#[doc = "Register `DLL_CON4` writer"]
pub type W = crate::W<DllCon4Spec>;
#[doc = "Field `PVTM_GPU_CAL_CNT` reader - gpu pvtm calculator counter"]
pub type PvtmGpuCalCntR = crate::FieldReader<u32>;
#[doc = "Field `PVTM_GPU_CAL_CNT` writer - gpu pvtm calculator counter"]
pub type PvtmGpuCalCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - gpu pvtm calculator counter"]
    #[inline(always)]
    pub fn pvtm_gpu_cal_cnt(&self) -> PvtmGpuCalCntR {
        PvtmGpuCalCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - gpu pvtm calculator counter"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_gpu_cal_cnt(&mut self) -> PvtmGpuCalCntW<DllCon4Spec> {
        PvtmGpuCalCntW::new(self, 0)
    }
}
#[doc = "pvtm control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll_con4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll_con4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DllCon4Spec;
impl crate::RegisterSpec for DllCon4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll_con4::R`](R) reader structure"]
impl crate::Readable for DllCon4Spec {}
#[doc = "`write(|w| ..)` method takes [`dll_con4::W`](W) writer structure"]
impl crate::Writable for DllCon4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLL_CON4 to value 0x016e_3600"]
impl crate::Resettable for DllCon4Spec {
    const RESET_VALUE: u32 = 0x016e_3600;
}
