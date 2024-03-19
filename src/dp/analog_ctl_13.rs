#[doc = "Register `ANALOG_CTL_13` reader"]
pub type R = crate::R<AnalogCtl13Spec>;
#[doc = "Register `ANALOG_CTL_13` writer"]
pub type W = crate::W<AnalogCtl13Spec>;
#[doc = "Field `R_AMP_400MV_6DB` reader - The lookup-table 1(for calculating \n\nchx_swing_bit) value when V_diff is 400mv \n\nand Pre_emphasis is 6db."]
pub type RAmp400mv6dbR = crate::FieldReader;
#[doc = "Field `R_AMP_400MV_6DB` writer - The lookup-table 1(for calculating \n\nchx_swing_bit) value when V_diff is 400mv \n\nand Pre_emphasis is 6db."]
pub type RAmp400mv6dbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The lookup-table 1(for calculating \n\nchx_swing_bit) value when V_diff is 400mv \n\nand Pre_emphasis is 6db."]
    #[inline(always)]
    pub fn r_amp_400mv_6db(&self) -> RAmp400mv6dbR {
        RAmp400mv6dbR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The lookup-table 1(for calculating \n\nchx_swing_bit) value when V_diff is 400mv \n\nand Pre_emphasis is 6db."]
    #[inline(always)]
    #[must_use]
    pub fn r_amp_400mv_6db(&mut self) -> RAmp400mv6dbW<AnalogCtl13Spec> {
        RAmp400mv6dbW::new(self, 0)
    }
}
#[doc = "AMP_400MV_6DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl13Spec;
impl crate::RegisterSpec for AnalogCtl13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_13::R`](R) reader structure"]
impl crate::Readable for AnalogCtl13Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_13::W`](W) writer structure"]
impl crate::Writable for AnalogCtl13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANALOG_CTL_13 to value 0x78"]
impl crate::Resettable for AnalogCtl13Spec {
    const RESET_VALUE: u32 = 0x78;
}
