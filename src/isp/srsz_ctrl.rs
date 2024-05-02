#[doc = "Register `SRSZ_CTRL` reader"]
pub type R = crate::R<SrszCtrlSpec>;
#[doc = "Register `SRSZ_CTRL` writer"]
pub type W = crate::W<SrszCtrlSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaleHyEnable {
    #[doc = "0: bypass horizontal luminance scaling unit"]
    B0 = 0,
    #[doc = "1: enable horizontal luminance scaling unit"]
    B1 = 1,
}
impl From<ScaleHyEnable> for bool {
    #[inline(always)]
    fn from(variant: ScaleHyEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `scale_hy_enable` reader - "]
pub type ScaleHyEnableR = crate::BitReader<ScaleHyEnable>;
impl ScaleHyEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScaleHyEnable {
        match self.bits {
            false => ScaleHyEnable::B0,
            true => ScaleHyEnable::B1,
        }
    }
    #[doc = "bypass horizontal luminance scaling unit"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ScaleHyEnable::B0
    }
    #[doc = "enable horizontal luminance scaling unit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ScaleHyEnable::B1
    }
}
#[doc = "Field `scale_hy_enable` writer - "]
pub type ScaleHyEnableW<'a, REG> = crate::BitWriter<'a, REG, ScaleHyEnable>;
impl<'a, REG> ScaleHyEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bypass horizontal luminance scaling unit"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ScaleHyEnable::B0)
    }
    #[doc = "enable horizontal luminance scaling unit"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ScaleHyEnable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaleHcEnable {
    #[doc = "0: bypass horizontal chrominance scaling unit"]
    B0 = 0,
    #[doc = "1: enable horizontal chrominance scaling unit"]
    B1 = 1,
}
impl From<ScaleHcEnable> for bool {
    #[inline(always)]
    fn from(variant: ScaleHcEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `scale_hc_enable` reader - "]
pub type ScaleHcEnableR = crate::BitReader<ScaleHcEnable>;
impl ScaleHcEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScaleHcEnable {
        match self.bits {
            false => ScaleHcEnable::B0,
            true => ScaleHcEnable::B1,
        }
    }
    #[doc = "bypass horizontal chrominance scaling unit"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ScaleHcEnable::B0
    }
    #[doc = "enable horizontal chrominance scaling unit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ScaleHcEnable::B1
    }
}
#[doc = "Field `scale_hc_enable` writer - "]
pub type ScaleHcEnableW<'a, REG> = crate::BitWriter<'a, REG, ScaleHcEnable>;
impl<'a, REG> ScaleHcEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bypass horizontal chrominance scaling unit"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ScaleHcEnable::B0)
    }
    #[doc = "enable horizontal chrominance scaling unit"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ScaleHcEnable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaleVyEnable {
    #[doc = "0: bypass vertical luminance scaling unit"]
    B0 = 0,
    #[doc = "1: enable vertical luminance scaling unit"]
    B1 = 1,
}
impl From<ScaleVyEnable> for bool {
    #[inline(always)]
    fn from(variant: ScaleVyEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `scale_vy_enable` reader - "]
pub type ScaleVyEnableR = crate::BitReader<ScaleVyEnable>;
impl ScaleVyEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScaleVyEnable {
        match self.bits {
            false => ScaleVyEnable::B0,
            true => ScaleVyEnable::B1,
        }
    }
    #[doc = "bypass vertical luminance scaling unit"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ScaleVyEnable::B0
    }
    #[doc = "enable vertical luminance scaling unit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ScaleVyEnable::B1
    }
}
#[doc = "Field `scale_vy_enable` writer - "]
pub type ScaleVyEnableW<'a, REG> = crate::BitWriter<'a, REG, ScaleVyEnable>;
impl<'a, REG> ScaleVyEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bypass vertical luminance scaling unit"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ScaleVyEnable::B0)
    }
    #[doc = "enable vertical luminance scaling unit"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ScaleVyEnable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaleVcEnable {
    #[doc = "0: bypass vertical chrominance scaling unit"]
    B0 = 0,
    #[doc = "1: enable vertical chrominance scaling unit"]
    B1 = 1,
}
impl From<ScaleVcEnable> for bool {
    #[inline(always)]
    fn from(variant: ScaleVcEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `scale_vc_enable` reader - "]
pub type ScaleVcEnableR = crate::BitReader<ScaleVcEnable>;
impl ScaleVcEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScaleVcEnable {
        match self.bits {
            false => ScaleVcEnable::B0,
            true => ScaleVcEnable::B1,
        }
    }
    #[doc = "bypass vertical chrominance scaling unit"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ScaleVcEnable::B0
    }
    #[doc = "enable vertical chrominance scaling unit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ScaleVcEnable::B1
    }
}
#[doc = "Field `scale_vc_enable` writer - "]
pub type ScaleVcEnableW<'a, REG> = crate::BitWriter<'a, REG, ScaleVcEnable>;
impl<'a, REG> ScaleVcEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bypass vertical chrominance scaling unit"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ScaleVcEnable::B0)
    }
    #[doc = "enable vertical chrominance scaling unit"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ScaleVcEnable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaleHyUp {
    #[doc = "1: horizontal luminance upscaling selected"]
    B1 = 1,
    #[doc = "0: horizontal luminance downscaling selected"]
    B0 = 0,
}
impl From<ScaleHyUp> for bool {
    #[inline(always)]
    fn from(variant: ScaleHyUp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `scale_hy_up` reader - "]
pub type ScaleHyUpR = crate::BitReader<ScaleHyUp>;
impl ScaleHyUpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScaleHyUp {
        match self.bits {
            true => ScaleHyUp::B1,
            false => ScaleHyUp::B0,
        }
    }
    #[doc = "horizontal luminance upscaling selected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ScaleHyUp::B1
    }
    #[doc = "horizontal luminance downscaling selected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ScaleHyUp::B0
    }
}
#[doc = "Field `scale_hy_up` writer - "]
pub type ScaleHyUpW<'a, REG> = crate::BitWriter<'a, REG, ScaleHyUp>;
impl<'a, REG> ScaleHyUpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "horizontal luminance upscaling selected"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ScaleHyUp::B1)
    }
    #[doc = "horizontal luminance downscaling selected"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ScaleHyUp::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaleHcUp {
    #[doc = "1: horizontal chrominance upscaling selected"]
    B1 = 1,
    #[doc = "0: horizontal chrominance downscaling selected"]
    B0 = 0,
}
impl From<ScaleHcUp> for bool {
    #[inline(always)]
    fn from(variant: ScaleHcUp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `scale_hc_up` reader - "]
pub type ScaleHcUpR = crate::BitReader<ScaleHcUp>;
impl ScaleHcUpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScaleHcUp {
        match self.bits {
            true => ScaleHcUp::B1,
            false => ScaleHcUp::B0,
        }
    }
    #[doc = "horizontal chrominance upscaling selected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ScaleHcUp::B1
    }
    #[doc = "horizontal chrominance downscaling selected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ScaleHcUp::B0
    }
}
#[doc = "Field `scale_hc_up` writer - "]
pub type ScaleHcUpW<'a, REG> = crate::BitWriter<'a, REG, ScaleHcUp>;
impl<'a, REG> ScaleHcUpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "horizontal chrominance upscaling selected"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ScaleHcUp::B1)
    }
    #[doc = "horizontal chrominance downscaling selected"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ScaleHcUp::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaleVyUp {
    #[doc = "1: vertical luminance upscaling selected"]
    B1 = 1,
    #[doc = "0: vertical luminance downscaling selected"]
    B0 = 0,
}
impl From<ScaleVyUp> for bool {
    #[inline(always)]
    fn from(variant: ScaleVyUp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `scale_vy_up` reader - "]
pub type ScaleVyUpR = crate::BitReader<ScaleVyUp>;
impl ScaleVyUpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScaleVyUp {
        match self.bits {
            true => ScaleVyUp::B1,
            false => ScaleVyUp::B0,
        }
    }
    #[doc = "vertical luminance upscaling selected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ScaleVyUp::B1
    }
    #[doc = "vertical luminance downscaling selected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ScaleVyUp::B0
    }
}
#[doc = "Field `scale_vy_up` writer - "]
pub type ScaleVyUpW<'a, REG> = crate::BitWriter<'a, REG, ScaleVyUp>;
impl<'a, REG> ScaleVyUpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vertical luminance upscaling selected"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ScaleVyUp::B1)
    }
    #[doc = "vertical luminance downscaling selected"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ScaleVyUp::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaleVcUp {
    #[doc = "1: vertical chrominance upscaling selected"]
    B1 = 1,
    #[doc = "0: vertical chrominance downscaling selected"]
    B0 = 0,
}
impl From<ScaleVcUp> for bool {
    #[inline(always)]
    fn from(variant: ScaleVcUp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `scale_vc_up` reader - "]
pub type ScaleVcUpR = crate::BitReader<ScaleVcUp>;
impl ScaleVcUpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScaleVcUp {
        match self.bits {
            true => ScaleVcUp::B1,
            false => ScaleVcUp::B0,
        }
    }
    #[doc = "vertical chrominance upscaling selected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ScaleVcUp::B1
    }
    #[doc = "vertical chrominance downscaling selected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ScaleVcUp::B0
    }
}
#[doc = "Field `scale_vc_up` writer - "]
pub type ScaleVcUpW<'a, REG> = crate::BitWriter<'a, REG, ScaleVcUp>;
impl<'a, REG> ScaleVcUpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vertical chrominance upscaling selected"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ScaleVcUp::B1)
    }
    #[doc = "vertical chrominance downscaling selected"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ScaleVcUp::B0)
    }
}
#[doc = "Field `cfg_upd` reader - write 0: nothing happens\n\nwrite 1: update shadow\n\nregisters read: always 0"]
pub type CfgUpdR = crate::BitReader;
#[doc = "Field `cfg_upd` writer - write 0: nothing happens\n\nwrite 1: update shadow\n\nregisters read: always 0"]
pub type CfgUpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutoUpd {
    #[doc = "1: automatic register update at frame end enabled."]
    B1 = 1,
    #[doc = "0: automatic register update at frame end disabled."]
    B0 = 0,
}
impl From<AutoUpd> for bool {
    #[inline(always)]
    fn from(variant: AutoUpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `auto_upd` reader - "]
pub type AutoUpdR = crate::BitReader<AutoUpd>;
impl AutoUpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AutoUpd {
        match self.bits {
            true => AutoUpd::B1,
            false => AutoUpd::B0,
        }
    }
    #[doc = "automatic register update at frame end enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AutoUpd::B1
    }
    #[doc = "automatic register update at frame end disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AutoUpd::B0
    }
}
#[doc = "Field `auto_upd` writer - "]
pub type AutoUpdW<'a, REG> = crate::BitWriter<'a, REG, AutoUpd>;
impl<'a, REG> AutoUpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "automatic register update at frame end enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AutoUpd::B1)
    }
    #[doc = "automatic register update at frame end disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AutoUpd::B0)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn scale_hy_enable(&self) -> ScaleHyEnableR {
        ScaleHyEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn scale_hc_enable(&self) -> ScaleHcEnableR {
        ScaleHcEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn scale_vy_enable(&self) -> ScaleVyEnableR {
        ScaleVyEnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn scale_vc_enable(&self) -> ScaleVcEnableR {
        ScaleVcEnableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn scale_hy_up(&self) -> ScaleHyUpR {
        ScaleHyUpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn scale_hc_up(&self) -> ScaleHcUpR {
        ScaleHcUpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn scale_vy_up(&self) -> ScaleVyUpR {
        ScaleVyUpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn scale_vc_up(&self) -> ScaleVcUpR {
        ScaleVcUpR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - write 0: nothing happens\n\nwrite 1: update shadow\n\nregisters read: always 0"]
    #[inline(always)]
    pub fn cfg_upd(&self) -> CfgUpdR {
        CfgUpdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn auto_upd(&self) -> AutoUpdR {
        AutoUpdR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn scale_hy_enable(&mut self) -> ScaleHyEnableW<SrszCtrlSpec> {
        ScaleHyEnableW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn scale_hc_enable(&mut self) -> ScaleHcEnableW<SrszCtrlSpec> {
        ScaleHcEnableW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn scale_vy_enable(&mut self) -> ScaleVyEnableW<SrszCtrlSpec> {
        ScaleVyEnableW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn scale_vc_enable(&mut self) -> ScaleVcEnableW<SrszCtrlSpec> {
        ScaleVcEnableW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn scale_hy_up(&mut self) -> ScaleHyUpW<SrszCtrlSpec> {
        ScaleHyUpW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn scale_hc_up(&mut self) -> ScaleHcUpW<SrszCtrlSpec> {
        ScaleHcUpW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn scale_vy_up(&mut self) -> ScaleVyUpW<SrszCtrlSpec> {
        ScaleVyUpW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn scale_vc_up(&mut self) -> ScaleVcUpW<SrszCtrlSpec> {
        ScaleVcUpW::new(self, 7)
    }
    #[doc = "Bit 8 - write 0: nothing happens\n\nwrite 1: update shadow\n\nregisters read: always 0"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_upd(&mut self) -> CfgUpdW<SrszCtrlSpec> {
        CfgUpdW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn auto_upd(&mut self) -> AutoUpdW<SrszCtrlSpec> {
        AutoUpdW::new(self, 9)
    }
}
#[doc = "global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsz_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrszCtrlSpec;
impl crate::RegisterSpec for SrszCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srsz_ctrl::R`](R) reader structure"]
impl crate::Readable for SrszCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`srsz_ctrl::W`](W) writer structure"]
impl crate::Writable for SrszCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRSZ_CTRL to value 0"]
impl crate::Resettable for SrszCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
