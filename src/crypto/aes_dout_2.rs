#[doc = "Register `AES_DOUT_2` reader"]
pub type R = crate::R<AesDout2Spec>;
#[doc = "Field `AES_DOUT_2` reader - Specifies AES Output data \\[63:32\\]."]
pub type AesDout2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES Output data \\[63:32\\]."]
    #[inline(always)]
    pub fn aes_dout_2(&self) -> AesDout2R {
        AesDout2R::new(self.bits)
    }
}
#[doc = "AES Output Data 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_dout_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesDout2Spec;
impl crate::RegisterSpec for AesDout2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_dout_2::R`](R) reader structure"]
impl crate::Readable for AesDout2Spec {}
#[doc = "`reset()` method sets AES_DOUT_2 to value 0"]
impl crate::Resettable for AesDout2Spec {
    const RESET_VALUE: u32 = 0;
}
