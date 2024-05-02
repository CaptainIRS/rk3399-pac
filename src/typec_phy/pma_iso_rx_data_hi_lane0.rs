#[doc = "Register `PMA_ISO_RX_DATA_HI_LANE0` reader"]
pub type R = crate::R<PmaIsoRxDataHiLane0Spec>;
#[doc = "Field `FIELD1` reader - Drives rx_rd\\[19:16\\]
PMA \n\ninput \n\nfor the associated \n\nlane. (Not re-\n\nsynchronized to apb_pclk). Valid for PMA lanes 2 and 3 only. For PMA \n\nlanes 0 and 3, reserved."]
pub type Field1R = crate::FieldReader;
#[doc = "Field `FIELD0` reader - Reserved"]
pub type Field0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Drives rx_rd\\[19:16\\]
PMA \n\ninput \n\nfor the associated \n\nlane. (Not re-\n\nsynchronized to apb_pclk). Valid for PMA lanes 2 and 3 only. For PMA \n\nlanes 0 and 3, reserved."]
    #[inline(always)]
    pub fn field1(&self) -> Field1R {
        Field1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Reserved"]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new((self.bits >> 4) & 0x0fff)
    }
}
#[doc = "PMA receive high data isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_rx_data_hi_lane0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmaIsoRxDataHiLane0Spec;
impl crate::RegisterSpec for PmaIsoRxDataHiLane0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pma_iso_rx_data_hi_lane0::R`](R) reader structure"]
impl crate::Readable for PmaIsoRxDataHiLane0Spec {}
#[doc = "`reset()` method sets PMA_ISO_RX_DATA_HI_LANE0 to value 0"]
impl crate::Resettable for PmaIsoRxDataHiLane0Spec {
    const RESET_VALUE: u16 = 0;
}
