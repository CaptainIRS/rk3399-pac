#[doc = "Register `WDR_TONECURVE_2` reader"]
pub type R = crate::R<WdrTonecurve2Spec>;
#[doc = "Register `WDR_TONECURVE_2` writer"]
pub type W = crate::W<WdrTonecurve2Spec>;
#[doc = "Field `WDR_dY9` reader - Tone curve sample point definition dY9 on the\n\nhorizontal axis (input)"]
pub type WdrDY9R = crate::FieldReader;
#[doc = "Field `WDR_dY9` writer - Tone curve sample point definition dY9 on the\n\nhorizontal axis (input)"]
pub type WdrDY9W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY10` reader - Tone curve sample point definition dY10 on the\n\nhorizontal axis (input)"]
pub type WdrDY10R = crate::FieldReader;
#[doc = "Field `WDR_dY10` writer - Tone curve sample point definition dY10 on the\n\nhorizontal axis (input)"]
pub type WdrDY10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY11` reader - Tone curve sample point definition dY11 on the\n\nhorizontal axis (input)"]
pub type WdrDY11R = crate::FieldReader;
#[doc = "Field `WDR_dY11` writer - Tone curve sample point definition dY11 on the\n\nhorizontal axis (input)"]
pub type WdrDY11W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY12` reader - Tone curve sample point definition dY12 on the\n\nhorizontal axis (input)"]
pub type WdrDY12R = crate::FieldReader;
#[doc = "Field `WDR_dY12` writer - Tone curve sample point definition dY12 on the\n\nhorizontal axis (input)"]
pub type WdrDY12W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY13` reader - Tone curve sample point definition dY13 on the\n\nhorizontal axis (input)"]
pub type WdrDY13R = crate::FieldReader;
#[doc = "Field `WDR_dY13` writer - Tone curve sample point definition dY13 on the\n\nhorizontal axis (input)"]
pub type WdrDY13W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY14` reader - Tone curve sample point definition dY14 on the\n\nhorizontal axis (input)"]
pub type WdrDY14R = crate::FieldReader;
#[doc = "Field `WDR_dY14` writer - Tone curve sample point definition dY14 on the\n\nhorizontal axis (input)"]
pub type WdrDY14W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY15` reader - Tone curve sample point definition dY15 on the\n\nhorizontal axis (input)"]
pub type WdrDY15R = crate::FieldReader;
#[doc = "Field `WDR_dY15` writer - Tone curve sample point definition dY15 on the\n\nhorizontal axis (input)"]
pub type WdrDY15W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY16` reader - Tone curve sample point definition dY16 on the\n\nhorizontal axis (input)"]
pub type WdrDY16R = crate::FieldReader;
#[doc = "Field `WDR_dY16` writer - Tone curve sample point definition dY16 on the\n\nhorizontal axis (input)"]
pub type WdrDY16W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
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
impl W {
    #[doc = "Bits 0:2 - Tone curve sample point definition dY9 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y9(&mut self) -> WdrDY9W<WdrTonecurve2Spec> {
        WdrDY9W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Tone curve sample point definition dY10 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y10(&mut self) -> WdrDY10W<WdrTonecurve2Spec> {
        WdrDY10W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Tone curve sample point definition dY11 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y11(&mut self) -> WdrDY11W<WdrTonecurve2Spec> {
        WdrDY11W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Tone curve sample point definition dY12 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y12(&mut self) -> WdrDY12W<WdrTonecurve2Spec> {
        WdrDY12W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Tone curve sample point definition dY13 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y13(&mut self) -> WdrDY13W<WdrTonecurve2Spec> {
        WdrDY13W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Tone curve sample point definition dY14 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y14(&mut self) -> WdrDY14W<WdrTonecurve2Spec> {
        WdrDY14W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Tone curve sample point definition dY15 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y15(&mut self) -> WdrDY15W<WdrTonecurve2Spec> {
        WdrDY15W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Tone curve sample point definition dY16 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y16(&mut self) -> WdrDY16W<WdrTonecurve2Spec> {
        WdrDY16W::new(self, 28)
    }
}
#[doc = "Tone Curve sample points dYn definition (part 2)\n\nNote: The interval widths dYn are to be defined in a 2^(value+3) notation, where \n\n\n\n'value' has to be written to the register. So the steps would be \n\n\n\ndYn=0 -> 8 (2^3), dYn=1 -> 16 (2^4), dYn=2 \n\n\n\n-> 32 (2^5),... dYn=6 -> 512 (2^9), dYn=7 -> \n\n\n\n1024 (2^10). \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_tonecurve_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdr_tonecurve_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdrTonecurve2Spec;
impl crate::RegisterSpec for WdrTonecurve2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdr_tonecurve_2::R`](R) reader structure"]
impl crate::Readable for WdrTonecurve2Spec {}
#[doc = "`write(|w| ..)` method takes [`wdr_tonecurve_2::W`](W) writer structure"]
impl crate::Writable for WdrTonecurve2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDR_TONECURVE_2 to value 0x4444_4444"]
impl crate::Resettable for WdrTonecurve2Spec {
    const RESET_VALUE: u32 = 0x4444_4444;
}
