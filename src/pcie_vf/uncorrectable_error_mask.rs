#[doc = "Register `UNCORRECTABLE_ERROR_MASK` reader"]
pub type R = crate::R<UncorrectableErrorMaskSpec>;
#[doc = "Field `R4` reader - Reserved \\[R4\\]
(no description)"]
pub type R4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved \\[R4\\]
(no description)"]
    #[inline(always)]
    pub fn r4(&self) -> R4R {
        R4R::new(self.bits)
    }
}
#[doc = "Uncorrectable Error Mask Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uncorrectable_error_mask::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UncorrectableErrorMaskSpec;
impl crate::RegisterSpec for UncorrectableErrorMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uncorrectable_error_mask::R`](R) reader structure"]
impl crate::Readable for UncorrectableErrorMaskSpec {}
#[doc = "`reset()` method sets UNCORRECTABLE_ERROR_MASK to value 0"]
impl crate::Resettable for UncorrectableErrorMaskSpec {
    const RESET_VALUE: u32 = 0;
}
