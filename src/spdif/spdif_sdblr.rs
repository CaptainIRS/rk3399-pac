#[doc = "Register `SPDIF_SDBLR` reader"]
pub type R = crate::R<SpdifSdblrSpec>;
#[doc = "Register `SPDIF_SDBLR` writer"]
pub type W = crate::W<SpdifSdblrSpec>;
#[doc = "Field `SDBLR` reader - Sample Date Buffer Level Register\n\nContains the number of valid data entries in the sample data\n\nbuffer."]
pub type SdblrR = crate::FieldReader;
#[doc = "Field `SDBLR` writer - Sample Date Buffer Level Register\n\nContains the number of valid data entries in the sample data\n\nbuffer."]
pub type SdblrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Sample Date Buffer Level Register\n\nContains the number of valid data entries in the sample data\n\nbuffer."]
    #[inline(always)]
    pub fn sdblr(&self) -> SdblrR {
        SdblrR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Sample Date Buffer Level Register\n\nContains the number of valid data entries in the sample data\n\nbuffer."]
    #[inline(always)]
    #[must_use]
    pub fn sdblr(&mut self) -> SdblrW<SpdifSdblrSpec> {
        SdblrW::new(self, 0)
    }
}
#[doc = "Sample Date Buffer Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_sdblr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_sdblr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdifSdblrSpec;
impl crate::RegisterSpec for SpdifSdblrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdif_sdblr::R`](R) reader structure"]
impl crate::Readable for SpdifSdblrSpec {}
#[doc = "`write(|w| ..)` method takes [`spdif_sdblr::W`](W) writer structure"]
impl crate::Writable for SpdifSdblrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPDIF_SDBLR to value 0"]
impl crate::Resettable for SpdifSdblrSpec {
    const RESET_VALUE: u32 = 0;
}
