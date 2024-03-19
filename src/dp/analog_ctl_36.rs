#[doc = "Register `ANALOG_CTL_36` reader"]
pub type R = crate::R<AnalogCtl36Spec>;
#[doc = "Register `ANALOG_CTL_36` writer"]
pub type W = crate::W<AnalogCtl36Spec>;
#[doc = "Field `R_CH0_AMP_FORCE_VALUE` reader - The forced ch0 amp value (for \n\ncalculating ch0_swing_bit) value in \n\nspecific V_diff and Pre_emphasis."]
pub type RCh0AmpForceValueR = crate::FieldReader;
#[doc = "Field `R_CH0_AMP_FORCE_VALUE` writer - The forced ch0 amp value (for \n\ncalculating ch0_swing_bit) value in \n\nspecific V_diff and Pre_emphasis."]
pub type RCh0AmpForceValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The forced ch0 amp value (for \n\ncalculating ch0_swing_bit) value in \n\nspecific V_diff and Pre_emphasis."]
    #[inline(always)]
    pub fn r_ch0_amp_force_value(&self) -> RCh0AmpForceValueR {
        RCh0AmpForceValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The forced ch0 amp value (for \n\ncalculating ch0_swing_bit) value in \n\nspecific V_diff and Pre_emphasis."]
    #[inline(always)]
    #[must_use]
    pub fn r_ch0_amp_force_value(&mut self) -> RCh0AmpForceValueW<AnalogCtl36Spec> {
        RCh0AmpForceValueW::new(self, 0)
    }
}
#[doc = "CH0_AMP_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_36::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_36::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl36Spec;
impl crate::RegisterSpec for AnalogCtl36Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_36::R`](R) reader structure"]
impl crate::Readable for AnalogCtl36Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_36::W`](W) writer structure"]
impl crate::Writable for AnalogCtl36Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets ANALOG_CTL_36 to value 0x50"]
impl crate::Resettable for AnalogCtl36Spec {
    const RESET_VALUE: u32 = 0x50;
}
