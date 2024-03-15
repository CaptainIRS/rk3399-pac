#[doc = "Register `HDCPREG_DPK0` writer"]
pub type W = crate::W<HdcpregDpk0Spec>;
#[doc = "Field `DPK_DATA` writer - Byte of the encrypted DPK value. dpk\\[7:0\\]
When this byte is written, a strobe signal is generated that triggers the decryption and/or storage of the DPK word on the DPK internal RAM memory."]
pub type DpkDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Byte of the encrypted DPK value. dpk\\[7:0\\]
When this byte is written, a strobe signal is generated that triggers the decryption and/or storage of the DPK word on the DPK internal RAM memory."]
    #[inline(always)]
    #[must_use]
    pub fn dpk_data(&mut self) -> DpkDataW<HdcpregDpk0Spec> {
        DpkDataW::new(self, 0)
    }
}
#[doc = "Byte of the encrypted DPK value. dpk\\[7:0\\]
When this byte is written, a strobe signal is generated that triggers the decryption and/or storage of the DPK word on the DPK internal RAM memory.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_dpk0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregDpk0Spec;
impl crate::RegisterSpec for HdcpregDpk0Spec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`hdcpreg_dpk0::W`](W) writer structure"]
impl crate::Writable for HdcpregDpk0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCPREG_DPK0 to value 0"]
impl crate::Resettable for HdcpregDpk0Spec {
    const RESET_VALUE: u8 = 0;
}
