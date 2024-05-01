#[doc = "Register `SWREG49_H264_REFER15_POC` reader"]
pub type R = crate::R<Swreg49H264Refer15PocSpec>;
#[doc = "Register `SWREG49_H264_REFER15_POC` writer"]
pub type W = crate::W<Swreg49H264Refer15PocSpec>;
#[doc = "Field `SW_REFER15_POC` reader - the poc of reference picture index 15\n\nthe poc of reference picture index 15"]
pub type SwRefer15PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER15_POC` writer - the poc of reference picture index 15\n\nthe poc of reference picture index 15"]
pub type SwRefer15PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 15\n\nthe poc of reference picture index 15"]
    #[inline(always)]
    pub fn sw_refer15_poc(&self) -> SwRefer15PocR {
        SwRefer15PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 15\n\nthe poc of reference picture index 15"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer15_poc(&mut self) -> SwRefer15PocW<Swreg49H264Refer15PocSpec> {
        SwRefer15PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg49_h264_refer15_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg49_h264_refer15_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg49H264Refer15PocSpec;
impl crate::RegisterSpec for Swreg49H264Refer15PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg49_h264_refer15_poc::R`](R) reader structure"]
impl crate::Readable for Swreg49H264Refer15PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg49_h264_refer15_poc::W`](W) writer structure"]
impl crate::Writable for Swreg49H264Refer15PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG49_H264_REFER15_POC to value 0"]
impl crate::Resettable for Swreg49H264Refer15PocSpec {
    const RESET_VALUE: u32 = 0;
}
