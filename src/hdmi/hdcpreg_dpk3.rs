#[doc = "Register `HDCPREG_DPK3` writer"]
pub type W = crate::W<HdcpregDpk3Spec>;
#[doc = "Field `DPK_DATA` writer - Byte of the encrypted DPK value. dpk\\[31:24\\]"]
pub type DpkDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Byte of the encrypted DPK value. dpk\\[31:24\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dpk_data(&mut self) -> DpkDataW<HdcpregDpk3Spec> {
        DpkDataW::new(self, 0)
    }
}
#[doc = "HDCP Encrypted DPK Data Register 3\n\nThis register contains an HDCP DPK byte. The required software configuration sequence is\n\ndocumented in the Cores HDMI Transmitter User Guide in the \"Programming\" chapter,\n\nSection 3.2.4, \"Configure HDCP.\"\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_dpk3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregDpk3Spec;
impl crate::RegisterSpec for HdcpregDpk3Spec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`hdcpreg_dpk3::W`](W) writer structure"]
impl crate::Writable for HdcpregDpk3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCPREG_DPK3 to value 0"]
impl crate::Resettable for HdcpregDpk3Spec {
    const RESET_VALUE: u8 = 0;
}
