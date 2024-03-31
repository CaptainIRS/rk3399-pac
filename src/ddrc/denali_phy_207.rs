#[doc = "Register `DENALI_PHY_207` reader"]
pub type R = crate::R<DenaliPhy207Spec>;
#[doc = "Register `DENALI_PHY_207` writer"]
pub type W = crate::W<DenaliPhy207Spec>;
#[doc = "Field `PHY_WRLVL_DELAY_PERIOD_THRESHOLD_1` reader - Write level delay threshold below which will add a cycle of write path latency for slice 1."]
pub type PhyWrlvlDelayPeriodThreshold1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WRLVL_DELAY_PERIOD_THRESHOLD_1` writer - Write level delay threshold below which will add a cycle of write path latency for slice 1."]
pub type PhyWrlvlDelayPeriodThreshold1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_WRLVL_EARLY_FORCE_ZERO_1` reader - Force the final write level delay value (that meets the early threshold) to 0 for slice 1."]
pub type PhyWrlvlEarlyForceZero1R = crate::BitReader;
#[doc = "Field `PHY_WRLVL_EARLY_FORCE_ZERO_1` writer - Force the final write level delay value (that meets the early threshold) to 0 for slice 1."]
pub type PhyWrlvlEarlyForceZero1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Write level delay threshold below which will add a cycle of write path latency for slice 1."]
    #[inline(always)]
    pub fn phy_wrlvl_delay_period_threshold_1(&self) -> PhyWrlvlDelayPeriodThreshold1R {
        PhyWrlvlDelayPeriodThreshold1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - Force the final write level delay value (that meets the early threshold) to 0 for slice 1."]
    #[inline(always)]
    pub fn phy_wrlvl_early_force_zero_1(&self) -> PhyWrlvlEarlyForceZero1R {
        PhyWrlvlEarlyForceZero1R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Write level delay threshold below which will add a cycle of write path latency for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_delay_period_threshold_1(
        &mut self,
    ) -> PhyWrlvlDelayPeriodThreshold1W<DenaliPhy207Spec> {
        PhyWrlvlDelayPeriodThreshold1W::new(self, 0)
    }
    #[doc = "Bit 16 - Force the final write level delay value (that meets the early threshold) to 0 for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_early_force_zero_1(&mut self) -> PhyWrlvlEarlyForceZero1W<DenaliPhy207Spec> {
        PhyWrlvlEarlyForceZero1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_207::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_207::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy207Spec;
impl crate::RegisterSpec for DenaliPhy207Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_207::R`](R) reader structure"]
impl crate::Readable for DenaliPhy207Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_207::W`](W) writer structure"]
impl crate::Writable for DenaliPhy207Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_207 to value 0"]
impl crate::Resettable for DenaliPhy207Spec {
    const RESET_VALUE: u32 = 0;
}
