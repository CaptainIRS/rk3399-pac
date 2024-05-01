#[doc = "Register `PHY_ULPS_CTRL` reader"]
pub type R = crate::R<PhyUlpsCtrlSpec>;
#[doc = "Register `PHY_ULPS_CTRL` writer"]
pub type W = crate::W<PhyUlpsCtrlSpec>;
#[doc = "Field `PHY_TXREQULPSCLK` reader - phy_txrequlpsclk\n\nULPS mode Request on clock lane."]
pub type PhyTxrequlpsclkR = crate::BitReader;
#[doc = "Field `PHY_TXREQULPSCLK` writer - phy_txrequlpsclk\n\nULPS mode Request on clock lane."]
pub type PhyTxrequlpsclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_TXEXITULPSCLK` reader - phy_txexitulpsclk\n\nULPS mode Exit on clock lane."]
pub type PhyTxexitulpsclkR = crate::BitReader;
#[doc = "Field `PHY_TXEXITULPSCLK` writer - phy_txexitulpsclk\n\nULPS mode Exit on clock lane."]
pub type PhyTxexitulpsclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_TXREQULPSLAN` reader - phy_txrequlpslan\n\nULPS mode Request on all active data lanes."]
pub type PhyTxrequlpslanR = crate::BitReader;
#[doc = "Field `PHY_TXREQULPSLAN` writer - phy_txrequlpslan\n\nULPS mode Request on all active data lanes."]
pub type PhyTxrequlpslanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_TXEXITULPSLAN` reader - phy_txexitulpslan\n\nULPS mode Exit on all active data lanes."]
pub type PhyTxexitulpslanR = crate::BitReader;
#[doc = "Field `PHY_TXEXITULPSLAN` writer - phy_txexitulpslan\n\nULPS mode Exit on all active data lanes."]
pub type PhyTxexitulpslanW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - phy_txrequlpsclk\n\nULPS mode Request on clock lane."]
    #[inline(always)]
    pub fn phy_txrequlpsclk(&self) -> PhyTxrequlpsclkR {
        PhyTxrequlpsclkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - phy_txexitulpsclk\n\nULPS mode Exit on clock lane."]
    #[inline(always)]
    pub fn phy_txexitulpsclk(&self) -> PhyTxexitulpsclkR {
        PhyTxexitulpsclkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - phy_txrequlpslan\n\nULPS mode Request on all active data lanes."]
    #[inline(always)]
    pub fn phy_txrequlpslan(&self) -> PhyTxrequlpslanR {
        PhyTxrequlpslanR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - phy_txexitulpslan\n\nULPS mode Exit on all active data lanes."]
    #[inline(always)]
    pub fn phy_txexitulpslan(&self) -> PhyTxexitulpslanR {
        PhyTxexitulpslanR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - phy_txrequlpsclk\n\nULPS mode Request on clock lane."]
    #[inline(always)]
    #[must_use]
    pub fn phy_txrequlpsclk(&mut self) -> PhyTxrequlpsclkW<PhyUlpsCtrlSpec> {
        PhyTxrequlpsclkW::new(self, 0)
    }
    #[doc = "Bit 1 - phy_txexitulpsclk\n\nULPS mode Exit on clock lane."]
    #[inline(always)]
    #[must_use]
    pub fn phy_txexitulpsclk(&mut self) -> PhyTxexitulpsclkW<PhyUlpsCtrlSpec> {
        PhyTxexitulpsclkW::new(self, 1)
    }
    #[doc = "Bit 2 - phy_txrequlpslan\n\nULPS mode Request on all active data lanes."]
    #[inline(always)]
    #[must_use]
    pub fn phy_txrequlpslan(&mut self) -> PhyTxrequlpslanW<PhyUlpsCtrlSpec> {
        PhyTxrequlpslanW::new(self, 2)
    }
    #[doc = "Bit 3 - phy_txexitulpslan\n\nULPS mode Exit on all active data lanes."]
    #[inline(always)]
    #[must_use]
    pub fn phy_txexitulpslan(&mut self) -> PhyTxexitulpslanW<PhyUlpsCtrlSpec> {
        PhyTxexitulpslanW::new(self, 3)
    }
}
#[doc = "D-PHY Ultra Low-Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_ulps_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_ulps_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyUlpsCtrlSpec;
impl crate::RegisterSpec for PhyUlpsCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_ulps_ctrl::R`](R) reader structure"]
impl crate::Readable for PhyUlpsCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_ulps_ctrl::W`](W) writer structure"]
impl crate::Writable for PhyUlpsCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHY_ULPS_CTRL to value 0"]
impl crate::Resettable for PhyUlpsCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
