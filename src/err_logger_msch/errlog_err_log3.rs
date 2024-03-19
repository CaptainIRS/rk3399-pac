#[doc = "Register `ERRLOG_ErrLog3` reader"]
pub type R = crate::R<ErrlogErrLog3Spec>;
#[doc = "Field `ERRLOG3` reader - Contains up to 32 LSBs of transport protocol packet header field\n\nAddr of the logged error. Unused bits are read as 0."]
pub type Errlog3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Contains up to 32 LSBs of transport protocol packet header field\n\nAddr of the logged error. Unused bits are read as 0."]
    #[inline(always)]
    pub fn errlog3(&self) -> Errlog3R {
        Errlog3R::new(self.bits)
    }
}
#[doc = "Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_err_log3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrlogErrLog3Spec;
impl crate::RegisterSpec for ErrlogErrLog3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errlog_err_log3::R`](R) reader structure"]
impl crate::Readable for ErrlogErrLog3Spec {}
#[doc = "`reset()` method sets ERRLOG_ErrLog3 to value 0"]
impl crate::Resettable for ErrlogErrLog3Spec {
    const RESET_VALUE: u32 = 0;
}
