#[doc = "Register `DENALI_PHY_291` reader"]
pub type R = crate::R<DenaliPhy291Spec>;
#[doc = "Field `PHY_LPBK_ERROR_COUNT_OBS_2` reader - Observation register containing total number of loopback error data for slice 2."]
pub type PhyLpbkErrorCountObs2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_MASTER_DLY_LOCK_OBS_2` reader - Observation register for master delay results for slice 2."]
pub type PhyMasterDlyLockObs2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Observation register containing total number of loopback error data for slice 2."]
    #[inline(always)]
    pub fn phy_lpbk_error_count_obs_2(&self) -> PhyLpbkErrorCountObs2R {
        PhyLpbkErrorCountObs2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - Observation register for master delay results for slice 2."]
    #[inline(always)]
    pub fn phy_master_dly_lock_obs_2(&self) -> PhyMasterDlyLockObs2R {
        PhyMasterDlyLockObs2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_291::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy291Spec;
impl crate::RegisterSpec for DenaliPhy291Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_291::R`](R) reader structure"]
impl crate::Readable for DenaliPhy291Spec {}
#[doc = "`reset()` method sets DENALI_PHY_291 to value 0"]
impl crate::Resettable for DenaliPhy291Spec {
    const RESET_VALUE: u32 = 0;
}
