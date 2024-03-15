#[doc = "Register `ANALOG_CTL_41` reader"]
pub type R = crate::R<AnalogCtl41Spec>;
#[doc = "Register `ANALOG_CTL_41` writer"]
pub type W = crate::W<AnalogCtl41Spec>;
#[doc = "Field `R_CH1_PC2_FORCE_VALUE` reader - The forced ch1 PC2 value (for calculating ch1_swing_bit and ch1_pc2_bit) value in specific V_diff and Pre_emphasis."]
pub type RCh1Pc2ForceValueR = crate::FieldReader;
#[doc = "Field `R_CH1_PC2_FORCE_VALUE` writer - The forced ch1 PC2 value (for calculating ch1_swing_bit and ch1_pc2_bit) value in specific V_diff and Pre_emphasis."]
pub type RCh1Pc2ForceValueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The forced ch1 PC2 value (for calculating ch1_swing_bit and ch1_pc2_bit) value in specific V_diff and Pre_emphasis."]
    #[inline(always)]
    pub fn r_ch1_pc2_force_value(&self) -> RCh1Pc2ForceValueR {
        RCh1Pc2ForceValueR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The forced ch1 PC2 value (for calculating ch1_swing_bit and ch1_pc2_bit) value in specific V_diff and Pre_emphasis."]
    #[inline(always)]
    #[must_use]
    pub fn r_ch1_pc2_force_value(&mut self) -> RCh1Pc2ForceValueW<AnalogCtl41Spec> {
        RCh1Pc2ForceValueW::new(self, 0)
    }
}
#[doc = "CH1_PC2_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_41::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_41::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl41Spec;
impl crate::RegisterSpec for AnalogCtl41Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_41::R`](R) reader structure"]
impl crate::Readable for AnalogCtl41Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_41::W`](W) writer structure"]
impl crate::Writable for AnalogCtl41Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets ANALOG_CTL_41 to value 0x04"]
impl crate::Resettable for AnalogCtl41Spec {
    const RESET_VALUE: u32 = 0x04;
}
