#[doc = "Register `DDR_DENALI_PHY_44` reader"]
pub type R = crate::R<DdrDenaliPhy44Spec>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_LE_DLY_OBS_0` reader - Observation register for read leveling data window leading edge slave delay setting for slice 0."]
pub type PhyRdlvlRddqsDqLeDlyObs0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_TE_DLY_OBS_0` reader - Observation register for read leveling data window trailing edge slave delay setting for slice 0."]
pub type PhyRdlvlRddqsDqTeDlyObs0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Observation register for read leveling data window leading edge slave delay setting for slice 0."]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_le_dly_obs_0(&self) -> PhyRdlvlRddqsDqLeDlyObs0R {
        PhyRdlvlRddqsDqLeDlyObs0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Observation register for read leveling data window trailing edge slave delay setting for slice 0."]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_te_dly_obs_0(&self) -> PhyRdlvlRddqsDqTeDlyObs0R {
        PhyRdlvlRddqsDqTeDlyObs0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_44::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy44Spec;
impl crate::RegisterSpec for DdrDenaliPhy44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_44::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy44Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_44 to value 0"]
impl crate::Resettable for DdrDenaliPhy44Spec {
    const RESET_VALUE: u32 = 0;
}
