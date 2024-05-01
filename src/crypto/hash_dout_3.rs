#[doc = "Register `HASH_DOUT_3` reader"]
pub type R = crate::R<HashDout3Spec>;
#[doc = "Field `HASH_RESULT_3` reader - Specifies the HASH Result \\[63:32\\]"]
pub type HashResult3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the HASH Result \\[63:32\\]"]
    #[inline(always)]
    pub fn hash_result_3(&self) -> HashResult3R {
        HashResult3R::new(self.bits)
    }
}
#[doc = "Hash Result Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_dout_3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashDout3Spec;
impl crate::RegisterSpec for HashDout3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_dout_3::R`](R) reader structure"]
impl crate::Readable for HashDout3Spec {}
#[doc = "`reset()` method sets HASH_DOUT_3 to value 0"]
impl crate::Resettable for HashDout3Spec {
    const RESET_VALUE: u32 = 0;
}
