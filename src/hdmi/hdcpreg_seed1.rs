#[doc = "Register `HDCPREG_SEED1` writer"]
pub type W = crate::W<HdcpregSeed1Spec>;
#[doc = "Field `HDCPREG_SEED1` writer - Most significant byte of the decryption seed value\n\n(dpk_decrypt_seed\\[15:8\\])."]
pub type HdcpregSeed1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Most significant byte of the decryption seed value\n\n(dpk_decrypt_seed\\[15:8\\])."]
    #[inline(always)]
    #[must_use]
    pub fn hdcpreg_seed1(&mut self) -> HdcpregSeed1W<HdcpregSeed1Spec> {
        HdcpregSeed1W::new(self, 0)
    }
}
#[doc = "HDCP Encrypted DPK Seed Register 1\n\nThis register contains a byte of the HDCP Encrypted DPK seed value used to encrypt the\n\nDevice Private Keys. The required software configuration sequence is documented in the\n\nCores HDMI Transmitter User Guide in the \"Programming\" chapter, Section 3.2.4,\n\n\"Configure HDCP.\"\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_seed1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregSeed1Spec;
impl crate::RegisterSpec for HdcpregSeed1Spec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`hdcpreg_seed1::W`](W) writer structure"]
impl crate::Writable for HdcpregSeed1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCPREG_SEED1 to value 0"]
impl crate::Resettable for HdcpregSeed1Spec {
    const RESET_VALUE: u8 = 0;
}
