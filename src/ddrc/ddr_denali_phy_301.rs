#[doc = "Register `DDR_DENALI_PHY_301` reader"]
pub type R = crate::R<DdrDenaliPhy301Spec>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_NUM_WINDOWS_OBS_2` reader - Observation register for read leveling number of windows found for slice 2."]
pub type PhyRdlvlRddqsDqNumWindowsObs2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Observation register for read leveling number of windows found for slice 2."]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_num_windows_obs_2(&self) -> PhyRdlvlRddqsDqNumWindowsObs2R {
        PhyRdlvlRddqsDqNumWindowsObs2R::new((self.bits & 3) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_301::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy301Spec;
impl crate::RegisterSpec for DdrDenaliPhy301Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_301::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy301Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_301 to value 0"]
impl crate::Resettable for DdrDenaliPhy301Spec {
    const RESET_VALUE: u32 = 0;
}
