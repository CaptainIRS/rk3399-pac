#[doc = "Register `SWREG54_H264_REFER20_POC` reader"]
pub type R = crate::R<Swreg54H264Refer20PocSpec>;
#[doc = "Register `SWREG54_H264_REFER20_POC` writer"]
pub type W = crate::W<Swreg54H264Refer20PocSpec>;
#[doc = "Field `SW_REFER20_POC` reader - the poc of reference picture index 20\n\nthe poc of reference picture index 20"]
pub type SwRefer20PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER20_POC` writer - the poc of reference picture index 20\n\nthe poc of reference picture index 20"]
pub type SwRefer20PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 20\n\nthe poc of reference picture index 20"]
    #[inline(always)]
    pub fn sw_refer20_poc(&self) -> SwRefer20PocR {
        SwRefer20PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 20\n\nthe poc of reference picture index 20"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer20_poc(&mut self) -> SwRefer20PocW<Swreg54H264Refer20PocSpec> {
        SwRefer20PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg54_h264_refer20_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg54_h264_refer20_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg54H264Refer20PocSpec;
impl crate::RegisterSpec for Swreg54H264Refer20PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg54_h264_refer20_poc::R`](R) reader structure"]
impl crate::Readable for Swreg54H264Refer20PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg54_h264_refer20_poc::W`](W) writer structure"]
impl crate::Writable for Swreg54H264Refer20PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG54_H264_REFER20_POC to value 0"]
impl crate::Resettable for Swreg54H264Refer20PocSpec {
    const RESET_VALUE: u32 = 0;
}
