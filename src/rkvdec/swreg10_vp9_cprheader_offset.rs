#[doc = "Register `SWREG10_VP9_CPRHEADER_OFFSET` reader"]
pub type R = crate::R<Swreg10Vp9CprheaderOffsetSpec>;
#[doc = "Register `SWREG10_VP9_CPRHEADER_OFFSET` writer"]
pub type W = crate::W<Swreg10Vp9CprheaderOffsetSpec>;
#[doc = "Field `SW_VP9_CPRHEADER_OFFSET` reader - vp9 compressed header offset\n\nvp9 compressed header offset, at most 2000 probs, 10bit per prob,\n\n20000 bit at most\n\nnow is for no use, because it can read from the last syntax of the\n\nuncompressed header"]
pub type SwVp9CprheaderOffsetR = crate::FieldReader<u16>;
#[doc = "Field `SW_VP9_CPRHEADER_OFFSET` writer - vp9 compressed header offset\n\nvp9 compressed header offset, at most 2000 probs, 10bit per prob,\n\n20000 bit at most\n\nnow is for no use, because it can read from the last syntax of the\n\nuncompressed header"]
pub type SwVp9CprheaderOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - vp9 compressed header offset\n\nvp9 compressed header offset, at most 2000 probs, 10bit per prob,\n\n20000 bit at most\n\nnow is for no use, because it can read from the last syntax of the\n\nuncompressed header"]
    #[inline(always)]
    pub fn sw_vp9_cprheader_offset(&self) -> SwVp9CprheaderOffsetR {
        SwVp9CprheaderOffsetR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - vp9 compressed header offset\n\nvp9 compressed header offset, at most 2000 probs, 10bit per prob,\n\n20000 bit at most\n\nnow is for no use, because it can read from the last syntax of the\n\nuncompressed header"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_cprheader_offset(
        &mut self,
    ) -> SwVp9CprheaderOffsetW<Swreg10Vp9CprheaderOffsetSpec> {
        SwVp9CprheaderOffsetW::new(self, 0)
    }
}
#[doc = "vp9 compressed header offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg10_vp9_cprheader_offset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg10_vp9_cprheader_offset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg10Vp9CprheaderOffsetSpec;
impl crate::RegisterSpec for Swreg10Vp9CprheaderOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg10_vp9_cprheader_offset::R`](R) reader structure"]
impl crate::Readable for Swreg10Vp9CprheaderOffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg10_vp9_cprheader_offset::W`](W) writer structure"]
impl crate::Writable for Swreg10Vp9CprheaderOffsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG10_VP9_CPRHEADER_OFFSET to value 0"]
impl crate::Resettable for Swreg10Vp9CprheaderOffsetSpec {
    const RESET_VALUE: u32 = 0;
}
