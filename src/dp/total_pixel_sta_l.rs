#[doc = "Register `TOTAL_PIXEL_STA_L` reader"]
pub type R = crate::R<TotalPixelStaLSpec>;
#[doc = "Register `TOTAL_PIXEL_STA_L` writer"]
pub type W = crate::W<TotalPixelStaLSpec>;
#[doc = "Field `TOTAL_PIXEL_STA_L` reader - TOTAL_PIXEL \\[7:0\\]
which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
pub type TotalPixelStaLR = crate::FieldReader;
#[doc = "Field `TOTAL_PIXEL_STA_L` writer - TOTAL_PIXEL \\[7:0\\]
which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
pub type TotalPixelStaLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TOTAL_PIXEL \\[7:0\\]
which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn total_pixel_sta_l(&self) -> TotalPixelStaLR {
        TotalPixelStaLR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TOTAL_PIXEL \\[7:0\\]
which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    #[must_use]
    pub fn total_pixel_sta_l(&mut self) -> TotalPixelStaLW<TotalPixelStaLSpec> {
        TotalPixelStaLW::new(self, 0)
    }
}
#[doc = "Total Pixel Status Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`total_pixel_sta_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`total_pixel_sta_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TotalPixelStaLSpec;
impl crate::RegisterSpec for TotalPixelStaLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`total_pixel_sta_l::R`](R) reader structure"]
impl crate::Readable for TotalPixelStaLSpec {}
#[doc = "`write(|w| ..)` method takes [`total_pixel_sta_l::W`](W) writer structure"]
impl crate::Writable for TotalPixelStaLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOTAL_PIXEL_STA_L to value 0"]
impl crate::Resettable for TotalPixelStaLSpec {
    const RESET_VALUE: u32 = 0;
}
