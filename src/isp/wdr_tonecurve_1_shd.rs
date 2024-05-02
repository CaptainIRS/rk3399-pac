#[doc = "Register `WDR_TONECURVE_1_SHD` reader"]
pub type R = crate::R<WdrTonecurve1ShdSpec>;
#[doc = "Field `WDR_dY1` reader - Tone curve sample point definition dY1 on the\n\nhorizontal axis (input)"]
pub type WdrDY1R = crate::FieldReader;
#[doc = "Field `WDR_dY2` reader - Tone curve sample point definition dY2 on the\n\nhorizontal axis (input)"]
pub type WdrDY2R = crate::FieldReader;
#[doc = "Field `WDR_dY3` reader - Tone curve sample point definition dY3 on the\n\nhorizontal axis (input)"]
pub type WdrDY3R = crate::FieldReader;
#[doc = "Field `WDR_dY4` reader - Tone curve sample point definition dY4 on the\n\nhorizontal axis (input)"]
pub type WdrDY4R = crate::FieldReader;
#[doc = "Field `WDR_dY5` reader - Tone curve sample point definition dY5 on the\n\nhorizontal axis (input)"]
pub type WdrDY5R = crate::FieldReader;
#[doc = "Field `WDR_dY6` reader - Tone curve sample point definition dY6 on the\n\nhorizontal axis (input)"]
pub type WdrDY6R = crate::FieldReader;
#[doc = "Field `WDR_dY7` reader - Tone curve sample point definition dY7 on the\n\nhorizontal axis (input)"]
pub type WdrDY7R = crate::FieldReader;
#[doc = "Field `WDR_dY8` reader - Tone curve sample point definition dY8 on the\n\nhorizontal axis (input)"]
pub type WdrDY8R = crate::FieldReader;
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
    #[doc = "Bits 20:22 - Tone curve sample point definition dY6 on the\n\nhorizontal axis (input)"]
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
#[doc = "Tone Curve sample points dYn definition shadow register (part 1)\n\nNote: see register ISP_WDR_TONECURVE_1. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_tonecurve_1_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdrTonecurve1ShdSpec;
impl crate::RegisterSpec for WdrTonecurve1ShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdr_tonecurve_1_shd::R`](R) reader structure"]
impl crate::Readable for WdrTonecurve1ShdSpec {}
#[doc = "`reset()` method sets WDR_TONECURVE_1_SHD to value 0x4444_4444"]
impl crate::Resettable for WdrTonecurve1ShdSpec {
    const RESET_VALUE: u32 = 0x4444_4444;
}
