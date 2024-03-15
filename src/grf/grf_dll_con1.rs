#[doc = "Register `GRF_DLL_CON1` reader"]
pub type R = crate::R<GrfDllCon1Spec>;
#[doc = "Register `GRF_DLL_CON1` writer"]
pub type W = crate::W<GrfDllCon1Spec>;
#[doc = "Field `PVTM_CORE_L_CAL_CNT` reader - pd_core_l pvtm calculator counter"]
pub type PvtmCoreLCalCntR = crate::FieldReader<u32>;
#[doc = "Field `PVTM_CORE_L_CAL_CNT` writer - pd_core_l pvtm calculator counter"]
pub type PvtmCoreLCalCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - pd_core_l pvtm calculator counter"]
    #[inline(always)]
    pub fn pvtm_core_l_cal_cnt(&self) -> PvtmCoreLCalCntR {
        PvtmCoreLCalCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - pd_core_l pvtm calculator counter"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_core_l_cal_cnt(&mut self) -> PvtmCoreLCalCntW<GrfDllCon1Spec> {
        PvtmCoreLCalCntW::new(self, 0)
    }
}
#[doc = "pvtm control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_dll_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_dll_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfDllCon1Spec;
impl crate::RegisterSpec for GrfDllCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_dll_con1::R`](R) reader structure"]
impl crate::Readable for GrfDllCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_dll_con1::W`](W) writer structure"]
impl crate::Writable for GrfDllCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_DLL_CON1 to value 0x016e_3600"]
impl crate::Resettable for GrfDllCon1Spec {
    const RESET_VALUE: u32 = 0x016e_3600;
}
