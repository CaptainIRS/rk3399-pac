#[doc = "Register `ANALOG_CTL_28` reader"]
pub type R = crate::R<AnalogCtl28Spec>;
#[doc = "Register `ANALOG_CTL_28` writer"]
pub type W = crate::W<AnalogCtl28Spec>;
#[doc = "Field `R_PC2_800MV_0DB` reader - The lookup-table 3(for calculating \n\nchx_swing_bit and chx_pc2_bit) value \n\nwhen V_diff is 800mv and Pre_emphasis \n\nis 0 db."]
pub type RPc2_800mv0dbR = crate::FieldReader;
#[doc = "Field `R_PC2_800MV_0DB` writer - The lookup-table 3(for calculating \n\nchx_swing_bit and chx_pc2_bit) value \n\nwhen V_diff is 800mv and Pre_emphasis \n\nis 0 db."]
pub type RPc2_800mv0dbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The lookup-table 3(for calculating \n\nchx_swing_bit and chx_pc2_bit) value \n\nwhen V_diff is 800mv and Pre_emphasis \n\nis 0 db."]
    #[inline(always)]
    pub fn r_pc2_800mv_0db(&self) -> RPc2_800mv0dbR {
        RPc2_800mv0dbR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The lookup-table 3(for calculating \n\nchx_swing_bit and chx_pc2_bit) value \n\nwhen V_diff is 800mv and Pre_emphasis \n\nis 0 db."]
    #[inline(always)]
    #[must_use]
    pub fn r_pc2_800mv_0db(&mut self) -> RPc2_800mv0dbW<AnalogCtl28Spec> {
        RPc2_800mv0dbW::new(self, 0)
    }
}
#[doc = "PC2_800MV_0DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_28::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_28::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl28Spec;
impl crate::RegisterSpec for AnalogCtl28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_28::R`](R) reader structure"]
impl crate::Readable for AnalogCtl28Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_28::W`](W) writer structure"]
impl crate::Writable for AnalogCtl28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets ANALOG_CTL_28 to value 0x08"]
impl crate::Resettable for AnalogCtl28Spec {
    const RESET_VALUE: u32 = 0x08;
}
