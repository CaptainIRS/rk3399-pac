#[doc = "Register `DLL_STATUS2` reader"]
pub type R = crate::R<DllStatus2Spec>;
#[doc = "Register `DLL_STATUS2` writer"]
pub type W = crate::W<DllStatus2Spec>;
#[doc = "Field `PVTM_CORE_B_FREQ_CNT` reader - pd_core_b pvtm frequency count"]
pub type PvtmCoreBFreqCntR = crate::FieldReader<u32>;
#[doc = "Field `PVTM_CORE_B_FREQ_CNT` writer - pd_core_b pvtm frequency count"]
pub type PvtmCoreBFreqCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - pd_core_b pvtm frequency count"]
    #[inline(always)]
    pub fn pvtm_core_b_freq_cnt(&self) -> PvtmCoreBFreqCntR {
        PvtmCoreBFreqCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - pd_core_b pvtm frequency count"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_core_b_freq_cnt(&mut self) -> PvtmCoreBFreqCntW<DllStatus2Spec> {
        PvtmCoreBFreqCntW::new(self, 0)
    }
}
#[doc = "pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll_status2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll_status2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DllStatus2Spec;
impl crate::RegisterSpec for DllStatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll_status2::R`](R) reader structure"]
impl crate::Readable for DllStatus2Spec {}
#[doc = "`write(|w| ..)` method takes [`dll_status2::W`](W) writer structure"]
impl crate::Writable for DllStatus2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLL_STATUS2 to value 0"]
impl crate::Resettable for DllStatus2Spec {
    const RESET_VALUE: u32 = 0;
}
