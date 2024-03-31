#[doc = "Register `DENALI_PHY_172` reader"]
pub type R = crate::R<DenaliPhy172Spec>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_LE_DLY_OBS_1` reader - Observation register for read leveling data window leading edge slave delay setting for slice 1."]
pub type PhyRdlvlRddqsDqLeDlyObs1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_TE_DLY_OBS_1` reader - Observation register for read leveling data window trailing edge slave delay setting for slice 1."]
pub type PhyRdlvlRddqsDqTeDlyObs1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Observation register for read leveling data window leading edge slave delay setting for slice 1."]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_le_dly_obs_1(&self) -> PhyRdlvlRddqsDqLeDlyObs1R {
        PhyRdlvlRddqsDqLeDlyObs1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Observation register for read leveling data window trailing edge slave delay setting for slice 1."]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_te_dly_obs_1(&self) -> PhyRdlvlRddqsDqTeDlyObs1R {
        PhyRdlvlRddqsDqTeDlyObs1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_172::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy172Spec;
impl crate::RegisterSpec for DenaliPhy172Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_172::R`](R) reader structure"]
impl crate::Readable for DenaliPhy172Spec {}
#[doc = "`reset()` method sets DENALI_PHY_172 to value 0"]
impl crate::Resettable for DenaliPhy172Spec {
    const RESET_VALUE: u32 = 0;
}
