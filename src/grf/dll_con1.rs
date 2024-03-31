#[doc = "Register `DLL_CON1` reader"]
pub type R = crate::R<DllCon1Spec>;
#[doc = "Register `DLL_CON1` writer"]
pub type W = crate::W<DllCon1Spec>;
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
    pub fn pvtm_core_l_cal_cnt(&mut self) -> PvtmCoreLCalCntW<DllCon1Spec> {
        PvtmCoreLCalCntW::new(self, 0)
    }
}
#[doc = "pvtm control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DllCon1Spec;
impl crate::RegisterSpec for DllCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll_con1::R`](R) reader structure"]
impl crate::Readable for DllCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`dll_con1::W`](W) writer structure"]
impl crate::Writable for DllCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLL_CON1 to value 0x016e_3600"]
impl crate::Resettable for DllCon1Spec {
    const RESET_VALUE: u32 = 0x016e_3600;
}
