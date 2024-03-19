#[doc = "Register `DDR_DENALI_PHY_303` reader"]
pub type R = crate::R<DdrDenaliPhy303Spec>;
#[doc = "Field `PHY_WDQLVL_DQDM_LE_DLY_OBS_2` reader - Observation register for write data leveling data window leading edge slave delay setting for slice 2."]
pub type PhyWdqlvlDqdmLeDlyObs2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DQDM_TE_DLY_OBS_2` reader - Observation register for write data leveling data window trailing edge slave delay setting for slice 2."]
pub type PhyWdqlvlDqdmTeDlyObs2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Observation register for write data leveling data window leading edge slave delay setting for slice 2."]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_le_dly_obs_2(&self) -> PhyWdqlvlDqdmLeDlyObs2R {
        PhyWdqlvlDqdmLeDlyObs2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Observation register for write data leveling data window trailing edge slave delay setting for slice 2."]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_te_dly_obs_2(&self) -> PhyWdqlvlDqdmTeDlyObs2R {
        PhyWdqlvlDqdmTeDlyObs2R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_303::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy303Spec;
impl crate::RegisterSpec for DdrDenaliPhy303Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_303::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy303Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_303 to value 0"]
impl crate::Resettable for DdrDenaliPhy303Spec {
    const RESET_VALUE: u32 = 0;
}
