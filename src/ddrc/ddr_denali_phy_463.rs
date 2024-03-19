#[doc = "Register `DDR_DENALI_PHY_463` reader"]
pub type R = crate::R<DdrDenaliPhy463Spec>;
#[doc = "Register `DDR_DENALI_PHY_463` writer"]
pub type W = crate::W<DdrDenaliPhy463Spec>;
#[doc = "Field `PHY_WRLVL_DELAY_PERIOD_THRESHOLD_3` reader - Write level delay threshold below which will add a cycle of write path latency for slice 3."]
pub type PhyWrlvlDelayPeriodThreshold3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WRLVL_DELAY_PERIOD_THRESHOLD_3` writer - Write level delay threshold below which will add a cycle of write path latency for slice 3."]
pub type PhyWrlvlDelayPeriodThreshold3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_WRLVL_EARLY_FORCE_ZERO_3` reader - Force the final write level delay value (that meets the early threshold) to 0 for slice 3."]
pub type PhyWrlvlEarlyForceZero3R = crate::BitReader;
#[doc = "Field `PHY_WRLVL_EARLY_FORCE_ZERO_3` writer - Force the final write level delay value (that meets the early threshold) to 0 for slice 3."]
pub type PhyWrlvlEarlyForceZero3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Write level delay threshold below which will add a cycle of write path latency for slice 3."]
    #[inline(always)]
    pub fn phy_wrlvl_delay_period_threshold_3(&self) -> PhyWrlvlDelayPeriodThreshold3R {
        PhyWrlvlDelayPeriodThreshold3R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - Force the final write level delay value (that meets the early threshold) to 0 for slice 3."]
    #[inline(always)]
    pub fn phy_wrlvl_early_force_zero_3(&self) -> PhyWrlvlEarlyForceZero3R {
        PhyWrlvlEarlyForceZero3R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Write level delay threshold below which will add a cycle of write path latency for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_delay_period_threshold_3(
        &mut self,
    ) -> PhyWrlvlDelayPeriodThreshold3W<DdrDenaliPhy463Spec> {
        PhyWrlvlDelayPeriodThreshold3W::new(self, 0)
    }
    #[doc = "Bit 16 - Force the final write level delay value (that meets the early threshold) to 0 for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_early_force_zero_3(
        &mut self,
    ) -> PhyWrlvlEarlyForceZero3W<DdrDenaliPhy463Spec> {
        PhyWrlvlEarlyForceZero3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_463::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_463::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy463Spec;
impl crate::RegisterSpec for DdrDenaliPhy463Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_463::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy463Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_463::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy463Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_463 to value 0"]
impl crate::Resettable for DdrDenaliPhy463Spec {
    const RESET_VALUE: u32 = 0;
}
