#[doc = "Register `PMA_ISO_TX_CFG_LANE1` reader"]
pub type R = crate::R<PmaIsoTxCfgLane1Spec>;
#[doc = "Register `PMA_ISO_TX_CFG_LANE1` writer"]
pub type W = crate::W<PmaIsoTxCfgLane1Spec>;
#[doc = "Field `FIELD5` reader - Drives tx_vmargin PMA input for the associated lane."]
pub type Field5R = crate::FieldReader;
#[doc = "Field `FIELD5` writer - Drives tx_vmargin PMA input for the associated lane."]
pub type Field5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FIELD4` reader - Reserved"]
pub type Field4R = crate::FieldReader;
#[doc = "Field `FIELD3` reader - Drives tx_low_power_swing_en PMA input for the associated lane."]
pub type Field3R = crate::BitReader;
#[doc = "Field `FIELD3` writer - Drives tx_low_power_swing_en PMA input for the associated lane."]
pub type Field3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD2` reader - Reserved"]
pub type Field2R = crate::FieldReader;
#[doc = "Field `FIELD1` reader - Drives tx_deemphasis PMA input for the associated lane when in PMA \n\nisolation mode"]
pub type Field1R = crate::FieldReader;
#[doc = "Field `FIELD1` writer - Drives tx_deemphasis PMA input for the associated lane when in PMA \n\nisolation mode"]
pub type Field1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIELD0` reader - Reserved"]
pub type Field0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Drives tx_vmargin PMA input for the associated lane."]
    #[inline(always)]
    pub fn field5(&self) -> Field5R {
        Field5R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - Reserved"]
    #[inline(always)]
    pub fn field4(&self) -> Field4R {
        Field4R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Drives tx_low_power_swing_en PMA input for the associated lane."]
    #[inline(always)]
    pub fn field3(&self) -> Field3R {
        Field3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Reserved"]
    #[inline(always)]
    pub fn field2(&self) -> Field2R {
        Field2R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Drives tx_deemphasis PMA input for the associated lane when in PMA \n\nisolation mode"]
    #[inline(always)]
    pub fn field1(&self) -> Field1R {
        Field1R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Reserved"]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Drives tx_vmargin PMA input for the associated lane."]
    #[inline(always)]
    #[must_use]
    pub fn field5(&mut self) -> Field5W<PmaIsoTxCfgLane1Spec> {
        Field5W::new(self, 0)
    }
    #[doc = "Bit 8 - Drives tx_low_power_swing_en PMA input for the associated lane."]
    #[inline(always)]
    #[must_use]
    pub fn field3(&mut self) -> Field3W<PmaIsoTxCfgLane1Spec> {
        Field3W::new(self, 8)
    }
    #[doc = "Bits 12:13 - Drives tx_deemphasis PMA input for the associated lane when in PMA \n\nisolation mode"]
    #[inline(always)]
    #[must_use]
    pub fn field1(&mut self) -> Field1W<PmaIsoTxCfgLane1Spec> {
        Field1W::new(self, 12)
    }
}
#[doc = "PMA TX configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_tx_cfg_lane1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_tx_cfg_lane1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmaIsoTxCfgLane1Spec;
impl crate::RegisterSpec for PmaIsoTxCfgLane1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pma_iso_tx_cfg_lane1::R`](R) reader structure"]
impl crate::Readable for PmaIsoTxCfgLane1Spec {}
#[doc = "`write(|w| ..)` method takes [`pma_iso_tx_cfg_lane1::W`](W) writer structure"]
impl crate::Writable for PmaIsoTxCfgLane1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PMA_ISO_TX_CFG_LANE1 to value 0"]
impl crate::Resettable for PmaIsoTxCfgLane1Spec {
    const RESET_VALUE: u16 = 0;
}
