#[doc = "Register `WDR_TONECURVE_4_SHD` reader"]
pub type R = crate::R<WdrTonecurve4ShdSpec>;
#[doc = "Field `WDR_dY25` reader - Tone curve sample point definition dY25 on the\n\nhorizontal axis (input)"]
pub type WdrDY25R = crate::FieldReader;
#[doc = "Field `WDR_dY26` reader - Tone curve sample point definition dY26 on the\n\nhorizontal axis (input)"]
pub type WdrDY26R = crate::FieldReader;
#[doc = "Field `WDR_dY27` reader - Tone curve sample point definition dY27 on the\n\nhorizontal axis (input)"]
pub type WdrDY27R = crate::FieldReader;
#[doc = "Field `WDR_dY28` reader - Tone curve sample point definition dY28 on the\n\nhorizontal axis (input)"]
pub type WdrDY28R = crate::FieldReader;
#[doc = "Field `WDR_dY29` reader - Tone curve sample point definition dY29 on the\n\nhorizontal axis (input)"]
pub type WdrDY29R = crate::FieldReader;
#[doc = "Field `WDR_dY30` reader - Tone curve sample point definition dY30 on the\n\nhorizontal axis (input)"]
pub type WdrDY30R = crate::FieldReader;
#[doc = "Field `WDR_dY31` reader - Tone curve sample point definition dY31 on the\n\nhorizontal axis (input)"]
pub type WdrDY31R = crate::FieldReader;
#[doc = "Field `WDR_dY32` reader - Tone curve sample point definition dY32 on the\n\nhorizontal axis (input)"]
pub type WdrDY32R = crate::FieldReader;
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
#[doc = "Tone Curve sample points dYn definition shadow register(part 4)\n\nNote: see register ISP_WDR_TONECURVE_4. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_tonecurve_4_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdrTonecurve4ShdSpec;
impl crate::RegisterSpec for WdrTonecurve4ShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdr_tonecurve_4_shd::R`](R) reader structure"]
impl crate::Readable for WdrTonecurve4ShdSpec {}
#[doc = "`reset()` method sets WDR_TONECURVE_4_SHD to value 0x4444_4444"]
impl crate::Resettable for WdrTonecurve4ShdSpec {
    const RESET_VALUE: u32 = 0x4444_4444;
}
