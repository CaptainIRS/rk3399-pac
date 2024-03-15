#[doc = "Register `DENALI_PHY_514` reader"]
pub type R = crate::R<DenaliPhy514Spec>;
#[doc = "Register `DENALI_PHY_514` writer"]
pub type W = crate::W<DenaliPhy514Spec>;
#[doc = "Field `PHY_ADR_CLK_BYPASS_OVERRIDE_0` reader - Bypass mode override setting for address slice 0. Set to 1 to enable."]
pub type PhyAdrClkBypassOverride0R = crate::BitReader;
#[doc = "Field `PHY_ADR_CLK_BYPASS_OVERRIDE_0` writer - Bypass mode override setting for address slice 0. Set to 1 to enable."]
pub type PhyAdrClkBypassOverride0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_ADR_MANUAL_CLEAR_0` writer - Manual reset/clear of internal logic for address slice 0. Bit (0) is reset of master delay min/max lock values. Bit (1) is manual reset of master delay unlock counter. Bit (2) clears the loopback error/results registers. Set each bit to 1 to reset. WRITE-ONLY"]
pub type ScPhyAdrManualClear0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Bypass mode override setting for address slice 0. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_adr_clk_bypass_override_0(&self) -> PhyAdrClkBypassOverride0R {
        PhyAdrClkBypassOverride0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass mode override setting for address slice 0. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_clk_bypass_override_0(&mut self) -> PhyAdrClkBypassOverride0W<DenaliPhy514Spec> {
        PhyAdrClkBypassOverride0W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Manual reset/clear of internal logic for address slice 0. Bit (0) is reset of master delay min/max lock values. Bit (1) is manual reset of master delay unlock counter. Bit (2) clears the loopback error/results registers. Set each bit to 1 to reset. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_adr_manual_clear_0(&mut self) -> ScPhyAdrManualClear0W<DenaliPhy514Spec> {
        ScPhyAdrManualClear0W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_514::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_514::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy514Spec;
impl crate::RegisterSpec for DenaliPhy514Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_514::R`](R) reader structure"]
impl crate::Readable for DenaliPhy514Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_514::W`](W) writer structure"]
impl crate::Writable for DenaliPhy514Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_514 to value 0"]
impl crate::Resettable for DenaliPhy514Spec {
    const RESET_VALUE: u32 = 0;
}
