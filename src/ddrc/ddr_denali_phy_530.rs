#[doc = "Register `DDR_DENALI_PHY_530` reader"]
pub type R = crate::R<DdrDenaliPhy530Spec>;
#[doc = "Register `DDR_DENALI_PHY_530` writer"]
pub type W = crate::W<DdrDenaliPhy530Spec>;
#[doc = "Field `PHY_ADR_CALVL_DEBUG_MODE_0` reader - Enables CA training debug mode for address slice 0. Set to 1 to enable."]
pub type PhyAdrCalvlDebugMode0R = crate::BitReader;
#[doc = "Field `PHY_ADR_CALVL_DEBUG_MODE_0` writer - Enables CA training debug mode for address slice 0. Set to 1 to enable."]
pub type PhyAdrCalvlDebugMode0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_ADR_CALVL_DEBUG_CONT_0` writer - Allows the CA training state machine to advance (when in debug mode) for address slice 0. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyAdrCalvlDebugCont0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_ADR_CALVL_ERROR_CLR_0` writer - Clears the CA training state machine error status for address slice 0. Set to 1 to trigger. WRITE- ONLY"]
pub type ScPhyAdrCalvlErrorClr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_ADR_CALVL_OBS_SELECT_0` reader - CA bit lane to observe result from (OBS0 and OBS1) during CA training for address slice 0."]
pub type PhyAdrCalvlObsSelect0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_OBS_SELECT_0` writer - CA bit lane to observe result from (OBS0 and OBS1) during CA training for address slice 0."]
pub type PhyAdrCalvlObsSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Enables CA training debug mode for address slice 0. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_adr_calvl_debug_mode_0(&self) -> PhyAdrCalvlDebugMode0R {
        PhyAdrCalvlDebugMode0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 24:26 - CA bit lane to observe result from (OBS0 and OBS1) during CA training for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_obs_select_0(&self) -> PhyAdrCalvlObsSelect0R {
        PhyAdrCalvlObsSelect0R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables CA training debug mode for address slice 0. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_debug_mode_0(&mut self) -> PhyAdrCalvlDebugMode0W<DdrDenaliPhy530Spec> {
        PhyAdrCalvlDebugMode0W::new(self, 0)
    }
    #[doc = "Bit 8 - Allows the CA training state machine to advance (when in debug mode) for address slice 0. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_adr_calvl_debug_cont_0(
        &mut self,
    ) -> ScPhyAdrCalvlDebugCont0W<DdrDenaliPhy530Spec> {
        ScPhyAdrCalvlDebugCont0W::new(self, 8)
    }
    #[doc = "Bit 16 - Clears the CA training state machine error status for address slice 0. Set to 1 to trigger. WRITE- ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_adr_calvl_error_clr_0(&mut self) -> ScPhyAdrCalvlErrorClr0W<DdrDenaliPhy530Spec> {
        ScPhyAdrCalvlErrorClr0W::new(self, 16)
    }
    #[doc = "Bits 24:26 - CA bit lane to observe result from (OBS0 and OBS1) during CA training for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_obs_select_0(&mut self) -> PhyAdrCalvlObsSelect0W<DdrDenaliPhy530Spec> {
        PhyAdrCalvlObsSelect0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_530::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_530::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy530Spec;
impl crate::RegisterSpec for DdrDenaliPhy530Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_530::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy530Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_530::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy530Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_530 to value 0"]
impl crate::Resettable for DdrDenaliPhy530Spec {
    const RESET_VALUE: u32 = 0;
}
