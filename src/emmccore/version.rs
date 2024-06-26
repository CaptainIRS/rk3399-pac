#[doc = "Register `VERSION` reader"]
pub type R = crate::R<VersionSpec>;
#[doc = "Field `SPECIFICATIONVERSION` reader - The Host Controller Version Number is set to 0x02 (SD Host\n\nSpecification Version 3.00)."]
pub type SpecificationversionR = crate::FieldReader;
#[doc = "Field `VENDORVERSION` reader - The Vendor Version Number is set to 0x10 (1.0)"]
pub type VendorversionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The Host Controller Version Number is set to 0x02 (SD Host\n\nSpecification Version 3.00)."]
    #[inline(always)]
    pub fn specificationversion(&self) -> SpecificationversionR {
        SpecificationversionR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The Vendor Version Number is set to 0x10 (1.0)"]
    #[inline(always)]
    pub fn vendorversion(&self) -> VendorversionR {
        VendorversionR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Host controller version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VersionSpec;
impl crate::RegisterSpec for VersionSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`version::R`](R) reader structure"]
impl crate::Readable for VersionSpec {}
#[doc = "`reset()` method sets VERSION to value 0x1002"]
impl crate::Resettable for VersionSpec {
    const RESET_VALUE: u16 = 0x1002;
}
