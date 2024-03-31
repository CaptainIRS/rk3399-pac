#[doc = "Register `INT_RAWSTAT` reader"]
pub type R = crate::R<IntRawstatSpec>;
#[doc = "Field `PAGE_FAULT` reader - page fault"]
pub type PageFaultR = crate::BitReader;
#[doc = "Field `READ_BUS_ERROR` reader - read bus error"]
pub type ReadBusErrorR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - page fault"]
    #[inline(always)]
    pub fn page_fault(&self) -> PageFaultR {
        PageFaultR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - read bus error"]
    #[inline(always)]
    pub fn read_bus_error(&self) -> ReadBusErrorR {
        ReadBusErrorR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "MMU raw interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_rawstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntRawstatSpec;
impl crate::RegisterSpec for IntRawstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_rawstat::R`](R) reader structure"]
impl crate::Readable for IntRawstatSpec {}
#[doc = "`reset()` method sets INT_RAWSTAT to value 0"]
impl crate::Resettable for IntRawstatSpec {
    const RESET_VALUE: u32 = 0;
}
