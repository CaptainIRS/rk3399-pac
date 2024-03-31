#[doc = "Register `DENALI_PHY_35` reader"]
pub type R = crate::R<DenaliPhy35Spec>;
#[doc = "Field `PHY_LPBK_ERROR_COUNT_OBS_0` reader - Observation register containing total number of loopback error data for slice 0."]
pub type PhyLpbkErrorCountObs0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_MASTER_DLY_LOCK_OBS_0` reader - Observation register for master delay results for slice 0."]
pub type PhyMasterDlyLockObs0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Observation register containing total number of loopback error data for slice 0."]
    #[inline(always)]
    pub fn phy_lpbk_error_count_obs_0(&self) -> PhyLpbkErrorCountObs0R {
        PhyLpbkErrorCountObs0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - Observation register for master delay results for slice 0."]
    #[inline(always)]
    pub fn phy_master_dly_lock_obs_0(&self) -> PhyMasterDlyLockObs0R {
        PhyMasterDlyLockObs0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_35::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy35Spec;
impl crate::RegisterSpec for DenaliPhy35Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_35::R`](R) reader structure"]
impl crate::Readable for DenaliPhy35Spec {}
#[doc = "`reset()` method sets DENALI_PHY_35 to value 0"]
impl crate::Resettable for DenaliPhy35Spec {
    const RESET_VALUE: u32 = 0;
}
