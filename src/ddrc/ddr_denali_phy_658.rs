#[doc = "Register `DDR_DENALI_PHY_658` reader"]
pub type R = crate::R<DdrDenaliPhy658Spec>;
#[doc = "Register `DDR_DENALI_PHY_658` writer"]
pub type W = crate::W<DdrDenaliPhy658Spec>;
#[doc = "Field `PHY_ADR_CALVL_DEBUG_MODE_1` reader - Enables CA training debug mode for address slice 1. Set to 1 to enable."]
pub type PhyAdrCalvlDebugMode1R = crate::BitReader;
#[doc = "Field `PHY_ADR_CALVL_DEBUG_MODE_1` writer - Enables CA training debug mode for address slice 1. Set to 1 to enable."]
pub type PhyAdrCalvlDebugMode1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_ADR_CALVL_DEBUG_CONT_1` writer - Allows the CA training state machine to advance (when in debug mode) for address slice 1. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyAdrCalvlDebugCont1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_ADR_CALVL_ERROR_CLR_1` writer - Clears the CA training state machine error status for address slice 1. Set to 1 to trigger. WRITE- ONLY"]
pub type ScPhyAdrCalvlErrorClr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_ADR_CALVL_OBS_SELECT_1` reader - CA bit lane to observe result from (OBS0 and OBS1) during CA training for address slice 1."]
pub type PhyAdrCalvlObsSelect1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_OBS_SELECT_1` writer - CA bit lane to observe result from (OBS0 and OBS1) during CA training for address slice 1."]
pub type PhyAdrCalvlObsSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Enables CA training debug mode for address slice 1. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_adr_calvl_debug_mode_1(&self) -> PhyAdrCalvlDebugMode1R {
        PhyAdrCalvlDebugMode1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 24:26 - CA bit lane to observe result from (OBS0 and OBS1) during CA training for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_obs_select_1(&self) -> PhyAdrCalvlObsSelect1R {
        PhyAdrCalvlObsSelect1R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables CA training debug mode for address slice 1. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_debug_mode_1(&mut self) -> PhyAdrCalvlDebugMode1W<DdrDenaliPhy658Spec> {
        PhyAdrCalvlDebugMode1W::new(self, 0)
    }
    #[doc = "Bit 8 - Allows the CA training state machine to advance (when in debug mode) for address slice 1. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_adr_calvl_debug_cont_1(
        &mut self,
    ) -> ScPhyAdrCalvlDebugCont1W<DdrDenaliPhy658Spec> {
        ScPhyAdrCalvlDebugCont1W::new(self, 8)
    }
    #[doc = "Bit 16 - Clears the CA training state machine error status for address slice 1. Set to 1 to trigger. WRITE- ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_adr_calvl_error_clr_1(&mut self) -> ScPhyAdrCalvlErrorClr1W<DdrDenaliPhy658Spec> {
        ScPhyAdrCalvlErrorClr1W::new(self, 16)
    }
    #[doc = "Bits 24:26 - CA bit lane to observe result from (OBS0 and OBS1) during CA training for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_obs_select_1(&mut self) -> PhyAdrCalvlObsSelect1W<DdrDenaliPhy658Spec> {
        PhyAdrCalvlObsSelect1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_658::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_658::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy658Spec;
impl crate::RegisterSpec for DdrDenaliPhy658Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_658::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy658Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_658::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy658Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_658 to value 0"]
impl crate::Resettable for DdrDenaliPhy658Spec {
    const RESET_VALUE: u32 = 0;
}
