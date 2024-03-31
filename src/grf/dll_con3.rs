#[doc = "Register `DLL_CON3` reader"]
pub type R = crate::R<DllCon3Spec>;
#[doc = "Register `DLL_CON3` writer"]
pub type W = crate::W<DllCon3Spec>;
#[doc = "Field `PVTM_DDR_CAL_CNT` reader - ddr pvtm calculator counter"]
pub type PvtmDdrCalCntR = crate::FieldReader<u32>;
#[doc = "Field `PVTM_DDR_CAL_CNT` writer - ddr pvtm calculator counter"]
pub type PvtmDdrCalCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ddr pvtm calculator counter"]
    #[inline(always)]
    pub fn pvtm_ddr_cal_cnt(&self) -> PvtmDdrCalCntR {
        PvtmDdrCalCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ddr pvtm calculator counter"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_ddr_cal_cnt(&mut self) -> PvtmDdrCalCntW<DllCon3Spec> {
        PvtmDdrCalCntW::new(self, 0)
    }
}
#[doc = "pvtm control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll_con3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll_con3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DllCon3Spec;
impl crate::RegisterSpec for DllCon3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll_con3::R`](R) reader structure"]
impl crate::Readable for DllCon3Spec {}
#[doc = "`write(|w| ..)` method takes [`dll_con3::W`](W) writer structure"]
impl crate::Writable for DllCon3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLL_CON3 to value 0x016e_3600"]
impl crate::Resettable for DllCon3Spec {
    const RESET_VALUE: u32 = 0x016e_3600;
}
