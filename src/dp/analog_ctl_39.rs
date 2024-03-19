#[doc = "Register `ANALOG_CTL_39` reader"]
pub type R = crate::R<AnalogCtl39Spec>;
#[doc = "Register `ANALOG_CTL_39` writer"]
pub type W = crate::W<AnalogCtl39Spec>;
#[doc = "Field `R_CH1_AMP_FORCE_VALUE` reader - The forced ch1 amp value (for \n\ncalculating ch1_swing_bit) value in \n\nspecific V_diff and Pre_emphasis."]
pub type RCh1AmpForceValueR = crate::FieldReader;
#[doc = "Field `R_CH1_AMP_FORCE_VALUE` writer - The forced ch1 amp value (for \n\ncalculating ch1_swing_bit) value in \n\nspecific V_diff and Pre_emphasis."]
pub type RCh1AmpForceValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The forced ch1 amp value (for \n\ncalculating ch1_swing_bit) value in \n\nspecific V_diff and Pre_emphasis."]
    #[inline(always)]
    pub fn r_ch1_amp_force_value(&self) -> RCh1AmpForceValueR {
        RCh1AmpForceValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The forced ch1 amp value (for \n\ncalculating ch1_swing_bit) value in \n\nspecific V_diff and Pre_emphasis."]
    #[inline(always)]
    #[must_use]
    pub fn r_ch1_amp_force_value(&mut self) -> RCh1AmpForceValueW<AnalogCtl39Spec> {
        RCh1AmpForceValueW::new(self, 0)
    }
}
#[doc = "CH1_AMP_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_39::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_39::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl39Spec;
impl crate::RegisterSpec for AnalogCtl39Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_39::R`](R) reader structure"]
impl crate::Readable for AnalogCtl39Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_39::W`](W) writer structure"]
impl crate::Writable for AnalogCtl39Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets ANALOG_CTL_39 to value 0x50"]
impl crate::Resettable for AnalogCtl39Spec {
    const RESET_VALUE: u32 = 0x50;
}
