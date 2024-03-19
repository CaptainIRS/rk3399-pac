#[doc = "Register `DDR_DENALI_CTL_99` reader"]
pub type R = crate::R<DdrDenaliCtl99Spec>;
#[doc = "Register `DDR_DENALI_CTL_99` writer"]
pub type W = crate::W<DdrDenaliCtl99Spec>;
#[doc = "Field `LPI_TIMER_WAKEUP_F2` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when the LPI timer expires for frequency copy 2."]
pub type LpiTimerWakeupF2R = crate::FieldReader;
#[doc = "Field `LPI_TIMER_WAKEUP_F2` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when the LPI timer expires for frequency copy 2."]
pub type LpiTimerWakeupF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_WAKEUP_EN` reader - Enables the various low power wakeup parameters. Bit (0) enables power-down wakeup, bit (1) enables self-refresh wakeup, bit (2) enables self-refresh with memory and controller clock gating wakeup, bit (3) is reserved and bit (4) enables the LPI timer. Set each bit to 1 to enable."]
pub type LpiWakeupEnR = crate::FieldReader;
#[doc = "Field `LPI_WAKEUP_EN` writer - Enables the various low power wakeup parameters. Bit (0) enables power-down wakeup, bit (1) enables self-refresh wakeup, bit (2) enables self-refresh with memory and controller clock gating wakeup, bit (3) is reserved and bit (4) enables the LPI timer. Set each bit to 1 to enable."]
pub type LpiWakeupEnW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LPI_TIMER_COUNT` reader - Defines the LPI timer count."]
pub type LpiTimerCountR = crate::FieldReader<u16>;
#[doc = "Field `LPI_TIMER_COUNT` writer - Defines the LPI timer count."]
pub type LpiTimerCountW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when the LPI timer expires for frequency copy 2."]
    #[inline(always)]
    pub fn lpi_timer_wakeup_f2(&self) -> LpiTimerWakeupF2R {
        LpiTimerWakeupF2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Enables the various low power wakeup parameters. Bit (0) enables power-down wakeup, bit (1) enables self-refresh wakeup, bit (2) enables self-refresh with memory and controller clock gating wakeup, bit (3) is reserved and bit (4) enables the LPI timer. Set each bit to 1 to enable."]
    #[inline(always)]
    pub fn lpi_wakeup_en(&self) -> LpiWakeupEnR {
        LpiWakeupEnR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - Defines the LPI timer count."]
    #[inline(always)]
    pub fn lpi_timer_count(&self) -> LpiTimerCountR {
        LpiTimerCountR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when the LPI timer expires for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_timer_wakeup_f2(&mut self) -> LpiTimerWakeupF2W<DdrDenaliCtl99Spec> {
        LpiTimerWakeupF2W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Enables the various low power wakeup parameters. Bit (0) enables power-down wakeup, bit (1) enables self-refresh wakeup, bit (2) enables self-refresh with memory and controller clock gating wakeup, bit (3) is reserved and bit (4) enables the LPI timer. Set each bit to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_wakeup_en(&mut self) -> LpiWakeupEnW<DdrDenaliCtl99Spec> {
        LpiWakeupEnW::new(self, 8)
    }
    #[doc = "Bits 16:27 - Defines the LPI timer count."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_timer_count(&mut self) -> LpiTimerCountW<DdrDenaliCtl99Spec> {
        LpiTimerCountW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_99::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_99::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl99Spec;
impl crate::RegisterSpec for DdrDenaliCtl99Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_99::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl99Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_99::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl99Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_99 to value 0"]
impl crate::Resettable for DdrDenaliCtl99Spec {
    const RESET_VALUE: u32 = 0;
}
