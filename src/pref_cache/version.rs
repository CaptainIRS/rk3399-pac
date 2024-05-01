#[doc = "Register `VERSION` reader"]
pub type R = crate::R<VersionSpec>;
#[doc = "Field `VERSION_MINOR` reader - VERSION_MINOR"]
pub type VersionMinorR = crate::FieldReader;
#[doc = "Field `VERSION_MAJOR` reader - VERSION_MAJOR"]
pub type VersionMajorR = crate::FieldReader;
#[doc = "Field `PRODUCT_ID` reader - PRODUCT_ID"]
pub type ProductIdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - VERSION_MINOR"]
    #[inline(always)]
    pub fn version_minor(&self) -> VersionMinorR {
        VersionMinorR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - VERSION_MAJOR"]
    #[inline(always)]
    pub fn version_major(&self) -> VersionMajorR {
        VersionMajorR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - PRODUCT_ID"]
    #[inline(always)]
    pub fn product_id(&self) -> ProductIdR {
        ProductIdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "VERSION register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VersionSpec;
impl crate::RegisterSpec for VersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version::R`](R) reader structure"]
impl crate::Readable for VersionSpec {}
#[doc = "`reset()` method sets VERSION to value 0xcac2_0101"]
impl crate::Resettable for VersionSpec {
    const RESET_VALUE: u32 = 0xcac2_0101;
}
