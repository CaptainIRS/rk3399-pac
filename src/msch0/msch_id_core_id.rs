#[doc = "Register `MSCH_Id_CoreId` reader"]
pub type R = crate::R<MschIdCoreIdSpec>;
#[doc = "Field `CORETYPEID` reader - Field identifying the type of IP."]
pub type CoretypeidR = crate::FieldReader;
#[doc = "Field `CORECHECKSUM` reader - Field containing a checksum of the parameters of the IP. For memory schedule 0 , this value is 0x00dc1b For memory schedule 1 , this value is 0xc2f11d"]
pub type CorechecksumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - Field identifying the type of IP."]
    #[inline(always)]
    pub fn coretypeid(&self) -> CoretypeidR {
        CoretypeidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Field containing a checksum of the parameters of the IP. For memory schedule 0 , this value is 0x00dc1b For memory schedule 1 , this value is 0xc2f11d"]
    #[inline(always)]
    pub fn corechecksum(&self) -> CorechecksumR {
        CorechecksumR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "Core ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msch_id_core_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MschIdCoreIdSpec;
impl crate::RegisterSpec for MschIdCoreIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msch_id_core_id::R`](R) reader structure"]
impl crate::Readable for MschIdCoreIdSpec {}
#[doc = "`reset()` method sets MSCH_Id_CoreId to value 0x0d86_7004"]
impl crate::Resettable for MschIdCoreIdSpec {
    const RESET_VALUE: u32 = 0x0d86_7004;
}
