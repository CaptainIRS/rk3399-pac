#[doc = "Register `DENALI_PHY_948` reader"]
pub type R = crate::R<DenaliPhy948Spec>;
#[doc = "Field `PHY_CAL_RESULT_OBS_0` reader - Pad calibration results observation values for block 0. READ-ONLY"]
pub type PhyCalResultObs0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Pad calibration results observation values for block 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_cal_result_obs_0(&self) -> PhyCalResultObs0R {
        PhyCalResultObs0R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_948::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy948Spec;
impl crate::RegisterSpec for DenaliPhy948Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_948::R`](R) reader structure"]
impl crate::Readable for DenaliPhy948Spec {}
#[doc = "`reset()` method sets DENALI_PHY_948 to value 0"]
impl crate::Resettable for DenaliPhy948Spec {
    const RESET_VALUE: u32 = 0;
}
