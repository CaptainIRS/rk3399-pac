#[doc = "Register `ANALOG_CTL_20` reader"]
pub type R = crate::R<AnalogCtl20Spec>;
#[doc = "Register `ANALOG_CTL_20` writer"]
pub type W = crate::W<AnalogCtl20Spec>;
#[doc = "Field `R_EMP_400MV_3P5DB` reader - The lookup-table 2(for calculating chx_pre_emp_bit) value when V_diff is 400mv and Pre_emphasis is 3.5 db."]
pub type REmp400mv3p5dbR = crate::FieldReader;
#[doc = "Field `R_EMP_400MV_3P5DB` writer - The lookup-table 2(for calculating chx_pre_emp_bit) value when V_diff is 400mv and Pre_emphasis is 3.5 db."]
pub type REmp400mv3p5dbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The lookup-table 2(for calculating chx_pre_emp_bit) value when V_diff is 400mv and Pre_emphasis is 3.5 db."]
    #[inline(always)]
    pub fn r_emp_400mv_3p5db(&self) -> REmp400mv3p5dbR {
        REmp400mv3p5dbR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The lookup-table 2(for calculating chx_pre_emp_bit) value when V_diff is 400mv and Pre_emphasis is 3.5 db."]
    #[inline(always)]
    #[must_use]
    pub fn r_emp_400mv_3p5db(&mut self) -> REmp400mv3p5dbW<AnalogCtl20Spec> {
        REmp400mv3p5dbW::new(self, 0)
    }
}
#[doc = "EMP_400MV_3P5DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl20Spec;
impl crate::RegisterSpec for AnalogCtl20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_20::R`](R) reader structure"]
impl crate::Readable for AnalogCtl20Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_20::W`](W) writer structure"]
impl crate::Writable for AnalogCtl20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANALOG_CTL_20 to value 0x28"]
impl crate::Resettable for AnalogCtl20Spec {
    const RESET_VALUE: u32 = 0x28;
}
