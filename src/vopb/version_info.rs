#[doc = "Register `VERSION_INFO` reader"]
pub type R = crate::R<VersionInfoSpec>;
#[doc = "Field `SVNBUILD` reader - rtl current svn number"]
pub type SvnbuildR = crate::FieldReader<u16>;
#[doc = "Field `MINOR` reader - minor vertion\n\nbig feature change under same structure"]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MAJOR` reader - IP major vertion\n\nused for IP structure"]
pub type MajorR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - rtl current svn number"]
    #[inline(always)]
    pub fn svnbuild(&self) -> SvnbuildR {
        SvnbuildR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - minor vertion\n\nbig feature change under same structure"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - IP major vertion\n\nused for IP structure"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Version for vop\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version_info::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VersionInfoSpec;
impl crate::RegisterSpec for VersionInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version_info::R`](R) reader structure"]
impl crate::Readable for VersionInfoSpec {}
#[doc = "`reset()` method sets VERSION_INFO to value 0"]
impl crate::Resettable for VersionInfoSpec {
    const RESET_VALUE: u32 = 0;
}
