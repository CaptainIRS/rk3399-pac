#[doc = "Register `ANALOG_CTL_48` reader"]
pub type R = crate::R<AnalogCtl48Spec>;
#[doc = "Register `ANALOG_CTL_48` writer"]
pub type W = crate::W<AnalogCtl48Spec>;
#[doc = "Field `R_CH3_PC2_FORCE_VALUE` reader - The forced ch3 PC2 value (for calculating ch3_swing_bit and ch3_pc2_bit) value in specific V_diff and Pre_emphasis."]
pub type RCh3Pc2ForceValueR = crate::FieldReader;
#[doc = "Field `R_CH3_PC2_FORCE_VALUE` writer - The forced ch3 PC2 value (for calculating ch3_swing_bit and ch3_pc2_bit) value in specific V_diff and Pre_emphasis."]
pub type RCh3Pc2ForceValueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The forced ch3 PC2 value (for calculating ch3_swing_bit and ch3_pc2_bit) value in specific V_diff and Pre_emphasis."]
    #[inline(always)]
    pub fn r_ch3_pc2_force_value(&self) -> RCh3Pc2ForceValueR {
        RCh3Pc2ForceValueR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The forced ch3 PC2 value (for calculating ch3_swing_bit and ch3_pc2_bit) value in specific V_diff and Pre_emphasis."]
    #[inline(always)]
    #[must_use]
    pub fn r_ch3_pc2_force_value(&mut self) -> RCh3Pc2ForceValueW<AnalogCtl48Spec> {
        RCh3Pc2ForceValueW::new(self, 0)
    }
}
#[doc = "CH3_PC2_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_48::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_48::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl48Spec;
impl crate::RegisterSpec for AnalogCtl48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_48::R`](R) reader structure"]
impl crate::Readable for AnalogCtl48Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_48::W`](W) writer structure"]
impl crate::Writable for AnalogCtl48Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets ANALOG_CTL_48 to value 0x04"]
impl crate::Resettable for AnalogCtl48Spec {
    const RESET_VALUE: u32 = 0x04;
}
