#[doc = "Register `SWREG55_H264_REFER21_POC` reader"]
pub type R = crate::R<Swreg55H264Refer21PocSpec>;
#[doc = "Register `SWREG55_H264_REFER21_POC` writer"]
pub type W = crate::W<Swreg55H264Refer21PocSpec>;
#[doc = "Field `SW_REFER21_POC` reader - the poc of reference picture index 21\n\nthe poc of reference picture index 21"]
pub type SwRefer21PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER21_POC` writer - the poc of reference picture index 21\n\nthe poc of reference picture index 21"]
pub type SwRefer21PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 21\n\nthe poc of reference picture index 21"]
    #[inline(always)]
    pub fn sw_refer21_poc(&self) -> SwRefer21PocR {
        SwRefer21PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 21\n\nthe poc of reference picture index 21"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer21_poc(&mut self) -> SwRefer21PocW<Swreg55H264Refer21PocSpec> {
        SwRefer21PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg55_h264_refer21_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg55_h264_refer21_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg55H264Refer21PocSpec;
impl crate::RegisterSpec for Swreg55H264Refer21PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg55_h264_refer21_poc::R`](R) reader structure"]
impl crate::Readable for Swreg55H264Refer21PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg55_h264_refer21_poc::W`](W) writer structure"]
impl crate::Writable for Swreg55H264Refer21PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG55_H264_REFER21_POC to value 0"]
impl crate::Resettable for Swreg55H264Refer21PocSpec {
    const RESET_VALUE: u32 = 0;
}
