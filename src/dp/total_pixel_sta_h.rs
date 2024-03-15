#[doc = "Register `TOTAL_PIXEL_STA_H` reader"]
pub type R = crate::R<TotalPixelStaHSpec>;
#[doc = "Register `TOTAL_PIXEL_STA_H` writer"]
pub type W = crate::W<TotalPixelStaHSpec>;
#[doc = "Field `TOTAL_PIXEL_STA_H` reader - TOTAL_PIXEL \\[13:8\\]
which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
pub type TotalPixelStaHR = crate::FieldReader;
#[doc = "Field `TOTAL_PIXEL_STA_H` writer - TOTAL_PIXEL \\[13:8\\]
which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
pub type TotalPixelStaHW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - TOTAL_PIXEL \\[13:8\\]
which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn total_pixel_sta_h(&self) -> TotalPixelStaHR {
        TotalPixelStaHR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - TOTAL_PIXEL \\[13:8\\]
which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    #[must_use]
    pub fn total_pixel_sta_h(&mut self) -> TotalPixelStaHW<TotalPixelStaHSpec> {
        TotalPixelStaHW::new(self, 0)
    }
}
#[doc = "Total Pixel Status High Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`total_pixel_sta_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`total_pixel_sta_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TotalPixelStaHSpec;
impl crate::RegisterSpec for TotalPixelStaHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`total_pixel_sta_h::R`](R) reader structure"]
impl crate::Readable for TotalPixelStaHSpec {}
#[doc = "`write(|w| ..)` method takes [`total_pixel_sta_h::W`](W) writer structure"]
impl crate::Writable for TotalPixelStaHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOTAL_PIXEL_STA_H to value 0"]
impl crate::Resettable for TotalPixelStaHSpec {
    const RESET_VALUE: u32 = 0;
}
