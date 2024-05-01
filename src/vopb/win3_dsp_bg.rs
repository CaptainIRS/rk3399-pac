#[doc = "Register `WIN3_DSP_BG` reader"]
pub type R = crate::R<Win3DspBgSpec>;
#[doc = "Register `WIN3_DSP_BG` writer"]
pub type W = crate::W<Win3DspBgSpec>;
#[doc = "Field `WIN3_DSP_BG_BLUE` reader - Win3 layer Background Blue color"]
pub type Win3DspBgBlueR = crate::FieldReader;
#[doc = "Field `WIN3_DSP_BG_BLUE` writer - Win3 layer Background Blue color"]
pub type Win3DspBgBlueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN3_DSP_BG_GREEN` reader - Win3 layer Background Green color"]
pub type Win3DspBgGreenR = crate::FieldReader;
#[doc = "Field `WIN3_DSP_BG_GREEN` writer - Win3 layer Background Green color"]
pub type Win3DspBgGreenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN3_DSP_BG_RED` reader - Win3 layer Background Red color"]
pub type Win3DspBgRedR = crate::FieldReader;
#[doc = "Field `WIN3_DSP_BG_RED` writer - Win3 layer Background Red color"]
pub type Win3DspBgRedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Win3 layer background enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3BgEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win3BgEn> for bool {
    #[inline(always)]
    fn from(variant: Win3BgEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_BG_EN` reader - Win3 layer background enable"]
pub type Win3BgEnR = crate::BitReader<Win3BgEn>;
impl Win3BgEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3BgEn {
        match self.bits {
            false => Win3BgEn::B0,
            true => Win3BgEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3BgEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3BgEn::B1
    }
}
#[doc = "Field `WIN3_BG_EN` writer - Win3 layer background enable"]
pub type Win3BgEnW<'a, REG> = crate::BitWriter<'a, REG, Win3BgEn>;
impl<'a, REG> Win3BgEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3BgEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3BgEn::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Win3 layer Background Blue color"]
    #[inline(always)]
    pub fn win3_dsp_bg_blue(&self) -> Win3DspBgBlueR {
        Win3DspBgBlueR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Win3 layer Background Green color"]
    #[inline(always)]
    pub fn win3_dsp_bg_green(&self) -> Win3DspBgGreenR {
        Win3DspBgGreenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Win3 layer Background Red color"]
    #[inline(always)]
    pub fn win3_dsp_bg_red(&self) -> Win3DspBgRedR {
        Win3DspBgRedR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Win3 layer background enable"]
    #[inline(always)]
    pub fn win3_bg_en(&self) -> Win3BgEnR {
        Win3BgEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Win3 layer Background Blue color"]
    #[inline(always)]
    #[must_use]
    pub fn win3_dsp_bg_blue(&mut self) -> Win3DspBgBlueW<Win3DspBgSpec> {
        Win3DspBgBlueW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Win3 layer Background Green color"]
    #[inline(always)]
    #[must_use]
    pub fn win3_dsp_bg_green(&mut self) -> Win3DspBgGreenW<Win3DspBgSpec> {
        Win3DspBgGreenW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Win3 layer Background Red color"]
    #[inline(always)]
    #[must_use]
    pub fn win3_dsp_bg_red(&mut self) -> Win3DspBgRedW<Win3DspBgSpec> {
        Win3DspBgRedW::new(self, 16)
    }
    #[doc = "Bit 31 - Win3 layer background enable"]
    #[inline(always)]
    #[must_use]
    pub fn win3_bg_en(&mut self) -> Win3BgEnW<Win3DspBgSpec> {
        Win3BgEnW::new(self, 31)
    }
}
#[doc = "Win3 layer background color\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_dsp_bg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_dsp_bg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win3DspBgSpec;
impl crate::RegisterSpec for Win3DspBgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win3_dsp_bg::R`](R) reader structure"]
impl crate::Readable for Win3DspBgSpec {}
#[doc = "`write(|w| ..)` method takes [`win3_dsp_bg::W`](W) writer structure"]
impl crate::Writable for Win3DspBgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN3_DSP_BG to value 0"]
impl crate::Resettable for Win3DspBgSpec {
    const RESET_VALUE: u32 = 0;
}
