#[doc = "Register `DENALI_PHY_431` reader"]
pub type R = crate::R<DenaliPhy431Spec>;
#[doc = "Field `PHY_WDQLVL_DQDM_LE_DLY_OBS_3` reader - Observation register for write data leveling data window leading edge slave delay setting for slice 3. READ-ONLY"]
pub type PhyWdqlvlDqdmLeDlyObs3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DQDM_TE_DLY_OBS_3` reader - Observation register for write data leveling data window trailing edge slave delay setting for slice 3. READ-ONLY"]
pub type PhyWdqlvlDqdmTeDlyObs3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Observation register for write data leveling data window leading edge slave delay setting for slice 3. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_le_dly_obs_3(&self) -> PhyWdqlvlDqdmLeDlyObs3R {
        PhyWdqlvlDqdmLeDlyObs3R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Observation register for write data leveling data window trailing edge slave delay setting for slice 3. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_te_dly_obs_3(&self) -> PhyWdqlvlDqdmTeDlyObs3R {
        PhyWdqlvlDqdmTeDlyObs3R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_431::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy431Spec;
impl crate::RegisterSpec for DenaliPhy431Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_431::R`](R) reader structure"]
impl crate::Readable for DenaliPhy431Spec {}
#[doc = "`reset()` method sets DENALI_PHY_431 to value 0"]
impl crate::Resettable for DenaliPhy431Spec {
    const RESET_VALUE: u32 = 0;
}
