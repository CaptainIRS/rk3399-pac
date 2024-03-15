#[doc = "Register `PWM_PWM0_PERIOD_HPR` reader"]
pub type R = crate::R<PwmPwm0PeriodHprSpec>;
#[doc = "Register `PWM_PWM0_PERIOD_HPR` writer"]
pub type W = crate::W<PwmPwm0PeriodHprSpec>;
#[doc = "Field `PERIOD_HPR` reader - Output Waveform Period/Input Waveform High Polarity Cycle If PWM is operated at the continuous mode or one-shot mode, this value defines the period of the output waveform. Note that, if the PWM is operated at the center-aligned mode, the period should be an even one, and therefore only the bit \\[31:1\\]
is taken into account and bit \\[0\\]
always considered as 0. If PWM is operated at the capture mode, this value indicates the effective high polarity cycles of input waveform. This value is based on the PWM clock. The value ranges from 0 to (2^32-1)."]
pub type PeriodHprR = crate::FieldReader<u32>;
#[doc = "Field `PERIOD_HPR` writer - Output Waveform Period/Input Waveform High Polarity Cycle If PWM is operated at the continuous mode or one-shot mode, this value defines the period of the output waveform. Note that, if the PWM is operated at the center-aligned mode, the period should be an even one, and therefore only the bit \\[31:1\\]
is taken into account and bit \\[0\\]
always considered as 0. If PWM is operated at the capture mode, this value indicates the effective high polarity cycles of input waveform. This value is based on the PWM clock. The value ranges from 0 to (2^32-1)."]
pub type PeriodHprW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Output Waveform Period/Input Waveform High Polarity Cycle If PWM is operated at the continuous mode or one-shot mode, this value defines the period of the output waveform. Note that, if the PWM is operated at the center-aligned mode, the period should be an even one, and therefore only the bit \\[31:1\\]
is taken into account and bit \\[0\\]
always considered as 0. If PWM is operated at the capture mode, this value indicates the effective high polarity cycles of input waveform. This value is based on the PWM clock. The value ranges from 0 to (2^32-1)."]
    #[inline(always)]
    pub fn period_hpr(&self) -> PeriodHprR {
        PeriodHprR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Output Waveform Period/Input Waveform High Polarity Cycle If PWM is operated at the continuous mode or one-shot mode, this value defines the period of the output waveform. Note that, if the PWM is operated at the center-aligned mode, the period should be an even one, and therefore only the bit \\[31:1\\]
is taken into account and bit \\[0\\]
always considered as 0. If PWM is operated at the capture mode, this value indicates the effective high polarity cycles of input waveform. This value is based on the PWM clock. The value ranges from 0 to (2^32-1)."]
    #[inline(always)]
    #[must_use]
    pub fn period_hpr(&mut self) -> PeriodHprW<PwmPwm0PeriodHprSpec> {
        PeriodHprW::new(self, 0)
    }
}
#[doc = "PWM Channel 0 Period Register/High Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm0_period_hpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm0_period_hpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmPwm0PeriodHprSpec;
impl crate::RegisterSpec for PwmPwm0PeriodHprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_pwm0_period_hpr::R`](R) reader structure"]
impl crate::Readable for PwmPwm0PeriodHprSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm_pwm0_period_hpr::W`](W) writer structure"]
impl crate::Writable for PwmPwm0PeriodHprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWM_PWM0_PERIOD_HPR to value 0"]
impl crate::Resettable for PwmPwm0PeriodHprSpec {
    const RESET_VALUE: u32 = 0;
}
