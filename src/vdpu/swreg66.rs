#[doc = "Register `SWREG66` reader"]
pub type R = crate::R<Swreg66Spec>;
#[doc = "Field `BUILD_VER` reader - build_ver"]
pub type BuildVerR = crate::FieldReader;
#[doc = "Field `ASCII_ID_EN` reader - enable for ASCII product ID"]
pub type AsciiIdEnR = crate::BitReader;
#[doc = "Field `MINOR_NUM` reader - minor_num"]
pub type MinorNumR = crate::FieldReader;
#[doc = "Field `MAJOR_NUM` reader - major_num"]
pub type MajorNumR = crate::FieldReader;
#[doc = "Field `PROD_ID` reader - product number"]
pub type ProdIdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:2 - build_ver"]
    #[inline(always)]
    pub fn build_ver(&self) -> BuildVerR {
        BuildVerR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - enable for ASCII product ID"]
    #[inline(always)]
    pub fn ascii_id_en(&self) -> AsciiIdEnR {
        AsciiIdEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - minor_num"]
    #[inline(always)]
    pub fn minor_num(&self) -> MinorNumR {
        MinorNumR::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - major_num"]
    #[inline(always)]
    pub fn major_num(&self) -> MajorNumR {
        MajorNumR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - product number"]
    #[inline(always)]
    pub fn prod_id(&self) -> ProdIdR {
        ProdIdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg66::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg66Spec;
impl crate::RegisterSpec for Swreg66Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg66::R`](R) reader structure"]
impl crate::Readable for Swreg66Spec {}
#[doc = "`reset()` method sets SWREG66 to value 0x6731_2688"]
impl crate::Resettable for Swreg66Spec {
    const RESET_VALUE: u32 = 0x6731_2688;
}
