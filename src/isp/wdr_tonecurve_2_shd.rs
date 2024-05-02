#[doc = "Register `WDR_TONECURVE_2_SHD` reader"]
pub type R = crate::R<WdrTonecurve2ShdSpec>;
#[doc = "Field `WDR_dY9` reader - Tone curve sample point definition dY9 on the\n\nhorizontal axis (input)"]
pub type WdrDY9R = crate::FieldReader;
#[doc = "Field `WDR_dY10` reader - Tone curve sample point definition dY10 on the\n\nhorizontal axis (input)"]
pub type WdrDY10R = crate::FieldReader;
#[doc = "Field `WDR_dY11` reader - Tone curve sample point definition dY11 on the\n\nhorizontal axis (input)"]
pub type WdrDY11R = crate::FieldReader;
#[doc = "Field `WDR_dY12` reader - Tone curve sample point definition dY12 on the\n\nhorizontal axis (input)"]
pub type WdrDY12R = crate::FieldReader;
#[doc = "Field `WDR_dY13` reader - Tone curve sample point definition dY13 on the\n\nhorizontal axis (input)"]
pub type WdrDY13R = crate::FieldReader;
#[doc = "Field `WDR_dY14` reader - Tone curve sample point definition dY14 on the\n\nhorizontal axis (input)"]
pub type WdrDY14R = crate::FieldReader;
#[doc = "Field `WDR_dY15` reader - Tone curve sample point definition dY15 on the\n\nhorizontal axis (input)"]
pub type WdrDY15R = crate::FieldReader;
#[doc = "Field `WDR_dY16` reader - Tone curve sample point definition dY16 on the\n\nhorizontal axis (input)"]
pub type WdrDY16R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Tone curve sample point definition dY9 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y9(&self) -> WdrDY9R {
        WdrDY9R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Tone curve sample point definition dY10 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y10(&self) -> WdrDY10R {
        WdrDY10R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Tone curve sample point definition dY11 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y11(&self) -> WdrDY11R {
        WdrDY11R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Tone curve sample point definition dY12 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y12(&self) -> WdrDY12R {
        WdrDY12R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Tone curve sample point definition dY13 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y13(&self) -> WdrDY13R {
        WdrDY13R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Tone curve sample point definition dY14 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y14(&self) -> WdrDY14R {
        WdrDY14R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Tone curve sample point definition dY15 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y15(&self) -> WdrDY15R {
        WdrDY15R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Tone curve sample point definition dY16 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y16(&self) -> WdrDY16R {
        WdrDY16R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "Tone Curve sample points dYn definition shadow register (part 2)\n\nNote: see register ISP_WDR_TONECURVE_2. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_tonecurve_2_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdrTonecurve2ShdSpec;
impl crate::RegisterSpec for WdrTonecurve2ShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdr_tonecurve_2_shd::R`](R) reader structure"]
impl crate::Readable for WdrTonecurve2ShdSpec {}
#[doc = "`reset()` method sets WDR_TONECURVE_2_SHD to value 0x4444_4444"]
impl crate::Resettable for WdrTonecurve2ShdSpec {
    const RESET_VALUE: u32 = 0x4444_4444;
}
