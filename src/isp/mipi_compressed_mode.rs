#[doc = "Register `MIPI_COMPRESSED_MODE` reader"]
pub type R = crate::R<MipiCompressedModeSpec>;
#[doc = "Register `MIPI_COMPRESSED_MODE` writer"]
pub type W = crate::W<MipiCompressedModeSpec>;
#[doc = "Field `compress_en` reader - 1: enable compressed mode\n\nprocessing 0: disable compressed\n\nmode"]
pub type CompressEnR = crate::BitReader;
#[doc = "Field `compress_en` writer - 1: enable compressed mode\n\nprocessing 0: disable compressed\n\nmode"]
pub type CompressEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "data compression scheme:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CompScheme {
    #[doc = "0: 12–8–12"]
    D0 = 0,
    #[doc = "1: 12–7–12"]
    D1 = 1,
    #[doc = "2: 12–6–12"]
    D2 = 2,
    #[doc = "3: 10–8–10"]
    D3 = 3,
    #[doc = "4: 10–7–10"]
    D4 = 4,
    #[doc = "5: 10–6–10 6..7: reserved"]
    D5 = 5,
}
impl From<CompScheme> for u8 {
    #[inline(always)]
    fn from(variant: CompScheme) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CompScheme {
    type Ux = u8;
}
#[doc = "Field `comp_scheme` reader - data compression scheme:"]
pub type CompSchemeR = crate::FieldReader<CompScheme>;
impl CompSchemeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CompScheme> {
        match self.bits {
            0 => Some(CompScheme::D0),
            1 => Some(CompScheme::D1),
            2 => Some(CompScheme::D2),
            3 => Some(CompScheme::D3),
            4 => Some(CompScheme::D4),
            5 => Some(CompScheme::D5),
            _ => None,
        }
    }
    #[doc = "12–8–12"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == CompScheme::D0
    }
    #[doc = "12–7–12"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == CompScheme::D1
    }
    #[doc = "12–6–12"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == CompScheme::D2
    }
    #[doc = "10–8–10"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == CompScheme::D3
    }
    #[doc = "10–7–10"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == CompScheme::D4
    }
    #[doc = "10–6–10 6..7: reserved"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == CompScheme::D5
    }
}
#[doc = "Field `comp_scheme` writer - data compression scheme:"]
pub type CompSchemeW<'a, REG> = crate::FieldWriter<'a, REG, 3, CompScheme>;
impl<'a, REG> CompSchemeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12–8–12"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(CompScheme::D0)
    }
    #[doc = "12–7–12"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(CompScheme::D1)
    }
    #[doc = "12–6–12"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(CompScheme::D2)
    }
    #[doc = "10–8–10"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(CompScheme::D3)
    }
    #[doc = "10–7–10"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut crate::W<REG> {
        self.variant(CompScheme::D4)
    }
    #[doc = "10–6–10 6..7: reserved"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut crate::W<REG> {
        self.variant(CompScheme::D5)
    }
}
#[doc = "Field `predictor_sel` reader - predictor to be\n\nused: 0: predictor1\n\n1: predictor 2"]
pub type PredictorSelR = crate::BitReader;
#[doc = "Field `predictor_sel` writer - predictor to be\n\nused: 0: predictor1\n\n1: predictor 2"]
pub type PredictorSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: enable compressed mode\n\nprocessing 0: disable compressed\n\nmode"]
    #[inline(always)]
    pub fn compress_en(&self) -> CompressEnR {
        CompressEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - data compression scheme:"]
    #[inline(always)]
    pub fn comp_scheme(&self) -> CompSchemeR {
        CompSchemeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - predictor to be\n\nused: 0: predictor1\n\n1: predictor 2"]
    #[inline(always)]
    pub fn predictor_sel(&self) -> PredictorSelR {
        PredictorSelR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: enable compressed mode\n\nprocessing 0: disable compressed\n\nmode"]
    #[inline(always)]
    #[must_use]
    pub fn compress_en(&mut self) -> CompressEnW<MipiCompressedModeSpec> {
        CompressEnW::new(self, 0)
    }
    #[doc = "Bits 4:6 - data compression scheme:"]
    #[inline(always)]
    #[must_use]
    pub fn comp_scheme(&mut self) -> CompSchemeW<MipiCompressedModeSpec> {
        CompSchemeW::new(self, 4)
    }
    #[doc = "Bit 8 - predictor to be\n\nused: 0: predictor1\n\n1: predictor 2"]
    #[inline(always)]
    #[must_use]
    pub fn predictor_sel(&mut self) -> PredictorSelW<MipiCompressedModeSpec> {
        PredictorSelW::new(self, 8)
    }
}
#[doc = "controls processing of compressed raw data types\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_compressed_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_compressed_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiCompressedModeSpec;
impl crate::RegisterSpec for MipiCompressedModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_compressed_mode::R`](R) reader structure"]
impl crate::Readable for MipiCompressedModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mipi_compressed_mode::W`](W) writer structure"]
impl crate::Writable for MipiCompressedModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIPI_COMPRESSED_MODE to value 0"]
impl crate::Resettable for MipiCompressedModeSpec {
    const RESET_VALUE: u32 = 0;
}
