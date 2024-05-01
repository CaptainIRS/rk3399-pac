#[doc = "Register `PHY_RSTZ` reader"]
pub type R = crate::R<PhyRstzSpec>;
#[doc = "Register `PHY_RSTZ` writer"]
pub type W = crate::W<PhyRstzSpec>;
#[doc = "Field `PHY_SHUTDOWNZ` reader - phy_shutdownz\n\nWhen set to 0, this bit places the D-PHY macro in power-down\n\nstate."]
pub type PhyShutdownzR = crate::BitReader;
#[doc = "Field `PHY_SHUTDOWNZ` writer - phy_shutdownz\n\nWhen set to 0, this bit places the D-PHY macro in power-down\n\nstate."]
pub type PhyShutdownzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_RSTZ` reader - phy_rstz\n\nWhen set to 0, this bit places the digital section of the D-PHY in the\n\nreset state."]
pub type PhyRstzR = crate::BitReader;
#[doc = "Field `PHY_RSTZ` writer - phy_rstz\n\nWhen set to 0, this bit places the digital section of the D-PHY in the\n\nreset state."]
pub type PhyRstzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_ENABLECLK` reader - phy_enableclk\n\nWhen set to1, this bit enables the D-PHY Clock Lane module."]
pub type PhyEnableclkR = crate::BitReader;
#[doc = "Field `PHY_ENABLECLK` writer - phy_enableclk\n\nWhen set to1, this bit enables the D-PHY Clock Lane module."]
pub type PhyEnableclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_FORCEPLL` reader - phy_forcepll\n\nWhen the D-PHY is in ULPS, this bit enables the D-PHY PLL.\n\nDependency: DSI_HOST_FPGA = 0. Otherwise, this bit is reserved."]
pub type PhyForcepllR = crate::BitReader;
#[doc = "Field `PHY_FORCEPLL` writer - phy_forcepll\n\nWhen the D-PHY is in ULPS, this bit enables the D-PHY PLL.\n\nDependency: DSI_HOST_FPGA = 0. Otherwise, this bit is reserved."]
pub type PhyForcepllW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - phy_shutdownz\n\nWhen set to 0, this bit places the D-PHY macro in power-down\n\nstate."]
    #[inline(always)]
    pub fn phy_shutdownz(&self) -> PhyShutdownzR {
        PhyShutdownzR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - phy_rstz\n\nWhen set to 0, this bit places the digital section of the D-PHY in the\n\nreset state."]
    #[inline(always)]
    pub fn phy_rstz(&self) -> PhyRstzR {
        PhyRstzR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - phy_enableclk\n\nWhen set to1, this bit enables the D-PHY Clock Lane module."]
    #[inline(always)]
    pub fn phy_enableclk(&self) -> PhyEnableclkR {
        PhyEnableclkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - phy_forcepll\n\nWhen the D-PHY is in ULPS, this bit enables the D-PHY PLL.\n\nDependency: DSI_HOST_FPGA = 0. Otherwise, this bit is reserved."]
    #[inline(always)]
    pub fn phy_forcepll(&self) -> PhyForcepllR {
        PhyForcepllR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - phy_shutdownz\n\nWhen set to 0, this bit places the D-PHY macro in power-down\n\nstate."]
    #[inline(always)]
    #[must_use]
    pub fn phy_shutdownz(&mut self) -> PhyShutdownzW<PhyRstzSpec> {
        PhyShutdownzW::new(self, 0)
    }
    #[doc = "Bit 1 - phy_rstz\n\nWhen set to 0, this bit places the digital section of the D-PHY in the\n\nreset state."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rstz(&mut self) -> PhyRstzW<PhyRstzSpec> {
        PhyRstzW::new(self, 1)
    }
    #[doc = "Bit 2 - phy_enableclk\n\nWhen set to1, this bit enables the D-PHY Clock Lane module."]
    #[inline(always)]
    #[must_use]
    pub fn phy_enableclk(&mut self) -> PhyEnableclkW<PhyRstzSpec> {
        PhyEnableclkW::new(self, 2)
    }
    #[doc = "Bit 3 - phy_forcepll\n\nWhen the D-PHY is in ULPS, this bit enables the D-PHY PLL.\n\nDependency: DSI_HOST_FPGA = 0. Otherwise, this bit is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn phy_forcepll(&mut self) -> PhyForcepllW<PhyRstzSpec> {
        PhyForcepllW::new(self, 3)
    }
}
#[doc = "D-PHY Reset Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_rstz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_rstz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyRstzSpec;
impl crate::RegisterSpec for PhyRstzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_rstz::R`](R) reader structure"]
impl crate::Readable for PhyRstzSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_rstz::W`](W) writer structure"]
impl crate::Writable for PhyRstzSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHY_RSTZ to value 0"]
impl crate::Resettable for PhyRstzSpec {
    const RESET_VALUE: u32 = 0;
}
