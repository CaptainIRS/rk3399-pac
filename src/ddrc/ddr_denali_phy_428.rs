#[doc = "Register `DDR_DENALI_PHY_428` reader"]
pub type R = crate::R<DdrDenaliPhy428Spec>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_LE_DLY_OBS_3` reader - Observation register for read leveling data window leading edge slave delay setting for slice 3."]
pub type PhyRdlvlRddqsDqLeDlyObs3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_TE_DLY_OBS_3` reader - Observation register for read leveling data window trailing edge slave delay setting for slice 3."]
pub type PhyRdlvlRddqsDqTeDlyObs3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Observation register for read leveling data window leading edge slave delay setting for slice 3."]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_le_dly_obs_3(&self) -> PhyRdlvlRddqsDqLeDlyObs3R {
        PhyRdlvlRddqsDqLeDlyObs3R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Observation register for read leveling data window trailing edge slave delay setting for slice 3."]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_te_dly_obs_3(&self) -> PhyRdlvlRddqsDqTeDlyObs3R {
        PhyRdlvlRddqsDqTeDlyObs3R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_428::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy428Spec;
impl crate::RegisterSpec for DdrDenaliPhy428Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_428::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy428Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_428 to value 0"]
impl crate::Resettable for DdrDenaliPhy428Spec {
    const RESET_VALUE: u32 = 0;
}
