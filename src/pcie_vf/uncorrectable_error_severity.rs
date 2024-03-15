#[doc = "Register `UNCORRECTABLE_ERROR_SEVERITY` reader"]
pub type R = crate::R<UncorrectableErrorSeveritySpec>;
#[doc = "Field `R8` reader - Reserved \\[R8\\]
(no description)"]
pub type R8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved \\[R8\\]
(no description)"]
    #[inline(always)]
    pub fn r8(&self) -> R8R {
        R8R::new(self.bits)
    }
}
#[doc = "Uncorrectable Error Severity Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uncorrectable_error_severity::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UncorrectableErrorSeveritySpec;
impl crate::RegisterSpec for UncorrectableErrorSeveritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uncorrectable_error_severity::R`](R) reader structure"]
impl crate::Readable for UncorrectableErrorSeveritySpec {}
#[doc = "`reset()` method sets UNCORRECTABLE_ERROR_SEVERITY to value 0"]
impl crate::Resettable for UncorrectableErrorSeveritySpec {
    const RESET_VALUE: u32 = 0;
}
