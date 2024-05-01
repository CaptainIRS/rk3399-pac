#[doc = "Register `RAW_ENH_RGB_CNFG` reader"]
pub type R = crate::R<RawEnhRgbCnfgSpec>;
#[doc = "Register `RAW_ENH_RGB_CNFG` writer"]
pub type W = crate::W<RawEnhRgbCnfgSpec>;
#[doc = "enhancement radius\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EnhRadius {
    #[doc = "0: R=1"]
    B00 = 0,
    #[doc = "1: R=2"]
    B01 = 1,
    #[doc = "2: R=3"]
    B10 = 2,
    #[doc = "3: R=4"]
    B11 = 3,
}
impl From<EnhRadius> for u8 {
    #[inline(always)]
    fn from(variant: EnhRadius) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EnhRadius {
    type Ux = u8;
}
#[doc = "Field `ENH_RADIUS` reader - enhancement radius"]
pub type EnhRadiusR = crate::FieldReader<EnhRadius>;
impl EnhRadiusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnhRadius {
        match self.bits {
            0 => EnhRadius::B00,
            1 => EnhRadius::B01,
            2 => EnhRadius::B10,
            3 => EnhRadius::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "R=1"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == EnhRadius::B00
    }
    #[doc = "R=2"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == EnhRadius::B01
    }
    #[doc = "R=3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == EnhRadius::B10
    }
    #[doc = "R=4"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == EnhRadius::B11
    }
}
#[doc = "Field `ENH_RADIUS` writer - enhancement radius"]
pub type EnhRadiusW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EnhRadius>;
impl<'a, REG> EnhRadiusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "R=1"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(EnhRadius::B00)
    }
    #[doc = "R=2"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(EnhRadius::B01)
    }
    #[doc = "R=3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(EnhRadius::B10)
    }
    #[doc = "R=4"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(EnhRadius::B11)
    }
}
#[doc = "Field `ENH_ALPHA` reader - enhancement alpha value\n\n0000000:0\n\n0000001:1/16\n\n0000010:2/16\n\n......\n\n0001111:15/16\n\n0010000:1\n\n0010001:1+1/16;\n\n0010010:1+2/16;\n\n0010011:1+3/16;\n\n......\n\n0100000:2;\n\n......\n\n0110000:3;\n\n......\n\n1000000:4;\n\n......\n\n1010000:5;\n\n......\n\n1100000:6;\n\nother : reserved"]
pub type EnhAlphaR = crate::FieldReader;
#[doc = "Field `ENH_ALPHA` writer - enhancement alpha value\n\n0000000:0\n\n0000001:1/16\n\n0000010:2/16\n\n......\n\n0001111:15/16\n\n0010000:1\n\n0010001:1+1/16;\n\n0010010:1+2/16;\n\n0010011:1+3/16;\n\n......\n\n0100000:2;\n\n......\n\n0110000:3;\n\n......\n\n1000000:4;\n\n......\n\n1010000:5;\n\n......\n\n1100000:6;\n\nother : reserved"]
pub type EnhAlphaW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ENH_THRESHOLD` reader - enhancement threshold\n\nIn denoise and detail enhancement operation, more than the\n\nthreshold, considering as detail; but if less than the threshold,\n\nconsidering as noise, need to to be filtered."]
pub type EnhThresholdR = crate::FieldReader;
#[doc = "Field `ENH_THRESHOLD` writer - enhancement threshold\n\nIn denoise and detail enhancement operation, more than the\n\nthreshold, considering as detail; but if less than the threshold,\n\nconsidering as noise, need to to be filtered."]
pub type EnhThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CHROMA_TEMP_SEL` reader - 3D denoise chroma temporal coefficient select"]
pub type ChromaTempSelR = crate::FieldReader;
#[doc = "Field `CHROMA_TEMP_SEL` writer - 3D denoise chroma temporal coefficient select"]
pub type ChromaTempSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHROMA_SPAT_SEL` reader - 3D denoise chroma spatial coefficient select"]
pub type ChromaSpatSelR = crate::FieldReader;
#[doc = "Field `CHROMA_SPAT_SEL` writer - 3D denoise chroma spatial coefficient select"]
pub type ChromaSpatSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LUMA_TEMP_SEL` reader - 3D denoise luma temporal coefficient select"]
pub type LumaTempSelR = crate::FieldReader;
#[doc = "Field `LUMA_TEMP_SEL` writer - 3D denoise luma temporal coefficient select"]
pub type LumaTempSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LUMA_SPAT_SEL` reader - 3D denoise luma spatial coefficient select"]
pub type LumaSpatSelR = crate::FieldReader;
#[doc = "Field `LUMA_SPAT_SEL` writer - 3D denoise luma spatial coefficient select"]
pub type LumaSpatSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - enhancement radius"]
    #[inline(always)]
    pub fn enh_radius(&self) -> EnhRadiusR {
        EnhRadiusR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:14 - enhancement alpha value\n\n0000000:0\n\n0000001:1/16\n\n0000010:2/16\n\n......\n\n0001111:15/16\n\n0010000:1\n\n0010001:1+1/16;\n\n0010010:1+2/16;\n\n0010011:1+3/16;\n\n......\n\n0100000:2;\n\n......\n\n0110000:3;\n\n......\n\n1000000:4;\n\n......\n\n1010000:5;\n\n......\n\n1100000:6;\n\nother : reserved"]
    #[inline(always)]
    pub fn enh_alpha(&self) -> EnhAlphaR {
        EnhAlphaR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - enhancement threshold\n\nIn denoise and detail enhancement operation, more than the\n\nthreshold, considering as detail; but if less than the threshold,\n\nconsidering as noise, need to to be filtered."]
    #[inline(always)]
    pub fn enh_threshold(&self) -> EnhThresholdR {
        EnhThresholdR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - 3D denoise chroma temporal coefficient select"]
    #[inline(always)]
    pub fn chroma_temp_sel(&self) -> ChromaTempSelR {
        ChromaTempSelR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - 3D denoise chroma spatial coefficient select"]
    #[inline(always)]
    pub fn chroma_spat_sel(&self) -> ChromaSpatSelR {
        ChromaSpatSelR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - 3D denoise luma temporal coefficient select"]
    #[inline(always)]
    pub fn luma_temp_sel(&self) -> LumaTempSelR {
        LumaTempSelR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 3D denoise luma spatial coefficient select"]
    #[inline(always)]
    pub fn luma_spat_sel(&self) -> LumaSpatSelR {
        LumaSpatSelR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - enhancement radius"]
    #[inline(always)]
    #[must_use]
    pub fn enh_radius(&mut self) -> EnhRadiusW<RawEnhRgbCnfgSpec> {
        EnhRadiusW::new(self, 0)
    }
    #[doc = "Bits 8:14 - enhancement alpha value\n\n0000000:0\n\n0000001:1/16\n\n0000010:2/16\n\n......\n\n0001111:15/16\n\n0010000:1\n\n0010001:1+1/16;\n\n0010010:1+2/16;\n\n0010011:1+3/16;\n\n......\n\n0100000:2;\n\n......\n\n0110000:3;\n\n......\n\n1000000:4;\n\n......\n\n1010000:5;\n\n......\n\n1100000:6;\n\nother : reserved"]
    #[inline(always)]
    #[must_use]
    pub fn enh_alpha(&mut self) -> EnhAlphaW<RawEnhRgbCnfgSpec> {
        EnhAlphaW::new(self, 8)
    }
    #[doc = "Bits 16:23 - enhancement threshold\n\nIn denoise and detail enhancement operation, more than the\n\nthreshold, considering as detail; but if less than the threshold,\n\nconsidering as noise, need to to be filtered."]
    #[inline(always)]
    #[must_use]
    pub fn enh_threshold(&mut self) -> EnhThresholdW<RawEnhRgbCnfgSpec> {
        EnhThresholdW::new(self, 16)
    }
    #[doc = "Bits 24:25 - 3D denoise chroma temporal coefficient select"]
    #[inline(always)]
    #[must_use]
    pub fn chroma_temp_sel(&mut self) -> ChromaTempSelW<RawEnhRgbCnfgSpec> {
        ChromaTempSelW::new(self, 24)
    }
    #[doc = "Bits 26:27 - 3D denoise chroma spatial coefficient select"]
    #[inline(always)]
    #[must_use]
    pub fn chroma_spat_sel(&mut self) -> ChromaSpatSelW<RawEnhRgbCnfgSpec> {
        ChromaSpatSelW::new(self, 26)
    }
    #[doc = "Bits 28:29 - 3D denoise luma temporal coefficient select"]
    #[inline(always)]
    #[must_use]
    pub fn luma_temp_sel(&mut self) -> LumaTempSelW<RawEnhRgbCnfgSpec> {
        LumaTempSelW::new(self, 28)
    }
    #[doc = "Bits 30:31 - 3D denoise luma spatial coefficient select"]
    #[inline(always)]
    #[must_use]
    pub fn luma_spat_sel(&mut self) -> LumaSpatSelW<RawEnhRgbCnfgSpec> {
        LumaSpatSelW::new(self, 30)
    }
}
#[doc = "enhancement RGB configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_enh_rgb_cnfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`raw_enh_rgb_cnfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawEnhRgbCnfgSpec;
impl crate::RegisterSpec for RawEnhRgbCnfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_enh_rgb_cnfg::R`](R) reader structure"]
impl crate::Readable for RawEnhRgbCnfgSpec {}
#[doc = "`write(|w| ..)` method takes [`raw_enh_rgb_cnfg::W`](W) writer structure"]
impl crate::Writable for RawEnhRgbCnfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAW_ENH_RGB_CNFG to value 0"]
impl crate::Resettable for RawEnhRgbCnfgSpec {
    const RESET_VALUE: u32 = 0;
}
