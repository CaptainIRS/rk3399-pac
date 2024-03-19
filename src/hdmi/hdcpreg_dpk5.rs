#[doc = "Register `HDCPREG_DPK5` writer"]
pub type W = crate::W<HdcpregDpk5Spec>;
#[doc = "Field `DPK_DATA` writer - Contains the value of DPK\\[x\\]\\[47:40\\]"]
pub type DpkDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Contains the value of DPK\\[x\\]\\[47:40\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dpk_data(&mut self) -> DpkDataW<HdcpregDpk5Spec> {
        DpkDataW::new(self, 0)
    }
}
#[doc = "HDCP Encrypted DPK Data Register 5\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\"\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_dpk5::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregDpk5Spec;
impl crate::RegisterSpec for HdcpregDpk5Spec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`hdcpreg_dpk5::W`](W) writer structure"]
impl crate::Writable for HdcpregDpk5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCPREG_DPK5 to value 0"]
impl crate::Resettable for HdcpregDpk5Spec {
    const RESET_VALUE: u8 = 0;
}
