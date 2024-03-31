#[doc = "Register `ErrLog1` reader"]
pub type R = crate::R<ErrLog1Spec>;
#[doc = "Field `ERRLOG1` reader - Contains transport protocol packet header field RouteID of the\n\nlogged error. Unused bits are read as 0."]
pub type Errlog1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - Contains transport protocol packet header field RouteID of the\n\nlogged error. Unused bits are read as 0."]
    #[inline(always)]
    pub fn errlog1(&self) -> Errlog1R {
        Errlog1R::new(self.bits & 0x003f_ffff)
    }
}
#[doc = "Route ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_log1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrLog1Spec;
impl crate::RegisterSpec for ErrLog1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_log1::R`](R) reader structure"]
impl crate::Readable for ErrLog1Spec {}
#[doc = "`reset()` method sets ErrLog1 to value 0"]
impl crate::Resettable for ErrLog1Spec {
    const RESET_VALUE: u32 = 0;
}
