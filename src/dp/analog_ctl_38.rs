#[doc = "Register `ANALOG_CTL_38` reader"]
pub type R = crate::R<AnalogCtl38Spec>;
#[doc = "Register `ANALOG_CTL_38` writer"]
pub type W = crate::W<AnalogCtl38Spec>;
#[doc = "Field `R_CH0_PC2_FORCE_VALUE` reader - The forced ch0 PC2 value (for calculating \n\nch0_swing_bit and ch0_pc2_bit) value in \n\nspecific V_diff and Pre_emphasis."]
pub type RCh0Pc2ForceValueR = crate::FieldReader;
#[doc = "Field `R_CH0_PC2_FORCE_VALUE` writer - The forced ch0 PC2 value (for calculating \n\nch0_swing_bit and ch0_pc2_bit) value in \n\nspecific V_diff and Pre_emphasis."]
pub type RCh0Pc2ForceValueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The forced ch0 PC2 value (for calculating \n\nch0_swing_bit and ch0_pc2_bit) value in \n\nspecific V_diff and Pre_emphasis."]
    #[inline(always)]
    pub fn r_ch0_pc2_force_value(&self) -> RCh0Pc2ForceValueR {
        RCh0Pc2ForceValueR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The forced ch0 PC2 value (for calculating \n\nch0_swing_bit and ch0_pc2_bit) value in \n\nspecific V_diff and Pre_emphasis."]
    #[inline(always)]
    #[must_use]
    pub fn r_ch0_pc2_force_value(&mut self) -> RCh0Pc2ForceValueW<AnalogCtl38Spec> {
        RCh0Pc2ForceValueW::new(self, 0)
    }
}
#[doc = "CH0_PC2_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_38::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_38::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl38Spec;
impl crate::RegisterSpec for AnalogCtl38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_38::R`](R) reader structure"]
impl crate::Readable for AnalogCtl38Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_38::W`](W) writer structure"]
impl crate::Writable for AnalogCtl38Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets ANALOG_CTL_38 to value 0x04"]
impl crate::Resettable for AnalogCtl38Spec {
    const RESET_VALUE: u32 = 0x04;
}
