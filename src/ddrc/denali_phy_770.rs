#[doc = "Register `DENALI_PHY_770` reader"]
pub type R = crate::R<DenaliPhy770Spec>;
#[doc = "Register `DENALI_PHY_770` writer"]
pub type W = crate::W<DenaliPhy770Spec>;
#[doc = "Field `PHY_ADR_CLK_BYPASS_OVERRIDE_2` reader - Bypass mode override setting for address slice 2. Set to 1 to enable."]
pub type PhyAdrClkBypassOverride2R = crate::BitReader;
#[doc = "Field `PHY_ADR_CLK_BYPASS_OVERRIDE_2` writer - Bypass mode override setting for address slice 2. Set to 1 to enable."]
pub type PhyAdrClkBypassOverride2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_ADR_MANUAL_CLEAR_2` writer - Manual reset/clear of internal logic for address slice 2. Bit (0) is reset of master delay min/max lock values. Bit (1) is manual reset of master delay unlock counter. Bit (2) clears the loopback error/results registers. Set each bit to 1 to reset."]
pub type ScPhyAdrManualClear2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Bypass mode override setting for address slice 2. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_adr_clk_bypass_override_2(&self) -> PhyAdrClkBypassOverride2R {
        PhyAdrClkBypassOverride2R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass mode override setting for address slice 2. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_clk_bypass_override_2(&mut self) -> PhyAdrClkBypassOverride2W<DenaliPhy770Spec> {
        PhyAdrClkBypassOverride2W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Manual reset/clear of internal logic for address slice 2. Bit (0) is reset of master delay min/max lock values. Bit (1) is manual reset of master delay unlock counter. Bit (2) clears the loopback error/results registers. Set each bit to 1 to reset."]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_adr_manual_clear_2(&mut self) -> ScPhyAdrManualClear2W<DenaliPhy770Spec> {
        ScPhyAdrManualClear2W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_770::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_770::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy770Spec;
impl crate::RegisterSpec for DenaliPhy770Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_770::R`](R) reader structure"]
impl crate::Readable for DenaliPhy770Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_770::W`](W) writer structure"]
impl crate::Writable for DenaliPhy770Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_770 to value 0"]
impl crate::Resettable for DenaliPhy770Spec {
    const RESET_VALUE: u32 = 0;
}
