#[doc = "Register `AES_DOUT_1` reader"]
pub type R = crate::R<AesDout1Spec>;
#[doc = "Field `AES_DOUT_1` reader - Specifies the Output data \\[95:64\\]."]
pub type AesDout1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the Output data \\[95:64\\]."]
    #[inline(always)]
    pub fn aes_dout_1(&self) -> AesDout1R {
        AesDout1R::new(self.bits)
    }
}
#[doc = "AES Output Data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_dout_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesDout1Spec;
impl crate::RegisterSpec for AesDout1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_dout_1::R`](R) reader structure"]
impl crate::Readable for AesDout1Spec {}
#[doc = "`reset()` method sets AES_DOUT_1 to value 0"]
impl crate::Resettable for AesDout1Spec {
    const RESET_VALUE: u32 = 0;
}
