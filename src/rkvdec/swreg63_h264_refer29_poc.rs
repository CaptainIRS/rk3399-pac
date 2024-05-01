#[doc = "Register `SWREG63_H264_REFER29_POC` reader"]
pub type R = crate::R<Swreg63H264Refer29PocSpec>;
#[doc = "Register `SWREG63_H264_REFER29_POC` writer"]
pub type W = crate::W<Swreg63H264Refer29PocSpec>;
#[doc = "Field `SW_REFER29_POC` reader - the poc of reference picture index 29\n\nthe poc of reference picture index 29"]
pub type SwRefer29PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER29_POC` writer - the poc of reference picture index 29\n\nthe poc of reference picture index 29"]
pub type SwRefer29PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 29\n\nthe poc of reference picture index 29"]
    #[inline(always)]
    pub fn sw_refer29_poc(&self) -> SwRefer29PocR {
        SwRefer29PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 29\n\nthe poc of reference picture index 29"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer29_poc(&mut self) -> SwRefer29PocW<Swreg63H264Refer29PocSpec> {
        SwRefer29PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg63_h264_refer29_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg63_h264_refer29_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg63H264Refer29PocSpec;
impl crate::RegisterSpec for Swreg63H264Refer29PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg63_h264_refer29_poc::R`](R) reader structure"]
impl crate::Readable for Swreg63H264Refer29PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg63_h264_refer29_poc::W`](W) writer structure"]
impl crate::Writable for Swreg63H264Refer29PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG63_H264_REFER29_POC to value 0"]
impl crate::Resettable for Swreg63H264Refer29PocSpec {
    const RESET_VALUE: u32 = 0;
}
