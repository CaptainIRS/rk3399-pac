#[doc = "Register `DDR_DENALI_PHY_45` reader"]
pub type R = crate::R<DdrDenaliPhy45Spec>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_NUM_WINDOWS_OBS_0` reader - Observation register for read leveling number of windows found for slice 0."]
pub type PhyRdlvlRddqsDqNumWindowsObs0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Observation register for read leveling number of windows found for slice 0."]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_num_windows_obs_0(&self) -> PhyRdlvlRddqsDqNumWindowsObs0R {
        PhyRdlvlRddqsDqNumWindowsObs0R::new((self.bits & 3) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_45::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy45Spec;
impl crate::RegisterSpec for DdrDenaliPhy45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_45::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy45Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_45 to value 0"]
impl crate::Resettable for DdrDenaliPhy45Spec {
    const RESET_VALUE: u32 = 0;
}
