#[doc = "Register `ANALOG_CTL_29` reader"]
pub type R = crate::R<AnalogCtl29Spec>;
#[doc = "Register `ANALOG_CTL_29` writer"]
pub type W = crate::W<AnalogCtl29Spec>;
#[doc = "Field `R_PC2_1200MV_0DB` reader - The lookup-table 3(for calculating chx_swing_bit and chx_pc2_bit) value when V_diff is 1200mv and Pre_emphasis is 0 db."]
pub type RPc2_1200mv0dbR = crate::FieldReader;
#[doc = "Field `R_PC2_1200MV_0DB` writer - The lookup-table 3(for calculating chx_swing_bit and chx_pc2_bit) value when V_diff is 1200mv and Pre_emphasis is 0 db."]
pub type RPc2_1200mv0dbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The lookup-table 3(for calculating chx_swing_bit and chx_pc2_bit) value when V_diff is 1200mv and Pre_emphasis is 0 db."]
    #[inline(always)]
    pub fn r_pc2_1200mv_0db(&self) -> RPc2_1200mv0dbR {
        RPc2_1200mv0dbR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The lookup-table 3(for calculating chx_swing_bit and chx_pc2_bit) value when V_diff is 1200mv and Pre_emphasis is 0 db."]
    #[inline(always)]
    #[must_use]
    pub fn r_pc2_1200mv_0db(&mut self) -> RPc2_1200mv0dbW<AnalogCtl29Spec> {
        RPc2_1200mv0dbW::new(self, 0)
    }
}
#[doc = "PC2_1200MV_0DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_29::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_29::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl29Spec;
impl crate::RegisterSpec for AnalogCtl29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_29::R`](R) reader structure"]
impl crate::Readable for AnalogCtl29Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_29::W`](W) writer structure"]
impl crate::Writable for AnalogCtl29Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets ANALOG_CTL_29 to value 0x0c"]
impl crate::Resettable for AnalogCtl29Spec {
    const RESET_VALUE: u32 = 0x0c;
}
