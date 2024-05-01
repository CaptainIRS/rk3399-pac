#[doc = "Register `SWREG75_H264_ERRORINFO_BASE` reader"]
pub type R = crate::R<Swreg75H264ErrorinfoBaseSpec>;
#[doc = "Register `SWREG75_H264_ERRORINFO_BASE` writer"]
pub type W = crate::W<Swreg75H264ErrorinfoBaseSpec>;
#[doc = "Field `SW_ERRORINFO_BASE` reader - error info base addr\n\nerror info base addr\n\nevery slice contains 256 bits error info"]
pub type SwErrorinfoBaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_ERRORINFO_BASE` writer - error info base addr\n\nerror info base addr\n\nevery slice contains 256 bits error info"]
pub type SwErrorinfoBaseW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 3:31 - error info base addr\n\nerror info base addr\n\nevery slice contains 256 bits error info"]
    #[inline(always)]
    pub fn sw_errorinfo_base(&self) -> SwErrorinfoBaseR {
        SwErrorinfoBaseR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - error info base addr\n\nerror info base addr\n\nevery slice contains 256 bits error info"]
    #[inline(always)]
    #[must_use]
    pub fn sw_errorinfo_base(&mut self) -> SwErrorinfoBaseW<Swreg75H264ErrorinfoBaseSpec> {
        SwErrorinfoBaseW::new(self, 3)
    }
}
#[doc = "h264 error info base addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg75_h264_errorinfo_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg75_h264_errorinfo_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg75H264ErrorinfoBaseSpec;
impl crate::RegisterSpec for Swreg75H264ErrorinfoBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg75_h264_errorinfo_base::R`](R) reader structure"]
impl crate::Readable for Swreg75H264ErrorinfoBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg75_h264_errorinfo_base::W`](W) writer structure"]
impl crate::Writable for Swreg75H264ErrorinfoBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG75_H264_ERRORINFO_BASE to value 0"]
impl crate::Resettable for Swreg75H264ErrorinfoBaseSpec {
    const RESET_VALUE: u32 = 0;
}
