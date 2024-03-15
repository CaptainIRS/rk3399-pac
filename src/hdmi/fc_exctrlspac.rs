#[doc = "Register `FC_EXCTRLSPAC` reader"]
pub type R = crate::R<FcExctrlspacSpec>;
#[doc = "Register `FC_EXCTRLSPAC` writer"]
pub type W = crate::W<FcExctrlspacSpec>;
#[doc = "Field `EXCTRLPERIODSPACING` reader - Configuration of the maximum spacing between consecutive extended control periods (maximum of 50ms; refer to the applicable HDMI specification). When using the HDMI 2.0 supported features (HDMI_TX_20 = 1): generated spacing = (1/freq tmds clock)*256*512*(extctrlperiodspacing +1) else generated spacing = (1/freq tmds clock)*256*256*(extctrlperiodspacing +1)"]
pub type ExctrlperiodspacingR = crate::FieldReader;
#[doc = "Field `EXCTRLPERIODSPACING` writer - Configuration of the maximum spacing between consecutive extended control periods (maximum of 50ms; refer to the applicable HDMI specification). When using the HDMI 2.0 supported features (HDMI_TX_20 = 1): generated spacing = (1/freq tmds clock)*256*512*(extctrlperiodspacing +1) else generated spacing = (1/freq tmds clock)*256*256*(extctrlperiodspacing +1)"]
pub type ExctrlperiodspacingW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Configuration of the maximum spacing between consecutive extended control periods (maximum of 50ms; refer to the applicable HDMI specification). When using the HDMI 2.0 supported features (HDMI_TX_20 = 1): generated spacing = (1/freq tmds clock)*256*512*(extctrlperiodspacing +1) else generated spacing = (1/freq tmds clock)*256*256*(extctrlperiodspacing +1)"]
    #[inline(always)]
    pub fn exctrlperiodspacing(&self) -> ExctrlperiodspacingR {
        ExctrlperiodspacingR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configuration of the maximum spacing between consecutive extended control periods (maximum of 50ms; refer to the applicable HDMI specification). When using the HDMI 2.0 supported features (HDMI_TX_20 = 1): generated spacing = (1/freq tmds clock)*256*512*(extctrlperiodspacing +1) else generated spacing = (1/freq tmds clock)*256*256*(extctrlperiodspacing +1)"]
    #[inline(always)]
    #[must_use]
    pub fn exctrlperiodspacing(&mut self) -> ExctrlperiodspacingW<FcExctrlspacSpec> {
        ExctrlperiodspacingW::new(self, 0)
    }
}
#[doc = "Configuration of the maximum spacing between consecutive extended control periods (maximum of 50ms; refer to the applicable HDMI specification). When using the HDMI 2.0 supported features (HDMI_TX_20 = 1): generated spacing = (1/freq tmds clock)*256*512*(extctrlperiodspacing +1) else generated spacing = (1/freq tmds clock)*256*256*(extctrlperiodspacing +1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_exctrlspac::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_exctrlspac::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcExctrlspacSpec;
impl crate::RegisterSpec for FcExctrlspacSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_exctrlspac::R`](R) reader structure"]
impl crate::Readable for FcExctrlspacSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_exctrlspac::W`](W) writer structure"]
impl crate::Writable for FcExctrlspacSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_EXCTRLSPAC to value 0"]
impl crate::Resettable for FcExctrlspacSpec {
    const RESET_VALUE: u8 = 0;
}
