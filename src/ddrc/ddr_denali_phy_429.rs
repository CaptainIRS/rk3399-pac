#[doc = "Register `DDR_DENALI_PHY_429` reader"]
pub type R = crate::R<DdrDenaliPhy429Spec>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_NUM_WINDOWS_OBS_3` reader - Observation register for read leveling number of windows found for slice 3."]
pub type PhyRdlvlRddqsDqNumWindowsObs3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Observation register for read leveling number of windows found for slice 3."]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_num_windows_obs_3(&self) -> PhyRdlvlRddqsDqNumWindowsObs3R {
        PhyRdlvlRddqsDqNumWindowsObs3R::new((self.bits & 3) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_429::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy429Spec;
impl crate::RegisterSpec for DdrDenaliPhy429Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_429::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy429Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_429 to value 0"]
impl crate::Resettable for DdrDenaliPhy429Spec {
    const RESET_VALUE: u32 = 0;
}
