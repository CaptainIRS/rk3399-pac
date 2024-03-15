#[doc = "Register `HDCPREG_SEED0` writer"]
pub type W = crate::W<HdcpregSeed0Spec>;
#[doc = "Field `HDCPREG_SEED0` writer - Least significant byte of the decryption seed value (dpk_decrypt_seed\\[7:0\\])."]
pub type HdcpregSeed0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Least significant byte of the decryption seed value (dpk_decrypt_seed\\[7:0\\])."]
    #[inline(always)]
    #[must_use]
    pub fn hdcpreg_seed0(&mut self) -> HdcpregSeed0W<HdcpregSeed0Spec> {
        HdcpregSeed0W::new(self, 0)
    }
}
#[doc = "Least significant byte of the decryption seed value (dpk_decrypt_seed\\[7:0\\]).\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_seed0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregSeed0Spec;
impl crate::RegisterSpec for HdcpregSeed0Spec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`hdcpreg_seed0::W`](W) writer structure"]
impl crate::Writable for HdcpregSeed0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCPREG_SEED0 to value 0"]
impl crate::Resettable for HdcpregSeed0Spec {
    const RESET_VALUE: u8 = 0;
}
