#[doc = "Register `SWREG72_H264_REFER30_POC` reader"]
pub type R = crate::R<Swreg72H264Refer30PocSpec>;
#[doc = "Register `SWREG72_H264_REFER30_POC` writer"]
pub type W = crate::W<Swreg72H264Refer30PocSpec>;
#[doc = "Field `SW_REFER30_POC` reader - the poc of reference picture index 30\n\nthe poc of reference picture index 30"]
pub type SwRefer30PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER30_POC` writer - the poc of reference picture index 30\n\nthe poc of reference picture index 30"]
pub type SwRefer30PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 30\n\nthe poc of reference picture index 30"]
    #[inline(always)]
    pub fn sw_refer30_poc(&self) -> SwRefer30PocR {
        SwRefer30PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 30\n\nthe poc of reference picture index 30"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer30_poc(&mut self) -> SwRefer30PocW<Swreg72H264Refer30PocSpec> {
        SwRefer30PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg72_h264_refer30_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg72_h264_refer30_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg72H264Refer30PocSpec;
impl crate::RegisterSpec for Swreg72H264Refer30PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg72_h264_refer30_poc::R`](R) reader structure"]
impl crate::Readable for Swreg72H264Refer30PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg72_h264_refer30_poc::W`](W) writer structure"]
impl crate::Writable for Swreg72H264Refer30PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG72_H264_REFER30_POC to value 0"]
impl crate::Resettable for Swreg72H264Refer30PocSpec {
    const RESET_VALUE: u32 = 0;
}
