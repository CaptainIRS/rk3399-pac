#[doc = "Register `SWREG61_H264_REFER27_POC` reader"]
pub type R = crate::R<Swreg61H264Refer27PocSpec>;
#[doc = "Register `SWREG61_H264_REFER27_POC` writer"]
pub type W = crate::W<Swreg61H264Refer27PocSpec>;
#[doc = "Field `SW_REFER27_POC` reader - the poc of reference picture index 27\n\nthe poc of reference picture index 27"]
pub type SwRefer27PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER27_POC` writer - the poc of reference picture index 27\n\nthe poc of reference picture index 27"]
pub type SwRefer27PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 27\n\nthe poc of reference picture index 27"]
    #[inline(always)]
    pub fn sw_refer27_poc(&self) -> SwRefer27PocR {
        SwRefer27PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 27\n\nthe poc of reference picture index 27"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer27_poc(&mut self) -> SwRefer27PocW<Swreg61H264Refer27PocSpec> {
        SwRefer27PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg61_h264_refer27_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg61_h264_refer27_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg61H264Refer27PocSpec;
impl crate::RegisterSpec for Swreg61H264Refer27PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg61_h264_refer27_poc::R`](R) reader structure"]
impl crate::Readable for Swreg61H264Refer27PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg61_h264_refer27_poc::W`](W) writer structure"]
impl crate::Writable for Swreg61H264Refer27PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG61_H264_REFER27_POC to value 0"]
impl crate::Resettable for Swreg61H264Refer27PocSpec {
    const RESET_VALUE: u32 = 0;
}
