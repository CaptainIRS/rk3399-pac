#[doc = "Register `DDR_DENALI_PHY_901` reader"]
pub type R = crate::R<DdrDenaliPhy901Spec>;
#[doc = "Register `DDR_DENALI_PHY_901` writer"]
pub type W = crate::W<DdrDenaliPhy901Spec>;
#[doc = "Field `PHY_CSLVL_COARSE_DLY` reader - Defines the CS training DLL coarse cycle delay value."]
pub type PhyCslvlCoarseDlyR = crate::FieldReader<u16>;
#[doc = "Field `PHY_CSLVL_COARSE_DLY` writer - Defines the CS training DLL coarse cycle delay value."]
pub type PhyCslvlCoarseDlyW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CSLVL_COARSE_CAPTURE_CNT` reader - Defines the number of samples to take at each GRP slave delay setting during coarse CS training."]
pub type PhyCslvlCoarseCaptureCntR = crate::FieldReader;
#[doc = "Field `PHY_CSLVL_COARSE_CAPTURE_CNT` writer - Defines the number of samples to take at each GRP slave delay setting during coarse CS training."]
pub type PhyCslvlCoarseCaptureCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_CSLVL_DEBUG_MODE` reader - Enables CS training debug mode. Set to 1 to enable."]
pub type PhyCslvlDebugModeR = crate::BitReader;
#[doc = "Field `PHY_CSLVL_DEBUG_MODE` writer - Enables CS training debug mode. Set to 1 to enable."]
pub type PhyCslvlDebugModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Defines the CS training DLL coarse cycle delay value."]
    #[inline(always)]
    pub fn phy_cslvl_coarse_dly(&self) -> PhyCslvlCoarseDlyR {
        PhyCslvlCoarseDlyR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:19 - Defines the number of samples to take at each GRP slave delay setting during coarse CS training."]
    #[inline(always)]
    pub fn phy_cslvl_coarse_capture_cnt(&self) -> PhyCslvlCoarseCaptureCntR {
        PhyCslvlCoarseCaptureCntR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Enables CS training debug mode. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_cslvl_debug_mode(&self) -> PhyCslvlDebugModeR {
        PhyCslvlDebugModeR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Defines the CS training DLL coarse cycle delay value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cslvl_coarse_dly(&mut self) -> PhyCslvlCoarseDlyW<DdrDenaliPhy901Spec> {
        PhyCslvlCoarseDlyW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Defines the number of samples to take at each GRP slave delay setting during coarse CS training."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cslvl_coarse_capture_cnt(
        &mut self,
    ) -> PhyCslvlCoarseCaptureCntW<DdrDenaliPhy901Spec> {
        PhyCslvlCoarseCaptureCntW::new(self, 16)
    }
    #[doc = "Bit 24 - Enables CS training debug mode. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cslvl_debug_mode(&mut self) -> PhyCslvlDebugModeW<DdrDenaliPhy901Spec> {
        PhyCslvlDebugModeW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_901::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_901::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy901Spec;
impl crate::RegisterSpec for DdrDenaliPhy901Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_901::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy901Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_901::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy901Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_901 to value 0"]
impl crate::Resettable for DdrDenaliPhy901Spec {
    const RESET_VALUE: u32 = 0;
}
