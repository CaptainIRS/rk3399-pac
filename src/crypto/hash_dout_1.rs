#[doc = "Register `HASH_DOUT_1` reader"]
pub type R = crate::R<HashDout1Spec>;
#[doc = "Field `HASH_RESULT_1` reader - Specifies the HASH Result \\[127:96\\]"]
pub type HashResult1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the HASH Result \\[127:96\\]"]
    #[inline(always)]
    pub fn hash_result_1(&self) -> HashResult1R {
        HashResult1R::new(self.bits)
    }
}
#[doc = "Hash Result Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_dout_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashDout1Spec;
impl crate::RegisterSpec for HashDout1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_dout_1::R`](R) reader structure"]
impl crate::Readable for HashDout1Spec {}
#[doc = "`reset()` method sets HASH_DOUT_1 to value 0"]
impl crate::Resettable for HashDout1Spec {
    const RESET_VALUE: u32 = 0;
}
