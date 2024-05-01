#[doc = "Register `WIN2_DSP_BG` reader"]
pub type R = crate::R<Win2DspBgSpec>;
#[doc = "Register `WIN2_DSP_BG` writer"]
pub type W = crate::W<Win2DspBgSpec>;
#[doc = "Field `WIN2_DSP_BG_BLUE` reader - Win2 layer Background Blue color"]
pub type Win2DspBgBlueR = crate::FieldReader;
#[doc = "Field `WIN2_DSP_BG_BLUE` writer - Win2 layer Background Blue color"]
pub type Win2DspBgBlueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN2_DSP_BG_GREEN` reader - Win2 layer Background Green color"]
pub type Win2DspBgGreenR = crate::FieldReader;
#[doc = "Field `WIN2_DSP_BG_GREEN` writer - Win2 layer Background Green color"]
pub type Win2DspBgGreenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN2_DSP_BG_RED` reader - Win2 layer Background Red color"]
pub type Win2DspBgRedR = crate::FieldReader;
#[doc = "Field `WIN2_DSP_BG_RED` writer - Win2 layer Background Red color"]
pub type Win2DspBgRedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Win2 layer background enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2BgEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win2BgEn> for bool {
    #[inline(always)]
    fn from(variant: Win2BgEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_BG_EN` reader - Win2 layer background enable"]
pub type Win2BgEnR = crate::BitReader<Win2BgEn>;
impl Win2BgEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2BgEn {
        match self.bits {
            false => Win2BgEn::B0,
            true => Win2BgEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2BgEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2BgEn::B1
    }
}
#[doc = "Field `WIN2_BG_EN` writer - Win2 layer background enable"]
pub type Win2BgEnW<'a, REG> = crate::BitWriter<'a, REG, Win2BgEn>;
impl<'a, REG> Win2BgEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2BgEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2BgEn::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Win2 layer Background Blue color"]
    #[inline(always)]
    pub fn win2_dsp_bg_blue(&self) -> Win2DspBgBlueR {
        Win2DspBgBlueR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Win2 layer Background Green color"]
    #[inline(always)]
    pub fn win2_dsp_bg_green(&self) -> Win2DspBgGreenR {
        Win2DspBgGreenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Win2 layer Background Red color"]
    #[inline(always)]
    pub fn win2_dsp_bg_red(&self) -> Win2DspBgRedR {
        Win2DspBgRedR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Win2 layer background enable"]
    #[inline(always)]
    pub fn win2_bg_en(&self) -> Win2BgEnR {
        Win2BgEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Win2 layer Background Blue color"]
    #[inline(always)]
    #[must_use]
    pub fn win2_dsp_bg_blue(&mut self) -> Win2DspBgBlueW<Win2DspBgSpec> {
        Win2DspBgBlueW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Win2 layer Background Green color"]
    #[inline(always)]
    #[must_use]
    pub fn win2_dsp_bg_green(&mut self) -> Win2DspBgGreenW<Win2DspBgSpec> {
        Win2DspBgGreenW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Win2 layer Background Red color"]
    #[inline(always)]
    #[must_use]
    pub fn win2_dsp_bg_red(&mut self) -> Win2DspBgRedW<Win2DspBgSpec> {
        Win2DspBgRedW::new(self, 16)
    }
    #[doc = "Bit 31 - Win2 layer background enable"]
    #[inline(always)]
    #[must_use]
    pub fn win2_bg_en(&mut self) -> Win2BgEnW<Win2DspBgSpec> {
        Win2BgEnW::new(self, 31)
    }
}
#[doc = "Win2 layer background color\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_dsp_bg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_dsp_bg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win2DspBgSpec;
impl crate::RegisterSpec for Win2DspBgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win2_dsp_bg::R`](R) reader structure"]
impl crate::Readable for Win2DspBgSpec {}
#[doc = "`write(|w| ..)` method takes [`win2_dsp_bg::W`](W) writer structure"]
impl crate::Writable for Win2DspBgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN2_DSP_BG to value 0"]
impl crate::Resettable for Win2DspBgSpec {
    const RESET_VALUE: u32 = 0;
}
