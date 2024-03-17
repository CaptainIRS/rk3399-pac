#[doc = "Register `DENALI_PHY_300` reader"]
pub type R = crate::R<DenaliPhy300Spec>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_LE_DLY_OBS_2` reader - Observation register for read leveling data window leading edge slave delay setting for slice 2. READ-ONLY"]
pub type PhyRdlvlRddqsDqLeDlyObs2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_TE_DLY_OBS_2` reader - Observation register for read leveling data window trailing edge slave delay setting for slice 2. READ-ONLY"]
pub type PhyRdlvlRddqsDqTeDlyObs2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Observation register for read leveling data window leading edge slave delay setting for slice 2. READ-ONLY"]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_le_dly_obs_2(&self) -> PhyRdlvlRddqsDqLeDlyObs2R {
        PhyRdlvlRddqsDqLeDlyObs2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Observation register for read leveling data window trailing edge slave delay setting for slice 2. READ-ONLY"]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_te_dly_obs_2(&self) -> PhyRdlvlRddqsDqTeDlyObs2R {
        PhyRdlvlRddqsDqTeDlyObs2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_300::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy300Spec;
impl crate::RegisterSpec for DenaliPhy300Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_300::R`](R) reader structure"]
impl crate::Readable for DenaliPhy300Spec {}
#[doc = "`reset()` method sets DENALI_PHY_300 to value 0"]
impl crate::Resettable for DenaliPhy300Spec {
    const RESET_VALUE: u32 = 0;
}
