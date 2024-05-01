#[doc = "Register `AES_DOUT_3` reader"]
pub type R = crate::R<AesDout3Spec>;
#[doc = "Field `AES_DOUT_3` reader - Specifies AES Output data \\[31:0\\]."]
pub type AesDout3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES Output data \\[31:0\\]."]
    #[inline(always)]
    pub fn aes_dout_3(&self) -> AesDout3R {
        AesDout3R::new(self.bits)
    }
}
#[doc = "AES Output Data 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_dout_3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesDout3Spec;
impl crate::RegisterSpec for AesDout3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_dout_3::R`](R) reader structure"]
impl crate::Readable for AesDout3Spec {}
#[doc = "`reset()` method sets AES_DOUT_3 to value 0"]
impl crate::Resettable for AesDout3Spec {
    const RESET_VALUE: u32 = 0;
}
