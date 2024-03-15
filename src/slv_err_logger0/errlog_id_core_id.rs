#[doc = "Register `ERRLOG_Id_CoreId` reader"]
pub type R = crate::R<ErrlogIdCoreIdSpec>;
#[doc = "Field `CORETYPEID` reader - Field identifying the type of IP. It is the same for both slv_err_logger0 and slv_err_logger1."]
pub type CoretypeidR = crate::FieldReader;
#[doc = "Field `CORECHECKSUM` reader - Field containing a checksum of the parameters of the IP. For slv_err_logger0, this filed's value is always 0x43F8FA. For slv_err_logger1, this filed's value is always 0xB5413E."]
pub type CorechecksumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - Field identifying the type of IP. It is the same for both slv_err_logger0 and slv_err_logger1."]
    #[inline(always)]
    pub fn coretypeid(&self) -> CoretypeidR {
        CoretypeidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Field containing a checksum of the parameters of the IP. For slv_err_logger0, this filed's value is always 0x43F8FA. For slv_err_logger1, this filed's value is always 0xB5413E."]
    #[inline(always)]
    pub fn corechecksum(&self) -> CorechecksumR {
        CorechecksumR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "Contain CoreTypeId and CoreChecksum\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_id_core_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrlogIdCoreIdSpec;
impl crate::RegisterSpec for ErrlogIdCoreIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errlog_id_core_id::R`](R) reader structure"]
impl crate::Readable for ErrlogIdCoreIdSpec {}
#[doc = "`reset()` method sets ERRLOG_Id_CoreId to value 0x0d"]
impl crate::Resettable for ErrlogIdCoreIdSpec {
    const RESET_VALUE: u32 = 0x0d;
}
