#[doc = "Register `WDR_TONECURVE_3` reader"]
pub type R = crate::R<WdrTonecurve3Spec>;
#[doc = "Register `WDR_TONECURVE_3` writer"]
pub type W = crate::W<WdrTonecurve3Spec>;
#[doc = "Field `WDR_dY17` reader - Tone curve sample point definition dY17 on the\n\nhorizontal axis (input)"]
pub type WdrDY17R = crate::FieldReader;
#[doc = "Field `WDR_dY17` writer - Tone curve sample point definition dY17 on the\n\nhorizontal axis (input)"]
pub type WdrDY17W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY18` reader - Tone curve sample point definition dY18 on the\n\nhorizontal axis (input)"]
pub type WdrDY18R = crate::FieldReader;
#[doc = "Field `WDR_dY18` writer - Tone curve sample point definition dY18 on the\n\nhorizontal axis (input)"]
pub type WdrDY18W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY19` reader - Tone curve sample point definition dY19 on the\n\nhorizontal axis (input)"]
pub type WdrDY19R = crate::FieldReader;
#[doc = "Field `WDR_dY19` writer - Tone curve sample point definition dY19 on the\n\nhorizontal axis (input)"]
pub type WdrDY19W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY20` reader - Tone curve sample point definition dY20 on the\n\nhorizontal axis (input)"]
pub type WdrDY20R = crate::FieldReader;
#[doc = "Field `WDR_dY20` writer - Tone curve sample point definition dY20 on the\n\nhorizontal axis (input)"]
pub type WdrDY20W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY21` reader - Tone curve sample point definition dY21 on the\n\nhorizontal axis (input)"]
pub type WdrDY21R = crate::FieldReader;
#[doc = "Field `WDR_dY21` writer - Tone curve sample point definition dY21 on the\n\nhorizontal axis (input)"]
pub type WdrDY21W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY22` reader - Tone curve sample point definition dY22 on the\n\nhorizontal axis (input)"]
pub type WdrDY22R = crate::FieldReader;
#[doc = "Field `WDR_dY22` writer - Tone curve sample point definition dY22 on the\n\nhorizontal axis (input)"]
pub type WdrDY22W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY23` reader - Tone curve sample point definition dY23 on the\n\nhorizontal axis (input)\n\n"]
pub type WdrDY23R = crate::FieldReader;
#[doc = "Field `WDR_dY23` writer - Tone curve sample point definition dY23 on the\n\nhorizontal axis (input)\n\n"]
pub type WdrDY23W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDR_dY24` reader - Tone curve sample point definition dY24 on the\n\nhorizontal axis (input)"]
pub type WdrDY24R = crate::FieldReader;
#[doc = "Field `WDR_dY24` writer - Tone curve sample point definition dY24 on the\n\nhorizontal axis (input)"]
pub type WdrDY24W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
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
    #[doc = "Bits 24:26 - Tone curve sample point definition dY23 on the\n\nhorizontal axis (input)\n\n"]
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
impl W {
    #[doc = "Bits 0:2 - Tone curve sample point definition dY17 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y17(&mut self) -> WdrDY17W<WdrTonecurve3Spec> {
        WdrDY17W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Tone curve sample point definition dY18 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y18(&mut self) -> WdrDY18W<WdrTonecurve3Spec> {
        WdrDY18W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Tone curve sample point definition dY19 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y19(&mut self) -> WdrDY19W<WdrTonecurve3Spec> {
        WdrDY19W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Tone curve sample point definition dY20 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y20(&mut self) -> WdrDY20W<WdrTonecurve3Spec> {
        WdrDY20W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Tone curve sample point definition dY21 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y21(&mut self) -> WdrDY21W<WdrTonecurve3Spec> {
        WdrDY21W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Tone curve sample point definition dY22 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y22(&mut self) -> WdrDY22W<WdrTonecurve3Spec> {
        WdrDY22W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Tone curve sample point definition dY23 on the\n\nhorizontal axis (input)\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y23(&mut self) -> WdrDY23W<WdrTonecurve3Spec> {
        WdrDY23W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Tone curve sample point definition dY24 on the\n\nhorizontal axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_d_y24(&mut self) -> WdrDY24W<WdrTonecurve3Spec> {
        WdrDY24W::new(self, 28)
    }
}
#[doc = "Tone Curve sample points dYn definition (part 3)\n\nNote: The interval widths dYn are to be defined in a 2^(value+3) notation, where \n\n\n\n'value' has to be written to the register. So the steps would be \n\n\n\ndYn=0 -> 8 (2^3), dYn=1 -> 16 (2^4), dYn=2 \n\n\n\n-> 32 (2^5),... dYn=6 -> 512 (2^9), dYn=7 -> \n\n\n\n1024 (2^10). \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_tonecurve_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdr_tonecurve_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdrTonecurve3Spec;
impl crate::RegisterSpec for WdrTonecurve3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdr_tonecurve_3::R`](R) reader structure"]
impl crate::Readable for WdrTonecurve3Spec {}
#[doc = "`write(|w| ..)` method takes [`wdr_tonecurve_3::W`](W) writer structure"]
impl crate::Writable for WdrTonecurve3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDR_TONECURVE_3 to value 0x4444_4444"]
impl crate::Resettable for WdrTonecurve3Spec {
    const RESET_VALUE: u32 = 0x4444_4444;
}
