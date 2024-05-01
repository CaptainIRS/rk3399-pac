#[doc = "Register `VERSION_INFO` reader"]
pub type R = crate::R<VersionInfoSpec>;
#[doc = "Register `VERSION_INFO` writer"]
pub type W = crate::W<VersionInfoSpec>;
#[doc = "Field `SVNBUILD` reader - rtl current svn number"]
pub type SvnbuildR = crate::FieldReader<u32>;
#[doc = "Field `SVNBUILD` writer - rtl current svn number"]
pub type SvnbuildW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `MINOR` reader - minor vertion\n\nbig feature change under same structure"]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MINOR` writer - minor vertion\n\nbig feature change under same structure"]
pub type MinorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAJOR` reader - IP major vertion\n\nused for IP structure version infomation"]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MAJOR` writer - IP major vertion\n\nused for IP structure version infomation"]
pub type MajorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:19 - rtl current svn number"]
    #[inline(always)]
    pub fn svnbuild(&self) -> SvnbuildR {
        SvnbuildR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:27 - minor vertion\n\nbig feature change under same structure"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - IP major vertion\n\nused for IP structure version infomation"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:19 - rtl current svn number"]
    #[inline(always)]
    #[must_use]
    pub fn svnbuild(&mut self) -> SvnbuildW<VersionInfoSpec> {
        SvnbuildW::new(self, 0)
    }
    #[doc = "Bits 20:27 - minor vertion\n\nbig feature change under same structure"]
    #[inline(always)]
    #[must_use]
    pub fn minor(&mut self) -> MinorW<VersionInfoSpec> {
        MinorW::new(self, 20)
    }
    #[doc = "Bits 28:31 - IP major vertion\n\nused for IP structure version infomation"]
    #[inline(always)]
    #[must_use]
    pub fn major(&mut self) -> MajorW<VersionInfoSpec> {
        MajorW::new(self, 28)
    }
}
#[doc = "Version number for iep\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VersionInfoSpec;
impl crate::RegisterSpec for VersionInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version_info::R`](R) reader structure"]
impl crate::Readable for VersionInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`version_info::W`](W) writer structure"]
impl crate::Writable for VersionInfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VERSION_INFO to value 0"]
impl crate::Resettable for VersionInfoSpec {
    const RESET_VALUE: u32 = 0;
}
