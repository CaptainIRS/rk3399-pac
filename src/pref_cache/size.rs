#[doc = "Register `SIZE` reader"]
pub type R = crate::R<SizeSpec>;
#[doc = "Field `LINE_SIZE` reader - Log2 line size in bytes"]
pub type LineSizeR = crate::FieldReader;
#[doc = "Field `ASSOCIATIVITY` reader - Log2 associativity"]
pub type AssociativityR = crate::FieldReader;
#[doc = "Field `CACHE_SIZE` reader - Log2 cache size in bytes"]
pub type CacheSizeR = crate::FieldReader;
#[doc = "Field `EXTERNAL_BUS_WIDTH` reader - Log2 external bus width in bits"]
pub type ExternalBusWidthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Log2 line size in bytes"]
    #[inline(always)]
    pub fn line_size(&self) -> LineSizeR {
        LineSizeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Log2 associativity"]
    #[inline(always)]
    pub fn associativity(&self) -> AssociativityR {
        AssociativityR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Log2 cache size in bytes"]
    #[inline(always)]
    pub fn cache_size(&self) -> CacheSizeR {
        CacheSizeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Log2 external bus width in bits"]
    #[inline(always)]
    pub fn external_bus_width(&self) -> ExternalBusWidthR {
        ExternalBusWidthR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "L2 cache SIZE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`size::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SizeSpec;
impl crate::RegisterSpec for SizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`size::R`](R) reader structure"]
impl crate::Readable for SizeSpec {}
#[doc = "`reset()` method sets SIZE to value 0x0711_0206"]
impl crate::Resettable for SizeSpec {
    const RESET_VALUE: u32 = 0x0711_0206;
}
