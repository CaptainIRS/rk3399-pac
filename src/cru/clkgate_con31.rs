#[doc = "Register `CLKGATE_CON31` reader"]
pub type R = crate::R<ClkgateCon31Spec>;
#[doc = "Register `CLKGATE_CON31` writer"]
pub type W = crate::W<ClkgateCon31Spec>;
#[doc = "Field `PCLK_GRF_EN` reader - pclk_grf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type PclkGrfEnR = crate::BitReader;
#[doc = "Field `PCLK_GRF_EN` writer - pclk_grf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type PclkGrfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_INTR_ARB_EN` reader - pclk_intr_arb clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkIntrArbEnR = crate::BitReader;
#[doc = "Field `PCLK_INTR_ARB_EN` writer - pclk_intr_arb clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkIntrArbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_GPIO2_EN` reader - pclk_gpio2 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkGpio2EnR = crate::BitReader;
#[doc = "Field `PCLK_GPIO2_EN` writer - pclk_gpio2 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkGpio2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_GPIO3_EN` reader - pclk_gpio3 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkGpio3EnR = crate::BitReader;
#[doc = "Field `PCLK_GPIO3_EN` writer - pclk_gpio3 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkGpio3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_GPIO4_EN` reader - pclk_gpio4 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkGpio4EnR = crate::BitReader;
#[doc = "Field `PCLK_GPIO4_EN` writer - pclk_gpio4 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkGpio4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_TIMER0_EN` reader - pclk_timer0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkTimer0EnR = crate::BitReader;
#[doc = "Field `PCLK_TIMER0_EN` writer - pclk_timer0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkTimer0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_TIMER1_EN` reader - pclk_timer1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkTimer1EnR = crate::BitReader;
#[doc = "Field `PCLK_TIMER1_EN` writer - pclk_timer1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkTimer1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_HSICPHY_EN` reader - pclk_hsicphy clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkHsicphyEnR = crate::BitReader;
#[doc = "Field `PCLK_HSICPHY_EN` writer - pclk_hsicphy clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkHsicphyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_PMU_INTR_ARB_EN` reader - pclk_pmu_intr_arb clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkPmuIntrArbEnR = crate::BitReader;
#[doc = "Field `PCLK_PMU_INTR_ARB_EN` writer - pclk_pmu_intr_arb clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkPmuIntrArbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_SGRF_EN` reader - pclk_sgrf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type PclkSgrfEnR = crate::BitReader;
#[doc = "Field `PCLK_SGRF_EN` writer - pclk_sgrf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type PclkSgrfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 1 - pclk_grf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn pclk_grf_en(&self) -> PclkGrfEnR {
        PclkGrfEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pclk_intr_arb clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_intr_arb_en(&self) -> PclkIntrArbEnR {
        PclkIntrArbEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - pclk_gpio2 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_gpio2_en(&self) -> PclkGpio2EnR {
        PclkGpio2EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - pclk_gpio3 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_gpio3_en(&self) -> PclkGpio3EnR {
        PclkGpio3EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - pclk_gpio4 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_gpio4_en(&self) -> PclkGpio4EnR {
        PclkGpio4EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pclk_timer0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_timer0_en(&self) -> PclkTimer0EnR {
        PclkTimer0EnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - pclk_timer1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_timer1_en(&self) -> PclkTimer1EnR {
        PclkTimer1EnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - pclk_hsicphy clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_hsicphy_en(&self) -> PclkHsicphyEnR {
        PclkHsicphyEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - pclk_pmu_intr_arb clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_pmu_intr_arb_en(&self) -> PclkPmuIntrArbEnR {
        PclkPmuIntrArbEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - pclk_sgrf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn pclk_sgrf_en(&self) -> PclkSgrfEnR {
        PclkSgrfEnR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - pclk_grf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_grf_en(&mut self) -> PclkGrfEnW<ClkgateCon31Spec> {
        PclkGrfEnW::new(self, 1)
    }
    #[doc = "Bit 2 - pclk_intr_arb clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_intr_arb_en(&mut self) -> PclkIntrArbEnW<ClkgateCon31Spec> {
        PclkIntrArbEnW::new(self, 2)
    }
    #[doc = "Bit 3 - pclk_gpio2 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_gpio2_en(&mut self) -> PclkGpio2EnW<ClkgateCon31Spec> {
        PclkGpio2EnW::new(self, 3)
    }
    #[doc = "Bit 4 - pclk_gpio3 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_gpio3_en(&mut self) -> PclkGpio3EnW<ClkgateCon31Spec> {
        PclkGpio3EnW::new(self, 4)
    }
    #[doc = "Bit 5 - pclk_gpio4 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_gpio4_en(&mut self) -> PclkGpio4EnW<ClkgateCon31Spec> {
        PclkGpio4EnW::new(self, 5)
    }
    #[doc = "Bit 6 - pclk_timer0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_timer0_en(&mut self) -> PclkTimer0EnW<ClkgateCon31Spec> {
        PclkTimer0EnW::new(self, 6)
    }
    #[doc = "Bit 7 - pclk_timer1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_timer1_en(&mut self) -> PclkTimer1EnW<ClkgateCon31Spec> {
        PclkTimer1EnW::new(self, 7)
    }
    #[doc = "Bit 8 - pclk_hsicphy clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_hsicphy_en(&mut self) -> PclkHsicphyEnW<ClkgateCon31Spec> {
        PclkHsicphyEnW::new(self, 8)
    }
    #[doc = "Bit 9 - pclk_pmu_intr_arb clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_pmu_intr_arb_en(&mut self) -> PclkPmuIntrArbEnW<ClkgateCon31Spec> {
        PclkPmuIntrArbEnW::new(self, 9)
    }
    #[doc = "Bit 10 - pclk_sgrf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_sgrf_en(&mut self) -> PclkSgrfEnW<ClkgateCon31Spec> {
        PclkSgrfEnW::new(self, 10)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkgateCon31Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con31::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con31::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkgateCon31Spec;
impl crate::RegisterSpec for ClkgateCon31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkgate_con31::R`](R) reader structure"]
impl crate::Readable for ClkgateCon31Spec {}
#[doc = "`write(|w| ..)` method takes [`clkgate_con31::W`](W) writer structure"]
impl crate::Writable for ClkgateCon31Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKGATE_CON31 to value 0"]
impl crate::Resettable for ClkgateCon31Spec {
    const RESET_VALUE: u32 = 0;
}
