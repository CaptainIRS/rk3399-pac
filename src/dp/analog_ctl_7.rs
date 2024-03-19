#[doc = "Register `ANALOG_CTL_7` reader"]
pub type R = crate::R<AnalogCtl7Spec>;
#[doc = "Register `ANALOG_CTL_7` writer"]
pub type W = crate::W<AnalogCtl7Spec>;
#[doc = "Field `R_AMP_600MV_0DB` reader - The lookup-table 1(for calculating \n\nchx_swing_bit) value when V_diff is 600mv \n\nand Pre_emphasis is 0 db."]
pub type RAmp600mv0dbR = crate::FieldReader;
#[doc = "Field `R_AMP_600MV_0DB` writer - The lookup-table 1(for calculating \n\nchx_swing_bit) value when V_diff is 600mv \n\nand Pre_emphasis is 0 db."]
pub type RAmp600mv0dbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The lookup-table 1(for calculating \n\nchx_swing_bit) value when V_diff is 600mv \n\nand Pre_emphasis is 0 db."]
    #[inline(always)]
    pub fn r_amp_600mv_0db(&self) -> RAmp600mv0dbR {
        RAmp600mv0dbR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The lookup-table 1(for calculating \n\nchx_swing_bit) value when V_diff is 600mv \n\nand Pre_emphasis is 0 db."]
    #[inline(always)]
    #[must_use]
    pub fn r_amp_600mv_0db(&mut self) -> RAmp600mv0dbW<AnalogCtl7Spec> {
        RAmp600mv0dbW::new(self, 0)
    }
}
#[doc = "AMP_600MV_0DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl7Spec;
impl crate::RegisterSpec for AnalogCtl7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_7::R`](R) reader structure"]
impl crate::Readable for AnalogCtl7Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_7::W`](W) writer structure"]
impl crate::Writable for AnalogCtl7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANALOG_CTL_7 to value 0x78"]
impl crate::Resettable for AnalogCtl7Spec {
    const RESET_VALUE: u32 = 0x78;
}
