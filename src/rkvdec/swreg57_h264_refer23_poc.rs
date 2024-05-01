#[doc = "Register `SWREG57_H264_REFER23_POC` reader"]
pub type R = crate::R<Swreg57H264Refer23PocSpec>;
#[doc = "Register `SWREG57_H264_REFER23_POC` writer"]
pub type W = crate::W<Swreg57H264Refer23PocSpec>;
#[doc = "Field `SW_REFER23_POC` reader - the poc of reference picture index 23\n\nthe poc of reference picture index 23"]
pub type SwRefer23PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER23_POC` writer - the poc of reference picture index 23\n\nthe poc of reference picture index 23"]
pub type SwRefer23PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 23\n\nthe poc of reference picture index 23"]
    #[inline(always)]
    pub fn sw_refer23_poc(&self) -> SwRefer23PocR {
        SwRefer23PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 23\n\nthe poc of reference picture index 23"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer23_poc(&mut self) -> SwRefer23PocW<Swreg57H264Refer23PocSpec> {
        SwRefer23PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg57_h264_refer23_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg57_h264_refer23_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg57H264Refer23PocSpec;
impl crate::RegisterSpec for Swreg57H264Refer23PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg57_h264_refer23_poc::R`](R) reader structure"]
impl crate::Readable for Swreg57H264Refer23PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg57_h264_refer23_poc::W`](W) writer structure"]
impl crate::Writable for Swreg57H264Refer23PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG57_H264_REFER23_POC to value 0"]
impl crate::Resettable for Swreg57H264Refer23PocSpec {
    const RESET_VALUE: u32 = 0;
}
