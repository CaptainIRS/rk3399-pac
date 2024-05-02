#[doc = "Register `PMA_ISO_TX_DATA_HI_LANE0` reader"]
pub type R = crate::R<PmaIsoTxDataHiLane0Spec>;
#[doc = "Register `PMA_ISO_TX_DATA_HI_LANE0` writer"]
pub type W = crate::W<PmaIsoTxDataHiLane0Spec>;
#[doc = "Field `FIELD1` reader - Drives tx_td\\[19:16\\]
PMA input for the associated lane when in PMA \n\nisolation mode. (Not re-synchronized to apb_pclk)."]
pub type Field1R = crate::FieldReader;
#[doc = "Field `FIELD1` writer - Drives tx_td\\[19:16\\]
PMA input for the associated lane when in PMA \n\nisolation mode. (Not re-synchronized to apb_pclk)."]
pub type Field1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FIELD0` reader - Reserved"]
pub type Field0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Drives tx_td\\[19:16\\]
PMA input for the associated lane when in PMA \n\nisolation mode. (Not re-synchronized to apb_pclk)."]
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
impl W {
    #[doc = "Bits 0:3 - Drives tx_td\\[19:16\\]
PMA input for the associated lane when in PMA \n\nisolation mode. (Not re-synchronized to apb_pclk)."]
    #[inline(always)]
    #[must_use]
    pub fn field1(&mut self) -> Field1W<PmaIsoTxDataHiLane0Spec> {
        Field1W::new(self, 0)
    }
}
#[doc = "PMA transmit high data isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_tx_data_hi_lane0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_tx_data_hi_lane0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmaIsoTxDataHiLane0Spec;
impl crate::RegisterSpec for PmaIsoTxDataHiLane0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pma_iso_tx_data_hi_lane0::R`](R) reader structure"]
impl crate::Readable for PmaIsoTxDataHiLane0Spec {}
#[doc = "`write(|w| ..)` method takes [`pma_iso_tx_data_hi_lane0::W`](W) writer structure"]
impl crate::Writable for PmaIsoTxDataHiLane0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PMA_ISO_TX_DATA_HI_LANE0 to value 0"]
impl crate::Resettable for PmaIsoTxDataHiLane0Spec {
    const RESET_VALUE: u16 = 0;
}
