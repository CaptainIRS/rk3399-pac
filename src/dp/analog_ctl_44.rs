#[doc = "Register `ANALOG_CTL_44` reader"]
pub type R = crate::R<AnalogCtl44Spec>;
#[doc = "Register `ANALOG_CTL_44` writer"]
pub type W = crate::W<AnalogCtl44Spec>;
#[doc = "Field `R_CH2_EMP_FORCE_VALUE` reader - The forced ch2 emp value (for calculating ch2_pre_emphasis_bit) value in specific V_diff and Pre_emphasis."]
pub type RCh2EmpForceValueR = crate::FieldReader;
#[doc = "Field `R_CH2_EMP_FORCE_VALUE` writer - The forced ch2 emp value (for calculating ch2_pre_emphasis_bit) value in specific V_diff and Pre_emphasis."]
pub type RCh2EmpForceValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The forced ch2 emp value (for calculating ch2_pre_emphasis_bit) value in specific V_diff and Pre_emphasis."]
    #[inline(always)]
    pub fn r_ch2_emp_force_value(&self) -> RCh2EmpForceValueR {
        RCh2EmpForceValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The forced ch2 emp value (for calculating ch2_pre_emphasis_bit) value in specific V_diff and Pre_emphasis."]
    #[inline(always)]
    #[must_use]
    pub fn r_ch2_emp_force_value(&mut self) -> RCh2EmpForceValueW<AnalogCtl44Spec> {
        RCh2EmpForceValueW::new(self, 0)
    }
}
#[doc = "CH2_EMP_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_44::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_44::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl44Spec;
impl crate::RegisterSpec for AnalogCtl44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_44::R`](R) reader structure"]
impl crate::Readable for AnalogCtl44Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_44::W`](W) writer structure"]
impl crate::Writable for AnalogCtl44Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets ANALOG_CTL_44 to value 0"]
impl crate::Resettable for AnalogCtl44Spec {
    const RESET_VALUE: u32 = 0;
}
