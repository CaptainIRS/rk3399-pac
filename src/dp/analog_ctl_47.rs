#[doc = "Register `ANALOG_CTL_47` reader"]
pub type R = crate::R<AnalogCtl47Spec>;
#[doc = "Register `ANALOG_CTL_47` writer"]
pub type W = crate::W<AnalogCtl47Spec>;
#[doc = "Field `R_CH3_EMP_FORCE_VALUE` reader - The forced ch3 emp value (for calculating \n\nch3_pre_emphasis_bit) value in specific V_diff \n\nand Pre_emphasis."]
pub type RCh3EmpForceValueR = crate::FieldReader;
#[doc = "Field `R_CH3_EMP_FORCE_VALUE` writer - The forced ch3 emp value (for calculating \n\nch3_pre_emphasis_bit) value in specific V_diff \n\nand Pre_emphasis."]
pub type RCh3EmpForceValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The forced ch3 emp value (for calculating \n\nch3_pre_emphasis_bit) value in specific V_diff \n\nand Pre_emphasis."]
    #[inline(always)]
    pub fn r_ch3_emp_force_value(&self) -> RCh3EmpForceValueR {
        RCh3EmpForceValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The forced ch3 emp value (for calculating \n\nch3_pre_emphasis_bit) value in specific V_diff \n\nand Pre_emphasis."]
    #[inline(always)]
    #[must_use]
    pub fn r_ch3_emp_force_value(&mut self) -> RCh3EmpForceValueW<AnalogCtl47Spec> {
        RCh3EmpForceValueW::new(self, 0)
    }
}
#[doc = "CH3_EMP_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_47::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_47::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl47Spec;
impl crate::RegisterSpec for AnalogCtl47Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_47::R`](R) reader structure"]
impl crate::Readable for AnalogCtl47Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_47::W`](W) writer structure"]
impl crate::Writable for AnalogCtl47Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets ANALOG_CTL_47 to value 0"]
impl crate::Resettable for AnalogCtl47Spec {
    const RESET_VALUE: u32 = 0;
}
