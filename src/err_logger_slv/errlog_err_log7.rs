#[doc = "Register `ERRLOG_ErrLog7` reader"]
pub type R = crate::R<ErrlogErrLog7Spec>;
#[doc = "Field `ERRLOG7` reader - Contains transport protocol packet header field Security of the\n\nlogged error."]
pub type Errlog7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Contains transport protocol packet header field Security of the\n\nlogged error."]
    #[inline(always)]
    pub fn errlog7(&self) -> Errlog7R {
        Errlog7R::new((self.bits & 1) != 0)
    }
}
#[doc = "Securrity flag in transport protocol header\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_err_log7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrlogErrLog7Spec;
impl crate::RegisterSpec for ErrlogErrLog7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errlog_err_log7::R`](R) reader structure"]
impl crate::Readable for ErrlogErrLog7Spec {}
#[doc = "`reset()` method sets ERRLOG_ErrLog7 to value 0"]
impl crate::Resettable for ErrlogErrLog7Spec {
    const RESET_VALUE: u32 = 0;
}
