#[doc = "Register `PMA_ISO_LINK_MODE_LANE3` reader"]
pub type R = crate::R<PmaIsoLinkModeLane3Spec>;
#[doc = "Register `PMA_ISO_LINK_MODE_LANE3` writer"]
pub type W = crate::W<PmaIsoLinkModeLane3Spec>;
#[doc = "Field `FIELD7` reader - Drives xcvr_data_width PMA input for the associated lane when in \n\nPMA isolation mode."]
pub type Field7R = crate::FieldReader;
#[doc = "Field `FIELD7` writer - Drives xcvr_data_width PMA input for the associated lane when in \n\nPMA isolation mode."]
pub type Field7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FIELD6` reader - Reserved"]
pub type Field6R = crate::BitReader;
#[doc = "Field `FIELD5` reader - Drives xcvr_standard_mode PMA input for the associated lane when \n\nin PMA isolation mode."]
pub type Field5R = crate::FieldReader;
#[doc = "Field `FIELD5` writer - Drives xcvr_standard_mode PMA input for the associated lane when \n\nin PMA isolation mode."]
pub type Field5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIELD4` reader - Reserved"]
pub type Field4R = crate::FieldReader;
#[doc = "Field `FIELD3` reader - Drives the tx_high_z PMA input for the associated lane when in PMA \n\nisolation mode"]
pub type Field3R = crate::BitReader;
#[doc = "Field `FIELD3` writer - Drives the tx_high_z PMA input for the associated lane when in PMA \n\nisolation mode"]
pub type Field3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD2` reader - Reserved"]
pub type Field2R = crate::BitReader;
#[doc = "Field `FIELD1` reader - rx_reset_n PMA input for the associated lane when in PMA isolation \n\nmode. Valid for PMA lanes 2 and 3 only. For PMA lanes 0 and 3, \n\nreserved."]
pub type Field1R = crate::BitReader;
#[doc = "Field `FIELD1` writer - rx_reset_n PMA input for the associated lane when in PMA isolation \n\nmode. Valid for PMA lanes 2 and 3 only. For PMA lanes 0 and 3, \n\nreserved."]
pub type Field1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD0` reader - tx_reset_n PMA input for the associated lane when in PMA isolation \n\nmode."]
pub type Field0R = crate::BitReader;
#[doc = "Field `FIELD0` writer - tx_reset_n PMA input for the associated lane when in PMA isolation \n\nmode."]
pub type Field0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Drives xcvr_data_width PMA input for the associated lane when in \n\nPMA isolation mode."]
    #[inline(always)]
    pub fn field7(&self) -> Field7R {
        Field7R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn field6(&self) -> Field6R {
        Field6R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Drives xcvr_standard_mode PMA input for the associated lane when \n\nin PMA isolation mode."]
    #[inline(always)]
    pub fn field5(&self) -> Field5R {
        Field5R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:11 - Reserved"]
    #[inline(always)]
    pub fn field4(&self) -> Field4R {
        Field4R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Drives the tx_high_z PMA input for the associated lane when in PMA \n\nisolation mode"]
    #[inline(always)]
    pub fn field3(&self) -> Field3R {
        Field3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn field2(&self) -> Field2R {
        Field2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - rx_reset_n PMA input for the associated lane when in PMA isolation \n\nmode. Valid for PMA lanes 2 and 3 only. For PMA lanes 0 and 3, \n\nreserved."]
    #[inline(always)]
    pub fn field1(&self) -> Field1R {
        Field1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - tx_reset_n PMA input for the associated lane when in PMA isolation \n\nmode."]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Drives xcvr_data_width PMA input for the associated lane when in \n\nPMA isolation mode."]
    #[inline(always)]
    #[must_use]
    pub fn field7(&mut self) -> Field7W<PmaIsoLinkModeLane3Spec> {
        Field7W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Drives xcvr_standard_mode PMA input for the associated lane when \n\nin PMA isolation mode."]
    #[inline(always)]
    #[must_use]
    pub fn field5(&mut self) -> Field5W<PmaIsoLinkModeLane3Spec> {
        Field5W::new(self, 4)
    }
    #[doc = "Bit 12 - Drives the tx_high_z PMA input for the associated lane when in PMA \n\nisolation mode"]
    #[inline(always)]
    #[must_use]
    pub fn field3(&mut self) -> Field3W<PmaIsoLinkModeLane3Spec> {
        Field3W::new(self, 12)
    }
    #[doc = "Bit 14 - rx_reset_n PMA input for the associated lane when in PMA isolation \n\nmode. Valid for PMA lanes 2 and 3 only. For PMA lanes 0 and 3, \n\nreserved."]
    #[inline(always)]
    #[must_use]
    pub fn field1(&mut self) -> Field1W<PmaIsoLinkModeLane3Spec> {
        Field1W::new(self, 14)
    }
    #[doc = "Bit 15 - tx_reset_n PMA input for the associated lane when in PMA isolation \n\nmode."]
    #[inline(always)]
    #[must_use]
    pub fn field0(&mut self) -> Field0W<PmaIsoLinkModeLane3Spec> {
        Field0W::new(self, 15)
    }
}
#[doc = "PMA Isolation mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_link_mode_lane3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_link_mode_lane3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmaIsoLinkModeLane3Spec;
impl crate::RegisterSpec for PmaIsoLinkModeLane3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pma_iso_link_mode_lane3::R`](R) reader structure"]
impl crate::Readable for PmaIsoLinkModeLane3Spec {}
#[doc = "`write(|w| ..)` method takes [`pma_iso_link_mode_lane3::W`](W) writer structure"]
impl crate::Writable for PmaIsoLinkModeLane3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PMA_ISO_LINK_MODE_LANE3 to value 0xc000"]
impl crate::Resettable for PmaIsoLinkModeLane3Spec {
    const RESET_VALUE: u16 = 0xc000;
}
