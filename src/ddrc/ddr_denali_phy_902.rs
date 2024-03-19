#[doc = "Register `DDR_DENALI_PHY_902` writer"]
pub type W = crate::W<DdrDenaliPhy902Spec>;
#[doc = "Field `SC_PHY_CSLVL_DEBUG_CONT` writer - Allows the CS training state machine to advance (when in debug mode). Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyCslvlDebugContW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_CSLVL_ERROR_CLR` writer - Clears the CS training state machine error status. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyCslvlErrorClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Allows the CS training state machine to advance (when in debug mode). Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_cslvl_debug_cont(&mut self) -> ScPhyCslvlDebugContW<DdrDenaliPhy902Spec> {
        ScPhyCslvlDebugContW::new(self, 0)
    }
    #[doc = "Bit 8 - Clears the CS training state machine error status. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_cslvl_error_clr(&mut self) -> ScPhyCslvlErrorClrW<DdrDenaliPhy902Spec> {
        ScPhyCslvlErrorClrW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_902::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy902Spec;
impl crate::RegisterSpec for DdrDenaliPhy902Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_902::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy902Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_902 to value 0"]
impl crate::Resettable for DdrDenaliPhy902Spec {
    const RESET_VALUE: u32 = 0;
}
