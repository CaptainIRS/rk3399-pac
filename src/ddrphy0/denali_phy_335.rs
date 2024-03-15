#[doc = "Register `DENALI_PHY_335` reader"]
pub type R = crate::R<DenaliPhy335Spec>;
#[doc = "Register `DENALI_PHY_335` writer"]
pub type W = crate::W<DenaliPhy335Spec>;
#[doc = "Field `PHY_WRLVL_DELAY_PERIOD_THRESHOLD_2` reader - Write level delay threshold below which will add a cycle of write path latency for slice 2."]
pub type PhyWrlvlDelayPeriodThreshold2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WRLVL_DELAY_PERIOD_THRESHOLD_2` writer - Write level delay threshold below which will add a cycle of write path latency for slice 2."]
pub type PhyWrlvlDelayPeriodThreshold2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_WRLVL_EARLY_FORCE_ZERO_2` reader - Force the final write level delay value (that meets the early threshold) to 0 for slice 2."]
pub type PhyWrlvlEarlyForceZero2R = crate::BitReader;
#[doc = "Field `PHY_WRLVL_EARLY_FORCE_ZERO_2` writer - Force the final write level delay value (that meets the early threshold) to 0 for slice 2."]
pub type PhyWrlvlEarlyForceZero2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Write level delay threshold below which will add a cycle of write path latency for slice 2."]
    #[inline(always)]
    pub fn phy_wrlvl_delay_period_threshold_2(&self) -> PhyWrlvlDelayPeriodThreshold2R {
        PhyWrlvlDelayPeriodThreshold2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - Force the final write level delay value (that meets the early threshold) to 0 for slice 2."]
    #[inline(always)]
    pub fn phy_wrlvl_early_force_zero_2(&self) -> PhyWrlvlEarlyForceZero2R {
        PhyWrlvlEarlyForceZero2R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Write level delay threshold below which will add a cycle of write path latency for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_delay_period_threshold_2(
        &mut self,
    ) -> PhyWrlvlDelayPeriodThreshold2W<DenaliPhy335Spec> {
        PhyWrlvlDelayPeriodThreshold2W::new(self, 0)
    }
    #[doc = "Bit 16 - Force the final write level delay value (that meets the early threshold) to 0 for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_early_force_zero_2(&mut self) -> PhyWrlvlEarlyForceZero2W<DenaliPhy335Spec> {
        PhyWrlvlEarlyForceZero2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_335::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_335::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy335Spec;
impl crate::RegisterSpec for DenaliPhy335Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_335::R`](R) reader structure"]
impl crate::Readable for DenaliPhy335Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_335::W`](W) writer structure"]
impl crate::Writable for DenaliPhy335Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_335 to value 0"]
impl crate::Resettable for DenaliPhy335Spec {
    const RESET_VALUE: u32 = 0;
}
