#[doc = "Register `PWM_INTSTS` reader"]
pub type R = crate::R<PwmIntstsSpec>;
#[doc = "Register `PWM_INTSTS` writer"]
pub type W = crate::W<PwmIntstsSpec>;
#[doc = "Channel 0 Raw Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0Intsts {
    #[doc = "0: Channel 0 Interrupt generated"]
    B0 = 0,
    #[doc = "1: Channel 0 Interrupt generated"]
    B1 = 1,
}
impl From<Ch0Intsts> for bool {
    #[inline(always)]
    fn from(variant: Ch0Intsts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0_INTSTS` reader - Channel 0 Raw Interrupt Status"]
pub type Ch0IntstsR = crate::BitReader<Ch0Intsts>;
impl Ch0IntstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0Intsts {
        match self.bits {
            false => Ch0Intsts::B0,
            true => Ch0Intsts::B1,
        }
    }
    #[doc = "Channel 0 Interrupt generated"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ch0Intsts::B0
    }
    #[doc = "Channel 0 Interrupt generated"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ch0Intsts::B1
    }
}
#[doc = "Field `CH0_INTSTS` writer - Channel 0 Raw Interrupt Status"]
pub type Ch0IntstsW<'a, REG> = crate::BitWriter<'a, REG, Ch0Intsts>;
impl<'a, REG> Ch0IntstsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 0 Interrupt generated"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0Intsts::B0)
    }
    #[doc = "Channel 0 Interrupt generated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0Intsts::B1)
    }
}
#[doc = "Channel 1 Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1Intsts {
    #[doc = "0: Channel 1 Interrupt generated"]
    B0 = 0,
    #[doc = "1: Channel 1 Interrupt generated"]
    B1 = 1,
}
impl From<Ch1Intsts> for bool {
    #[inline(always)]
    fn from(variant: Ch1Intsts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1_INTSTS` reader - Channel 1 Interrupt Status"]
pub type Ch1IntstsR = crate::BitReader<Ch1Intsts>;
impl Ch1IntstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1Intsts {
        match self.bits {
            false => Ch1Intsts::B0,
            true => Ch1Intsts::B1,
        }
    }
    #[doc = "Channel 1 Interrupt generated"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ch1Intsts::B0
    }
    #[doc = "Channel 1 Interrupt generated"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ch1Intsts::B1
    }
}
#[doc = "Field `CH1_INTSTS` writer - Channel 1 Interrupt Status"]
pub type Ch1IntstsW<'a, REG> = crate::BitWriter<'a, REG, Ch1Intsts>;
impl<'a, REG> Ch1IntstsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 1 Interrupt generated"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1Intsts::B0)
    }
    #[doc = "Channel 1 Interrupt generated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1Intsts::B1)
    }
}
#[doc = "Channel 2 Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2Intsts {
    #[doc = "0: Channel 2 Interrupt generated"]
    B0 = 0,
    #[doc = "1: Channel 2 Interrupt generated"]
    B1 = 1,
}
impl From<Ch2Intsts> for bool {
    #[inline(always)]
    fn from(variant: Ch2Intsts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2_INTSTS` reader - Channel 2 Interrupt Status"]
pub type Ch2IntstsR = crate::BitReader<Ch2Intsts>;
impl Ch2IntstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2Intsts {
        match self.bits {
            false => Ch2Intsts::B0,
            true => Ch2Intsts::B1,
        }
    }
    #[doc = "Channel 2 Interrupt generated"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ch2Intsts::B0
    }
    #[doc = "Channel 2 Interrupt generated"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ch2Intsts::B1
    }
}
#[doc = "Field `CH2_INTSTS` writer - Channel 2 Interrupt Status"]
pub type Ch2IntstsW<'a, REG> = crate::BitWriter<'a, REG, Ch2Intsts>;
impl<'a, REG> Ch2IntstsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 2 Interrupt generated"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2Intsts::B0)
    }
    #[doc = "Channel 2 Interrupt generated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2Intsts::B1)
    }
}
#[doc = "Channel 3 Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3Intsts {
    #[doc = "0: Channel 3 Interrupt generated"]
    B0 = 0,
    #[doc = "1: Channel 3 Interrupt generated"]
    B1 = 1,
}
impl From<Ch3Intsts> for bool {
    #[inline(always)]
    fn from(variant: Ch3Intsts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3_INTSTS` reader - Channel 3 Interrupt Status"]
pub type Ch3IntstsR = crate::BitReader<Ch3Intsts>;
impl Ch3IntstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3Intsts {
        match self.bits {
            false => Ch3Intsts::B0,
            true => Ch3Intsts::B1,
        }
    }
    #[doc = "Channel 3 Interrupt generated"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ch3Intsts::B0
    }
    #[doc = "Channel 3 Interrupt generated"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ch3Intsts::B1
    }
}
#[doc = "Field `CH3_INTSTS` writer - Channel 3 Interrupt Status"]
pub type Ch3IntstsW<'a, REG> = crate::BitWriter<'a, REG, Ch3Intsts>;
impl<'a, REG> Ch3IntstsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 3 Interrupt generated"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3Intsts::B0)
    }
    #[doc = "Channel 3 Interrupt generated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3Intsts::B1)
    }
}
#[doc = "Field `CH0_POL` reader - Channel 0 Interrupt Polarity Flag This bit is used in capture mode in order to identify the transition of the input waveform when interrupt is generated. When bit is 1, please refer to PWM0_PERIOD_HPR to know the effective high cycle of Channel 0 input waveform. Otherwise, please refer to PWM0_PERIOD_LPR to know the effective low cycle of Channel 0 input waveform. Write 1 to CH0_IntSts will clear this bit."]
pub type Ch0PolR = crate::BitReader;
#[doc = "Field `CH1_POL` reader - Channel 1 Interrupt Polarity Flag This bit is used in capture mode in order to identify the transition of the input waveform when interrupt is generated. When bit is 1, please refer to PWM1_PERIOD_HPR to know the effective high cycle of Channel 1 input waveform. Otherwise, please refer to PWM1_PERIOD_LPR to know the effective low cycle of Channel 1 input waveform. Write 1 to CH1_IntSts will clear this bit."]
pub type Ch1PolR = crate::BitReader;
#[doc = "Field `CH2_POL` reader - Channel 2 Interrupt Polarity Flag This bit is used in capture mode in order to identify the transition of the input waveform when interrupt is generated. When bit is 1, please refer to PWM2_PERIOD_HPR to know the effective high cycle of Channel 2 input waveform. Otherwise, please refer to PWM2_PERIOD_LPR to know the effective low cycle of Channel 2 input waveform. Write 1 to CH2_IntSts will clear this bit."]
pub type Ch2PolR = crate::BitReader;
#[doc = "Field `CH3_POL` reader - Channel 3 Interrupt Polarity Flag This bit is used in capture mode in order to identify the transition of the input waveform when interrupt is generated. When bit is 1, please refer to PWM3_PERIOD_HPR to know the effective high cycle of Channel 3 input waveform. Otherwise, please refer to PWM3_PERIOD_LPR to know the effective low cycle of Channel 3 input waveform. Write 1 to CH3_IntSts will clear this bit."]
pub type Ch3PolR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Raw Interrupt Status"]
    #[inline(always)]
    pub fn ch0_intsts(&self) -> Ch0IntstsR {
        Ch0IntstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Interrupt Status"]
    #[inline(always)]
    pub fn ch1_intsts(&self) -> Ch1IntstsR {
        Ch1IntstsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Interrupt Status"]
    #[inline(always)]
    pub fn ch2_intsts(&self) -> Ch2IntstsR {
        Ch2IntstsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Interrupt Status"]
    #[inline(always)]
    pub fn ch3_intsts(&self) -> Ch3IntstsR {
        Ch3IntstsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 0 Interrupt Polarity Flag This bit is used in capture mode in order to identify the transition of the input waveform when interrupt is generated. When bit is 1, please refer to PWM0_PERIOD_HPR to know the effective high cycle of Channel 0 input waveform. Otherwise, please refer to PWM0_PERIOD_LPR to know the effective low cycle of Channel 0 input waveform. Write 1 to CH0_IntSts will clear this bit."]
    #[inline(always)]
    pub fn ch0_pol(&self) -> Ch0PolR {
        Ch0PolR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 Interrupt Polarity Flag This bit is used in capture mode in order to identify the transition of the input waveform when interrupt is generated. When bit is 1, please refer to PWM1_PERIOD_HPR to know the effective high cycle of Channel 1 input waveform. Otherwise, please refer to PWM1_PERIOD_LPR to know the effective low cycle of Channel 1 input waveform. Write 1 to CH1_IntSts will clear this bit."]
    #[inline(always)]
    pub fn ch1_pol(&self) -> Ch1PolR {
        Ch1PolR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Interrupt Polarity Flag This bit is used in capture mode in order to identify the transition of the input waveform when interrupt is generated. When bit is 1, please refer to PWM2_PERIOD_HPR to know the effective high cycle of Channel 2 input waveform. Otherwise, please refer to PWM2_PERIOD_LPR to know the effective low cycle of Channel 2 input waveform. Write 1 to CH2_IntSts will clear this bit."]
    #[inline(always)]
    pub fn ch2_pol(&self) -> Ch2PolR {
        Ch2PolR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Interrupt Polarity Flag This bit is used in capture mode in order to identify the transition of the input waveform when interrupt is generated. When bit is 1, please refer to PWM3_PERIOD_HPR to know the effective high cycle of Channel 3 input waveform. Otherwise, please refer to PWM3_PERIOD_LPR to know the effective low cycle of Channel 3 input waveform. Write 1 to CH3_IntSts will clear this bit."]
    #[inline(always)]
    pub fn ch3_pol(&self) -> Ch3PolR {
        Ch3PolR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_intsts(&mut self) -> Ch0IntstsW<PwmIntstsSpec> {
        Ch0IntstsW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_intsts(&mut self) -> Ch1IntstsW<PwmIntstsSpec> {
        Ch1IntstsW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_intsts(&mut self) -> Ch2IntstsW<PwmIntstsSpec> {
        Ch2IntstsW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_intsts(&mut self) -> Ch3IntstsW<PwmIntstsSpec> {
        Ch3IntstsW::new(self, 3)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_intsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_intsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmIntstsSpec;
impl crate::RegisterSpec for PwmIntstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_intsts::R`](R) reader structure"]
impl crate::Readable for PwmIntstsSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm_intsts::W`](W) writer structure"]
impl crate::Writable for PwmIntstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWM_INTSTS to value 0"]
impl crate::Resettable for PwmIntstsSpec {
    const RESET_VALUE: u32 = 0;
}
