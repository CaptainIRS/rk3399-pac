#[doc = "Register `ANALOG_CTL_18` reader"]
pub type R = crate::R<AnalogCtl18Spec>;
#[doc = "Register `ANALOG_CTL_18` writer"]
pub type W = crate::W<AnalogCtl18Spec>;
#[doc = "Field `R_EMP_800MV_0DB` reader - The lookup-table 2(for calculating \n\nchx_pre_emp_bit) value when V_diff is \n\n800mv and Pre_emphasis is 0 db."]
pub type REmp800mv0dbR = crate::FieldReader;
#[doc = "Field `R_EMP_800MV_0DB` writer - The lookup-table 2(for calculating \n\nchx_pre_emp_bit) value when V_diff is \n\n800mv and Pre_emphasis is 0 db."]
pub type REmp800mv0dbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The lookup-table 2(for calculating \n\nchx_pre_emp_bit) value when V_diff is \n\n800mv and Pre_emphasis is 0 db."]
    #[inline(always)]
    pub fn r_emp_800mv_0db(&self) -> REmp800mv0dbR {
        REmp800mv0dbR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The lookup-table 2(for calculating \n\nchx_pre_emp_bit) value when V_diff is \n\n800mv and Pre_emphasis is 0 db."]
    #[inline(always)]
    #[must_use]
    pub fn r_emp_800mv_0db(&mut self) -> REmp800mv0dbW<AnalogCtl18Spec> {
        REmp800mv0dbW::new(self, 0)
    }
}
#[doc = "EMP_800MV_0DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl18Spec;
impl crate::RegisterSpec for AnalogCtl18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_18::R`](R) reader structure"]
impl crate::Readable for AnalogCtl18Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_18::W`](W) writer structure"]
impl crate::Writable for AnalogCtl18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANALOG_CTL_18 to value 0"]
impl crate::Resettable for AnalogCtl18Spec {
    const RESET_VALUE: u32 = 0;
}
