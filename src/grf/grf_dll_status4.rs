#[doc = "Register `GRF_DLL_STATUS4` reader"]
pub type R = crate::R<GrfDllStatus4Spec>;
#[doc = "Register `GRF_DLL_STATUS4` writer"]
pub type W = crate::W<GrfDllStatus4Spec>;
#[doc = "Field `PVTM_DDR_FREQ_CNT` reader - ddr pvtm frequency count"]
pub type PvtmDdrFreqCntR = crate::FieldReader<u32>;
#[doc = "Field `PVTM_DDR_FREQ_CNT` writer - ddr pvtm frequency count"]
pub type PvtmDdrFreqCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ddr pvtm frequency count"]
    #[inline(always)]
    pub fn pvtm_ddr_freq_cnt(&self) -> PvtmDdrFreqCntR {
        PvtmDdrFreqCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ddr pvtm frequency count"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_ddr_freq_cnt(&mut self) -> PvtmDdrFreqCntW<GrfDllStatus4Spec> {
        PvtmDdrFreqCntW::new(self, 0)
    }
}
#[doc = "pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_dll_status4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_dll_status4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfDllStatus4Spec;
impl crate::RegisterSpec for GrfDllStatus4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_dll_status4::R`](R) reader structure"]
impl crate::Readable for GrfDllStatus4Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_dll_status4::W`](W) writer structure"]
impl crate::Writable for GrfDllStatus4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_DLL_STATUS4 to value 0"]
impl crate::Resettable for GrfDllStatus4Spec {
    const RESET_VALUE: u32 = 0;
}
