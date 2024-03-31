#[doc = "Register `ErrLog3` reader"]
pub type R = crate::R<ErrLog3Spec>;
#[doc = "Field `ERRLOG3` reader - Contains transport protocol packet header field Addr of the logged\n\nerror. Unused bits are read as 0."]
pub type Errlog3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Contains transport protocol packet header field Addr of the logged\n\nerror. Unused bits are read as 0."]
    #[inline(always)]
    pub fn errlog3(&self) -> Errlog3R {
        Errlog3R::new(self.bits)
    }
}
#[doc = "Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_log3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrLog3Spec;
impl crate::RegisterSpec for ErrLog3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_log3::R`](R) reader structure"]
impl crate::Readable for ErrLog3Spec {}
#[doc = "`reset()` method sets ErrLog3 to value 0"]
impl crate::Resettable for ErrLog3Spec {
    const RESET_VALUE: u32 = 0;
}
