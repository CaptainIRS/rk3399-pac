#[doc = "Register `SWREG50_H264_REFER16_POC` reader"]
pub type R = crate::R<Swreg50H264Refer16PocSpec>;
#[doc = "Register `SWREG50_H264_REFER16_POC` writer"]
pub type W = crate::W<Swreg50H264Refer16PocSpec>;
#[doc = "Field `SW_REFER16_POC` reader - the poc of reference picture index 16\n\nthe poc of reference picture index 16"]
pub type SwRefer16PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER16_POC` writer - the poc of reference picture index 16\n\nthe poc of reference picture index 16"]
pub type SwRefer16PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 16\n\nthe poc of reference picture index 16"]
    #[inline(always)]
    pub fn sw_refer16_poc(&self) -> SwRefer16PocR {
        SwRefer16PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 16\n\nthe poc of reference picture index 16"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer16_poc(&mut self) -> SwRefer16PocW<Swreg50H264Refer16PocSpec> {
        SwRefer16PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg50_h264_refer16_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg50_h264_refer16_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg50H264Refer16PocSpec;
impl crate::RegisterSpec for Swreg50H264Refer16PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg50_h264_refer16_poc::R`](R) reader structure"]
impl crate::Readable for Swreg50H264Refer16PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg50_h264_refer16_poc::W`](W) writer structure"]
impl crate::Writable for Swreg50H264Refer16PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG50_H264_REFER16_POC to value 0"]
impl crate::Resettable for Swreg50H264Refer16PocSpec {
    const RESET_VALUE: u32 = 0;
}
