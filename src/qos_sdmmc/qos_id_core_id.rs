#[doc = "Register `QOS_Id_CoreId` reader"]
pub type R = crate::R<QosIdCoreIdSpec>;
#[doc = "Field `CORETYPEID` reader - Field identifying the type of IP."]
pub type CoretypeidR = crate::FieldReader;
#[doc = "Field `CORECHECKSUM` reader - Field containing a checksum of the parameters of the IP."]
pub type CorechecksumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - Field identifying the type of IP."]
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
#[doc = "Core ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_id_core_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QosIdCoreIdSpec;
impl crate::RegisterSpec for QosIdCoreIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qos_id_core_id::R`](R) reader structure"]
impl crate::Readable for QosIdCoreIdSpec {}
#[doc = "`reset()` method sets QOS_Id_CoreId to value 0x0d86_7004"]
impl crate::Resettable for QosIdCoreIdSpec {
    const RESET_VALUE: u32 = 0x0d86_7004;
}
