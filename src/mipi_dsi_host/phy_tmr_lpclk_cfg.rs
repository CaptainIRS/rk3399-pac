#[doc = "Register `PHY_TMR_LPCLK_CFG` reader"]
pub type R = crate::R<PhyTmrLpclkCfgSpec>;
#[doc = "Register `PHY_TMR_LPCLK_CFG` writer"]
pub type W = crate::W<PhyTmrLpclkCfgSpec>;
#[doc = "Field `PHY_CLKLP2HS_TIME` reader - phy_clklp2hs_time\n\nThis field configures the maximum time that the D-PHY clock lane\n\ntakes to go from low-power to high-speed transmission measured\n\nin lane byte clock cycles."]
pub type PhyClklp2hsTimeR = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLKLP2HS_TIME` writer - phy_clklp2hs_time\n\nThis field configures the maximum time that the D-PHY clock lane\n\ntakes to go from low-power to high-speed transmission measured\n\nin lane byte clock cycles."]
pub type PhyClklp2hsTimeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_CLKHS2LP_TIME` reader - phy_clkhs2lp_time\n\nThis field configures the maximum time that the D-PHY clock lane\n\ntakes to go from high-speed to low-power transmission measured\n\nin lane byte clock cycles."]
pub type PhyClkhs2lpTimeR = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLKHS2LP_TIME` writer - phy_clkhs2lp_time\n\nThis field configures the maximum time that the D-PHY clock lane\n\ntakes to go from high-speed to low-power transmission measured\n\nin lane byte clock cycles."]
pub type PhyClkhs2lpTimeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - phy_clklp2hs_time\n\nThis field configures the maximum time that the D-PHY clock lane\n\ntakes to go from low-power to high-speed transmission measured\n\nin lane byte clock cycles."]
    #[inline(always)]
    pub fn phy_clklp2hs_time(&self) -> PhyClklp2hsTimeR {
        PhyClklp2hsTimeR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - phy_clkhs2lp_time\n\nThis field configures the maximum time that the D-PHY clock lane\n\ntakes to go from high-speed to low-power transmission measured\n\nin lane byte clock cycles."]
    #[inline(always)]
    pub fn phy_clkhs2lp_time(&self) -> PhyClkhs2lpTimeR {
        PhyClkhs2lpTimeR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - phy_clklp2hs_time\n\nThis field configures the maximum time that the D-PHY clock lane\n\ntakes to go from low-power to high-speed transmission measured\n\nin lane byte clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clklp2hs_time(&mut self) -> PhyClklp2hsTimeW<PhyTmrLpclkCfgSpec> {
        PhyClklp2hsTimeW::new(self, 0)
    }
    #[doc = "Bits 16:25 - phy_clkhs2lp_time\n\nThis field configures the maximum time that the D-PHY clock lane\n\ntakes to go from high-speed to low-power transmission measured\n\nin lane byte clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clkhs2lp_time(&mut self) -> PhyClkhs2lpTimeW<PhyTmrLpclkCfgSpec> {
        PhyClkhs2lpTimeW::new(self, 16)
    }
}
#[doc = "D-PHY Timing Configuration for the Clock Lane\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_tmr_lpclk_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tmr_lpclk_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyTmrLpclkCfgSpec;
impl crate::RegisterSpec for PhyTmrLpclkCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_tmr_lpclk_cfg::R`](R) reader structure"]
impl crate::Readable for PhyTmrLpclkCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_tmr_lpclk_cfg::W`](W) writer structure"]
impl crate::Writable for PhyTmrLpclkCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHY_TMR_LPCLK_CFG to value 0"]
impl crate::Resettable for PhyTmrLpclkCfgSpec {
    const RESET_VALUE: u32 = 0;
}
