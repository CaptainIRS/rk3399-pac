#[doc = "Register `ANALOG_CTL_37` reader"]
pub type R = crate::R<AnalogCtl37Spec>;
#[doc = "Register `ANALOG_CTL_37` writer"]
pub type W = crate::W<AnalogCtl37Spec>;
#[doc = "Field `R_CH0_EMP_FORCE_VALUE` reader - The forced ch0 emp value (for \n\ncalculating ch0_pre_emphasis_bit) value \n\nin specific V_diff and Pre_emphasis."]
pub type RCh0EmpForceValueR = crate::FieldReader;
#[doc = "Field `R_CH0_EMP_FORCE_VALUE` writer - The forced ch0 emp value (for \n\ncalculating ch0_pre_emphasis_bit) value \n\nin specific V_diff and Pre_emphasis."]
pub type RCh0EmpForceValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The forced ch0 emp value (for \n\ncalculating ch0_pre_emphasis_bit) value \n\nin specific V_diff and Pre_emphasis."]
    #[inline(always)]
    pub fn r_ch0_emp_force_value(&self) -> RCh0EmpForceValueR {
        RCh0EmpForceValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The forced ch0 emp value (for \n\ncalculating ch0_pre_emphasis_bit) value \n\nin specific V_diff and Pre_emphasis."]
    #[inline(always)]
    #[must_use]
    pub fn r_ch0_emp_force_value(&mut self) -> RCh0EmpForceValueW<AnalogCtl37Spec> {
        RCh0EmpForceValueW::new(self, 0)
    }
}
#[doc = "CH0_EMP_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_37::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_37::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl37Spec;
impl crate::RegisterSpec for AnalogCtl37Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_37::R`](R) reader structure"]
impl crate::Readable for AnalogCtl37Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_37::W`](W) writer structure"]
impl crate::Writable for AnalogCtl37Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets ANALOG_CTL_37 to value 0"]
impl crate::Resettable for AnalogCtl37Spec {
    const RESET_VALUE: u32 = 0;
}
