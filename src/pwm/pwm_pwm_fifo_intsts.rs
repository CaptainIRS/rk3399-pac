#[doc = "Register `PWM_PWM_FIFO_INTSTS` reader"]
pub type R = crate::R<PwmPwmFifoIntstsSpec>;
#[doc = "Register `PWM_PWM_FIFO_INTSTS` writer"]
pub type W = crate::W<PwmPwmFifoIntstsSpec>;
#[doc = "Field `FIFO_FULL_INTSTS` reader - FIFO Full Interrupt Status\n\nThis bit indicates the FIFO is full"]
pub type FifoFullIntstsR = crate::BitReader;
#[doc = "Field `FIFO_FULL_INTSTS` writer - FIFO Full Interrupt Status\n\nThis bit indicates the FIFO is full"]
pub type FifoFullIntstsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FIFO_OVERFLOW_INTSTS` reader - FIFO Overflow Interrupt Status\n\nThis bit indicates the FIFO is overflow"]
pub type FifoOverflowIntstsR = crate::BitReader;
#[doc = "Field `FIFO_OVERFLOW_INTSTS` writer - FIFO Overflow Interrupt Status\n\nThis bit indicates the FIFO is overflow"]
pub type FifoOverflowIntstsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FIFO_WATERMARK_FULL_INTSTS` reader - FIFO Watermark Full Interrupt Status\n\nThis bit indicates the FIFO is Watermark Full"]
pub type FifoWatermarkFullIntstsR = crate::BitReader;
#[doc = "Field `FIFO_WATERMARK_FULL_INTSTS` writer - FIFO Watermark Full Interrupt Status\n\nThis bit indicates the FIFO is Watermark Full"]
pub type FifoWatermarkFullIntstsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TIMEOUT_INTSTS` reader - Timeout Interrupt"]
pub type TimeoutIntstsR = crate::BitReader;
#[doc = "Field `TIMEOUT_INTSTS` writer - Timeout Interrupt"]
pub type TimeoutIntstsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FIFO_EMPTY_STATUS` reader - FIFO Empty Status\n\nThis bit indicates the FIFO is empty"]
pub type FifoEmptyStatusR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - FIFO Full Interrupt Status\n\nThis bit indicates the FIFO is full"]
    #[inline(always)]
    pub fn fifo_full_intsts(&self) -> FifoFullIntstsR {
        FifoFullIntstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Overflow Interrupt Status\n\nThis bit indicates the FIFO is overflow"]
    #[inline(always)]
    pub fn fifo_overflow_intsts(&self) -> FifoOverflowIntstsR {
        FifoOverflowIntstsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO Watermark Full Interrupt Status\n\nThis bit indicates the FIFO is Watermark Full"]
    #[inline(always)]
    pub fn fifo_watermark_full_intsts(&self) -> FifoWatermarkFullIntstsR {
        FifoWatermarkFullIntstsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout Interrupt"]
    #[inline(always)]
    pub fn timeout_intsts(&self) -> TimeoutIntstsR {
        TimeoutIntstsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO Empty Status\n\nThis bit indicates the FIFO is empty"]
    #[inline(always)]
    pub fn fifo_empty_status(&self) -> FifoEmptyStatusR {
        FifoEmptyStatusR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO Full Interrupt Status\n\nThis bit indicates the FIFO is full"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_full_intsts(&mut self) -> FifoFullIntstsW<PwmPwmFifoIntstsSpec> {
        FifoFullIntstsW::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO Overflow Interrupt Status\n\nThis bit indicates the FIFO is overflow"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_overflow_intsts(&mut self) -> FifoOverflowIntstsW<PwmPwmFifoIntstsSpec> {
        FifoOverflowIntstsW::new(self, 1)
    }
    #[doc = "Bit 2 - FIFO Watermark Full Interrupt Status\n\nThis bit indicates the FIFO is Watermark Full"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_watermark_full_intsts(&mut self) -> FifoWatermarkFullIntstsW<PwmPwmFifoIntstsSpec> {
        FifoWatermarkFullIntstsW::new(self, 2)
    }
    #[doc = "Bit 3 - Timeout Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_intsts(&mut self) -> TimeoutIntstsW<PwmPwmFifoIntstsSpec> {
        TimeoutIntstsW::new(self, 3)
    }
}
#[doc = "FIFO Interrupts Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm_fifo_intsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm_fifo_intsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmPwmFifoIntstsSpec;
impl crate::RegisterSpec for PwmPwmFifoIntstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_pwm_fifo_intsts::R`](R) reader structure"]
impl crate::Readable for PwmPwmFifoIntstsSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm_pwm_fifo_intsts::W`](W) writer structure"]
impl crate::Writable for PwmPwmFifoIntstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets PWM_PWM_FIFO_INTSTS to value 0"]
impl crate::Resettable for PwmPwmFifoIntstsSpec {
    const RESET_VALUE: u32 = 0;
}
