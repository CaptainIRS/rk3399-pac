#[doc = "Register `WDR_OFFSET` reader"]
pub type R = crate::R<WdrOffsetSpec>;
#[doc = "Register `WDR_OFFSET` writer"]
pub type W = crate::W<WdrOffsetSpec>;
#[doc = "Field `RGB_OFFSET` reader - RGB Offset value (b) for RGB operation mode\n\nunsigned 12 bit value\n\n"]
pub type RgbOffsetR = crate::FieldReader<u16>;
#[doc = "Field `RGB_OFFSET` writer - RGB Offset value (b) for RGB operation mode\n\nunsigned 12 bit value\n\n"]
pub type RgbOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `LUM_OFFSET` reader - Luminance Offset value (a) for RGB operation mode\n\nunsigned 12 bit value"]
pub type LumOffsetR = crate::FieldReader<u16>;
#[doc = "Field `LUM_OFFSET` writer - Luminance Offset value (a) for RGB operation mode\n\nunsigned 12 bit value"]
pub type LumOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - RGB Offset value (b) for RGB operation mode\n\nunsigned 12 bit value\n\n"]
    #[inline(always)]
    pub fn rgb_offset(&self) -> RgbOffsetR {
        RgbOffsetR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Luminance Offset value (a) for RGB operation mode\n\nunsigned 12 bit value"]
    #[inline(always)]
    pub fn lum_offset(&self) -> LumOffsetR {
        LumOffsetR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - RGB Offset value (b) for RGB operation mode\n\nunsigned 12 bit value\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn rgb_offset(&mut self) -> RgbOffsetW<WdrOffsetSpec> {
        RgbOffsetW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Luminance Offset value (a) for RGB operation mode\n\nunsigned 12 bit value"]
    #[inline(always)]
    #[must_use]
    pub fn lum_offset(&mut self) -> LumOffsetW<WdrOffsetSpec> {
        LumOffsetW::new(self, 16)
    }
}
#[doc = "Offset values for RGB path\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_offset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdr_offset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdrOffsetSpec;
impl crate::RegisterSpec for WdrOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdr_offset::R`](R) reader structure"]
impl crate::Readable for WdrOffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`wdr_offset::W`](W) writer structure"]
impl crate::Writable for WdrOffsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDR_OFFSET to value 0"]
impl crate::Resettable for WdrOffsetSpec {
    const RESET_VALUE: u32 = 0;
}
