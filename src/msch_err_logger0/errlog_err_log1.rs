#[doc = "Register `ERRLOG_ErrLog1` reader"]
pub type R = crate::R<ErrlogErrLog1Spec>;
#[doc = "Field `ERRLOG1` reader - Contains transport protocol packet header field RouteID of the logged error. Unused bits are read as 0."]
pub type Errlog1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - Contains transport protocol packet header field RouteID of the logged error. Unused bits are read as 0."]
    #[inline(always)]
    pub fn errlog1(&self) -> Errlog1R {
        Errlog1R::new(self.bits & 0x003f_ffff)
    }
}
#[doc = "Route ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_err_log1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrlogErrLog1Spec;
impl crate::RegisterSpec for ErrlogErrLog1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errlog_err_log1::R`](R) reader structure"]
impl crate::Readable for ErrlogErrLog1Spec {}
#[doc = "`reset()` method sets ERRLOG_ErrLog1 to value 0"]
impl crate::Resettable for ErrlogErrLog1Spec {
    const RESET_VALUE: u32 = 0;
}
