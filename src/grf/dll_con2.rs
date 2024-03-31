#[doc = "Register `DLL_CON2` reader"]
pub type R = crate::R<DllCon2Spec>;
#[doc = "Register `DLL_CON2` writer"]
pub type W = crate::W<DllCon2Spec>;
#[doc = "Field `PVTM_CORE_B_CAL_CNT` reader - pd_core_b pvtm calculator counter"]
pub type PvtmCoreBCalCntR = crate::FieldReader<u32>;
#[doc = "Field `PVTM_CORE_B_CAL_CNT` writer - pd_core_b pvtm calculator counter"]
pub type PvtmCoreBCalCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - pd_core_b pvtm calculator counter"]
    #[inline(always)]
    pub fn pvtm_core_b_cal_cnt(&self) -> PvtmCoreBCalCntR {
        PvtmCoreBCalCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - pd_core_b pvtm calculator counter"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_core_b_cal_cnt(&mut self) -> PvtmCoreBCalCntW<DllCon2Spec> {
        PvtmCoreBCalCntW::new(self, 0)
    }
}
#[doc = "pvtm control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll_con2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll_con2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DllCon2Spec;
impl crate::RegisterSpec for DllCon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll_con2::R`](R) reader structure"]
impl crate::Readable for DllCon2Spec {}
#[doc = "`write(|w| ..)` method takes [`dll_con2::W`](W) writer structure"]
impl crate::Writable for DllCon2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLL_CON2 to value 0x016e_3600"]
impl crate::Resettable for DllCon2Spec {
    const RESET_VALUE: u32 = 0x016e_3600;
}
