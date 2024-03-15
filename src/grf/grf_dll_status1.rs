#[doc = "Register `GRF_DLL_STATUS1` reader"]
pub type R = crate::R<GrfDllStatus1Spec>;
#[doc = "Register `GRF_DLL_STATUS1` writer"]
pub type W = crate::W<GrfDllStatus1Spec>;
#[doc = "Field `PVTM_CORE_L_FREQ_CNT` reader - pd_core_l pvtm frequency count"]
pub type PvtmCoreLFreqCntR = crate::FieldReader<u32>;
#[doc = "Field `PVTM_CORE_L_FREQ_CNT` writer - pd_core_l pvtm frequency count"]
pub type PvtmCoreLFreqCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - pd_core_l pvtm frequency count"]
    #[inline(always)]
    pub fn pvtm_core_l_freq_cnt(&self) -> PvtmCoreLFreqCntR {
        PvtmCoreLFreqCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - pd_core_l pvtm frequency count"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_core_l_freq_cnt(&mut self) -> PvtmCoreLFreqCntW<GrfDllStatus1Spec> {
        PvtmCoreLFreqCntW::new(self, 0)
    }
}
#[doc = "pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_dll_status1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_dll_status1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfDllStatus1Spec;
impl crate::RegisterSpec for GrfDllStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_dll_status1::R`](R) reader structure"]
impl crate::Readable for GrfDllStatus1Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_dll_status1::W`](W) writer structure"]
impl crate::Writable for GrfDllStatus1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_DLL_STATUS1 to value 0"]
impl crate::Resettable for GrfDllStatus1Spec {
    const RESET_VALUE: u32 = 0;
}
