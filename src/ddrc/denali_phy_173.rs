#[doc = "Register `DENALI_PHY_173` reader"]
pub type R = crate::R<DenaliPhy173Spec>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_NUM_WINDOWS_OBS_1` reader - Observation register for read leveling number of windows found for slice 1."]
pub type PhyRdlvlRddqsDqNumWindowsObs1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Observation register for read leveling number of windows found for slice 1."]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_num_windows_obs_1(&self) -> PhyRdlvlRddqsDqNumWindowsObs1R {
        PhyRdlvlRddqsDqNumWindowsObs1R::new((self.bits & 3) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_173::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy173Spec;
impl crate::RegisterSpec for DenaliPhy173Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_173::R`](R) reader structure"]
impl crate::Readable for DenaliPhy173Spec {}
#[doc = "`reset()` method sets DENALI_PHY_173 to value 0"]
impl crate::Resettable for DenaliPhy173Spec {
    const RESET_VALUE: u32 = 0;
}
