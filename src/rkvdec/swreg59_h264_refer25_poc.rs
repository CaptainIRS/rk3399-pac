#[doc = "Register `SWREG59_H264_REFER25_POC` reader"]
pub type R = crate::R<Swreg59H264Refer25PocSpec>;
#[doc = "Register `SWREG59_H264_REFER25_POC` writer"]
pub type W = crate::W<Swreg59H264Refer25PocSpec>;
#[doc = "Field `SW_REFER25_POC` reader - the poc of reference picture index 25\n\nthe poc of reference picture index 25"]
pub type SwRefer25PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER25_POC` writer - the poc of reference picture index 25\n\nthe poc of reference picture index 25"]
pub type SwRefer25PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 25\n\nthe poc of reference picture index 25"]
    #[inline(always)]
    pub fn sw_refer25_poc(&self) -> SwRefer25PocR {
        SwRefer25PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 25\n\nthe poc of reference picture index 25"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer25_poc(&mut self) -> SwRefer25PocW<Swreg59H264Refer25PocSpec> {
        SwRefer25PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg59_h264_refer25_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg59_h264_refer25_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg59H264Refer25PocSpec;
impl crate::RegisterSpec for Swreg59H264Refer25PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg59_h264_refer25_poc::R`](R) reader structure"]
impl crate::Readable for Swreg59H264Refer25PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg59_h264_refer25_poc::W`](W) writer structure"]
impl crate::Writable for Swreg59H264Refer25PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG59_H264_REFER25_POC to value 0"]
impl crate::Resettable for Swreg59H264Refer25PocSpec {
    const RESET_VALUE: u32 = 0;
}
