#[doc = "Register `WIN0_DSP_BG` reader"]
pub type R = crate::R<Win0DspBgSpec>;
#[doc = "Register `WIN0_DSP_BG` writer"]
pub type W = crate::W<Win0DspBgSpec>;
#[doc = "Field `WIN0_DSP_BG_BLUE` reader - Win0 layer Background Blue color"]
pub type Win0DspBgBlueR = crate::FieldReader;
#[doc = "Field `WIN0_DSP_BG_BLUE` writer - Win0 layer Background Blue color"]
pub type Win0DspBgBlueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN0_DSP_BG_GREEN` reader - Win0 layer Background Green color"]
pub type Win0DspBgGreenR = crate::FieldReader;
#[doc = "Field `WIN0_DSP_BG_GREEN` writer - Win0 layer Background Green color"]
pub type Win0DspBgGreenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN0_DSP_BG_RED` reader - Win0 layer Background Red color"]
pub type Win0DspBgRedR = crate::FieldReader;
#[doc = "Field `WIN0_DSP_BG_RED` writer - Win0 layer Background Red color"]
pub type Win0DspBgRedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Win0 layer background enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0BgEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win0BgEn> for bool {
    #[inline(always)]
    fn from(variant: Win0BgEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_BG_EN` reader - Win0 layer background enable"]
pub type Win0BgEnR = crate::BitReader<Win0BgEn>;
impl Win0BgEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0BgEn {
        match self.bits {
            false => Win0BgEn::B0,
            true => Win0BgEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0BgEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0BgEn::B1
    }
}
#[doc = "Field `WIN0_BG_EN` writer - Win0 layer background enable"]
pub type Win0BgEnW<'a, REG> = crate::BitWriter<'a, REG, Win0BgEn>;
impl<'a, REG> Win0BgEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0BgEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0BgEn::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Win0 layer Background Blue color"]
    #[inline(always)]
    pub fn win0_dsp_bg_blue(&self) -> Win0DspBgBlueR {
        Win0DspBgBlueR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Win0 layer Background Green color"]
    #[inline(always)]
    pub fn win0_dsp_bg_green(&self) -> Win0DspBgGreenR {
        Win0DspBgGreenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Win0 layer Background Red color"]
    #[inline(always)]
    pub fn win0_dsp_bg_red(&self) -> Win0DspBgRedR {
        Win0DspBgRedR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Win0 layer background enable"]
    #[inline(always)]
    pub fn win0_bg_en(&self) -> Win0BgEnR {
        Win0BgEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Win0 layer Background Blue color"]
    #[inline(always)]
    #[must_use]
    pub fn win0_dsp_bg_blue(&mut self) -> Win0DspBgBlueW<Win0DspBgSpec> {
        Win0DspBgBlueW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Win0 layer Background Green color"]
    #[inline(always)]
    #[must_use]
    pub fn win0_dsp_bg_green(&mut self) -> Win0DspBgGreenW<Win0DspBgSpec> {
        Win0DspBgGreenW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Win0 layer Background Red color"]
    #[inline(always)]
    #[must_use]
    pub fn win0_dsp_bg_red(&mut self) -> Win0DspBgRedW<Win0DspBgSpec> {
        Win0DspBgRedW::new(self, 16)
    }
    #[doc = "Bit 31 - Win0 layer background enable"]
    #[inline(always)]
    #[must_use]
    pub fn win0_bg_en(&mut self) -> Win0BgEnW<Win0DspBgSpec> {
        Win0BgEnW::new(self, 31)
    }
}
#[doc = "Win0 layer background color\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_dsp_bg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_dsp_bg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win0DspBgSpec;
impl crate::RegisterSpec for Win0DspBgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win0_dsp_bg::R`](R) reader structure"]
impl crate::Readable for Win0DspBgSpec {}
#[doc = "`write(|w| ..)` method takes [`win0_dsp_bg::W`](W) writer structure"]
impl crate::Writable for Win0DspBgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN0_DSP_BG to value 0"]
impl crate::Resettable for Win0DspBgSpec {
    const RESET_VALUE: u32 = 0;
}
