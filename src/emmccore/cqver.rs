#[doc = "Register `CQVER` reader"]
pub type R = crate::R<CqverSpec>;
#[doc = "Field `SUFFIX` reader - eMMC Version Suffix (2nd digit right of decimal point), in BCD\n\nformat"]
pub type SuffixR = crate::FieldReader;
#[doc = "Field `MINOR` reader - eMMC Minor Version Number(digit right of decimal point), in BCD\n\nformat"]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MAJOR` reader - eMMC Major Version Number (digit left of decimal point), in BCD\n\nformat"]
pub type MajorR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - eMMC Version Suffix (2nd digit right of decimal point), in BCD\n\nformat"]
    #[inline(always)]
    pub fn suffix(&self) -> SuffixR {
        SuffixR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - eMMC Minor Version Number(digit right of decimal point), in BCD\n\nformat"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - eMMC Major Version Number (digit left of decimal point), in BCD\n\nformat"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "Command queueing version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqver::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqverSpec;
impl crate::RegisterSpec for CqverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqver::R`](R) reader structure"]
impl crate::Readable for CqverSpec {}
#[doc = "`reset()` method sets CQVER to value 0x0510"]
impl crate::Resettable for CqverSpec {
    const RESET_VALUE: u32 = 0x0510;
}
