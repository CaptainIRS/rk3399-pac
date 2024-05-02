#[doc = "Register `WDR_TONECURVE_1` reader"]
pub type R = crate::R<WdrTonecurve1Spec>;
#[doc = "Register `WDR_TONECURVE_1` writer"]
pub type W = crate::W<WdrTonecurve1Spec>;
#[doc = "Field `WDR_dY1` reader - Tone curve sample point definition dY1 on the\n\nhorizontal axis (input)"]
pub type WdrDY1R = crate::FieldReader;
#[doc = "Field `WDR_dY1` writer - Tone curve sample point definition dY1 on the\n\nhorizontal axis (input)"]
pub type WdrDY1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY2` reader - Tone curve sample point definition dY2 on the\n\nhorizontal axis (input)"]
pub type WdrDY2R = crate::FieldReader;
#[doc = "Field `WDR_dY2` writer - Tone curve sample point definition dY2 on the\n\nhorizontal axis (input)"]
pub type WdrDY2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY3` reader - Tone curve sample point definition dY3 on the\n\nhorizontal axis (input)"]
pub type WdrDY3R = crate::FieldReader;
#[doc = "Field `WDR_dY3` writer - Tone curve sample point definition dY3 on the\n\nhorizontal axis (input)"]
pub type WdrDY3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY4` reader - Tone curve sample point definition dY4 on the\n\nhorizontal axis (input)"]
pub type WdrDY4R = crate::FieldReader;
#[doc = "Field `WDR_dY4` writer - Tone curve sample point definition dY4 on the\n\nhorizontal axis (input)"]
pub type WdrDY4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY5` reader - Tone curve sample point definition dY5 on the\n\nhorizontal axis (input)"]
pub type WdrDY5R = crate::FieldReader;
#[doc = "Field `WDR_dY5` writer - Tone curve sample point definition dY5 on the\n\nhorizontal axis (input)"]
pub type WdrDY5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY6` reader - Tone curve sample point definition dY6 on the\n\nhorizontal axis (input)\n\n"]
pub type WdrDY6R = crate::FieldReader;
#[doc = "Field `WDR_dY6` writer - Tone curve sample point definition dY6 on the\n\nhorizontal axis (input)\n\n"]
pub type WdrDY6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY7` reader - Tone curve sample point definition dY7 on the\n\nhorizontal axis (input)"]
pub type WdrDY7R = crate::FieldReader;
#[doc = "Field `WDR_dY7` writer - Tone curve sample point definition dY7 on the\n\nhorizontal axis (input)"]
pub type WdrDY7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY8` reader - Tone curve sample point definition dY8 on the\n\nhorizontal axis (input)"]
pub type WdrDY8R = crate::FieldReader;
#[doc = "Field `WDR_dY8` writer - Tone curve sample point definition dY8 on the\n\nhorizontal axis (input)"]
pub type WdrDY8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Tone curve sample point definition dY1 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y1(&self) -> WdrDY1R {
        WdrDY1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Tone curve sample point definition dY2 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y2(&self) -> WdrDY2R {
        WdrDY2R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Tone curve sample point definition dY3 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y3(&self) -> WdrDY3R {
        WdrDY3R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Tone curve sample point definition dY4 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y4(&self) -> WdrDY4R {
        WdrDY4R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Tone curve sample point definition dY5 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y5(&self) -> WdrDY5R {
        WdrDY5R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Tone curve sample point definition dY6 on the\n\nhorizontal axis (input)\n\n"]
    #[inline(always)]
    pub fn wdr_d_y6(&self) -> WdrDY6R {
        WdrDY6R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Tone curve sample point definition dY7 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y7(&self) -> WdrDY7R {
        WdrDY7R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Tone curve sample point definition dY8 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y8(&self) -> WdrDY8R {
        WdrDY8R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tone curve sample point definition dY1 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y1(&mut self) -> WdrDY1W<WdrTonecurve1Spec> {
        WdrDY1W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Tone curve sample point definition dY2 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y2(&mut self) -> WdrDY2W<WdrTonecurve1Spec> {
        WdrDY2W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Tone curve sample point definition dY3 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y3(&mut self) -> WdrDY3W<WdrTonecurve1Spec> {
        WdrDY3W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Tone curve sample point definition dY4 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y4(&mut self) -> WdrDY4W<WdrTonecurve1Spec> {
        WdrDY4W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Tone curve sample point definition dY5 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y5(&mut self) -> WdrDY5W<WdrTonecurve1Spec> {
        WdrDY5W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Tone curve sample point definition dY6 on the\n\nhorizontal axis (input)\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y6(&mut self) -> WdrDY6W<WdrTonecurve1Spec> {
        WdrDY6W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Tone curve sample point definition dY7 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y7(&mut self) -> WdrDY7W<WdrTonecurve1Spec> {
        WdrDY7W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Tone curve sample point definition dY8 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y8(&mut self) -> WdrDY8W<WdrTonecurve1Spec> {
        WdrDY8W::new(self, 28)
    }
}
#[doc = "Tone Curve sample points dYn definition (part 1)\n\nNote: The interval widths dYn are to be defined in a 2^(value+3) notation, where \n\n\n\n'value' has to be written to the register. So the steps would be \n\n\n\ndYn=0 -> 8 (2^3), dYn=1 -> 16 (2^4), dYn=2 \n\n\n\n-> 32 (2^5),... dYn=6 -> 512 (2^9), dYn=7 -> \n\n\n\n1024 (2^10). \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_tonecurve_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdr_tonecurve_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdrTonecurve1Spec;
impl crate::RegisterSpec for WdrTonecurve1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdr_tonecurve_1::R`](R) reader structure"]
impl crate::Readable for WdrTonecurve1Spec {}
#[doc = "`write(|w| ..)` method takes [`wdr_tonecurve_1::W`](W) writer structure"]
impl crate::Writable for WdrTonecurve1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDR_TONECURVE_1 to value 0x4444_4444"]
impl crate::Resettable for WdrTonecurve1Spec {
    const RESET_VALUE: u32 = 0x4444_4444;
}
