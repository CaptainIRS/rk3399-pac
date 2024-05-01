#[doc = "Register `HASH_DOUT_4` reader"]
pub type R = crate::R<HashDout4Spec>;
#[doc = "Field `HASH_RESULT_4` reader - Specifies the HASH Result \\[31:0\\]"]
pub type HashResult4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the HASH Result \\[31:0\\]"]
    #[inline(always)]
    pub fn hash_result_4(&self) -> HashResult4R {
        HashResult4R::new(self.bits)
    }
}
#[doc = "Hash Result Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_dout_4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashDout4Spec;
impl crate::RegisterSpec for HashDout4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_dout_4::R`](R) reader structure"]
impl crate::Readable for HashDout4Spec {}
#[doc = "`reset()` method sets HASH_DOUT_4 to value 0"]
impl crate::Resettable for HashDout4Spec {
    const RESET_VALUE: u32 = 0;
}
