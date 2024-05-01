#[doc = "Register `PHY_TX_TRIGGERS` reader"]
pub type R = crate::R<PhyTxTriggersSpec>;
#[doc = "Register `PHY_TX_TRIGGERS` writer"]
pub type W = crate::W<PhyTxTriggersSpec>;
#[doc = "Field `PHY_TX_TRIGGERS` reader - phy_tx_triggers\n\nThis field controls the trigger transmissions."]
pub type PhyTxTriggersR = crate::FieldReader;
#[doc = "Field `PHY_TX_TRIGGERS` writer - phy_tx_triggers\n\nThis field controls the trigger transmissions."]
pub type PhyTxTriggersW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - phy_tx_triggers\n\nThis field controls the trigger transmissions."]
    #[inline(always)]
    pub fn phy_tx_triggers(&self) -> PhyTxTriggersR {
        PhyTxTriggersR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - phy_tx_triggers\n\nThis field controls the trigger transmissions."]
    #[inline(always)]
    #[must_use]
    pub fn phy_tx_triggers(&mut self) -> PhyTxTriggersW<PhyTxTriggersSpec> {
        PhyTxTriggersW::new(self, 0)
    }
}
#[doc = "D-PHY Transmit Triggers Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_tx_triggers::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tx_triggers::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyTxTriggersSpec;
impl crate::RegisterSpec for PhyTxTriggersSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_tx_triggers::R`](R) reader structure"]
impl crate::Readable for PhyTxTriggersSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_tx_triggers::W`](W) writer structure"]
impl crate::Writable for PhyTxTriggersSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHY_TX_TRIGGERS to value 0"]
impl crate::Resettable for PhyTxTriggersSpec {
    const RESET_VALUE: u32 = 0;
}
