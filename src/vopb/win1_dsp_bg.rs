#[doc = "Register `WIN1_DSP_BG` reader"]
pub type R = crate::R<Win1DspBgSpec>;
#[doc = "Register `WIN1_DSP_BG` writer"]
pub type W = crate::W<Win1DspBgSpec>;
#[doc = "Field `WIN1_DSP_BG_BLUE` reader - Win1 layer Background Blue color"]
pub type Win1DspBgBlueR = crate::FieldReader;
#[doc = "Field `WIN1_DSP_BG_BLUE` writer - Win1 layer Background Blue color"]
pub type Win1DspBgBlueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN1_DSP_BG_GREEN` reader - Win1 layer Background Green color"]
pub type Win1DspBgGreenR = crate::FieldReader;
#[doc = "Field `WIN1_DSP_BG_GREEN` writer - Win1 layer Background Green color"]
pub type Win1DspBgGreenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN1_DSP_BG_RED` reader - Win1 layer Background Red color"]
pub type Win1DspBgRedR = crate::FieldReader;
#[doc = "Field `WIN1_DSP_BG_RED` writer - Win1 layer Background Red color"]
pub type Win1DspBgRedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Win1 layer background enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1BgEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win1BgEn> for bool {
    #[inline(always)]
    fn from(variant: Win1BgEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_BG_EN` reader - Win1 layer background enable"]
pub type Win1BgEnR = crate::BitReader<Win1BgEn>;
impl Win1BgEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1BgEn {
        match self.bits {
            false => Win1BgEn::B0,
            true => Win1BgEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1BgEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1BgEn::B1
    }
}
#[doc = "Field `WIN1_BG_EN` writer - Win1 layer background enable"]
pub type Win1BgEnW<'a, REG> = crate::BitWriter<'a, REG, Win1BgEn>;
impl<'a, REG> Win1BgEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1BgEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1BgEn::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Win1 layer Background Blue color"]
    #[inline(always)]
    pub fn win1_dsp_bg_blue(&self) -> Win1DspBgBlueR {
        Win1DspBgBlueR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Win1 layer Background Green color"]
    #[inline(always)]
    pub fn win1_dsp_bg_green(&self) -> Win1DspBgGreenR {
        Win1DspBgGreenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Win1 layer Background Red color"]
    #[inline(always)]
    pub fn win1_dsp_bg_red(&self) -> Win1DspBgRedR {
        Win1DspBgRedR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Win1 layer background enable"]
    #[inline(always)]
    pub fn win1_bg_en(&self) -> Win1BgEnR {
        Win1BgEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Win1 layer Background Blue color"]
    #[inline(always)]
    #[must_use]
    pub fn win1_dsp_bg_blue(&mut self) -> Win1DspBgBlueW<Win1DspBgSpec> {
        Win1DspBgBlueW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Win1 layer Background Green color"]
    #[inline(always)]
    #[must_use]
    pub fn win1_dsp_bg_green(&mut self) -> Win1DspBgGreenW<Win1DspBgSpec> {
        Win1DspBgGreenW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Win1 layer Background Red color"]
    #[inline(always)]
    #[must_use]
    pub fn win1_dsp_bg_red(&mut self) -> Win1DspBgRedW<Win1DspBgSpec> {
        Win1DspBgRedW::new(self, 16)
    }
    #[doc = "Bit 31 - Win1 layer background enable"]
    #[inline(always)]
    #[must_use]
    pub fn win1_bg_en(&mut self) -> Win1BgEnW<Win1DspBgSpec> {
        Win1BgEnW::new(self, 31)
    }
}
#[doc = "Win1 layer background color\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_dsp_bg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_dsp_bg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win1DspBgSpec;
impl crate::RegisterSpec for Win1DspBgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win1_dsp_bg::R`](R) reader structure"]
impl crate::Readable for Win1DspBgSpec {}
#[doc = "`write(|w| ..)` method takes [`win1_dsp_bg::W`](W) writer structure"]
impl crate::Writable for Win1DspBgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN1_DSP_BG to value 0"]
impl crate::Resettable for Win1DspBgSpec {
    const RESET_VALUE: u32 = 0;
}
