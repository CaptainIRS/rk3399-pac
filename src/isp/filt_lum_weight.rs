#[doc = "Register `FILT_LUM_WEIGHT` reader"]
pub type R = crate::R<FiltLumWeightSpec>;
#[doc = "Register `FILT_LUM_WEIGHT` writer"]
pub type W = crate::W<FiltLumWeightSpec>;
#[doc = "Field `lum_weight_min` reader - Minimum value of luminance weight function\n\n"]
pub type LumWeightMinR = crate::FieldReader;
#[doc = "Field `lum_weight_min` writer - Minimum value of luminance weight function\n\n"]
pub type LumWeightMinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lum_weight_kink` reader - Kink position of luminance weight function"]
pub type LumWeightKinkR = crate::FieldReader;
#[doc = "Field `lum_weight_kink` writer - Kink position of luminance weight function"]
pub type LumWeightKinkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lum_weight_gain` reader - Gain select of luminance weight function"]
pub type LumWeightGainR = crate::FieldReader;
#[doc = "Field `lum_weight_gain` writer - Gain select of luminance weight function"]
pub type LumWeightGainW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - Minimum value of luminance weight function\n\n"]
    #[inline(always)]
    pub fn lum_weight_min(&self) -> LumWeightMinR {
        LumWeightMinR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Kink position of luminance weight function"]
    #[inline(always)]
    pub fn lum_weight_kink(&self) -> LumWeightKinkR {
        LumWeightKinkR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Gain select of luminance weight function"]
    #[inline(always)]
    pub fn lum_weight_gain(&self) -> LumWeightGainR {
        LumWeightGainR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Minimum value of luminance weight function\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn lum_weight_min(&mut self) -> LumWeightMinW<FiltLumWeightSpec> {
        LumWeightMinW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Kink position of luminance weight function"]
    #[inline(always)]
    #[must_use]
    pub fn lum_weight_kink(&mut self) -> LumWeightKinkW<FiltLumWeightSpec> {
        LumWeightKinkW::new(self, 8)
    }
    #[doc = "Bits 16:18 - Gain select of luminance weight function"]
    #[inline(always)]
    #[must_use]
    pub fn lum_weight_gain(&mut self) -> LumWeightGainW<FiltLumWeightSpec> {
        LumWeightGainW::new(self, 16)
    }
}
#[doc = "Parameters for luminance weight function\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_lum_weight::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_lum_weight::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FiltLumWeightSpec;
impl crate::RegisterSpec for FiltLumWeightSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filt_lum_weight::R`](R) reader structure"]
impl crate::Readable for FiltLumWeightSpec {}
#[doc = "`write(|w| ..)` method takes [`filt_lum_weight::W`](W) writer structure"]
impl crate::Writable for FiltLumWeightSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILT_LUM_WEIGHT to value 0x0002_2040"]
impl crate::Resettable for FiltLumWeightSpec {
    const RESET_VALUE: u32 = 0x0002_2040;
}
