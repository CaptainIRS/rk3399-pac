#[doc = "Register `DENALI_PHY_419` reader"]
pub type R = crate::R<DenaliPhy419Spec>;
#[doc = "Field `PHY_LPBK_ERROR_COUNT_OBS_3` reader - Observation register containing total number of loopback error data for slice 3. READ-ONLY"]
pub type PhyLpbkErrorCountObs3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_MASTER_DLY_LOCK_OBS_3` reader - Observation register for master delay results for slice 3. READ- ONLY"]
pub type PhyMasterDlyLockObs3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Observation register containing total number of loopback error data for slice 3. READ-ONLY"]
    #[inline(always)]
    pub fn phy_lpbk_error_count_obs_3(&self) -> PhyLpbkErrorCountObs3R {
        PhyLpbkErrorCountObs3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - Observation register for master delay results for slice 3. READ- ONLY"]
    #[inline(always)]
    pub fn phy_master_dly_lock_obs_3(&self) -> PhyMasterDlyLockObs3R {
        PhyMasterDlyLockObs3R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_419::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy419Spec;
impl crate::RegisterSpec for DenaliPhy419Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_419::R`](R) reader structure"]
impl crate::Readable for DenaliPhy419Spec {}
#[doc = "`reset()` method sets DENALI_PHY_419 to value 0"]
impl crate::Resettable for DenaliPhy419Spec {
    const RESET_VALUE: u32 = 0;
}
