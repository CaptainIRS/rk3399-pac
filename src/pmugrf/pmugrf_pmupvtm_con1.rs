#[doc = "Register `PMUGRF_PMUPVTM_CON1` reader"]
pub type R = crate::R<PmugrfPmupvtmCon1Spec>;
#[doc = "Register `PMUGRF_PMUPVTM_CON1` writer"]
pub type W = crate::W<PmugrfPmupvtmCon1Spec>;
#[doc = "Field `PVTM_CORE_CAL_CNT` reader - pd_core pvtm calculator counter"]
pub type PvtmCoreCalCntR = crate::FieldReader<u32>;
#[doc = "Field `PVTM_CORE_CAL_CNT` writer - pd_core pvtm calculator counter"]
pub type PvtmCoreCalCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - pd_core pvtm calculator counter"]
    #[inline(always)]
    pub fn pvtm_core_cal_cnt(&self) -> PvtmCoreCalCntR {
        PvtmCoreCalCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - pd_core pvtm calculator counter"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_core_cal_cnt(&mut self) -> PvtmCoreCalCntW<PmugrfPmupvtmCon1Spec> {
        PvtmCoreCalCntW::new(self, 0)
    }
}
#[doc = "pmu pvtm configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_pmupvtm_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_pmupvtm_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmugrfPmupvtmCon1Spec;
impl crate::RegisterSpec for PmugrfPmupvtmCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmugrf_pmupvtm_con1::R`](R) reader structure"]
impl crate::Readable for PmugrfPmupvtmCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`pmugrf_pmupvtm_con1::W`](W) writer structure"]
impl crate::Writable for PmugrfPmupvtmCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUGRF_PMUPVTM_CON1 to value 0"]
impl crate::Resettable for PmugrfPmupvtmCon1Spec {
    const RESET_VALUE: u32 = 0;
}
