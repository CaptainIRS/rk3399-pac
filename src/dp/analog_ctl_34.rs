#[doc = "Register `ANALOG_CTL_34` reader"]
pub type R = crate::R<AnalogCtl34Spec>;
#[doc = "Register `ANALOG_CTL_34` writer"]
pub type W = crate::W<AnalogCtl34Spec>;
#[doc = "Field `R_PC2_600MV_6DB` reader - The lookup-table 3(for calculating chx_swing_bit and chx_pc2_bit) value when V_diff is 600mv and Pre_emphasis is 6db."]
pub type RPc2_600mv6dbR = crate::FieldReader;
#[doc = "Field `R_PC2_600MV_6DB` writer - The lookup-table 3(for calculating chx_swing_bit and chx_pc2_bit) value when V_diff is 600mv and Pre_emphasis is 6db."]
pub type RPc2_600mv6dbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The lookup-table 3(for calculating chx_swing_bit and chx_pc2_bit) value when V_diff is 600mv and Pre_emphasis is 6db."]
    #[inline(always)]
    pub fn r_pc2_600mv_6db(&self) -> RPc2_600mv6dbR {
        RPc2_600mv6dbR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The lookup-table 3(for calculating chx_swing_bit and chx_pc2_bit) value when V_diff is 600mv and Pre_emphasis is 6db."]
    #[inline(always)]
    #[must_use]
    pub fn r_pc2_600mv_6db(&mut self) -> RPc2_600mv6dbW<AnalogCtl34Spec> {
        RPc2_600mv6dbW::new(self, 0)
    }
}
#[doc = "PC2_600MV_6DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_34::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_34::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl34Spec;
impl crate::RegisterSpec for AnalogCtl34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_34::R`](R) reader structure"]
impl crate::Readable for AnalogCtl34Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_34::W`](W) writer structure"]
impl crate::Writable for AnalogCtl34Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets ANALOG_CTL_34 to value 0x0c"]
impl crate::Resettable for AnalogCtl34Spec {
    const RESET_VALUE: u32 = 0x0c;
}
