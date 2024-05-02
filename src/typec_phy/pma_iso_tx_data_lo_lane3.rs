#[doc = "Register `PMA_ISO_TX_DATA_LO_LANE3` reader"]
pub type R = crate::R<PmaIsoTxDataLoLane3Spec>;
#[doc = "Register `PMA_ISO_TX_DATA_LO_LANE3` writer"]
pub type W = crate::W<PmaIsoTxDataLoLane3Spec>;
#[doc = "Field `FIELD0` reader - Drives tx_td\\[15:0\\]
PMA input for the associated lane when in PMA \n\nisolation mode. (Not re-synchronized to apb_pclk)."]
pub type Field0R = crate::FieldReader<u16>;
#[doc = "Field `FIELD0` writer - Drives tx_td\\[15:0\\]
PMA input for the associated lane when in PMA \n\nisolation mode. (Not re-synchronized to apb_pclk)."]
pub type Field0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Drives tx_td\\[15:0\\]
PMA input for the associated lane when in PMA \n\nisolation mode. (Not re-synchronized to apb_pclk)."]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Drives tx_td\\[15:0\\]
PMA input for the associated lane when in PMA \n\nisolation mode. (Not re-synchronized to apb_pclk)."]
    #[inline(always)]
    #[must_use]
    pub fn field0(&mut self) -> Field0W<PmaIsoTxDataLoLane3Spec> {
        Field0W::new(self, 0)
    }
}
#[doc = "PMA transmit low data isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_tx_data_lo_lane3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_tx_data_lo_lane3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmaIsoTxDataLoLane3Spec;
impl crate::RegisterSpec for PmaIsoTxDataLoLane3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pma_iso_tx_data_lo_lane3::R`](R) reader structure"]
impl crate::Readable for PmaIsoTxDataLoLane3Spec {}
#[doc = "`write(|w| ..)` method takes [`pma_iso_tx_data_lo_lane3::W`](W) writer structure"]
impl crate::Writable for PmaIsoTxDataLoLane3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PMA_ISO_TX_DATA_LO_LANE3 to value 0"]
impl crate::Resettable for PmaIsoTxDataLoLane3Spec {
    const RESET_VALUE: u16 = 0;
}
