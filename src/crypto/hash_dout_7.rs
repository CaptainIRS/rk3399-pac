#[doc = "Register `HASH_DOUT_7` reader"]
pub type R = crate::R<HashDout7Spec>;
#[doc = "Field `HASH_RESULT_7` reader - Specifies the HASH Result \\[31:0\\]"]
pub type HashResult7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the HASH Result \\[31:0\\]"]
    #[inline(always)]
    pub fn hash_result_7(&self) -> HashResult7R {
        HashResult7R::new(self.bits)
    }
}
#[doc = "Hash Result Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_dout_7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashDout7Spec;
impl crate::RegisterSpec for HashDout7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_dout_7::R`](R) reader structure"]
impl crate::Readable for HashDout7Spec {}
#[doc = "`reset()` method sets HASH_DOUT_7 to value 0"]
impl crate::Resettable for HashDout7Spec {
    const RESET_VALUE: u32 = 0;
}
