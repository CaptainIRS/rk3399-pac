#[doc = "Register `DDR_DENALI_PHY_958` reader"]
pub type R = crate::R<DdrDenaliPhy958Spec>;
#[doc = "Field `PHY_AC_INIT_COMPLETE_OBS` reader - Observation register for dfi_init_complete for adr and ac slice. Bit 0 is for dfi_init_complete for all slices. Bit(7:4) is for adr slice, bit4 is adr_slice0..., if the adr slice number is 3, bit7 is 0. Bit8 is for ac_slice0; bit9 is for ac_slice 1,..."]
pub type PhyAcInitCompleteObsR = crate::FieldReader<u16>;
#[doc = "Field `PHY_DS_INIT_COMPLETE_OBS` reader - Observation register for dfi_init_complete for data slice. Bit0 is for data_slice0; bit1 is for data_slice 1,..."]
pub type PhyDsInitCompleteObsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - Observation register for dfi_init_complete for adr and ac slice. Bit 0 is for dfi_init_complete for all slices. Bit(7:4) is for adr slice, bit4 is adr_slice0..., if the adr slice number is 3, bit7 is 0. Bit8 is for ac_slice0; bit9 is for ac_slice 1,..."]
    #[inline(always)]
    pub fn phy_ac_init_complete_obs(&self) -> PhyAcInitCompleteObsR {
        PhyAcInitCompleteObsR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Observation register for dfi_init_complete for data slice. Bit0 is for data_slice0; bit1 is for data_slice 1,..."]
    #[inline(always)]
    pub fn phy_ds_init_complete_obs(&self) -> PhyDsInitCompleteObsR {
        PhyDsInitCompleteObsR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_958::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy958Spec;
impl crate::RegisterSpec for DdrDenaliPhy958Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_958::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy958Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_958 to value 0"]
impl crate::Resettable for DdrDenaliPhy958Spec {
    const RESET_VALUE: u32 = 0;
}
