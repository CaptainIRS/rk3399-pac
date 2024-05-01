#[doc = "Register `PHY_STATUS` reader"]
pub type R = crate::R<PhyStatusSpec>;
#[doc = "Field `PHY_LOCK` reader - phy_lock\n\nThis bit indicates the status of phylock D-PHY signal."]
pub type PhyLockR = crate::BitReader;
#[doc = "Field `PHY_DIRECTION` reader - phy_direction\n\nThis bit indicates the status of phydirection D-PHY signal."]
pub type PhyDirectionR = crate::BitReader;
#[doc = "Field `PHY_STOPSTATECLKLANE` reader - phy_stopstateclklane\n\nphy_stopstateclklane"]
pub type PhyStopstateclklaneR = crate::BitReader;
#[doc = "Field `PHY_ULPSACTIVENOTCLK` reader - phy_ulpsactivenotclk\n\nThis bit indicates the status of phyulpsactivenotclk D-PHY signal."]
pub type PhyUlpsactivenotclkR = crate::BitReader;
#[doc = "Field `PHY_STOPSTATE0LANE` reader - phy_stopstate0lane\n\nThis bit indicates the status of phystopstate0lane D-PHY signal."]
pub type PhyStopstate0laneR = crate::BitReader;
#[doc = "Field `PHY_ULPSACTIVENOT0LANE` reader - phy_ulpsactivenot0lane\n\nThis bit indicates the status of ulpsactivenot0lane D-PHY signal."]
pub type PhyUlpsactivenot0laneR = crate::BitReader;
#[doc = "Field `PHY_RXULPSESC0LANE` reader - phy_rxulpsesc0lane\n\nThis bit indicates the status of rxulpsesc0lane D-PHY signal."]
pub type PhyRxulpsesc0laneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - phy_lock\n\nThis bit indicates the status of phylock D-PHY signal."]
    #[inline(always)]
    pub fn phy_lock(&self) -> PhyLockR {
        PhyLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - phy_direction\n\nThis bit indicates the status of phydirection D-PHY signal."]
    #[inline(always)]
    pub fn phy_direction(&self) -> PhyDirectionR {
        PhyDirectionR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - phy_stopstateclklane\n\nphy_stopstateclklane"]
    #[inline(always)]
    pub fn phy_stopstateclklane(&self) -> PhyStopstateclklaneR {
        PhyStopstateclklaneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - phy_ulpsactivenotclk\n\nThis bit indicates the status of phyulpsactivenotclk D-PHY signal."]
    #[inline(always)]
    pub fn phy_ulpsactivenotclk(&self) -> PhyUlpsactivenotclkR {
        PhyUlpsactivenotclkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - phy_stopstate0lane\n\nThis bit indicates the status of phystopstate0lane D-PHY signal."]
    #[inline(always)]
    pub fn phy_stopstate0lane(&self) -> PhyStopstate0laneR {
        PhyStopstate0laneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - phy_ulpsactivenot0lane\n\nThis bit indicates the status of ulpsactivenot0lane D-PHY signal."]
    #[inline(always)]
    pub fn phy_ulpsactivenot0lane(&self) -> PhyUlpsactivenot0laneR {
        PhyUlpsactivenot0laneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - phy_rxulpsesc0lane\n\nThis bit indicates the status of rxulpsesc0lane D-PHY signal."]
    #[inline(always)]
    pub fn phy_rxulpsesc0lane(&self) -> PhyRxulpsesc0laneR {
        PhyRxulpsesc0laneR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Register0010 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyStatusSpec;
impl crate::RegisterSpec for PhyStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_status::R`](R) reader structure"]
impl crate::Readable for PhyStatusSpec {}
#[doc = "`reset()` method sets PHY_STATUS to value 0"]
impl crate::Resettable for PhyStatusSpec {
    const RESET_VALUE: u32 = 0;
}
