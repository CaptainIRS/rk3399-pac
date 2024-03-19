#[doc = "Register `DDR_DENALI_PHY_46` reader"]
pub type R = crate::R<DdrDenaliPhy46Spec>;
#[doc = "Field `PHY_RDLVL_STATUS_OBS_0` reader - Observation register for read leveling status for slice 0."]
pub type PhyRdlvlStatusObs0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register for read leveling status for slice 0."]
    #[inline(always)]
    pub fn phy_rdlvl_status_obs_0(&self) -> PhyRdlvlStatusObs0R {
        PhyRdlvlStatusObs0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_46::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy46Spec;
impl crate::RegisterSpec for DdrDenaliPhy46Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_46::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy46Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_46 to value 0"]
impl crate::Resettable for DdrDenaliPhy46Spec {
    const RESET_VALUE: u32 = 0;
}
