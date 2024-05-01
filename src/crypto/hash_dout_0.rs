#[doc = "Register `HASH_DOUT_0` reader"]
pub type R = crate::R<HashDout0Spec>;
#[doc = "Field `HASH_RESULT_0` reader - Specifies the HASH Result \\[159:128\\]"]
pub type HashResult0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the HASH Result \\[159:128\\]"]
    #[inline(always)]
    pub fn hash_result_0(&self) -> HashResult0R {
        HashResult0R::new(self.bits)
    }
}
#[doc = "Hash Result Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_dout_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashDout0Spec;
impl crate::RegisterSpec for HashDout0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_dout_0::R`](R) reader structure"]
impl crate::Readable for HashDout0Spec {}
#[doc = "`reset()` method sets HASH_DOUT_0 to value 0"]
impl crate::Resettable for HashDout0Spec {
    const RESET_VALUE: u32 = 0;
}
