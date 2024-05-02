#[doc = "Register `PMA_ISO_RX_DATA_LO_LANE3` reader"]
pub type R = crate::R<PmaIsoRxDataLoLane3Spec>;
#[doc = "Field `FIELD0` reader - Current value of rx_rd\\[15:0\\]
PMA output for the associated lane. (Not \n\nre-synchronized to apb_pclk). Valid for PMA lanes 2 and 3 only. For \n\nPMA lanes 0 and 3, reserved."]
pub type Field0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Current value of rx_rd\\[15:0\\]
PMA output for the associated lane. (Not \n\nre-synchronized to apb_pclk). Valid for PMA lanes 2 and 3 only. For \n\nPMA lanes 0 and 3, reserved."]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new(self.bits)
    }
}
#[doc = "PMA receive low data isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_rx_data_lo_lane3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmaIsoRxDataLoLane3Spec;
impl crate::RegisterSpec for PmaIsoRxDataLoLane3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pma_iso_rx_data_lo_lane3::R`](R) reader structure"]
impl crate::Readable for PmaIsoRxDataLoLane3Spec {}
#[doc = "`reset()` method sets PMA_ISO_RX_DATA_LO_LANE3 to value 0"]
impl crate::Resettable for PmaIsoRxDataLoLane3Spec {
    const RESET_VALUE: u16 = 0;
}
