#[doc = "Register `ANALOG_CTL_45` reader"]
pub type R = crate::R<AnalogCtl45Spec>;
#[doc = "Register `ANALOG_CTL_45` writer"]
pub type W = crate::W<AnalogCtl45Spec>;
#[doc = "Field `R_CH2_PC2_FORCE_VALUE` reader - The forced ch2 PC2 value (for calculating \n\nch2_swing_bit and ch2_pc2_bit) value in \n\nspecific V_diff and Pre_emphasis."]
pub type RCh2Pc2ForceValueR = crate::FieldReader;
#[doc = "Field `R_CH2_PC2_FORCE_VALUE` writer - The forced ch2 PC2 value (for calculating \n\nch2_swing_bit and ch2_pc2_bit) value in \n\nspecific V_diff and Pre_emphasis."]
pub type RCh2Pc2ForceValueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The forced ch2 PC2 value (for calculating \n\nch2_swing_bit and ch2_pc2_bit) value in \n\nspecific V_diff and Pre_emphasis."]
    #[inline(always)]
    pub fn r_ch2_pc2_force_value(&self) -> RCh2Pc2ForceValueR {
        RCh2Pc2ForceValueR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The forced ch2 PC2 value (for calculating \n\nch2_swing_bit and ch2_pc2_bit) value in \n\nspecific V_diff and Pre_emphasis."]
    #[inline(always)]
    #[must_use]
    pub fn r_ch2_pc2_force_value(&mut self) -> RCh2Pc2ForceValueW<AnalogCtl45Spec> {
        RCh2Pc2ForceValueW::new(self, 0)
    }
}
#[doc = "CH2_PC2_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl45Spec;
impl crate::RegisterSpec for AnalogCtl45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_45::R`](R) reader structure"]
impl crate::Readable for AnalogCtl45Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_45::W`](W) writer structure"]
impl crate::Writable for AnalogCtl45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets ANALOG_CTL_45 to value 0x04"]
impl crate::Resettable for AnalogCtl45Spec {
    const RESET_VALUE: u32 = 0x04;
}
