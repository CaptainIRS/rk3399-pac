#[doc = "Register `DDR_DENALI_PHY_163` reader"]
pub type R = crate::R<DdrDenaliPhy163Spec>;
#[doc = "Field `PHY_LPBK_ERROR_COUNT_OBS_1` reader - Observation register containing total number of loopback error data for slice 1."]
pub type PhyLpbkErrorCountObs1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_MASTER_DLY_LOCK_OBS_1` reader - Observation register for master delay results for slice 1."]
pub type PhyMasterDlyLockObs1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Observation register containing total number of loopback error data for slice 1."]
    #[inline(always)]
    pub fn phy_lpbk_error_count_obs_1(&self) -> PhyLpbkErrorCountObs1R {
        PhyLpbkErrorCountObs1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - Observation register for master delay results for slice 1."]
    #[inline(always)]
    pub fn phy_master_dly_lock_obs_1(&self) -> PhyMasterDlyLockObs1R {
        PhyMasterDlyLockObs1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_163::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy163Spec;
impl crate::RegisterSpec for DdrDenaliPhy163Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_163::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy163Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_163 to value 0"]
impl crate::Resettable for DdrDenaliPhy163Spec {
    const RESET_VALUE: u32 = 0;
}
