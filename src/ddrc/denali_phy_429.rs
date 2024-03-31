#[doc = "Register `DENALI_PHY_429` reader"]
pub type R = crate::R<DenaliPhy429Spec>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_NUM_WINDOWS_OBS_3` reader - Observation register for read leveling number of windows found for slice 3."]
pub type PhyRdlvlRddqsDqNumWindowsObs3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Observation register for read leveling number of windows found for slice 3."]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_num_windows_obs_3(&self) -> PhyRdlvlRddqsDqNumWindowsObs3R {
        PhyRdlvlRddqsDqNumWindowsObs3R::new((self.bits & 3) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_429::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy429Spec;
impl crate::RegisterSpec for DenaliPhy429Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_429::R`](R) reader structure"]
impl crate::Readable for DenaliPhy429Spec {}
#[doc = "`reset()` method sets DENALI_PHY_429 to value 0"]
impl crate::Resettable for DenaliPhy429Spec {
    const RESET_VALUE: u32 = 0;
}
