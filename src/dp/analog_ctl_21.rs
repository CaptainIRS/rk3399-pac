#[doc = "Register `ANALOG_CTL_21` reader"]
pub type R = crate::R<AnalogCtl21Spec>;
#[doc = "Register `ANALOG_CTL_21` writer"]
pub type W = crate::W<AnalogCtl21Spec>;
#[doc = "Field `R_EMP_600MV_3P5DB` reader - The lookup-table 2(for calculating \n\nchx_pre_emp_bit) value when V_diff is \n\n600mv and Pre_emphasis is 3.5 db."]
pub type REmp600mv3p5dbR = crate::FieldReader;
#[doc = "Field `R_EMP_600MV_3P5DB` writer - The lookup-table 2(for calculating \n\nchx_pre_emp_bit) value when V_diff is \n\n600mv and Pre_emphasis is 3.5 db."]
pub type REmp600mv3p5dbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The lookup-table 2(for calculating \n\nchx_pre_emp_bit) value when V_diff is \n\n600mv and Pre_emphasis is 3.5 db."]
    #[inline(always)]
    pub fn r_emp_600mv_3p5db(&self) -> REmp600mv3p5dbR {
        REmp600mv3p5dbR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The lookup-table 2(for calculating \n\nchx_pre_emp_bit) value when V_diff is \n\n600mv and Pre_emphasis is 3.5 db."]
    #[inline(always)]
    #[must_use]
    pub fn r_emp_600mv_3p5db(&mut self) -> REmp600mv3p5dbW<AnalogCtl21Spec> {
        REmp600mv3p5dbW::new(self, 0)
    }
}
#[doc = "EMP_600MV_3P5DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl21Spec;
impl crate::RegisterSpec for AnalogCtl21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_21::R`](R) reader structure"]
impl crate::Readable for AnalogCtl21Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_21::W`](W) writer structure"]
impl crate::Writable for AnalogCtl21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANALOG_CTL_21 to value 0x3c"]
impl crate::Resettable for AnalogCtl21Spec {
    const RESET_VALUE: u32 = 0x3c;
}
