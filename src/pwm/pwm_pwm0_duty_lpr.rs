#[doc = "Register `PWM_PWM0_DUTY_LPR` reader"]
pub type R = crate::R<PwmPwm0DutyLprSpec>;
#[doc = "Register `PWM_PWM0_DUTY_LPR` writer"]
pub type W = crate::W<PwmPwm0DutyLprSpec>;
#[doc = "Field `DUTY_LPR` reader - Output Waveform Duty Cycle/Input Waveform Low Polarity Cycle If PWM is operated at the continuous mode or one-shot mode, this value defines the duty cycle of the output waveform. The PWM starts the output waveform with duty cycle. Note that, if the PWM is operated at the center-aligned mode, the period should be an even one, and therefore only the \\[31:1\\]
is taken into account. If PWM is operated at the capture mode, this value indicates the effective low polarity cycles of input waveform. This value is based on the PWM clock. The value ranges from 0 to (2^32-1)."]
pub type DutyLprR = crate::FieldReader<u32>;
#[doc = "Field `DUTY_LPR` writer - Output Waveform Duty Cycle/Input Waveform Low Polarity Cycle If PWM is operated at the continuous mode or one-shot mode, this value defines the duty cycle of the output waveform. The PWM starts the output waveform with duty cycle. Note that, if the PWM is operated at the center-aligned mode, the period should be an even one, and therefore only the \\[31:1\\]
is taken into account. If PWM is operated at the capture mode, this value indicates the effective low polarity cycles of input waveform. This value is based on the PWM clock. The value ranges from 0 to (2^32-1)."]
pub type DutyLprW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Output Waveform Duty Cycle/Input Waveform Low Polarity Cycle If PWM is operated at the continuous mode or one-shot mode, this value defines the duty cycle of the output waveform. The PWM starts the output waveform with duty cycle. Note that, if the PWM is operated at the center-aligned mode, the period should be an even one, and therefore only the \\[31:1\\]
is taken into account. If PWM is operated at the capture mode, this value indicates the effective low polarity cycles of input waveform. This value is based on the PWM clock. The value ranges from 0 to (2^32-1)."]
    #[inline(always)]
    pub fn duty_lpr(&self) -> DutyLprR {
        DutyLprR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Output Waveform Duty Cycle/Input Waveform Low Polarity Cycle If PWM is operated at the continuous mode or one-shot mode, this value defines the duty cycle of the output waveform. The PWM starts the output waveform with duty cycle. Note that, if the PWM is operated at the center-aligned mode, the period should be an even one, and therefore only the \\[31:1\\]
is taken into account. If PWM is operated at the capture mode, this value indicates the effective low polarity cycles of input waveform. This value is based on the PWM clock. The value ranges from 0 to (2^32-1)."]
    #[inline(always)]
    #[must_use]
    pub fn duty_lpr(&mut self) -> DutyLprW<PwmPwm0DutyLprSpec> {
        DutyLprW::new(self, 0)
    }
}
#[doc = "PWM Channel 0 Duty Register/Low Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm0_duty_lpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm0_duty_lpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmPwm0DutyLprSpec;
impl crate::RegisterSpec for PwmPwm0DutyLprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_pwm0_duty_lpr::R`](R) reader structure"]
impl crate::Readable for PwmPwm0DutyLprSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm_pwm0_duty_lpr::W`](W) writer structure"]
impl crate::Writable for PwmPwm0DutyLprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWM_PWM0_DUTY_LPR to value 0"]
impl crate::Resettable for PwmPwm0DutyLprSpec {
    const RESET_VALUE: u32 = 0;
}
