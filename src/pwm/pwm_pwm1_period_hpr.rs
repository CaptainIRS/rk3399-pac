#[doc = "Register `PWM_PWM1_PERIOD_HPR` reader"]
pub type R = crate::R<PwmPwm1PeriodHprSpec>;
#[doc = "Register `PWM_PWM1_PERIOD_HPR` writer"]
pub type W = crate::W<PwmPwm1PeriodHprSpec>;
#[doc = "Field `PERIOD_HPR` reader - Output Waveform Period/Input Waveform High Polarity Cycle\n\nIf PWM is operated at the continuous mode or one-shot mode,\n\nthis value defines the period of the output waveform. Note that, if\n\nthe PWM is operated at the center-aligned mode, the period\n\nshould be an even one, and therefore only the bit \\[31:1\\]
is taken\n\ninto account and bit \\[0\\]
always considered as 0.\n\nIf PWM is operated at the capture mode, this value indicates the\n\neffective high polarity cycles of input waveform.\n\nThis value is based on the PWM clock. The value ranges from 0 to\n\n(2^32-1)."]
pub type PeriodHprR = crate::FieldReader<u32>;
#[doc = "Field `PERIOD_HPR` writer - Output Waveform Period/Input Waveform High Polarity Cycle\n\nIf PWM is operated at the continuous mode or one-shot mode,\n\nthis value defines the period of the output waveform. Note that, if\n\nthe PWM is operated at the center-aligned mode, the period\n\nshould be an even one, and therefore only the bit \\[31:1\\]
is taken\n\ninto account and bit \\[0\\]
always considered as 0.\n\nIf PWM is operated at the capture mode, this value indicates the\n\neffective high polarity cycles of input waveform.\n\nThis value is based on the PWM clock. The value ranges from 0 to\n\n(2^32-1)."]
pub type PeriodHprW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Output Waveform Period/Input Waveform High Polarity Cycle\n\nIf PWM is operated at the continuous mode or one-shot mode,\n\nthis value defines the period of the output waveform. Note that, if\n\nthe PWM is operated at the center-aligned mode, the period\n\nshould be an even one, and therefore only the bit \\[31:1\\]
is taken\n\ninto account and bit \\[0\\]
always considered as 0.\n\nIf PWM is operated at the capture mode, this value indicates the\n\neffective high polarity cycles of input waveform.\n\nThis value is based on the PWM clock. The value ranges from 0 to\n\n(2^32-1)."]
    #[inline(always)]
    pub fn period_hpr(&self) -> PeriodHprR {
        PeriodHprR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Output Waveform Period/Input Waveform High Polarity Cycle\n\nIf PWM is operated at the continuous mode or one-shot mode,\n\nthis value defines the period of the output waveform. Note that, if\n\nthe PWM is operated at the center-aligned mode, the period\n\nshould be an even one, and therefore only the bit \\[31:1\\]
is taken\n\ninto account and bit \\[0\\]
always considered as 0.\n\nIf PWM is operated at the capture mode, this value indicates the\n\neffective high polarity cycles of input waveform.\n\nThis value is based on the PWM clock. The value ranges from 0 to\n\n(2^32-1)."]
    #[inline(always)]
    #[must_use]
    pub fn period_hpr(&mut self) -> PeriodHprW<PwmPwm1PeriodHprSpec> {
        PeriodHprW::new(self, 0)
    }
}
#[doc = "PWM Channel 1 Period Register/High Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm1_period_hpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm1_period_hpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmPwm1PeriodHprSpec;
impl crate::RegisterSpec for PwmPwm1PeriodHprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_pwm1_period_hpr::R`](R) reader structure"]
impl crate::Readable for PwmPwm1PeriodHprSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm_pwm1_period_hpr::W`](W) writer structure"]
impl crate::Writable for PwmPwm1PeriodHprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWM_PWM1_PERIOD_HPR to value 0"]
impl crate::Resettable for PwmPwm1PeriodHprSpec {
    const RESET_VALUE: u32 = 0;
}
