#[doc = "Register `ACTIVE_PIXEL_STA_H` reader"]
pub type R = crate::R<ActivePixelStaHSpec>;
#[doc = "Register `ACTIVE_PIXEL_STA_H` writer"]
pub type W = crate::W<ActivePixelStaHSpec>;
#[doc = "Field `ACTIVE_PIXEL_STA_H` reader - ACTIVE_PIXEL \\[13:8\\]
which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
pub type ActivePixelStaHR = crate::FieldReader;
#[doc = "Field `ACTIVE_PIXEL_STA_H` writer - ACTIVE_PIXEL \\[13:8\\]
which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
pub type ActivePixelStaHW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - ACTIVE_PIXEL \\[13:8\\]
which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn active_pixel_sta_h(&self) -> ActivePixelStaHR {
        ActivePixelStaHR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - ACTIVE_PIXEL \\[13:8\\]
which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    #[must_use]
    pub fn active_pixel_sta_h(&mut self) -> ActivePixelStaHW<ActivePixelStaHSpec> {
        ActivePixelStaHW::new(self, 0)
    }
}
#[doc = "Active Pixel Status High Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`active_pixel_sta_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`active_pixel_sta_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActivePixelStaHSpec;
impl crate::RegisterSpec for ActivePixelStaHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`active_pixel_sta_h::R`](R) reader structure"]
impl crate::Readable for ActivePixelStaHSpec {}
#[doc = "`write(|w| ..)` method takes [`active_pixel_sta_h::W`](W) writer structure"]
impl crate::Writable for ActivePixelStaHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets ACTIVE_PIXEL_STA_H to value 0"]
impl crate::Resettable for ActivePixelStaHSpec {
    const RESET_VALUE: u32 = 0;
}
