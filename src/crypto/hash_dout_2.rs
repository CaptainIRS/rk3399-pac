#[doc = "Register `HASH_DOUT_2` reader"]
pub type R = crate::R<HashDout2Spec>;
#[doc = "Field `HASH_RESULT_2` reader - Specifies the HASH Result \\[95:64\\]"]
pub type HashResult2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the HASH Result \\[95:64\\]"]
    #[inline(always)]
    pub fn hash_result_2(&self) -> HashResult2R {
        HashResult2R::new(self.bits)
    }
}
#[doc = "Hash Result Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_dout_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashDout2Spec;
impl crate::RegisterSpec for HashDout2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_dout_2::R`](R) reader structure"]
impl crate::Readable for HashDout2Spec {}
#[doc = "`reset()` method sets HASH_DOUT_2 to value 0"]
impl crate::Resettable for HashDout2Spec {
    const RESET_VALUE: u32 = 0;
}
