#[doc = "Register `HASH_DOUT_5` reader"]
pub type R = crate::R<HashDout5Spec>;
#[doc = "Field `HASH_RESULT_5` reader - Specifies the HASH Result \\[31:0\\]"]
pub type HashResult5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the HASH Result \\[31:0\\]"]
    #[inline(always)]
    pub fn hash_result_5(&self) -> HashResult5R {
        HashResult5R::new(self.bits)
    }
}
#[doc = "Hash Result Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_dout_5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashDout5Spec;
impl crate::RegisterSpec for HashDout5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_dout_5::R`](R) reader structure"]
impl crate::Readable for HashDout5Spec {}
#[doc = "`reset()` method sets HASH_DOUT_5 to value 0"]
impl crate::Resettable for HashDout5Spec {
    const RESET_VALUE: u32 = 0;
}
