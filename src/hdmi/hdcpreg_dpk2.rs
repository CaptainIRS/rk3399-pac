#[doc = "Register `HDCPREG_DPK2` writer"]
pub type W = crate::W<HdcpregDpk2Spec>;
#[doc = "Field `DPK_DATA` writer - Byte of the encrypted DPK value. dpk\\[23:16\\]"]
pub type DpkDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Byte of the encrypted DPK value. dpk\\[23:16\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dpk_data(&mut self) -> DpkDataW<HdcpregDpk2Spec> {
        DpkDataW::new(self, 0)
    }
}
#[doc = "HDCP Encrypted DPK Data Register 2\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\"\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_dpk2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregDpk2Spec;
impl crate::RegisterSpec for HdcpregDpk2Spec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`hdcpreg_dpk2::W`](W) writer structure"]
impl crate::Writable for HdcpregDpk2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCPREG_DPK2 to value 0"]
impl crate::Resettable for HdcpregDpk2Spec {
    const RESET_VALUE: u8 = 0;
}
