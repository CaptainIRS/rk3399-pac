#[doc = "Register `SWREG60_H264_REFER26_POC` reader"]
pub type R = crate::R<Swreg60H264Refer26PocSpec>;
#[doc = "Register `SWREG60_H264_REFER26_POC` writer"]
pub type W = crate::W<Swreg60H264Refer26PocSpec>;
#[doc = "Field `SW_REFER26_POC` reader - the poc of reference picture index 26\n\nthe poc of reference picture index 26"]
pub type SwRefer26PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER26_POC` writer - the poc of reference picture index 26\n\nthe poc of reference picture index 26"]
pub type SwRefer26PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 26\n\nthe poc of reference picture index 26"]
    #[inline(always)]
    pub fn sw_refer26_poc(&self) -> SwRefer26PocR {
        SwRefer26PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 26\n\nthe poc of reference picture index 26"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer26_poc(&mut self) -> SwRefer26PocW<Swreg60H264Refer26PocSpec> {
        SwRefer26PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg60_h264_refer26_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg60_h264_refer26_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg60H264Refer26PocSpec;
impl crate::RegisterSpec for Swreg60H264Refer26PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg60_h264_refer26_poc::R`](R) reader structure"]
impl crate::Readable for Swreg60H264Refer26PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg60_h264_refer26_poc::W`](W) writer structure"]
impl crate::Writable for Swreg60H264Refer26PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG60_H264_REFER26_POC to value 0"]
impl crate::Resettable for Swreg60H264Refer26PocSpec {
    const RESET_VALUE: u32 = 0;
}
