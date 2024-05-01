#[doc = "Register `PHY_TMR_CFG` reader"]
pub type R = crate::R<PhyTmrCfgSpec>;
#[doc = "Register `PHY_TMR_CFG` writer"]
pub type W = crate::W<PhyTmrCfgSpec>;
#[doc = "Field `MAX_RD_TIME` reader - max_rd_time\n\nThis field configures the maximum time required to perform a read\n\ncommand in lane byte clock cycles. This register can only be\n\nmodified when no read command is in progress."]
pub type MaxRdTimeR = crate::FieldReader<u16>;
#[doc = "Field `MAX_RD_TIME` writer - max_rd_time\n\nThis field configures the maximum time required to perform a read\n\ncommand in lane byte clock cycles. This register can only be\n\nmodified when no read command is in progress."]
pub type MaxRdTimeW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `PHY_LP2HS_TIME` reader - phy_lp2hs_time\n\nThis field configures the maximum time that the D-PHY data lanes\n\ntake to go from low-power to high-speed transmission measured in\n\nlane byte clock cycles."]
pub type PhyLp2hsTimeR = crate::FieldReader;
#[doc = "Field `PHY_LP2HS_TIME` writer - phy_lp2hs_time\n\nThis field configures the maximum time that the D-PHY data lanes\n\ntake to go from low-power to high-speed transmission measured in\n\nlane byte clock cycles."]
pub type PhyLp2hsTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_HS2LP_TIME` reader - phy_hs2lp_time\n\nThis field configures the maximum time that the D-PHY data lanes\n\ntake to go from high-speed to low-power transmission measured in\n\nlane byte clock cycles."]
pub type PhyHs2lpTimeR = crate::FieldReader;
#[doc = "Field `PHY_HS2LP_TIME` writer - phy_hs2lp_time\n\nThis field configures the maximum time that the D-PHY data lanes\n\ntake to go from high-speed to low-power transmission measured in\n\nlane byte clock cycles."]
pub type PhyHs2lpTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:14 - max_rd_time\n\nThis field configures the maximum time required to perform a read\n\ncommand in lane byte clock cycles. This register can only be\n\nmodified when no read command is in progress."]
    #[inline(always)]
    pub fn max_rd_time(&self) -> MaxRdTimeR {
        MaxRdTimeR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:23 - phy_lp2hs_time\n\nThis field configures the maximum time that the D-PHY data lanes\n\ntake to go from low-power to high-speed transmission measured in\n\nlane byte clock cycles."]
    #[inline(always)]
    pub fn phy_lp2hs_time(&self) -> PhyLp2hsTimeR {
        PhyLp2hsTimeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - phy_hs2lp_time\n\nThis field configures the maximum time that the D-PHY data lanes\n\ntake to go from high-speed to low-power transmission measured in\n\nlane byte clock cycles."]
    #[inline(always)]
    pub fn phy_hs2lp_time(&self) -> PhyHs2lpTimeR {
        PhyHs2lpTimeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - max_rd_time\n\nThis field configures the maximum time required to perform a read\n\ncommand in lane byte clock cycles. This register can only be\n\nmodified when no read command is in progress."]
    #[inline(always)]
    #[must_use]
    pub fn max_rd_time(&mut self) -> MaxRdTimeW<PhyTmrCfgSpec> {
        MaxRdTimeW::new(self, 0)
    }
    #[doc = "Bits 16:23 - phy_lp2hs_time\n\nThis field configures the maximum time that the D-PHY data lanes\n\ntake to go from low-power to high-speed transmission measured in\n\nlane byte clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp2hs_time(&mut self) -> PhyLp2hsTimeW<PhyTmrCfgSpec> {
        PhyLp2hsTimeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - phy_hs2lp_time\n\nThis field configures the maximum time that the D-PHY data lanes\n\ntake to go from high-speed to low-power transmission measured in\n\nlane byte clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn phy_hs2lp_time(&mut self) -> PhyHs2lpTimeW<PhyTmrCfgSpec> {
        PhyHs2lpTimeW::new(self, 24)
    }
}
#[doc = "D-PHY Data Lanes Timing Configuration Registe\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_tmr_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tmr_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyTmrCfgSpec;
impl crate::RegisterSpec for PhyTmrCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_tmr_cfg::R`](R) reader structure"]
impl crate::Readable for PhyTmrCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_tmr_cfg::W`](W) writer structure"]
impl crate::Writable for PhyTmrCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHY_TMR_CFG to value 0"]
impl crate::Resettable for PhyTmrCfgSpec {
    const RESET_VALUE: u32 = 0;
}
