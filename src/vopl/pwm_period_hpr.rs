#[doc = "Register `PWM_PERIOD_HPR` reader"]
pub type R = crate::R<PwmPeriodHprSpec>;
#[doc = "Register `PWM_PERIOD_HPR` writer"]
pub type W = crate::W<PwmPeriodHprSpec>;
#[doc = "Field `PWM_PERIOD` reader - Output Waveform Period/Input Waveform High Polarity Cycle\n\nIf PWM is operated at the continuous mode or one-shot mode, this\n\nvalue defines the period of the output waveform. Note that, if the\n\nPWM is operated at the center-aligned mode, the period should be\n\nan even one, and therefore only the bit \\[31:1\\]
is taken into account\n\nand bit \\[0\\]
always considered as 0.\n\nIf PWM is operated at the capture mode, this value indicates the\n\neffective high polarity cycles of input waveform. This value is based\n\non the PWM clock.\n\nThe value ranges from 0 to (2^32-1)."]
pub type PwmPeriodR = crate::FieldReader<u32>;
#[doc = "Field `PWM_PERIOD` writer - Output Waveform Period/Input Waveform High Polarity Cycle\n\nIf PWM is operated at the continuous mode or one-shot mode, this\n\nvalue defines the period of the output waveform. Note that, if the\n\nPWM is operated at the center-aligned mode, the period should be\n\nan even one, and therefore only the bit \\[31:1\\]
is taken into account\n\nand bit \\[0\\]
always considered as 0.\n\nIf PWM is operated at the capture mode, this value indicates the\n\neffective high polarity cycles of input waveform. This value is based\n\non the PWM clock.\n\nThe value ranges from 0 to (2^32-1)."]
pub type PwmPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Output Waveform Period/Input Waveform High Polarity Cycle\n\nIf PWM is operated at the continuous mode or one-shot mode, this\n\nvalue defines the period of the output waveform. Note that, if the\n\nPWM is operated at the center-aligned mode, the period should be\n\nan even one, and therefore only the bit \\[31:1\\]
is taken into account\n\nand bit \\[0\\]
always considered as 0.\n\nIf PWM is operated at the capture mode, this value indicates the\n\neffective high polarity cycles of input waveform. This value is based\n\non the PWM clock.\n\nThe value ranges from 0 to (2^32-1)."]
    #[inline(always)]
    pub fn pwm_period(&self) -> PwmPeriodR {
        PwmPeriodR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Output Waveform Period/Input Waveform High Polarity Cycle\n\nIf PWM is operated at the continuous mode or one-shot mode, this\n\nvalue defines the period of the output waveform. Note that, if the\n\nPWM is operated at the center-aligned mode, the period should be\n\nan even one, and therefore only the bit \\[31:1\\]
is taken into account\n\nand bit \\[0\\]
always considered as 0.\n\nIf PWM is operated at the capture mode, this value indicates the\n\neffective high polarity cycles of input waveform. This value is based\n\non the PWM clock.\n\nThe value ranges from 0 to (2^32-1)."]
    #[inline(always)]
    #[must_use]
    pub fn pwm_period(&mut self) -> PwmPeriodW<PwmPeriodHprSpec> {
        PwmPeriodW::new(self, 0)
    }
}
#[doc = "PWM Period Register/High Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_period_hpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_period_hpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmPeriodHprSpec;
impl crate::RegisterSpec for PwmPeriodHprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_period_hpr::R`](R) reader structure"]
impl crate::Readable for PwmPeriodHprSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm_period_hpr::W`](W) writer structure"]
impl crate::Writable for PwmPeriodHprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWM_PERIOD_HPR to value 0"]
impl crate::Resettable for PwmPeriodHprSpec {
    const RESET_VALUE: u32 = 0;
}
