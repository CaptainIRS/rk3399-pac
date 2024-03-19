#[doc = "Register `ANALOG_CTL_12` reader"]
pub type R = crate::R<AnalogCtl12Spec>;
#[doc = "Register `ANALOG_CTL_12` writer"]
pub type W = crate::W<AnalogCtl12Spec>;
#[doc = "Field `R_AMP_800MV_3P5DB` reader - The lookup-table 1(for calculating \n\nchx_swing_bit) value when V_diff is 800mv \n\nand Pre_emphasis is 3.5 db."]
pub type RAmp800mv3p5dbR = crate::FieldReader;
#[doc = "Field `R_AMP_800MV_3P5DB` writer - The lookup-table 1(for calculating \n\nchx_swing_bit) value when V_diff is 800mv \n\nand Pre_emphasis is 3.5 db."]
pub type RAmp800mv3p5dbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The lookup-table 1(for calculating \n\nchx_swing_bit) value when V_diff is 800mv \n\nand Pre_emphasis is 3.5 db."]
    #[inline(always)]
    pub fn r_amp_800mv_3p5db(&self) -> RAmp800mv3p5dbR {
        RAmp800mv3p5dbR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The lookup-table 1(for calculating \n\nchx_swing_bit) value when V_diff is 800mv \n\nand Pre_emphasis is 3.5 db."]
    #[inline(always)]
    #[must_use]
    pub fn r_amp_800mv_3p5db(&mut self) -> RAmp800mv3p5dbW<AnalogCtl12Spec> {
        RAmp800mv3p5dbW::new(self, 0)
    }
}
#[doc = "AMP_800MV_3P5DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl12Spec;
impl crate::RegisterSpec for AnalogCtl12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_12::R`](R) reader structure"]
impl crate::Readable for AnalogCtl12Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_12::W`](W) writer structure"]
impl crate::Writable for AnalogCtl12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANALOG_CTL_12 to value 0xc8"]
impl crate::Resettable for AnalogCtl12Spec {
    const RESET_VALUE: u32 = 0xc8;
}
