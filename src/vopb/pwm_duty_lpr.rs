#[doc = "Register `PWM_DUTY_LPR` reader"]
pub type R = crate::R<PwmDutyLprSpec>;
#[doc = "Register `PWM_DUTY_LPR` writer"]
pub type W = crate::W<PwmDutyLprSpec>;
#[doc = "Field `PWM_DUTY` reader - Output Waveform Duty Cycle/Input Waveform Low Polarity Cycle\n\nIf PWM is operated at the continuous mode or one-shot mode, this\n\nvalue defines the duty cycle of the output waveform. The PWM\n\nstarts the output waveform with duty cycle. Note that, if the PWM is\n\noperated at the center-aligned mode, the period should be an even\n\none, and therefore only the \\[31:1\\]
is taken into account.\n\nIf PWM is operated at the capture mode, this value indicates the\n\neffective low polarity cycles of input waveform.\n\nThis value is based on the PWM clock. The value ranges from 0 to\n\n(2^32-1)."]
pub type PwmDutyR = crate::FieldReader<u32>;
#[doc = "Field `PWM_DUTY` writer - Output Waveform Duty Cycle/Input Waveform Low Polarity Cycle\n\nIf PWM is operated at the continuous mode or one-shot mode, this\n\nvalue defines the duty cycle of the output waveform. The PWM\n\nstarts the output waveform with duty cycle. Note that, if the PWM is\n\noperated at the center-aligned mode, the period should be an even\n\none, and therefore only the \\[31:1\\]
is taken into account.\n\nIf PWM is operated at the capture mode, this value indicates the\n\neffective low polarity cycles of input waveform.\n\nThis value is based on the PWM clock. The value ranges from 0 to\n\n(2^32-1)."]
pub type PwmDutyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Output Waveform Duty Cycle/Input Waveform Low Polarity Cycle\n\nIf PWM is operated at the continuous mode or one-shot mode, this\n\nvalue defines the duty cycle of the output waveform. The PWM\n\nstarts the output waveform with duty cycle. Note that, if the PWM is\n\noperated at the center-aligned mode, the period should be an even\n\none, and therefore only the \\[31:1\\]
is taken into account.\n\nIf PWM is operated at the capture mode, this value indicates the\n\neffective low polarity cycles of input waveform.\n\nThis value is based on the PWM clock. The value ranges from 0 to\n\n(2^32-1)."]
    #[inline(always)]
    pub fn pwm_duty(&self) -> PwmDutyR {
        PwmDutyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Output Waveform Duty Cycle/Input Waveform Low Polarity Cycle\n\nIf PWM is operated at the continuous mode or one-shot mode, this\n\nvalue defines the duty cycle of the output waveform. The PWM\n\nstarts the output waveform with duty cycle. Note that, if the PWM is\n\noperated at the center-aligned mode, the period should be an even\n\none, and therefore only the \\[31:1\\]
is taken into account.\n\nIf PWM is operated at the capture mode, this value indicates the\n\neffective low polarity cycles of input waveform.\n\nThis value is based on the PWM clock. The value ranges from 0 to\n\n(2^32-1)."]
    #[inline(always)]
    #[must_use]
    pub fn pwm_duty(&mut self) -> PwmDutyW<PwmDutyLprSpec> {
        PwmDutyW::new(self, 0)
    }
}
#[doc = "PWM Duty Register/Low Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_duty_lpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_duty_lpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmDutyLprSpec;
impl crate::RegisterSpec for PwmDutyLprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_duty_lpr::R`](R) reader structure"]
impl crate::Readable for PwmDutyLprSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm_duty_lpr::W`](W) writer structure"]
impl crate::Writable for PwmDutyLprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWM_DUTY_LPR to value 0"]
impl crate::Resettable for PwmDutyLprSpec {
    const RESET_VALUE: u32 = 0;
}
