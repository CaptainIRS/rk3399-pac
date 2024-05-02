#[doc = "Register `WDR_TONECURVE_3_SHD` reader"]
pub type R = crate::R<WdrTonecurve3ShdSpec>;
#[doc = "Field `WDR_dY17` reader - Tone curve sample point definition dY17 on the\n\nhorizontal axis (input)"]
pub type WdrDY17R = crate::FieldReader;
#[doc = "Field `WDR_dY18` reader - Tone curve sample point definition dY18 on the\n\nhorizontal axis (input)"]
pub type WdrDY18R = crate::FieldReader;
#[doc = "Field `WDR_dY19` reader - Tone curve sample point definition dY19 on the\n\nhorizontal axis (input)"]
pub type WdrDY19R = crate::FieldReader;
#[doc = "Field `WDR_dY20` reader - Tone curve sample point definition dY20 on the\n\nhorizontal axis (input)"]
pub type WdrDY20R = crate::FieldReader;
#[doc = "Field `WDR_dY21` reader - Tone curve sample point definition dY21 on the\n\nhorizontal axis (input)"]
pub type WdrDY21R = crate::FieldReader;
#[doc = "Field `WDR_dY22` reader - Tone curve sample point definition dY22 on the\n\nhorizontal axis (input)"]
pub type WdrDY22R = crate::FieldReader;
#[doc = "Field `WDR_dY23` reader - Tone curve sample point definition dY23 on the\n\nhorizontal axis (input)"]
pub type WdrDY23R = crate::FieldReader;
#[doc = "Field `WDR_dY24` reader - Tone curve sample point definition dY24 on the\n\nhorizontal axis (input)"]
pub type WdrDY24R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Tone curve sample point definition dY17 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y17(&self) -> WdrDY17R {
        WdrDY17R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Tone curve sample point definition dY18 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y18(&self) -> WdrDY18R {
        WdrDY18R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Tone curve sample point definition dY19 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y19(&self) -> WdrDY19R {
        WdrDY19R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Tone curve sample point definition dY20 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y20(&self) -> WdrDY20R {
        WdrDY20R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Tone curve sample point definition dY21 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y21(&self) -> WdrDY21R {
        WdrDY21R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Tone curve sample point definition dY22 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y22(&self) -> WdrDY22R {
        WdrDY22R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Tone curve sample point definition dY23 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y23(&self) -> WdrDY23R {
        WdrDY23R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Tone curve sample point definition dY24 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    pub fn wdr_d_y24(&self) -> WdrDY24R {
        WdrDY24R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "Tone Curve sample points dYn definition shadow register (part 3)\n\nNote: see register ISP_WDR_TONECURVE_3. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_tonecurve_3_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdrTonecurve3ShdSpec;
impl crate::RegisterSpec for WdrTonecurve3ShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdr_tonecurve_3_shd::R`](R) reader structure"]
impl crate::Readable for WdrTonecurve3ShdSpec {}
#[doc = "`reset()` method sets WDR_TONECURVE_3_SHD to value 0x4444_4444"]
impl crate::Resettable for WdrTonecurve3ShdSpec {
    const RESET_VALUE: u32 = 0x4444_4444;
}
