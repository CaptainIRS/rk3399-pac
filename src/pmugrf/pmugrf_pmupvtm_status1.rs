#[doc = "Register `PMUGRF_PMUPVTM_STATUS1` reader"]
pub type R = crate::R<PmugrfPmupvtmStatus1Spec>;
#[doc = "Register `PMUGRF_PMUPVTM_STATUS1` writer"]
pub type W = crate::W<PmugrfPmupvtmStatus1Spec>;
#[doc = "Field `PVTM_FREQ_CNT` reader - pvtm frequency count"]
pub type PvtmFreqCntR = crate::FieldReader<u32>;
#[doc = "Field `PVTM_FREQ_CNT` writer - pvtm frequency count"]
pub type PvtmFreqCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - pvtm frequency count"]
    #[inline(always)]
    pub fn pvtm_freq_cnt(&self) -> PvtmFreqCntR {
        PvtmFreqCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - pvtm frequency count"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_freq_cnt(&mut self) -> PvtmFreqCntW<PmugrfPmupvtmStatus1Spec> {
        PvtmFreqCntW::new(self, 0)
    }
}
#[doc = "pmu pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_pmupvtm_status1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_pmupvtm_status1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmugrfPmupvtmStatus1Spec;
impl crate::RegisterSpec for PmugrfPmupvtmStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmugrf_pmupvtm_status1::R`](R) reader structure"]
impl crate::Readable for PmugrfPmupvtmStatus1Spec {}
#[doc = "`write(|w| ..)` method takes [`pmugrf_pmupvtm_status1::W`](W) writer structure"]
impl crate::Writable for PmugrfPmupvtmStatus1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUGRF_PMUPVTM_STATUS1 to value 0"]
impl crate::Resettable for PmugrfPmupvtmStatus1Spec {
    const RESET_VALUE: u32 = 0;
}
