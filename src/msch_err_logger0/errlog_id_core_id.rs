#[doc = "Register `ERRLOG_Id_CoreId` reader"]
pub type R = crate::R<ErrlogIdCoreIdSpec>;
#[doc = "Field `CORETYPEID` reader - Field identifying the type of IP. It is the same for bothe msch_err_logger0 and msch_err_logger1."]
pub type CoretypeidR = crate::FieldReader;
#[doc = "Field `CORECHECKSUM` reader - Field containing a checksum of the parameters of the IP."]
pub type CorechecksumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - Field identifying the type of IP. It is the same for bothe msch_err_logger0 and msch_err_logger1."]
    #[inline(always)]
    pub fn coretypeid(&self) -> CoretypeidR {
        CoretypeidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Field containing a checksum of the parameters of the IP."]
    #[inline(always)]
    pub fn corechecksum(&self) -> CorechecksumR {
        CorechecksumR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "This may be different for each error logger.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_id_core_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrlogIdCoreIdSpec;
impl crate::RegisterSpec for ErrlogIdCoreIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errlog_id_core_id::R`](R) reader structure"]
impl crate::Readable for ErrlogIdCoreIdSpec {}
#[doc = "`reset()` method sets ERRLOG_Id_CoreId to value 0xbb25_140d"]
impl crate::Resettable for ErrlogIdCoreIdSpec {
    const RESET_VALUE: u32 = 0xbb25_140d;
}
