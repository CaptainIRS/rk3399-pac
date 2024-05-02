#[doc = "Register `WDR_TONECURVE_4` reader"]
pub type R = crate::R<WdrTonecurve4Spec>;
#[doc = "Register `WDR_TONECURVE_4` writer"]
pub type W = crate::W<WdrTonecurve4Spec>;
#[doc = "Field `WDR_dY25` reader - Tone curve sample point definition dY25 on the\n\nhorizontal axis (input)"]
pub type WdrDY25R = crate::FieldReader;
#[doc = "Field `WDR_dY25` writer - Tone curve sample point definition dY25 on the\n\nhorizontal axis (input)"]
pub type WdrDY25W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY26` reader - Tone curve sample point definition dY26 on the\n\nhorizontal axis (input)"]
pub type WdrDY26R = crate::FieldReader;
#[doc = "Field `WDR_dY26` writer - Tone curve sample point definition dY26 on the\n\nhorizontal axis (input)"]
pub type WdrDY26W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY27` reader - Tone curve sample point definition dY27 on the\n\nhorizontal axis (input)"]
pub type WdrDY27R = crate::FieldReader;
#[doc = "Field `WDR_dY27` writer - Tone curve sample point definition dY27 on the\n\nhorizontal axis (input)"]
pub type WdrDY27W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY28` reader - Tone curve sample point definition dY28 on the\n\nhorizontal axis (input)"]
pub type WdrDY28R = crate::FieldReader;
#[doc = "Field `WDR_dY28` writer - Tone curve sample point definition dY28 on the\n\nhorizontal axis (input)"]
pub type WdrDY28W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY29` reader - Tone curve sample point definition dY29 on the\n\nhorizontal axis (input)"]
pub type WdrDY29R = crate::FieldReader;
#[doc = "Field `WDR_dY29` writer - Tone curve sample point definition dY29 on the\n\nhorizontal axis (input)"]
pub type WdrDY29W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY30` reader - Tone curve sample point definition dY30 on the\n\nhorizontal axis (input)"]
pub type WdrDY30R = crate::FieldReader;
#[doc = "Field `WDR_dY30` writer - Tone curve sample point definition dY30 on the\n\nhorizontal axis (input)"]
pub type WdrDY30W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY31` reader - Tone curve sample point definition dY31 on the\n\nhorizontal axis (input)"]
pub type WdrDY31R = crate::FieldReader;
#[doc = "Field `WDR_dY31` writer - Tone curve sample point definition dY31 on the\n\nhorizontal axis (input)"]
pub type WdrDY31W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY32` reader - Tone curve sample point definition dY32 on the\n\nhorizontal axis (input)"]
pub type WdrDY32R = crate::FieldReader;
#[doc = "Field `WDR_dY32` writer - Tone curve sample point definition dY32 on the\n\nhorizontal axis (input)"]
pub type WdrDY32W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Tone curve sample point definition dY25 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y25(&self) -> WdrDY25R {
        WdrDY25R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Tone curve sample point definition dY26 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y26(&self) -> WdrDY26R {
        WdrDY26R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Tone curve sample point definition dY27 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y27(&self) -> WdrDY27R {
        WdrDY27R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Tone curve sample point definition dY28 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y28(&self) -> WdrDY28R {
        WdrDY28R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Tone curve sample point definition dY29 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y29(&self) -> WdrDY29R {
        WdrDY29R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Tone curve sample point definition dY30 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y30(&self) -> WdrDY30R {
        WdrDY30R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Tone curve sample point definition dY31 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y31(&self) -> WdrDY31R {
        WdrDY31R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Tone curve sample point definition dY32 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y32(&self) -> WdrDY32R {
        WdrDY32R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tone curve sample point definition dY25 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y25(&mut self) -> WdrDY25W<WdrTonecurve4Spec> {
        WdrDY25W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Tone curve sample point definition dY26 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y26(&mut self) -> WdrDY26W<WdrTonecurve4Spec> {
        WdrDY26W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Tone curve sample point definition dY27 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y27(&mut self) -> WdrDY27W<WdrTonecurve4Spec> {
        WdrDY27W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Tone curve sample point definition dY28 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y28(&mut self) -> WdrDY28W<WdrTonecurve4Spec> {
        WdrDY28W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Tone curve sample point definition dY29 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y29(&mut self) -> WdrDY29W<WdrTonecurve4Spec> {
        WdrDY29W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Tone curve sample point definition dY30 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y30(&mut self) -> WdrDY30W<WdrTonecurve4Spec> {
        WdrDY30W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Tone curve sample point definition dY31 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y31(&mut self) -> WdrDY31W<WdrTonecurve4Spec> {
        WdrDY31W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Tone curve sample point definition dY32 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y32(&mut self) -> WdrDY32W<WdrTonecurve4Spec> {
        WdrDY32W::new(self, 28)
    }
}
#[doc = "Tone Curve sample points dYn definition (part 4)\n\nNote: The interval widths dYn are to be defined in a 2^(value+3) notation, where 'value' \n\nhas to be written to the register. So the steps would be \n\n\n\ndYn=0 -> 8 (2^3), dYn=1 -> 16 (2^4), dYn=2 -> 32 (2^5),... dYn=6 -> 512 (2^9), \n\ndYn=7 -> 1024 (2^10). \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_tonecurve_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdr_tonecurve_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdrTonecurve4Spec;
impl crate::RegisterSpec for WdrTonecurve4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdr_tonecurve_4::R`](R) reader structure"]
impl crate::Readable for WdrTonecurve4Spec {}
#[doc = "`write(|w| ..)` method takes [`wdr_tonecurve_4::W`](W) writer structure"]
impl crate::Writable for WdrTonecurve4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDR_TONECURVE_4 to value 0x4444_4444"]
impl crate::Resettable for WdrTonecurve4Spec {
    const RESET_VALUE: u32 = 0x4444_4444;
}
