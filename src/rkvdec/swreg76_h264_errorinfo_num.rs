#[doc = "Register `SWREG76_H264_ERRORINFO_NUM` reader"]
pub type R = crate::R<Swreg76H264ErrorinfoNumSpec>;
#[doc = "Register `SWREG76_H264_ERRORINFO_NUM` writer"]
pub type W = crate::W<Swreg76H264ErrorinfoNumSpec>;
#[doc = "Field `SW_SLICEDEC_NUM` reader - slice dec num\n\nh264 decoded num, the max slice num for H264 is 4096"]
pub type SwSlicedecNumR = crate::FieldReader<u16>;
#[doc = "Field `SW_SLICEDEC_NUM` writer - slice dec num\n\nh264 decoded num, the max slice num for H264 is 4096"]
pub type SwSlicedecNumW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `SW_STRMD_DETECT_ERROR_FLAG` reader - strmd error detect flag\n\nstreamd detect error flag"]
pub type SwStrmdDetectErrorFlagR = crate::BitReader;
#[doc = "Field `SW_STRMD_DETECT_ERROR_FLAG` writer - strmd error detect flag\n\nstreamd detect error flag"]
pub type SwStrmdDetectErrorFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_ERROR_PACKET_NUM` reader - error packet number\n\nerror packet number"]
pub type SwErrorPacketNumR = crate::FieldReader<u16>;
#[doc = "Field `SW_ERROR_PACKET_NUM` writer - error packet number\n\nerror packet number"]
pub type SwErrorPacketNumW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - slice dec num\n\nh264 decoded num, the max slice num for H264 is 4096"]
    #[inline(always)]
    pub fn sw_slicedec_num(&self) -> SwSlicedecNumR {
        SwSlicedecNumR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 15 - strmd error detect flag\n\nstreamd detect error flag"]
    #[inline(always)]
    pub fn sw_strmd_detect_error_flag(&self) -> SwStrmdDetectErrorFlagR {
        SwStrmdDetectErrorFlagR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:29 - error packet number\n\nerror packet number"]
    #[inline(always)]
    pub fn sw_error_packet_num(&self) -> SwErrorPacketNumR {
        SwErrorPacketNumR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - slice dec num\n\nh264 decoded num, the max slice num for H264 is 4096"]
    #[inline(always)]
    #[must_use]
    pub fn sw_slicedec_num(&mut self) -> SwSlicedecNumW<Swreg76H264ErrorinfoNumSpec> {
        SwSlicedecNumW::new(self, 0)
    }
    #[doc = "Bit 15 - strmd error detect flag\n\nstreamd detect error flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_strmd_detect_error_flag(
        &mut self,
    ) -> SwStrmdDetectErrorFlagW<Swreg76H264ErrorinfoNumSpec> {
        SwStrmdDetectErrorFlagW::new(self, 15)
    }
    #[doc = "Bits 16:29 - error packet number\n\nerror packet number"]
    #[inline(always)]
    #[must_use]
    pub fn sw_error_packet_num(&mut self) -> SwErrorPacketNumW<Swreg76H264ErrorinfoNumSpec> {
        SwErrorPacketNumW::new(self, 16)
    }
}
#[doc = "h264 error info num\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg76_h264_errorinfo_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg76_h264_errorinfo_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg76H264ErrorinfoNumSpec;
impl crate::RegisterSpec for Swreg76H264ErrorinfoNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg76_h264_errorinfo_num::R`](R) reader structure"]
impl crate::Readable for Swreg76H264ErrorinfoNumSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg76_h264_errorinfo_num::W`](W) writer structure"]
impl crate::Writable for Swreg76H264ErrorinfoNumSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG76_H264_ERRORINFO_NUM to value 0"]
impl crate::Resettable for Swreg76H264ErrorinfoNumSpec {
    const RESET_VALUE: u32 = 0;
}
