#[doc = "Register `ANALOG_CTL_33` reader"]
pub type R = crate::R<AnalogCtl33Spec>;
#[doc = "Register `ANALOG_CTL_33` writer"]
pub type W = crate::W<AnalogCtl33Spec>;
#[doc = "Field `R_PC2_400MV_6DB` reader - The lookup-table 3(for calculating \n\nchx_swing_bit and chx_pc2_bit) value \n\nwhen V_diff is 400mv and Pre_emphasis \n\nis 6db."]
pub type RPc2_400mv6dbR = crate::FieldReader;
#[doc = "Field `R_PC2_400MV_6DB` writer - The lookup-table 3(for calculating \n\nchx_swing_bit and chx_pc2_bit) value \n\nwhen V_diff is 400mv and Pre_emphasis \n\nis 6db."]
pub type RPc2_400mv6dbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The lookup-table 3(for calculating \n\nchx_swing_bit and chx_pc2_bit) value \n\nwhen V_diff is 400mv and Pre_emphasis \n\nis 6db."]
    #[inline(always)]
    pub fn r_pc2_400mv_6db(&self) -> RPc2_400mv6dbR {
        RPc2_400mv6dbR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The lookup-table 3(for calculating \n\nchx_swing_bit and chx_pc2_bit) value \n\nwhen V_diff is 400mv and Pre_emphasis \n\nis 6db."]
    #[inline(always)]
    #[must_use]
    pub fn r_pc2_400mv_6db(&mut self) -> RPc2_400mv6dbW<AnalogCtl33Spec> {
        RPc2_400mv6dbW::new(self, 0)
    }
}
#[doc = "PC2_400MV_6DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_33::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_33::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl33Spec;
impl crate::RegisterSpec for AnalogCtl33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_33::R`](R) reader structure"]
impl crate::Readable for AnalogCtl33Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_33::W`](W) writer structure"]
impl crate::Writable for AnalogCtl33Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets ANALOG_CTL_33 to value 0x08"]
impl crate::Resettable for AnalogCtl33Spec {
    const RESET_VALUE: u32 = 0x08;
}
