#[doc = "Register `ANALOG_CTL_46` reader"]
pub type R = crate::R<AnalogCtl46Spec>;
#[doc = "Register `ANALOG_CTL_46` writer"]
pub type W = crate::W<AnalogCtl46Spec>;
#[doc = "Field `R_CH3_AMP_FORCE_VALUE` reader - The forced ch3 amp value (for calculating ch3_swing_bit) value in specific V_diff and Pre_emphasis."]
pub type RCh3AmpForceValueR = crate::FieldReader;
#[doc = "Field `R_CH3_AMP_FORCE_VALUE` writer - The forced ch3 amp value (for calculating ch3_swing_bit) value in specific V_diff and Pre_emphasis."]
pub type RCh3AmpForceValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The forced ch3 amp value (for calculating ch3_swing_bit) value in specific V_diff and Pre_emphasis."]
    #[inline(always)]
    pub fn r_ch3_amp_force_value(&self) -> RCh3AmpForceValueR {
        RCh3AmpForceValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The forced ch3 amp value (for calculating ch3_swing_bit) value in specific V_diff and Pre_emphasis."]
    #[inline(always)]
    #[must_use]
    pub fn r_ch3_amp_force_value(&mut self) -> RCh3AmpForceValueW<AnalogCtl46Spec> {
        RCh3AmpForceValueW::new(self, 0)
    }
}
#[doc = "CH3_AMP_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_46::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_46::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl46Spec;
impl crate::RegisterSpec for AnalogCtl46Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_46::R`](R) reader structure"]
impl crate::Readable for AnalogCtl46Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_46::W`](W) writer structure"]
impl crate::Writable for AnalogCtl46Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets ANALOG_CTL_46 to value 0x50"]
impl crate::Resettable for AnalogCtl46Spec {
    const RESET_VALUE: u32 = 0x50;
}
