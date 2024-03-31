#[doc = "Register `DENALI_PHY_949` reader"]
pub type R = crate::R<DenaliPhy949Spec>;
#[doc = "Field `PHY_CAL_RESULT2_OBS_0` reader - Pad calibration results (CKE/ RESET_N) observation values for block 0."]
pub type PhyCalResult2Obs0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Pad calibration results (CKE/ RESET_N) observation values for block 0."]
    #[inline(always)]
    pub fn phy_cal_result2_obs_0(&self) -> PhyCalResult2Obs0R {
        PhyCalResult2Obs0R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_949::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy949Spec;
impl crate::RegisterSpec for DenaliPhy949Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_949::R`](R) reader structure"]
impl crate::Readable for DenaliPhy949Spec {}
#[doc = "`reset()` method sets DENALI_PHY_949 to value 0"]
impl crate::Resettable for DenaliPhy949Spec {
    const RESET_VALUE: u32 = 0;
}
