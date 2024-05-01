#[doc = "Register `SWREG_110_READ` reader"]
pub type R = crate::R<Swreg110ReadSpec>;
#[doc = "Field `SYNTHESIS` reader - synthesis"]
pub type SynthesisR = crate::FieldReader;
#[doc = "Field `MINOR_NUM` reader - Minor number"]
pub type MinorNumR = crate::FieldReader;
#[doc = "Field `MAJOR_NUM` reader - Major number"]
pub type MajorNumR = crate::FieldReader;
#[doc = "Field `PROD_ID` reader - Product ID"]
pub type ProdIdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - synthesis"]
    #[inline(always)]
    pub fn synthesis(&self) -> SynthesisR {
        SynthesisR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - Minor number"]
    #[inline(always)]
    pub fn minor_num(&self) -> MinorNumR {
        MinorNumR::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - Major number"]
    #[inline(always)]
    pub fn major_num(&self) -> MajorNumR {
        MajorNumR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Product ID"]
    #[inline(always)]
    pub fn prod_id(&self) -> ProdIdR {
        ProdIdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "product ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_110_read::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg110ReadSpec;
impl crate::RegisterSpec for Swreg110ReadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_110_read::R`](R) reader structure"]
impl crate::Readable for Swreg110ReadSpec {}
#[doc = "`reset()` method sets SWREG_110_READ to value 0x4831_1220"]
impl crate::Resettable for Swreg110ReadSpec {
    const RESET_VALUE: u32 = 0x4831_1220;
}
