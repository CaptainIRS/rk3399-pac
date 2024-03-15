#[doc = "Register `PWM_PWM_FIFO_CTRL` reader"]
pub type R = crate::R<PwmPwmFifoCtrlSpec>;
#[doc = "Register `PWM_PWM_FIFO_CTRL` writer"]
pub type W = crate::W<PwmPwmFifoCtrlSpec>;
#[doc = "Field `FIFO_MODE_SEL` reader - FIFO MODE Sel When high, PWM FIFO mode is activated"]
pub type FifoModeSelR = crate::BitReader;
#[doc = "Field `FIFO_MODE_SEL` writer - FIFO MODE Sel When high, PWM FIFO mode is activated"]
pub type FifoModeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULL_INT_EN` reader - FIFO Full Interrupt Enable When high, an interrupt asserts when the channel 3 FIFO is full."]
pub type FullIntEnR = crate::BitReader;
#[doc = "Field `FULL_INT_EN` writer - FIFO Full Interrupt Enable When high, an interrupt asserts when the channel 3 FIFO is full."]
pub type FullIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFLOW_INT_EN` reader - FIFO Overflow Interrupt Enable When high, an interrupt asserts when the channel 3 FIFO is overflow."]
pub type OverflowIntEnR = crate::BitReader;
#[doc = "Field `OVERFLOW_INT_EN` writer - FIFO Overflow Interrupt Enable When high, an interrupt asserts when the channel 3 FIFO is overflow."]
pub type OverflowIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WATERMARK_INT_EN` reader - Watermark Full Interrupt"]
pub type WatermarkIntEnR = crate::BitReader;
#[doc = "Field `WATERMARK_INT_EN` writer - Watermark Full Interrupt"]
pub type WatermarkIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALMOST_FULL_WATERMARK` reader - Almost Full Watermark Level"]
pub type AlmostFullWatermarkR = crate::FieldReader;
#[doc = "Field `ALMOST_FULL_WATERMARK` writer - Almost Full Watermark Level"]
pub type AlmostFullWatermarkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "DMA Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaModeEn {
    #[doc = "1: disable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<DmaModeEn> for bool {
    #[inline(always)]
    fn from(variant: DmaModeEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_MODE_EN` reader - DMA Mode Enable"]
pub type DmaModeEnR = crate::BitReader<DmaModeEn>;
impl DmaModeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaModeEn {
        match self.bits {
            true => DmaModeEn::B1,
            false => DmaModeEn::B0,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DmaModeEn::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DmaModeEn::B0
    }
}
#[doc = "Field `DMA_MODE_EN` writer - DMA Mode Enable"]
pub type DmaModeEnW<'a, REG> = crate::BitWriter<'a, REG, DmaModeEn>;
impl<'a, REG> DmaModeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DmaModeEn::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DmaModeEn::B0)
    }
}
#[doc = "Field `TIMEOUT_EN` reader - FIFO Timeout Enable"]
pub type TimeoutEnR = crate::BitReader;
#[doc = "Field `TIMEOUT_EN` writer - FIFO Timeout Enable"]
pub type TimeoutEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FIFO MODE Sel When high, PWM FIFO mode is activated"]
    #[inline(always)]
    pub fn fifo_mode_sel(&self) -> FifoModeSelR {
        FifoModeSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Full Interrupt Enable When high, an interrupt asserts when the channel 3 FIFO is full."]
    #[inline(always)]
    pub fn full_int_en(&self) -> FullIntEnR {
        FullIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO Overflow Interrupt Enable When high, an interrupt asserts when the channel 3 FIFO is overflow."]
    #[inline(always)]
    pub fn overflow_int_en(&self) -> OverflowIntEnR {
        OverflowIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watermark Full Interrupt"]
    #[inline(always)]
    pub fn watermark_int_en(&self) -> WatermarkIntEnR {
        WatermarkIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Almost Full Watermark Level"]
    #[inline(always)]
    pub fn almost_full_watermark(&self) -> AlmostFullWatermarkR {
        AlmostFullWatermarkR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - DMA Mode Enable"]
    #[inline(always)]
    pub fn dma_mode_en(&self) -> DmaModeEnR {
        DmaModeEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FIFO Timeout Enable"]
    #[inline(always)]
    pub fn timeout_en(&self) -> TimeoutEnR {
        TimeoutEnR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO MODE Sel When high, PWM FIFO mode is activated"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_mode_sel(&mut self) -> FifoModeSelW<PwmPwmFifoCtrlSpec> {
        FifoModeSelW::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO Full Interrupt Enable When high, an interrupt asserts when the channel 3 FIFO is full."]
    #[inline(always)]
    #[must_use]
    pub fn full_int_en(&mut self) -> FullIntEnW<PwmPwmFifoCtrlSpec> {
        FullIntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - FIFO Overflow Interrupt Enable When high, an interrupt asserts when the channel 3 FIFO is overflow."]
    #[inline(always)]
    #[must_use]
    pub fn overflow_int_en(&mut self) -> OverflowIntEnW<PwmPwmFifoCtrlSpec> {
        OverflowIntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Watermark Full Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn watermark_int_en(&mut self) -> WatermarkIntEnW<PwmPwmFifoCtrlSpec> {
        WatermarkIntEnW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Almost Full Watermark Level"]
    #[inline(always)]
    #[must_use]
    pub fn almost_full_watermark(&mut self) -> AlmostFullWatermarkW<PwmPwmFifoCtrlSpec> {
        AlmostFullWatermarkW::new(self, 4)
    }
    #[doc = "Bit 8 - DMA Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_mode_en(&mut self) -> DmaModeEnW<PwmPwmFifoCtrlSpec> {
        DmaModeEnW::new(self, 8)
    }
    #[doc = "Bit 9 - FIFO Timeout Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_en(&mut self) -> TimeoutEnW<PwmPwmFifoCtrlSpec> {
        TimeoutEnW::new(self, 9)
    }
}
#[doc = "PWM Channel 3 FIFO Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm_fifo_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm_fifo_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmPwmFifoCtrlSpec;
impl crate::RegisterSpec for PwmPwmFifoCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_pwm_fifo_ctrl::R`](R) reader structure"]
impl crate::Readable for PwmPwmFifoCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm_pwm_fifo_ctrl::W`](W) writer structure"]
impl crate::Writable for PwmPwmFifoCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWM_PWM_FIFO_CTRL to value 0"]
impl crate::Resettable for PwmPwmFifoCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
