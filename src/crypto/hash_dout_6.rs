#[doc = "Register `HASH_DOUT_6` reader"]
pub type R = crate::R<HashDout6Spec>;
#[doc = "Field `HASH_RESULT_6` reader - Specifies the HASH Result \\[31:0\\]"]
pub type HashResult6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the HASH Result \\[31:0\\]"]
    #[inline(always)]
    pub fn hash_result_6(&self) -> HashResult6R {
        HashResult6R::new(self.bits)
    }
}
#[doc = "Hash Result Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_dout_6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashDout6Spec;
impl crate::RegisterSpec for HashDout6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_dout_6::R`](R) reader structure"]
impl crate::Readable for HashDout6Spec {}
#[doc = "`reset()` method sets HASH_DOUT_6 to value 0"]
impl crate::Resettable for HashDout6Spec {
    const RESET_VALUE: u32 = 0;
}
