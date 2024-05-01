#[doc = "Register `SWREG58_H264_REFER24_POC` reader"]
pub type R = crate::R<Swreg58H264Refer24PocSpec>;
#[doc = "Register `SWREG58_H264_REFER24_POC` writer"]
pub type W = crate::W<Swreg58H264Refer24PocSpec>;
#[doc = "Field `SW_REFER24_POC` reader - the poc of reference picture index 24\n\nthe poc of reference picture index 24"]
pub type SwRefer24PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER24_POC` writer - the poc of reference picture index 24\n\nthe poc of reference picture index 24"]
pub type SwRefer24PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 24\n\nthe poc of reference picture index 24"]
    #[inline(always)]
    pub fn sw_refer24_poc(&self) -> SwRefer24PocR {
        SwRefer24PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 24\n\nthe poc of reference picture index 24"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer24_poc(&mut self) -> SwRefer24PocW<Swreg58H264Refer24PocSpec> {
        SwRefer24PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg58_h264_refer24_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg58_h264_refer24_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg58H264Refer24PocSpec;
impl crate::RegisterSpec for Swreg58H264Refer24PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg58_h264_refer24_poc::R`](R) reader structure"]
impl crate::Readable for Swreg58H264Refer24PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg58_h264_refer24_poc::W`](W) writer structure"]
impl crate::Writable for Swreg58H264Refer24PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG58_H264_REFER24_POC to value 0"]
impl crate::Resettable for Swreg58H264Refer24PocSpec {
    const RESET_VALUE: u32 = 0;
}
