#[doc = "Register `DDR_DENALI_PHY_79` reader"]
pub type R = crate::R<DdrDenaliPhy79Spec>;
#[doc = "Register `DDR_DENALI_PHY_79` writer"]
pub type W = crate::W<DdrDenaliPhy79Spec>;
#[doc = "Field `PHY_WRLVL_DELAY_PERIOD_THRESHOLD_0` reader - Write level delay threshold below which will add a cycle of write path latency for slice 0."]
pub type PhyWrlvlDelayPeriodThreshold0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WRLVL_DELAY_PERIOD_THRESHOLD_0` writer - Write level delay threshold below which will add a cycle of write path latency for slice 0."]
pub type PhyWrlvlDelayPeriodThreshold0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_WRLVL_EARLY_FORCE_ZERO_0` reader - Force the final write level delay value (that meets the early threshold) to 0 for slice 0."]
pub type PhyWrlvlEarlyForceZero0R = crate::BitReader;
#[doc = "Field `PHY_WRLVL_EARLY_FORCE_ZERO_0` writer - Force the final write level delay value (that meets the early threshold) to 0 for slice 0."]
pub type PhyWrlvlEarlyForceZero0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Write level delay threshold below which will add a cycle of write path latency for slice 0."]
    #[inline(always)]
    pub fn phy_wrlvl_delay_period_threshold_0(&self) -> PhyWrlvlDelayPeriodThreshold0R {
        PhyWrlvlDelayPeriodThreshold0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - Force the final write level delay value (that meets the early threshold) to 0 for slice 0."]
    #[inline(always)]
    pub fn phy_wrlvl_early_force_zero_0(&self) -> PhyWrlvlEarlyForceZero0R {
        PhyWrlvlEarlyForceZero0R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Write level delay threshold below which will add a cycle of write path latency for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_delay_period_threshold_0(
        &mut self,
    ) -> PhyWrlvlDelayPeriodThreshold0W<DdrDenaliPhy79Spec> {
        PhyWrlvlDelayPeriodThreshold0W::new(self, 0)
    }
    #[doc = "Bit 16 - Force the final write level delay value (that meets the early threshold) to 0 for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_early_force_zero_0(&mut self) -> PhyWrlvlEarlyForceZero0W<DdrDenaliPhy79Spec> {
        PhyWrlvlEarlyForceZero0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_79::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_79::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy79Spec;
impl crate::RegisterSpec for DdrDenaliPhy79Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_79::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy79Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_79::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy79Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_79 to value 0"]
impl crate::Resettable for DdrDenaliPhy79Spec {
    const RESET_VALUE: u32 = 0;
}
