#[doc = "Register `DDR_DENALI_PHY_47` reader"]
pub type R = crate::R<DdrDenaliPhy47Spec>;
#[doc = "Field `PHY_WDQLVL_DQDM_LE_DLY_OBS_0` reader - Observation register for write data leveling data window leading edge slave delay setting for slice 0."]
pub type PhyWdqlvlDqdmLeDlyObs0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DQDM_TE_DLY_OBS_0` reader - Observation register for write data leveling data window trailing edge slave delay setting for slice 0."]
pub type PhyWdqlvlDqdmTeDlyObs0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Observation register for write data leveling data window leading edge slave delay setting for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_le_dly_obs_0(&self) -> PhyWdqlvlDqdmLeDlyObs0R {
        PhyWdqlvlDqdmLeDlyObs0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Observation register for write data leveling data window trailing edge slave delay setting for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_te_dly_obs_0(&self) -> PhyWdqlvlDqdmTeDlyObs0R {
        PhyWdqlvlDqdmTeDlyObs0R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_47::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy47Spec;
impl crate::RegisterSpec for DdrDenaliPhy47Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_47::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy47Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_47 to value 0"]
impl crate::Resettable for DdrDenaliPhy47Spec {
    const RESET_VALUE: u32 = 0;
}
