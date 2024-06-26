#[doc = "Register `PWM_FIFO%s` reader"]
pub type R = crate::R<PwmFifoSpec>;
#[doc = "Register `PWM_FIFO%s` writer"]
pub type W = crate::W<PwmFifoSpec>;
#[doc = "Field `CYCLE_CNT` reader - High/Low Cycle Counter\n\nThis 31-bit counter indicates the effective cycles of high/low\n\nwaveform."]
pub type CycleCntR = crate::FieldReader<u32>;
#[doc = "Polarity\n\nThis bit indicates the polarity of the lower 31-bit counter.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pol {
    #[doc = "0: Low"]
    B0 = 0,
    #[doc = "1: High"]
    B1 = 1,
}
impl From<Pol> for bool {
    #[inline(always)]
    fn from(variant: Pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL` reader - Polarity\n\nThis bit indicates the polarity of the lower 31-bit counter."]
pub type PolR = crate::BitReader<Pol>;
impl PolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pol {
        match self.bits {
            false => Pol::B0,
            true => Pol::B1,
        }
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pol::B0
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pol::B1
    }
}
#[doc = "Field `POL` writer - Polarity\n\nThis bit indicates the polarity of the lower 31-bit counter."]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG, Pol>;
impl<'a, REG> PolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pol::B0)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pol::B1)
    }
}
impl R {
    #[doc = "Bits 0:30 - High/Low Cycle Counter\n\nThis 31-bit counter indicates the effective cycles of high/low\n\nwaveform."]
    #[inline(always)]
    pub fn cycle_cnt(&self) -> CycleCntR {
        CycleCntR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Polarity\n\nThis bit indicates the polarity of the lower 31-bit counter."]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Polarity\n\nThis bit indicates the polarity of the lower 31-bit counter."]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> PolW<PwmFifoSpec> {
        PolW::new(self, 31)
    }
}
#[doc = "FIFO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_fifo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_fifo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmFifoSpec;
impl crate::RegisterSpec for PwmFifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_fifo::R`](R) reader structure"]
impl crate::Readable for PwmFifoSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm_fifo::W`](W) writer structure"]
impl crate::Writable for PwmFifoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWM_FIFO%s to value 0"]
impl crate::Resettable for PwmFifoSpec {
    const RESET_VALUE: u32 = 0;
}
