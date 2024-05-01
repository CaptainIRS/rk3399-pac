#[doc = "Register `LPCLK_CTRL` reader"]
pub type R = crate::R<LpclkCtrlSpec>;
#[doc = "Register `LPCLK_CTRL` writer"]
pub type W = crate::W<LpclkCtrlSpec>;
#[doc = "Field `PHY_TXREQUESTCLKHS` reader - phy_txrequestclkhs\n\nThis bit controls the D-PHY PPI txrequestclkhs signal."]
pub type PhyTxrequestclkhsR = crate::BitReader;
#[doc = "Field `PHY_TXREQUESTCLKHS` writer - phy_txrequestclkhs\n\nThis bit controls the D-PHY PPI txrequestclkhs signal."]
pub type PhyTxrequestclkhsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CLKLANE_CTRL` reader - auto_clklane_ctrl\n\nThis bit enables the automatic mechanism to stop providing clock in\n\nthe clock lane when time allows."]
pub type AutoClklaneCtrlR = crate::BitReader;
#[doc = "Field `AUTO_CLKLANE_CTRL` writer - auto_clklane_ctrl\n\nThis bit enables the automatic mechanism to stop providing clock in\n\nthe clock lane when time allows."]
pub type AutoClklaneCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - phy_txrequestclkhs\n\nThis bit controls the D-PHY PPI txrequestclkhs signal."]
    #[inline(always)]
    pub fn phy_txrequestclkhs(&self) -> PhyTxrequestclkhsR {
        PhyTxrequestclkhsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - auto_clklane_ctrl\n\nThis bit enables the automatic mechanism to stop providing clock in\n\nthe clock lane when time allows."]
    #[inline(always)]
    pub fn auto_clklane_ctrl(&self) -> AutoClklaneCtrlR {
        AutoClklaneCtrlR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - phy_txrequestclkhs\n\nThis bit controls the D-PHY PPI txrequestclkhs signal."]
    #[inline(always)]
    #[must_use]
    pub fn phy_txrequestclkhs(&mut self) -> PhyTxrequestclkhsW<LpclkCtrlSpec> {
        PhyTxrequestclkhsW::new(self, 0)
    }
    #[doc = "Bit 1 - auto_clklane_ctrl\n\nThis bit enables the automatic mechanism to stop providing clock in\n\nthe clock lane when time allows."]
    #[inline(always)]
    #[must_use]
    pub fn auto_clklane_ctrl(&mut self) -> AutoClklaneCtrlW<LpclkCtrlSpec> {
        AutoClklaneCtrlW::new(self, 1)
    }
}
#[doc = "Low-power in Clock Lane Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpclk_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpclk_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpclkCtrlSpec;
impl crate::RegisterSpec for LpclkCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpclk_ctrl::R`](R) reader structure"]
impl crate::Readable for LpclkCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`lpclk_ctrl::W`](W) writer structure"]
impl crate::Writable for LpclkCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPCLK_CTRL to value 0"]
impl crate::Resettable for LpclkCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
