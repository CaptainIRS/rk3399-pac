#[doc = "Register `ANALOG_CTL_31` reader"]
pub type R = crate::R<AnalogCtl31Spec>;
#[doc = "Register `ANALOG_CTL_31` writer"]
pub type W = crate::W<AnalogCtl31Spec>;
#[doc = "Field `R_PC2_600MV_3P5DB` reader - The lookup-table 3(for calculating chx_swing_bit and chx_pc2_bit) value when V_diff is 600mv and Pre_emphasis is 3.5 db."]
pub type RPc2_600mv3p5dbR = crate::FieldReader;
#[doc = "Field `R_PC2_600MV_3P5DB` writer - The lookup-table 3(for calculating chx_swing_bit and chx_pc2_bit) value when V_diff is 600mv and Pre_emphasis is 3.5 db."]
pub type RPc2_600mv3p5dbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The lookup-table 3(for calculating chx_swing_bit and chx_pc2_bit) value when V_diff is 600mv and Pre_emphasis is 3.5 db."]
    #[inline(always)]
    pub fn r_pc2_600mv_3p5db(&self) -> RPc2_600mv3p5dbR {
        RPc2_600mv3p5dbR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The lookup-table 3(for calculating chx_swing_bit and chx_pc2_bit) value when V_diff is 600mv and Pre_emphasis is 3.5 db."]
    #[inline(always)]
    #[must_use]
    pub fn r_pc2_600mv_3p5db(&mut self) -> RPc2_600mv3p5dbW<AnalogCtl31Spec> {
        RPc2_600mv3p5dbW::new(self, 0)
    }
}
#[doc = "PC2_600MV_3P5DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_31::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_31::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl31Spec;
impl crate::RegisterSpec for AnalogCtl31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_31::R`](R) reader structure"]
impl crate::Readable for AnalogCtl31Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_31::W`](W) writer structure"]
impl crate::Writable for AnalogCtl31Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets ANALOG_CTL_31 to value 0x09"]
impl crate::Resettable for AnalogCtl31Spec {
    const RESET_VALUE: u32 = 0x09;
}
