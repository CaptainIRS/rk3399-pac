#[doc = "Register `DSP_CTRL1` reader"]
pub type R = crate::R<DspCtrl1Spec>;
#[doc = "Register `DSP_CTRL1` writer"]
pub type W = crate::W<DspCtrl1Spec>;
#[doc = "Display LUT ram enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DspLutEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable *This bit should be '0' when CPU updates the LUT, and should be '1' when Display LUT mode enable."]
    B1 = 1,
}
impl From<DspLutEn> for bool {
    #[inline(always)]
    fn from(variant: DspLutEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP_LUT_EN` reader - Display LUT ram enable"]
pub type DspLutEnR = crate::BitReader<DspLutEn>;
impl DspLutEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DspLutEn {
        match self.bits {
            false => DspLutEn::B0,
            true => DspLutEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DspLutEn::B0
    }
    #[doc = "enable *This bit should be '0' when CPU updates the LUT, and should be '1' when Display LUT mode enable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DspLutEn::B1
    }
}
#[doc = "Field `DSP_LUT_EN` writer - Display LUT ram enable"]
pub type DspLutEnW<'a, REG> = crate::BitWriter<'a, REG, DspLutEn>;
impl<'a, REG> DspLutEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DspLutEn::B0)
    }
    #[doc = "enable *This bit should be '0' when CPU updates the LUT, and should be '1' when Display LUT mode enable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DspLutEn::B1)
    }
}
#[doc = "Field `PRE_DITHER_DOWN_EN` reader - 10bit -> 8bit (allegro)"]
pub type PreDitherDownEnR = crate::BitReader;
#[doc = "Field `PRE_DITHER_DOWN_EN` writer - 10bit -> 8bit (allegro)"]
pub type PreDitherDownEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Dither-down enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DitherDownEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<DitherDownEn> for bool {
    #[inline(always)]
    fn from(variant: DitherDownEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DITHER_DOWN_EN` reader - Dither-down enable"]
pub type DitherDownEnR = crate::BitReader<DitherDownEn>;
impl DitherDownEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DitherDownEn {
        match self.bits {
            false => DitherDownEn::B0,
            true => DitherDownEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DitherDownEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DitherDownEn::B1
    }
}
#[doc = "Field `DITHER_DOWN_EN` writer - Dither-down enable"]
pub type DitherDownEnW<'a, REG> = crate::BitWriter<'a, REG, DitherDownEn>;
impl<'a, REG> DitherDownEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DitherDownEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DitherDownEn::B1)
    }
}
#[doc = "Dither-down mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DitherDownMode {
    #[doc = "0: RGB888 to RGB565"]
    B0 = 0,
    #[doc = "1: RGB888 to RGB666"]
    B1 = 1,
}
impl From<DitherDownMode> for bool {
    #[inline(always)]
    fn from(variant: DitherDownMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DITHER_DOWN_MODE` reader - Dither-down mode"]
pub type DitherDownModeR = crate::BitReader<DitherDownMode>;
impl DitherDownModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DitherDownMode {
        match self.bits {
            false => DitherDownMode::B0,
            true => DitherDownMode::B1,
        }
    }
    #[doc = "RGB888 to RGB565"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DitherDownMode::B0
    }
    #[doc = "RGB888 to RGB666"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DitherDownMode::B1
    }
}
#[doc = "Field `DITHER_DOWN_MODE` writer - Dither-down mode"]
pub type DitherDownModeW<'a, REG> = crate::BitWriter<'a, REG, DitherDownMode>;
impl<'a, REG> DitherDownModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RGB888 to RGB565"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DitherDownMode::B0)
    }
    #[doc = "RGB888 to RGB666"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DitherDownMode::B1)
    }
}
#[doc = "dither down mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DitherDownSel {
    #[doc = "0: allegro"]
    B0 = 0,
    #[doc = "1: FRC"]
    B1 = 1,
}
impl From<DitherDownSel> for bool {
    #[inline(always)]
    fn from(variant: DitherDownSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DITHER_DOWN_SEL` reader - dither down mode select"]
pub type DitherDownSelR = crate::BitReader<DitherDownSel>;
impl DitherDownSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DitherDownSel {
        match self.bits {
            false => DitherDownSel::B0,
            true => DitherDownSel::B1,
        }
    }
    #[doc = "allegro"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DitherDownSel::B0
    }
    #[doc = "FRC"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DitherDownSel::B1
    }
}
#[doc = "Field `DITHER_DOWN_SEL` writer - dither down mode select"]
pub type DitherDownSelW<'a, REG> = crate::BitWriter<'a, REG, DitherDownSel>;
impl<'a, REG> DitherDownSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "allegro"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DitherDownSel::B0)
    }
    #[doc = "FRC"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DitherDownSel::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DitherUpEn {
    #[doc = "0: no dither up"]
    B0 = 0,
    #[doc = "1: rgb565 dither up to rgb888"]
    B1 = 1,
}
impl From<DitherUpEn> for bool {
    #[inline(always)]
    fn from(variant: DitherUpEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DITHER_UP_EN` reader - "]
pub type DitherUpEnR = crate::BitReader<DitherUpEn>;
impl DitherUpEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DitherUpEn {
        match self.bits {
            false => DitherUpEn::B0,
            true => DitherUpEn::B1,
        }
    }
    #[doc = "no dither up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DitherUpEn::B0
    }
    #[doc = "rgb565 dither up to rgb888"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DitherUpEn::B1
    }
}
#[doc = "Field `DITHER_UP_EN` writer - "]
pub type DitherUpEnW<'a, REG> = crate::BitWriter<'a, REG, DitherUpEn>;
impl<'a, REG> DitherUpEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no dither up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DitherUpEn::B0)
    }
    #[doc = "rgb565 dither up to rgb888"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DitherUpEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpdateGammaLut {
    #[doc = "0: no update gamma_lut"]
    B0 = 0,
    #[doc = "1: update gamma_lut"]
    B1 = 1,
}
impl From<UpdateGammaLut> for bool {
    #[inline(always)]
    fn from(variant: UpdateGammaLut) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDATE_GAMMA_LUT` reader - "]
pub type UpdateGammaLutR = crate::BitReader<UpdateGammaLut>;
impl UpdateGammaLutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UpdateGammaLut {
        match self.bits {
            false => UpdateGammaLut::B0,
            true => UpdateGammaLut::B1,
        }
    }
    #[doc = "no update gamma_lut"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == UpdateGammaLut::B0
    }
    #[doc = "update gamma_lut"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == UpdateGammaLut::B1
    }
}
#[doc = "Field `UPDATE_GAMMA_LUT` writer - "]
pub type UpdateGammaLutW<'a, REG> = crate::BitWriter<'a, REG, UpdateGammaLut>;
impl<'a, REG> UpdateGammaLutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no update gamma_lut"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(UpdateGammaLut::B0)
    }
    #[doc = "update gamma_lut"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(UpdateGammaLut::B1)
    }
}
#[doc = "Field `DSP_LAYER0_SEL` reader - layer0 selection"]
pub type DspLayer0SelR = crate::FieldReader;
#[doc = "Field `DSP_LAYER0_SEL` writer - layer0 selection"]
pub type DspLayer0SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DSP_LAYER1_SEL` reader - layer1 selection"]
pub type DspLayer1SelR = crate::FieldReader;
#[doc = "Field `DSP_LAYER1_SEL` writer - layer1 selection"]
pub type DspLayer1SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DSP_LAYER2_SEL` reader - layer2 selection"]
pub type DspLayer2SelR = crate::FieldReader;
#[doc = "Field `DSP_LAYER2_SEL` writer - layer2 selection"]
pub type DspLayer2SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DSP_LAYER3_SEL` reader - layer3 selection"]
pub type DspLayer3SelR = crate::FieldReader;
#[doc = "Field `DSP_LAYER3_SEL` writer - layer3 selection"]
pub type DspLayer3SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "HSYNC polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DpLvdsHsyncPol {
    #[doc = "0: negative"]
    B0 = 0,
    #[doc = "1: positive"]
    B1 = 1,
}
impl From<DpLvdsHsyncPol> for bool {
    #[inline(always)]
    fn from(variant: DpLvdsHsyncPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DP_LVDS_HSYNC_POL` reader - HSYNC polarity"]
pub type DpLvdsHsyncPolR = crate::BitReader<DpLvdsHsyncPol>;
impl DpLvdsHsyncPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DpLvdsHsyncPol {
        match self.bits {
            false => DpLvdsHsyncPol::B0,
            true => DpLvdsHsyncPol::B1,
        }
    }
    #[doc = "negative"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DpLvdsHsyncPol::B0
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DpLvdsHsyncPol::B1
    }
}
#[doc = "Field `DP_LVDS_HSYNC_POL` writer - HSYNC polarity"]
pub type DpLvdsHsyncPolW<'a, REG> = crate::BitWriter<'a, REG, DpLvdsHsyncPol>;
impl<'a, REG> DpLvdsHsyncPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "negative"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DpLvdsHsyncPol::B0)
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DpLvdsHsyncPol::B1)
    }
}
#[doc = "VSYNC polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DpLvdsVsyncPol {
    #[doc = "0: negative"]
    B0 = 0,
    #[doc = "1: positive"]
    B1 = 1,
}
impl From<DpLvdsVsyncPol> for bool {
    #[inline(always)]
    fn from(variant: DpLvdsVsyncPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DP_LVDS_VSYNC_POL` reader - VSYNC polarity"]
pub type DpLvdsVsyncPolR = crate::BitReader<DpLvdsVsyncPol>;
impl DpLvdsVsyncPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DpLvdsVsyncPol {
        match self.bits {
            false => DpLvdsVsyncPol::B0,
            true => DpLvdsVsyncPol::B1,
        }
    }
    #[doc = "negative"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DpLvdsVsyncPol::B0
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DpLvdsVsyncPol::B1
    }
}
#[doc = "Field `DP_LVDS_VSYNC_POL` writer - VSYNC polarity"]
pub type DpLvdsVsyncPolW<'a, REG> = crate::BitWriter<'a, REG, DpLvdsVsyncPol>;
impl<'a, REG> DpLvdsVsyncPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "negative"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DpLvdsVsyncPol::B0)
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DpLvdsVsyncPol::B1)
    }
}
#[doc = "DEN polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DpLvdsDenPol {
    #[doc = "0: positive"]
    B0 = 0,
    #[doc = "1: negative"]
    B1 = 1,
}
impl From<DpLvdsDenPol> for bool {
    #[inline(always)]
    fn from(variant: DpLvdsDenPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DP_LVDS_DEN_POL` reader - DEN polarity"]
pub type DpLvdsDenPolR = crate::BitReader<DpLvdsDenPol>;
impl DpLvdsDenPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DpLvdsDenPol {
        match self.bits {
            false => DpLvdsDenPol::B0,
            true => DpLvdsDenPol::B1,
        }
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DpLvdsDenPol::B0
    }
    #[doc = "negative"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DpLvdsDenPol::B1
    }
}
#[doc = "Field `DP_LVDS_DEN_POL` writer - DEN polarity"]
pub type DpLvdsDenPolW<'a, REG> = crate::BitWriter<'a, REG, DpLvdsDenPol>;
impl<'a, REG> DpLvdsDenPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "positive"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DpLvdsDenPol::B0)
    }
    #[doc = "negative"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DpLvdsDenPol::B1)
    }
}
#[doc = "DCLK invert enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DpLvdsDclkPol {
    #[doc = "0: normal"]
    B0 = 0,
    #[doc = "1: invert default dclk invert"]
    B1 = 1,
}
impl From<DpLvdsDclkPol> for bool {
    #[inline(always)]
    fn from(variant: DpLvdsDclkPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DP_LVDS_DCLK_POL` reader - DCLK invert enable"]
pub type DpLvdsDclkPolR = crate::BitReader<DpLvdsDclkPol>;
impl DpLvdsDclkPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DpLvdsDclkPol {
        match self.bits {
            false => DpLvdsDclkPol::B0,
            true => DpLvdsDclkPol::B1,
        }
    }
    #[doc = "normal"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DpLvdsDclkPol::B0
    }
    #[doc = "invert default dclk invert"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DpLvdsDclkPol::B1
    }
}
#[doc = "Field `DP_LVDS_DCLK_POL` writer - DCLK invert enable"]
pub type DpLvdsDclkPolW<'a, REG> = crate::BitWriter<'a, REG, DpLvdsDclkPol>;
impl<'a, REG> DpLvdsDclkPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DpLvdsDclkPol::B0)
    }
    #[doc = "invert default dclk invert"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DpLvdsDclkPol::B1)
    }
}
#[doc = "HSYNC polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HdmiHsyncPol {
    #[doc = "0: negative"]
    B0 = 0,
    #[doc = "1: positive"]
    B1 = 1,
}
impl From<HdmiHsyncPol> for bool {
    #[inline(always)]
    fn from(variant: HdmiHsyncPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDMI_HSYNC_POL` reader - HSYNC polarity"]
pub type HdmiHsyncPolR = crate::BitReader<HdmiHsyncPol>;
impl HdmiHsyncPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HdmiHsyncPol {
        match self.bits {
            false => HdmiHsyncPol::B0,
            true => HdmiHsyncPol::B1,
        }
    }
    #[doc = "negative"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HdmiHsyncPol::B0
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HdmiHsyncPol::B1
    }
}
#[doc = "Field `HDMI_HSYNC_POL` writer - HSYNC polarity"]
pub type HdmiHsyncPolW<'a, REG> = crate::BitWriter<'a, REG, HdmiHsyncPol>;
impl<'a, REG> HdmiHsyncPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "negative"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HdmiHsyncPol::B0)
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HdmiHsyncPol::B1)
    }
}
#[doc = "VSYNC polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HdmiVsyncPol {
    #[doc = "0: negative"]
    B0 = 0,
    #[doc = "1: positive"]
    B1 = 1,
}
impl From<HdmiVsyncPol> for bool {
    #[inline(always)]
    fn from(variant: HdmiVsyncPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDMI_VSYNC_POL` reader - VSYNC polarity"]
pub type HdmiVsyncPolR = crate::BitReader<HdmiVsyncPol>;
impl HdmiVsyncPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HdmiVsyncPol {
        match self.bits {
            false => HdmiVsyncPol::B0,
            true => HdmiVsyncPol::B1,
        }
    }
    #[doc = "negative"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HdmiVsyncPol::B0
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HdmiVsyncPol::B1
    }
}
#[doc = "Field `HDMI_VSYNC_POL` writer - VSYNC polarity"]
pub type HdmiVsyncPolW<'a, REG> = crate::BitWriter<'a, REG, HdmiVsyncPol>;
impl<'a, REG> HdmiVsyncPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "negative"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HdmiVsyncPol::B0)
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HdmiVsyncPol::B1)
    }
}
#[doc = "DEN polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HdmiDenPol {
    #[doc = "0: positive"]
    B0 = 0,
    #[doc = "1: negative"]
    B1 = 1,
}
impl From<HdmiDenPol> for bool {
    #[inline(always)]
    fn from(variant: HdmiDenPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDMI_DEN_POL` reader - DEN polarity"]
pub type HdmiDenPolR = crate::BitReader<HdmiDenPol>;
impl HdmiDenPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HdmiDenPol {
        match self.bits {
            false => HdmiDenPol::B0,
            true => HdmiDenPol::B1,
        }
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HdmiDenPol::B0
    }
    #[doc = "negative"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HdmiDenPol::B1
    }
}
#[doc = "Field `HDMI_DEN_POL` writer - DEN polarity"]
pub type HdmiDenPolW<'a, REG> = crate::BitWriter<'a, REG, HdmiDenPol>;
impl<'a, REG> HdmiDenPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "positive"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HdmiDenPol::B0)
    }
    #[doc = "negative"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HdmiDenPol::B1)
    }
}
#[doc = "DCLK invert enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HdmiDclkPol {
    #[doc = "0: normal"]
    B0 = 0,
    #[doc = "1: invert default dclk invert"]
    B1 = 1,
}
impl From<HdmiDclkPol> for bool {
    #[inline(always)]
    fn from(variant: HdmiDclkPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDMI_DCLK_POL` reader - DCLK invert enable"]
pub type HdmiDclkPolR = crate::BitReader<HdmiDclkPol>;
impl HdmiDclkPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HdmiDclkPol {
        match self.bits {
            false => HdmiDclkPol::B0,
            true => HdmiDclkPol::B1,
        }
    }
    #[doc = "normal"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HdmiDclkPol::B0
    }
    #[doc = "invert default dclk invert"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HdmiDclkPol::B1
    }
}
#[doc = "Field `HDMI_DCLK_POL` writer - DCLK invert enable"]
pub type HdmiDclkPolW<'a, REG> = crate::BitWriter<'a, REG, HdmiDclkPol>;
impl<'a, REG> HdmiDclkPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HdmiDclkPol::B0)
    }
    #[doc = "invert default dclk invert"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HdmiDclkPol::B1)
    }
}
#[doc = "HSYNC polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdpHsyncPol {
    #[doc = "0: negative"]
    B0 = 0,
    #[doc = "1: positive"]
    B1 = 1,
}
impl From<EdpHsyncPol> for bool {
    #[inline(always)]
    fn from(variant: EdpHsyncPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDP_HSYNC_POL` reader - HSYNC polarity"]
pub type EdpHsyncPolR = crate::BitReader<EdpHsyncPol>;
impl EdpHsyncPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EdpHsyncPol {
        match self.bits {
            false => EdpHsyncPol::B0,
            true => EdpHsyncPol::B1,
        }
    }
    #[doc = "negative"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EdpHsyncPol::B0
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EdpHsyncPol::B1
    }
}
#[doc = "Field `EDP_HSYNC_POL` writer - HSYNC polarity"]
pub type EdpHsyncPolW<'a, REG> = crate::BitWriter<'a, REG, EdpHsyncPol>;
impl<'a, REG> EdpHsyncPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "negative"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EdpHsyncPol::B0)
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EdpHsyncPol::B1)
    }
}
#[doc = "VSYNC polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdpVsyncPol {
    #[doc = "0: negative"]
    B0 = 0,
    #[doc = "1: positive"]
    B1 = 1,
}
impl From<EdpVsyncPol> for bool {
    #[inline(always)]
    fn from(variant: EdpVsyncPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDP_VSYNC_POL` reader - VSYNC polarity"]
pub type EdpVsyncPolR = crate::BitReader<EdpVsyncPol>;
impl EdpVsyncPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EdpVsyncPol {
        match self.bits {
            false => EdpVsyncPol::B0,
            true => EdpVsyncPol::B1,
        }
    }
    #[doc = "negative"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EdpVsyncPol::B0
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EdpVsyncPol::B1
    }
}
#[doc = "Field `EDP_VSYNC_POL` writer - VSYNC polarity"]
pub type EdpVsyncPolW<'a, REG> = crate::BitWriter<'a, REG, EdpVsyncPol>;
impl<'a, REG> EdpVsyncPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "negative"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EdpVsyncPol::B0)
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EdpVsyncPol::B1)
    }
}
#[doc = "DEN polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdpDenPol {
    #[doc = "0: positive"]
    B0 = 0,
    #[doc = "1: negative"]
    B1 = 1,
}
impl From<EdpDenPol> for bool {
    #[inline(always)]
    fn from(variant: EdpDenPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDP_DEN_POL` reader - DEN polarity"]
pub type EdpDenPolR = crate::BitReader<EdpDenPol>;
impl EdpDenPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EdpDenPol {
        match self.bits {
            false => EdpDenPol::B0,
            true => EdpDenPol::B1,
        }
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EdpDenPol::B0
    }
    #[doc = "negative"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EdpDenPol::B1
    }
}
#[doc = "Field `EDP_DEN_POL` writer - DEN polarity"]
pub type EdpDenPolW<'a, REG> = crate::BitWriter<'a, REG, EdpDenPol>;
impl<'a, REG> EdpDenPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "positive"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EdpDenPol::B0)
    }
    #[doc = "negative"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EdpDenPol::B1)
    }
}
#[doc = "DCLK invert enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdpDclkPol {
    #[doc = "0: normal"]
    B0 = 0,
    #[doc = "1: invert default dclk invert"]
    B1 = 1,
}
impl From<EdpDclkPol> for bool {
    #[inline(always)]
    fn from(variant: EdpDclkPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDP_DCLK_POL` reader - DCLK invert enable"]
pub type EdpDclkPolR = crate::BitReader<EdpDclkPol>;
impl EdpDclkPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EdpDclkPol {
        match self.bits {
            false => EdpDclkPol::B0,
            true => EdpDclkPol::B1,
        }
    }
    #[doc = "normal"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EdpDclkPol::B0
    }
    #[doc = "invert default dclk invert"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EdpDclkPol::B1
    }
}
#[doc = "Field `EDP_DCLK_POL` writer - DCLK invert enable"]
pub type EdpDclkPolW<'a, REG> = crate::BitWriter<'a, REG, EdpDclkPol>;
impl<'a, REG> EdpDclkPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EdpDclkPol::B0)
    }
    #[doc = "invert default dclk invert"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EdpDclkPol::B1)
    }
}
#[doc = "HSYNC polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MipiHsyncPol {
    #[doc = "0: negative"]
    B0 = 0,
    #[doc = "1: positive"]
    B1 = 1,
}
impl From<MipiHsyncPol> for bool {
    #[inline(always)]
    fn from(variant: MipiHsyncPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIPI_HSYNC_POL` reader - HSYNC polarity"]
pub type MipiHsyncPolR = crate::BitReader<MipiHsyncPol>;
impl MipiHsyncPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MipiHsyncPol {
        match self.bits {
            false => MipiHsyncPol::B0,
            true => MipiHsyncPol::B1,
        }
    }
    #[doc = "negative"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == MipiHsyncPol::B0
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == MipiHsyncPol::B1
    }
}
#[doc = "Field `MIPI_HSYNC_POL` writer - HSYNC polarity"]
pub type MipiHsyncPolW<'a, REG> = crate::BitWriter<'a, REG, MipiHsyncPol>;
impl<'a, REG> MipiHsyncPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "negative"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(MipiHsyncPol::B0)
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(MipiHsyncPol::B1)
    }
}
#[doc = "VSYNC polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MipiVsyncPol {
    #[doc = "0: negative"]
    B0 = 0,
    #[doc = "1: positive"]
    B1 = 1,
}
impl From<MipiVsyncPol> for bool {
    #[inline(always)]
    fn from(variant: MipiVsyncPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIPI_VSYNC_POL` reader - VSYNC polarity"]
pub type MipiVsyncPolR = crate::BitReader<MipiVsyncPol>;
impl MipiVsyncPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MipiVsyncPol {
        match self.bits {
            false => MipiVsyncPol::B0,
            true => MipiVsyncPol::B1,
        }
    }
    #[doc = "negative"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == MipiVsyncPol::B0
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == MipiVsyncPol::B1
    }
}
#[doc = "Field `MIPI_VSYNC_POL` writer - VSYNC polarity"]
pub type MipiVsyncPolW<'a, REG> = crate::BitWriter<'a, REG, MipiVsyncPol>;
impl<'a, REG> MipiVsyncPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "negative"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(MipiVsyncPol::B0)
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(MipiVsyncPol::B1)
    }
}
#[doc = "DEN polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MipiDenPol {
    #[doc = "0: positive"]
    B0 = 0,
    #[doc = "1: negative"]
    B1 = 1,
}
impl From<MipiDenPol> for bool {
    #[inline(always)]
    fn from(variant: MipiDenPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIPI_DEN_POL` reader - DEN polarity"]
pub type MipiDenPolR = crate::BitReader<MipiDenPol>;
impl MipiDenPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MipiDenPol {
        match self.bits {
            false => MipiDenPol::B0,
            true => MipiDenPol::B1,
        }
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == MipiDenPol::B0
    }
    #[doc = "negative"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == MipiDenPol::B1
    }
}
#[doc = "Field `MIPI_DEN_POL` writer - DEN polarity"]
pub type MipiDenPolW<'a, REG> = crate::BitWriter<'a, REG, MipiDenPol>;
impl<'a, REG> MipiDenPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "positive"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(MipiDenPol::B0)
    }
    #[doc = "negative"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(MipiDenPol::B1)
    }
}
#[doc = "DCLK invert enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MipiDclkPol {
    #[doc = "0: normal"]
    B0 = 0,
    #[doc = "1: invert default dclk invert"]
    B1 = 1,
}
impl From<MipiDclkPol> for bool {
    #[inline(always)]
    fn from(variant: MipiDclkPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIPI_DCLK_POL` reader - DCLK invert enable"]
pub type MipiDclkPolR = crate::BitReader<MipiDclkPol>;
impl MipiDclkPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MipiDclkPol {
        match self.bits {
            false => MipiDclkPol::B0,
            true => MipiDclkPol::B1,
        }
    }
    #[doc = "normal"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == MipiDclkPol::B0
    }
    #[doc = "invert default dclk invert"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == MipiDclkPol::B1
    }
}
#[doc = "Field `MIPI_DCLK_POL` writer - DCLK invert enable"]
pub type MipiDclkPolW<'a, REG> = crate::BitWriter<'a, REG, MipiDclkPol>;
impl<'a, REG> MipiDclkPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(MipiDclkPol::B0)
    }
    #[doc = "invert default dclk invert"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(MipiDclkPol::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Display LUT ram enable"]
    #[inline(always)]
    pub fn dsp_lut_en(&self) -> DspLutEnR {
        DspLutEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 10bit -> 8bit (allegro)"]
    #[inline(always)]
    pub fn pre_dither_down_en(&self) -> PreDitherDownEnR {
        PreDitherDownEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Dither-down enable"]
    #[inline(always)]
    pub fn dither_down_en(&self) -> DitherDownEnR {
        DitherDownEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Dither-down mode"]
    #[inline(always)]
    pub fn dither_down_mode(&self) -> DitherDownModeR {
        DitherDownModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - dither down mode select"]
    #[inline(always)]
    pub fn dither_down_sel(&self) -> DitherDownSelR {
        DitherDownSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dither_up_en(&self) -> DitherUpEnR {
        DitherUpEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn update_gamma_lut(&self) -> UpdateGammaLutR {
        UpdateGammaLutR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - layer0 selection"]
    #[inline(always)]
    pub fn dsp_layer0_sel(&self) -> DspLayer0SelR {
        DspLayer0SelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - layer1 selection"]
    #[inline(always)]
    pub fn dsp_layer1_sel(&self) -> DspLayer1SelR {
        DspLayer1SelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - layer2 selection"]
    #[inline(always)]
    pub fn dsp_layer2_sel(&self) -> DspLayer2SelR {
        DspLayer2SelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - layer3 selection"]
    #[inline(always)]
    pub fn dsp_layer3_sel(&self) -> DspLayer3SelR {
        DspLayer3SelR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - HSYNC polarity"]
    #[inline(always)]
    pub fn dp_lvds_hsync_pol(&self) -> DpLvdsHsyncPolR {
        DpLvdsHsyncPolR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VSYNC polarity"]
    #[inline(always)]
    pub fn dp_lvds_vsync_pol(&self) -> DpLvdsVsyncPolR {
        DpLvdsVsyncPolR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DEN polarity"]
    #[inline(always)]
    pub fn dp_lvds_den_pol(&self) -> DpLvdsDenPolR {
        DpLvdsDenPolR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DCLK invert enable"]
    #[inline(always)]
    pub fn dp_lvds_dclk_pol(&self) -> DpLvdsDclkPolR {
        DpLvdsDclkPolR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - HSYNC polarity"]
    #[inline(always)]
    pub fn hdmi_hsync_pol(&self) -> HdmiHsyncPolR {
        HdmiHsyncPolR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - VSYNC polarity"]
    #[inline(always)]
    pub fn hdmi_vsync_pol(&self) -> HdmiVsyncPolR {
        HdmiVsyncPolR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DEN polarity"]
    #[inline(always)]
    pub fn hdmi_den_pol(&self) -> HdmiDenPolR {
        HdmiDenPolR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DCLK invert enable"]
    #[inline(always)]
    pub fn hdmi_dclk_pol(&self) -> HdmiDclkPolR {
        HdmiDclkPolR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - HSYNC polarity"]
    #[inline(always)]
    pub fn edp_hsync_pol(&self) -> EdpHsyncPolR {
        EdpHsyncPolR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - VSYNC polarity"]
    #[inline(always)]
    pub fn edp_vsync_pol(&self) -> EdpVsyncPolR {
        EdpVsyncPolR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DEN polarity"]
    #[inline(always)]
    pub fn edp_den_pol(&self) -> EdpDenPolR {
        EdpDenPolR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DCLK invert enable"]
    #[inline(always)]
    pub fn edp_dclk_pol(&self) -> EdpDclkPolR {
        EdpDclkPolR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - HSYNC polarity"]
    #[inline(always)]
    pub fn mipi_hsync_pol(&self) -> MipiHsyncPolR {
        MipiHsyncPolR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - VSYNC polarity"]
    #[inline(always)]
    pub fn mipi_vsync_pol(&self) -> MipiVsyncPolR {
        MipiVsyncPolR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DEN polarity"]
    #[inline(always)]
    pub fn mipi_den_pol(&self) -> MipiDenPolR {
        MipiDenPolR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DCLK invert enable"]
    #[inline(always)]
    pub fn mipi_dclk_pol(&self) -> MipiDclkPolR {
        MipiDclkPolR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Display LUT ram enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_lut_en(&mut self) -> DspLutEnW<DspCtrl1Spec> {
        DspLutEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 10bit -> 8bit (allegro)"]
    #[inline(always)]
    #[must_use]
    pub fn pre_dither_down_en(&mut self) -> PreDitherDownEnW<DspCtrl1Spec> {
        PreDitherDownEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Dither-down enable"]
    #[inline(always)]
    #[must_use]
    pub fn dither_down_en(&mut self) -> DitherDownEnW<DspCtrl1Spec> {
        DitherDownEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Dither-down mode"]
    #[inline(always)]
    #[must_use]
    pub fn dither_down_mode(&mut self) -> DitherDownModeW<DspCtrl1Spec> {
        DitherDownModeW::new(self, 3)
    }
    #[doc = "Bit 4 - dither down mode select"]
    #[inline(always)]
    #[must_use]
    pub fn dither_down_sel(&mut self) -> DitherDownSelW<DspCtrl1Spec> {
        DitherDownSelW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn dither_up_en(&mut self) -> DitherUpEnW<DspCtrl1Spec> {
        DitherUpEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn update_gamma_lut(&mut self) -> UpdateGammaLutW<DspCtrl1Spec> {
        UpdateGammaLutW::new(self, 7)
    }
    #[doc = "Bits 8:9 - layer0 selection"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_layer0_sel(&mut self) -> DspLayer0SelW<DspCtrl1Spec> {
        DspLayer0SelW::new(self, 8)
    }
    #[doc = "Bits 10:11 - layer1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_layer1_sel(&mut self) -> DspLayer1SelW<DspCtrl1Spec> {
        DspLayer1SelW::new(self, 10)
    }
    #[doc = "Bits 12:13 - layer2 selection"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_layer2_sel(&mut self) -> DspLayer2SelW<DspCtrl1Spec> {
        DspLayer2SelW::new(self, 12)
    }
    #[doc = "Bits 14:15 - layer3 selection"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_layer3_sel(&mut self) -> DspLayer3SelW<DspCtrl1Spec> {
        DspLayer3SelW::new(self, 14)
    }
    #[doc = "Bit 16 - HSYNC polarity"]
    #[inline(always)]
    #[must_use]
    pub fn dp_lvds_hsync_pol(&mut self) -> DpLvdsHsyncPolW<DspCtrl1Spec> {
        DpLvdsHsyncPolW::new(self, 16)
    }
    #[doc = "Bit 17 - VSYNC polarity"]
    #[inline(always)]
    #[must_use]
    pub fn dp_lvds_vsync_pol(&mut self) -> DpLvdsVsyncPolW<DspCtrl1Spec> {
        DpLvdsVsyncPolW::new(self, 17)
    }
    #[doc = "Bit 18 - DEN polarity"]
    #[inline(always)]
    #[must_use]
    pub fn dp_lvds_den_pol(&mut self) -> DpLvdsDenPolW<DspCtrl1Spec> {
        DpLvdsDenPolW::new(self, 18)
    }
    #[doc = "Bit 19 - DCLK invert enable"]
    #[inline(always)]
    #[must_use]
    pub fn dp_lvds_dclk_pol(&mut self) -> DpLvdsDclkPolW<DspCtrl1Spec> {
        DpLvdsDclkPolW::new(self, 19)
    }
    #[doc = "Bit 20 - HSYNC polarity"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_hsync_pol(&mut self) -> HdmiHsyncPolW<DspCtrl1Spec> {
        HdmiHsyncPolW::new(self, 20)
    }
    #[doc = "Bit 21 - VSYNC polarity"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_vsync_pol(&mut self) -> HdmiVsyncPolW<DspCtrl1Spec> {
        HdmiVsyncPolW::new(self, 21)
    }
    #[doc = "Bit 22 - DEN polarity"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_den_pol(&mut self) -> HdmiDenPolW<DspCtrl1Spec> {
        HdmiDenPolW::new(self, 22)
    }
    #[doc = "Bit 23 - DCLK invert enable"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_dclk_pol(&mut self) -> HdmiDclkPolW<DspCtrl1Spec> {
        HdmiDclkPolW::new(self, 23)
    }
    #[doc = "Bit 24 - HSYNC polarity"]
    #[inline(always)]
    #[must_use]
    pub fn edp_hsync_pol(&mut self) -> EdpHsyncPolW<DspCtrl1Spec> {
        EdpHsyncPolW::new(self, 24)
    }
    #[doc = "Bit 25 - VSYNC polarity"]
    #[inline(always)]
    #[must_use]
    pub fn edp_vsync_pol(&mut self) -> EdpVsyncPolW<DspCtrl1Spec> {
        EdpVsyncPolW::new(self, 25)
    }
    #[doc = "Bit 26 - DEN polarity"]
    #[inline(always)]
    #[must_use]
    pub fn edp_den_pol(&mut self) -> EdpDenPolW<DspCtrl1Spec> {
        EdpDenPolW::new(self, 26)
    }
    #[doc = "Bit 27 - DCLK invert enable"]
    #[inline(always)]
    #[must_use]
    pub fn edp_dclk_pol(&mut self) -> EdpDclkPolW<DspCtrl1Spec> {
        EdpDclkPolW::new(self, 27)
    }
    #[doc = "Bit 28 - HSYNC polarity"]
    #[inline(always)]
    #[must_use]
    pub fn mipi_hsync_pol(&mut self) -> MipiHsyncPolW<DspCtrl1Spec> {
        MipiHsyncPolW::new(self, 28)
    }
    #[doc = "Bit 29 - VSYNC polarity"]
    #[inline(always)]
    #[must_use]
    pub fn mipi_vsync_pol(&mut self) -> MipiVsyncPolW<DspCtrl1Spec> {
        MipiVsyncPolW::new(self, 29)
    }
    #[doc = "Bit 30 - DEN polarity"]
    #[inline(always)]
    #[must_use]
    pub fn mipi_den_pol(&mut self) -> MipiDenPolW<DspCtrl1Spec> {
        MipiDenPolW::new(self, 30)
    }
    #[doc = "Bit 31 - DCLK invert enable"]
    #[inline(always)]
    #[must_use]
    pub fn mipi_dclk_pol(&mut self) -> MipiDclkPolW<DspCtrl1Spec> {
        MipiDclkPolW::new(self, 31)
    }
}
#[doc = "Display control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspCtrl1Spec;
impl crate::RegisterSpec for DspCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_ctrl1::R`](R) reader structure"]
impl crate::Readable for DspCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`dsp_ctrl1::W`](W) writer structure"]
impl crate::Writable for DspCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP_CTRL1 to value 0xe400"]
impl crate::Resettable for DspCtrl1Spec {
    const RESET_VALUE: u32 = 0xe400;
}
