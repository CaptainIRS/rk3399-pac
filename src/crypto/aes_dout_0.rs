#[doc = "Register `AES_DOUT_0` reader"]
pub type R = crate::R<AesDout0Spec>;
#[doc = "Field `AES_DOUT_0` reader - Specifies AES Output data \\[127:96\\]."]
pub type AesDout0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES Output data \\[127:96\\]."]
    #[inline(always)]
    pub fn aes_dout_0(&self) -> AesDout0R {
        AesDout0R::new(self.bits)
    }
}
#[doc = "AES Output Data 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_dout_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesDout0Spec;
impl crate::RegisterSpec for AesDout0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_dout_0::R`](R) reader structure"]
impl crate::Readable for AesDout0Spec {}
#[doc = "`reset()` method sets AES_DOUT_0 to value 0"]
impl crate::Resettable for AesDout0Spec {
    const RESET_VALUE: u32 = 0;
}
