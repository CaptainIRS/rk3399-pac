#[doc = "Register `DSP_CTRL0` reader"]
pub type R = crate::R<DspCtrl0Spec>;
#[doc = "Register `DSP_CTRL0` writer"]
pub type W = crate::W<DspCtrl0Spec>;
#[doc = "Display output format\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DspOutMode {
    #[doc = "0: Parallel 24-bit RGB888 output R\\[7:0\\],G\\[7:0\\],B\\[7:0\\]"]
    B0000 = 0,
    #[doc = "1: Parallel 18-bit RGB666 output 6'b0,R\\[5:0\\],G\\[5:0\\],B\\[5:0\\]"]
    B0001 = 1,
    #[doc = "2: Parallel 16-bit RGB565 output 8'b0,R\\[4:0\\],G\\[5:0\\],B\\[4:0\\]"]
    B0010 = 2,
    #[doc = "3: Parallel 24-bit RGB888 double pixel mix out phase0:G1\\[3:0\\],B1\\[7:0\\],G0\\[3:0\\],B0\\[7:0\\]
phase1:R1\\[7:0\\],G1\\[7:4\\],R0\\[7:0\\],G0\\[7:4\\]"]
    B0011 = 3,
    #[doc = "4: Serial 2x12-bit 12'b0,G\\[3:0\\],B\\[7:0\\]
+ 12'b0,R\\[7:0\\],G\\[7:4\\]"]
    B0100 = 4,
    #[doc = "5: ITU-656 output mode0 16'b0,pixel_data\\[7:0\\]"]
    B0101 = 5,
    #[doc = "6: ITU-656 output mode1 8'b0,pixel_data\\[7:0\\],8'b0"]
    B0110 = 6,
    #[doc = "7: ITU-656 output mode2 9'b0,pixel_data\\[7:0\\],7'b0"]
    B0111 = 7,
    #[doc = "8: Serial 3x8-bit RGB888 16'b0, B\\[7:0\\]+16'b0,G\\[7:0\\]+16'b0,R\\[7:0\\]"]
    B1000 = 8,
    #[doc = "9: Serial 3x8-bit RGB888 + dummy 16'b0, B\\[7:0\\]+16'b0,G\\[7:0\\]+16'b0,R\\[7:0\\]
+ dummy"]
    B1001 = 9,
    #[doc = "14: YUV420 output for HDMI"]
    B1110 = 14,
    #[doc = "12: DP_YUV422"]
    B1100 = 12,
    #[doc = "13: DP_YUV420"]
    B1101 = 13,
    #[doc = "15: Parallel 30-bit RGBaaa output R\\[9:0\\],G\\[9:0\\],B\\[9:0\\]
Others: Reserved."]
    B1111 = 15,
}
impl From<DspOutMode> for u8 {
    #[inline(always)]
    fn from(variant: DspOutMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DspOutMode {
    type Ux = u8;
}
#[doc = "Field `DSP_OUT_MODE` reader - Display output format"]
pub type DspOutModeR = crate::FieldReader<DspOutMode>;
impl DspOutModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DspOutMode> {
        match self.bits {
            0 => Some(DspOutMode::B0000),
            1 => Some(DspOutMode::B0001),
            2 => Some(DspOutMode::B0010),
            3 => Some(DspOutMode::B0011),
            4 => Some(DspOutMode::B0100),
            5 => Some(DspOutMode::B0101),
            6 => Some(DspOutMode::B0110),
            7 => Some(DspOutMode::B0111),
            8 => Some(DspOutMode::B1000),
            9 => Some(DspOutMode::B1001),
            14 => Some(DspOutMode::B1110),
            12 => Some(DspOutMode::B1100),
            13 => Some(DspOutMode::B1101),
            15 => Some(DspOutMode::B1111),
            _ => None,
        }
    }
    #[doc = "Parallel 24-bit RGB888 output R\\[7:0\\],G\\[7:0\\],B\\[7:0\\]"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == DspOutMode::B0000
    }
    #[doc = "Parallel 18-bit RGB666 output 6'b0,R\\[5:0\\],G\\[5:0\\],B\\[5:0\\]"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == DspOutMode::B0001
    }
    #[doc = "Parallel 16-bit RGB565 output 8'b0,R\\[4:0\\],G\\[5:0\\],B\\[4:0\\]"]
    #[inline(always)]
    pub fn is_b0010(&self) -> bool {
        *self == DspOutMode::B0010
    }
    #[doc = "Parallel 24-bit RGB888 double pixel mix out phase0:G1\\[3:0\\],B1\\[7:0\\],G0\\[3:0\\],B0\\[7:0\\]
phase1:R1\\[7:0\\],G1\\[7:4\\],R0\\[7:0\\],G0\\[7:4\\]"]
    #[inline(always)]
    pub fn is_b0011(&self) -> bool {
        *self == DspOutMode::B0011
    }
    #[doc = "Serial 2x12-bit 12'b0,G\\[3:0\\],B\\[7:0\\]
+ 12'b0,R\\[7:0\\],G\\[7:4\\]"]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == DspOutMode::B0100
    }
    #[doc = "ITU-656 output mode0 16'b0,pixel_data\\[7:0\\]"]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == DspOutMode::B0101
    }
    #[doc = "ITU-656 output mode1 8'b0,pixel_data\\[7:0\\],8'b0"]
    #[inline(always)]
    pub fn is_b0110(&self) -> bool {
        *self == DspOutMode::B0110
    }
    #[doc = "ITU-656 output mode2 9'b0,pixel_data\\[7:0\\],7'b0"]
    #[inline(always)]
    pub fn is_b0111(&self) -> bool {
        *self == DspOutMode::B0111
    }
    #[doc = "Serial 3x8-bit RGB888 16'b0, B\\[7:0\\]+16'b0,G\\[7:0\\]+16'b0,R\\[7:0\\]"]
    #[inline(always)]
    pub fn is_b1000(&self) -> bool {
        *self == DspOutMode::B1000
    }
    #[doc = "Serial 3x8-bit RGB888 + dummy 16'b0, B\\[7:0\\]+16'b0,G\\[7:0\\]+16'b0,R\\[7:0\\]
+ dummy"]
    #[inline(always)]
    pub fn is_b1001(&self) -> bool {
        *self == DspOutMode::B1001
    }
    #[doc = "YUV420 output for HDMI"]
    #[inline(always)]
    pub fn is_b1110(&self) -> bool {
        *self == DspOutMode::B1110
    }
    #[doc = "DP_YUV422"]
    #[inline(always)]
    pub fn is_b1100(&self) -> bool {
        *self == DspOutMode::B1100
    }
    #[doc = "DP_YUV420"]
    #[inline(always)]
    pub fn is_b1101(&self) -> bool {
        *self == DspOutMode::B1101
    }
    #[doc = "Parallel 30-bit RGBaaa output R\\[9:0\\],G\\[9:0\\],B\\[9:0\\]
Others: Reserved."]
    #[inline(always)]
    pub fn is_b1111(&self) -> bool {
        *self == DspOutMode::B1111
    }
}
#[doc = "Field `DSP_OUT_MODE` writer - Display output format"]
pub type DspOutModeW<'a, REG> = crate::FieldWriter<'a, REG, 4, DspOutMode>;
impl<'a, REG> DspOutModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Parallel 24-bit RGB888 output R\\[7:0\\],G\\[7:0\\],B\\[7:0\\]"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(DspOutMode::B0000)
    }
    #[doc = "Parallel 18-bit RGB666 output 6'b0,R\\[5:0\\],G\\[5:0\\],B\\[5:0\\]"]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(DspOutMode::B0001)
    }
    #[doc = "Parallel 16-bit RGB565 output 8'b0,R\\[4:0\\],G\\[5:0\\],B\\[4:0\\]"]
    #[inline(always)]
    pub fn b0010(self) -> &'a mut crate::W<REG> {
        self.variant(DspOutMode::B0010)
    }
    #[doc = "Parallel 24-bit RGB888 double pixel mix out phase0:G1\\[3:0\\],B1\\[7:0\\],G0\\[3:0\\],B0\\[7:0\\]
phase1:R1\\[7:0\\],G1\\[7:4\\],R0\\[7:0\\],G0\\[7:4\\]"]
    #[inline(always)]
    pub fn b0011(self) -> &'a mut crate::W<REG> {
        self.variant(DspOutMode::B0011)
    }
    #[doc = "Serial 2x12-bit 12'b0,G\\[3:0\\],B\\[7:0\\]
+ 12'b0,R\\[7:0\\],G\\[7:4\\]"]
    #[inline(always)]
    pub fn b0100(self) -> &'a mut crate::W<REG> {
        self.variant(DspOutMode::B0100)
    }
    #[doc = "ITU-656 output mode0 16'b0,pixel_data\\[7:0\\]"]
    #[inline(always)]
    pub fn b0101(self) -> &'a mut crate::W<REG> {
        self.variant(DspOutMode::B0101)
    }
    #[doc = "ITU-656 output mode1 8'b0,pixel_data\\[7:0\\],8'b0"]
    #[inline(always)]
    pub fn b0110(self) -> &'a mut crate::W<REG> {
        self.variant(DspOutMode::B0110)
    }
    #[doc = "ITU-656 output mode2 9'b0,pixel_data\\[7:0\\],7'b0"]
    #[inline(always)]
    pub fn b0111(self) -> &'a mut crate::W<REG> {
        self.variant(DspOutMode::B0111)
    }
    #[doc = "Serial 3x8-bit RGB888 16'b0, B\\[7:0\\]+16'b0,G\\[7:0\\]+16'b0,R\\[7:0\\]"]
    #[inline(always)]
    pub fn b1000(self) -> &'a mut crate::W<REG> {
        self.variant(DspOutMode::B1000)
    }
    #[doc = "Serial 3x8-bit RGB888 + dummy 16'b0, B\\[7:0\\]+16'b0,G\\[7:0\\]+16'b0,R\\[7:0\\]
+ dummy"]
    #[inline(always)]
    pub fn b1001(self) -> &'a mut crate::W<REG> {
        self.variant(DspOutMode::B1001)
    }
    #[doc = "YUV420 output for HDMI"]
    #[inline(always)]
    pub fn b1110(self) -> &'a mut crate::W<REG> {
        self.variant(DspOutMode::B1110)
    }
    #[doc = "DP_YUV422"]
    #[inline(always)]
    pub fn b1100(self) -> &'a mut crate::W<REG> {
        self.variant(DspOutMode::B1100)
    }
    #[doc = "DP_YUV420"]
    #[inline(always)]
    pub fn b1101(self) -> &'a mut crate::W<REG> {
        self.variant(DspOutMode::B1101)
    }
    #[doc = "Parallel 30-bit RGBaaa output R\\[9:0\\],G\\[9:0\\],B\\[9:0\\]
Others: Reserved."]
    #[inline(always)]
    pub fn b1111(self) -> &'a mut crate::W<REG> {
        self.variant(DspOutMode::B1111)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwCoreDclkSel {
    #[doc = "0: dclk_core sel dclk"]
    B0 = 0,
    #[doc = "1: dclk_core sel dclk div2"]
    B1 = 1,
}
impl From<SwCoreDclkSel> for bool {
    #[inline(always)]
    fn from(variant: SwCoreDclkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_CORE_DCLK_SEL` reader - "]
pub type SwCoreDclkSelR = crate::BitReader<SwCoreDclkSel>;
impl SwCoreDclkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwCoreDclkSel {
        match self.bits {
            false => SwCoreDclkSel::B0,
            true => SwCoreDclkSel::B1,
        }
    }
    #[doc = "dclk_core sel dclk"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwCoreDclkSel::B0
    }
    #[doc = "dclk_core sel dclk div2"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwCoreDclkSel::B1
    }
}
#[doc = "Field `SW_CORE_DCLK_SEL` writer - "]
pub type SwCoreDclkSelW<'a, REG> = crate::BitWriter<'a, REG, SwCoreDclkSel>;
impl<'a, REG> SwCoreDclkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "dclk_core sel dclk"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwCoreDclkSel::B0)
    }
    #[doc = "dclk_core sel dclk div2"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwCoreDclkSel::B1)
    }
}
#[doc = "Field `P2I_EN` reader - p2i_en"]
pub type P2iEnR = crate::BitReader;
#[doc = "Field `P2I_EN` writer - p2i_en"]
pub type P2iEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "dclk output mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DspDclkDdr {
    #[doc = "0: SDR"]
    B0 = 0,
    #[doc = "1: DDR"]
    B1 = 1,
}
impl From<DspDclkDdr> for bool {
    #[inline(always)]
    fn from(variant: DspDclkDdr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP_DCLK_DDR` reader - dclk output mode"]
pub type DspDclkDdrR = crate::BitReader<DspDclkDdr>;
impl DspDclkDdrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DspDclkDdr {
        match self.bits {
            false => DspDclkDdr::B0,
            true => DspDclkDdr::B1,
        }
    }
    #[doc = "SDR"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DspDclkDdr::B0
    }
    #[doc = "DDR"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DspDclkDdr::B1
    }
}
#[doc = "Field `DSP_DCLK_DDR` writer - dclk output mode"]
pub type DspDclkDdrW<'a, REG> = crate::BitWriter<'a, REG, DspDclkDdr>;
impl<'a, REG> DspDclkDdrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDR"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DspDclkDdr::B0)
    }
    #[doc = "DDR"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DspDclkDdr::B1)
    }
}
#[doc = "dclk phase lock\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DspDdrPhase {
    #[doc = "0: no lock"]
    B0 = 0,
    #[doc = "1: lock every line"]
    B1 = 1,
}
impl From<DspDdrPhase> for bool {
    #[inline(always)]
    fn from(variant: DspDdrPhase) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP_DDR_PHASE` reader - dclk phase lock"]
pub type DspDdrPhaseR = crate::BitReader<DspDdrPhase>;
impl DspDdrPhaseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DspDdrPhase {
        match self.bits {
            false => DspDdrPhase::B0,
            true => DspDdrPhase::B1,
        }
    }
    #[doc = "no lock"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DspDdrPhase::B0
    }
    #[doc = "lock every line"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DspDdrPhase::B1
    }
}
#[doc = "Field `DSP_DDR_PHASE` writer - dclk phase lock"]
pub type DspDdrPhaseW<'a, REG> = crate::BitWriter<'a, REG, DspDdrPhase>;
impl<'a, REG> DspDdrPhaseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no lock"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DspDdrPhase::B0)
    }
    #[doc = "lock every line"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DspDdrPhase::B1)
    }
}
#[doc = "Interlace display enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DspInterlace {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable *This mode is related to the ITU-R656 output, the display timing of odd field must be set correctly. (lcdc_dsp_vs_st_end_f1/lcdc_dsp_vact_end_f1)"]
    B1 = 1,
}
impl From<DspInterlace> for bool {
    #[inline(always)]
    fn from(variant: DspInterlace) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP_INTERLACE` reader - Interlace display enable"]
pub type DspInterlaceR = crate::BitReader<DspInterlace>;
impl DspInterlaceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DspInterlace {
        match self.bits {
            false => DspInterlace::B0,
            true => DspInterlace::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DspInterlace::B0
    }
    #[doc = "enable *This mode is related to the ITU-R656 output, the display timing of odd field must be set correctly. (lcdc_dsp_vs_st_end_f1/lcdc_dsp_vact_end_f1)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DspInterlace::B1
    }
}
#[doc = "Field `DSP_INTERLACE` writer - Interlace display enable"]
pub type DspInterlaceW<'a, REG> = crate::BitWriter<'a, REG, DspInterlace>;
impl<'a, REG> DspInterlaceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DspInterlace::B0)
    }
    #[doc = "enable *This mode is related to the ITU-R656 output, the display timing of odd field must be set correctly. (lcdc_dsp_vs_st_end_f1/lcdc_dsp_vact_end_f1)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DspInterlace::B1)
    }
}
#[doc = "field polarity when interlace dsp\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DspFieldPol {
    #[doc = "0: normal"]
    B0 = 0,
    #[doc = "1: invert"]
    B1 = 1,
}
impl From<DspFieldPol> for bool {
    #[inline(always)]
    fn from(variant: DspFieldPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP_FIELD_POL` reader - field polarity when interlace dsp"]
pub type DspFieldPolR = crate::BitReader<DspFieldPol>;
impl DspFieldPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DspFieldPol {
        match self.bits {
            false => DspFieldPol::B0,
            true => DspFieldPol::B1,
        }
    }
    #[doc = "normal"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DspFieldPol::B0
    }
    #[doc = "invert"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DspFieldPol::B1
    }
}
#[doc = "Field `DSP_FIELD_POL` writer - field polarity when interlace dsp"]
pub type DspFieldPolW<'a, REG> = crate::BitWriter<'a, REG, DspFieldPol>;
impl<'a, REG> DspFieldPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DspFieldPol::B0)
    }
    #[doc = "invert"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DspFieldPol::B1)
    }
}
#[doc = "Display output blue and green swap enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DspBgSwap {
    #[doc = "0: RGB"]
    B0 = 0,
    #[doc = "1: RBG"]
    B1 = 1,
}
impl From<DspBgSwap> for bool {
    #[inline(always)]
    fn from(variant: DspBgSwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP_BG_SWAP` reader - Display output blue and green swap enable"]
pub type DspBgSwapR = crate::BitReader<DspBgSwap>;
impl DspBgSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DspBgSwap {
        match self.bits {
            false => DspBgSwap::B0,
            true => DspBgSwap::B1,
        }
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DspBgSwap::B0
    }
    #[doc = "RBG"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DspBgSwap::B1
    }
}
#[doc = "Field `DSP_BG_SWAP` writer - Display output blue and green swap enable"]
pub type DspBgSwapW<'a, REG> = crate::BitWriter<'a, REG, DspBgSwap>;
impl<'a, REG> DspBgSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DspBgSwap::B0)
    }
    #[doc = "RBG"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DspBgSwap::B1)
    }
}
#[doc = "Display output red and blue swap enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DspRbSwap {
    #[doc = "0: RGB"]
    B0 = 0,
    #[doc = "1: BGR"]
    B1 = 1,
}
impl From<DspRbSwap> for bool {
    #[inline(always)]
    fn from(variant: DspRbSwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP_RB_SWAP` reader - Display output red and blue swap enable"]
pub type DspRbSwapR = crate::BitReader<DspRbSwap>;
impl DspRbSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DspRbSwap {
        match self.bits {
            false => DspRbSwap::B0,
            true => DspRbSwap::B1,
        }
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DspRbSwap::B0
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DspRbSwap::B1
    }
}
#[doc = "Field `DSP_RB_SWAP` writer - Display output red and blue swap enable"]
pub type DspRbSwapW<'a, REG> = crate::BitWriter<'a, REG, DspRbSwap>;
impl<'a, REG> DspRbSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DspRbSwap::B0)
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DspRbSwap::B1)
    }
}
#[doc = "Display output red and green swap enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DspRgSwap {
    #[doc = "0: RGB"]
    B0 = 0,
    #[doc = "1: GRB"]
    B1 = 1,
}
impl From<DspRgSwap> for bool {
    #[inline(always)]
    fn from(variant: DspRgSwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP_RG_SWAP` reader - Display output red and green swap enable"]
pub type DspRgSwapR = crate::BitReader<DspRgSwap>;
impl DspRgSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DspRgSwap {
        match self.bits {
            false => DspRgSwap::B0,
            true => DspRgSwap::B1,
        }
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DspRgSwap::B0
    }
    #[doc = "GRB"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DspRgSwap::B1
    }
}
#[doc = "Field `DSP_RG_SWAP` writer - Display output red and green swap enable"]
pub type DspRgSwapW<'a, REG> = crate::BitWriter<'a, REG, DspRgSwap>;
impl<'a, REG> DspRgSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DspRgSwap::B0)
    }
    #[doc = "GRB"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DspRgSwap::B1)
    }
}
#[doc = "Display delta swap enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DspDeltaSwap {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable *See detail description in Delta display charpter."]
    B1 = 1,
}
impl From<DspDeltaSwap> for bool {
    #[inline(always)]
    fn from(variant: DspDeltaSwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP_DELTA_SWAP` reader - Display delta swap enable"]
pub type DspDeltaSwapR = crate::BitReader<DspDeltaSwap>;
impl DspDeltaSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DspDeltaSwap {
        match self.bits {
            false => DspDeltaSwap::B0,
            true => DspDeltaSwap::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DspDeltaSwap::B0
    }
    #[doc = "enable *See detail description in Delta display charpter."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DspDeltaSwap::B1
    }
}
#[doc = "Field `DSP_DELTA_SWAP` writer - Display delta swap enable"]
pub type DspDeltaSwapW<'a, REG> = crate::BitWriter<'a, REG, DspDeltaSwap>;
impl<'a, REG> DspDeltaSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DspDeltaSwap::B0)
    }
    #[doc = "enable *See detail description in Delta display charpter."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DspDeltaSwap::B1)
    }
}
#[doc = "Display dummy swap enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DspDummySwap {
    #[doc = "0: B+G+R+dummy"]
    B0 = 0,
    #[doc = "1: dummy+B+G+R"]
    B1 = 1,
}
impl From<DspDummySwap> for bool {
    #[inline(always)]
    fn from(variant: DspDummySwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP_DUMMY_SWAP` reader - Display dummy swap enable"]
pub type DspDummySwapR = crate::BitReader<DspDummySwap>;
impl DspDummySwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DspDummySwap {
        match self.bits {
            false => DspDummySwap::B0,
            true => DspDummySwap::B1,
        }
    }
    #[doc = "B+G+R+dummy"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DspDummySwap::B0
    }
    #[doc = "dummy+B+G+R"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DspDummySwap::B1
    }
}
#[doc = "Field `DSP_DUMMY_SWAP` writer - Display dummy swap enable"]
pub type DspDummySwapW<'a, REG> = crate::BitWriter<'a, REG, DspDummySwap>;
impl<'a, REG> DspDummySwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "B+G+R+dummy"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DspDummySwap::B0)
    }
    #[doc = "dummy+B+G+R"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DspDummySwap::B1)
    }
}
#[doc = "Hsync/Vsync/Den output software ctrl\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DspOutZero {
    #[doc = "0: normal output"]
    B0 = 0,
    #[doc = "1: all output '0'"]
    B1 = 1,
}
impl From<DspOutZero> for bool {
    #[inline(always)]
    fn from(variant: DspOutZero) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP_OUT_ZERO` reader - Hsync/Vsync/Den output software ctrl"]
pub type DspOutZeroR = crate::BitReader<DspOutZero>;
impl DspOutZeroR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DspOutZero {
        match self.bits {
            false => DspOutZero::B0,
            true => DspOutZero::B1,
        }
    }
    #[doc = "normal output"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DspOutZero::B0
    }
    #[doc = "all output '0'"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DspOutZero::B1
    }
}
#[doc = "Field `DSP_OUT_ZERO` writer - Hsync/Vsync/Den output software ctrl"]
pub type DspOutZeroW<'a, REG> = crate::BitWriter<'a, REG, DspOutZero>;
impl<'a, REG> DspOutZeroW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal output"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DspOutZero::B0)
    }
    #[doc = "all output '0'"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DspOutZero::B1)
    }
}
#[doc = "Field `DSP_BLANK_EN` reader - Blank display mode\n\nWhen this bit enable, the Hsync/Vsync/Den output is blank"]
pub type DspBlankEnR = crate::BitReader;
#[doc = "Field `DSP_BLANK_EN` writer - Blank display mode\n\nWhen this bit enable, the Hsync/Vsync/Den output is blank"]
pub type DspBlankEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP_BLACK_EN` reader - Black display mode\n\nWhen this bit enable, the pixel data output is all black\n\n(0x000000)"]
pub type DspBlackEnR = crate::BitReader;
#[doc = "Field `DSP_BLACK_EN` writer - Black display mode\n\nWhen this bit enable, the pixel data output is all black\n\n(0x000000)"]
pub type DspBlackEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Cb-Cr filter in CCIR656 mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DspCcir656Avg {
    #[doc = "0: drop mode"]
    B0 = 0,
    #[doc = "1: average mode"]
    B1 = 1,
}
impl From<DspCcir656Avg> for bool {
    #[inline(always)]
    fn from(variant: DspCcir656Avg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP_CCIR656_AVG` reader - Cb-Cr filter in CCIR656 mode"]
pub type DspCcir656AvgR = crate::BitReader<DspCcir656Avg>;
impl DspCcir656AvgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DspCcir656Avg {
        match self.bits {
            false => DspCcir656Avg::B0,
            true => DspCcir656Avg::B1,
        }
    }
    #[doc = "drop mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DspCcir656Avg::B0
    }
    #[doc = "average mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DspCcir656Avg::B1
    }
}
#[doc = "Field `DSP_CCIR656_AVG` writer - Cb-Cr filter in CCIR656 mode"]
pub type DspCcir656AvgW<'a, REG> = crate::BitWriter<'a, REG, DspCcir656Avg>;
impl<'a, REG> DspCcir656AvgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "drop mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DspCcir656Avg::B0)
    }
    #[doc = "average mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DspCcir656Avg::B1)
    }
}
#[doc = "YCrCb clip\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DspYuvClip {
    #[doc = "0: disable, YCbCr no clip"]
    B0 = 0,
    #[doc = "1: enable, YCbCr clip before YCbCr2RGB *Y clip: 16~235, CbCr clip: 16~239"]
    B1 = 1,
}
impl From<DspYuvClip> for bool {
    #[inline(always)]
    fn from(variant: DspYuvClip) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP_YUV_CLIP` reader - YCrCb clip"]
pub type DspYuvClipR = crate::BitReader<DspYuvClip>;
impl DspYuvClipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DspYuvClip {
        match self.bits {
            false => DspYuvClip::B0,
            true => DspYuvClip::B1,
        }
    }
    #[doc = "disable, YCbCr no clip"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DspYuvClip::B0
    }
    #[doc = "enable, YCbCr clip before YCbCr2RGB *Y clip: 16~235, CbCr clip: 16~239"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DspYuvClip::B1
    }
}
#[doc = "Field `DSP_YUV_CLIP` writer - YCrCb clip"]
pub type DspYuvClipW<'a, REG> = crate::BitWriter<'a, REG, DspYuvClip>;
impl<'a, REG> DspYuvClipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable, YCbCr no clip"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DspYuvClip::B0)
    }
    #[doc = "enable, YCbCr clip before YCbCr2RGB *Y clip: 16~235, CbCr clip: 16~239"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DspYuvClip::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DspXMirEn {
    #[doc = "0: no x_mirror"]
    B0 = 0,
    #[doc = "1: x_mirror"]
    B1 = 1,
}
impl From<DspXMirEn> for bool {
    #[inline(always)]
    fn from(variant: DspXMirEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP_X_MIR_EN` reader - "]
pub type DspXMirEnR = crate::BitReader<DspXMirEn>;
impl DspXMirEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DspXMirEn {
        match self.bits {
            false => DspXMirEn::B0,
            true => DspXMirEn::B1,
        }
    }
    #[doc = "no x_mirror"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DspXMirEn::B0
    }
    #[doc = "x_mirror"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DspXMirEn::B1
    }
}
#[doc = "Field `DSP_X_MIR_EN` writer - "]
pub type DspXMirEnW<'a, REG> = crate::BitWriter<'a, REG, DspXMirEn>;
impl<'a, REG> DspXMirEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no x_mirror"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DspXMirEn::B0)
    }
    #[doc = "x_mirror"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DspXMirEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DspYMirEn {
    #[doc = "0: no y_mirror"]
    B0 = 0,
    #[doc = "1: y_mirror"]
    B1 = 1,
}
impl From<DspYMirEn> for bool {
    #[inline(always)]
    fn from(variant: DspYMirEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP_Y_MIR_EN` reader - "]
pub type DspYMirEnR = crate::BitReader<DspYMirEn>;
impl DspYMirEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DspYMirEn {
        match self.bits {
            false => DspYMirEn::B0,
            true => DspYMirEn::B1,
        }
    }
    #[doc = "no y_mirror"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DspYMirEn::B0
    }
    #[doc = "y_mirror"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DspYMirEn::B1
    }
}
#[doc = "Field `DSP_Y_MIR_EN` writer - "]
pub type DspYMirEnW<'a, REG> = crate::BitWriter<'a, REG, DspYMirEn>;
impl<'a, REG> DspYMirEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no y_mirror"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DspYMirEn::B0)
    }
    #[doc = "y_mirror"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DspYMirEn::B1)
    }
}
#[doc = "Field `SW_TVE_OUTPUT_SEL` reader - sw_tve_output_sel"]
pub type SwTveOutputSelR = crate::BitReader;
#[doc = "Field `SW_TVE_OUTPUT_SEL` writer - sw_tve_output_sel"]
pub type SwTveOutputSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP_FIELD` reader - dsp_field"]
pub type DspFieldR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Display output format"]
    #[inline(always)]
    pub fn dsp_out_mode(&self) -> DspOutModeR {
        DspOutModeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sw_core_dclk_sel(&self) -> SwCoreDclkSelR {
        SwCoreDclkSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - p2i_en"]
    #[inline(always)]
    pub fn p2i_en(&self) -> P2iEnR {
        P2iEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - dclk output mode"]
    #[inline(always)]
    pub fn dsp_dclk_ddr(&self) -> DspDclkDdrR {
        DspDclkDdrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - dclk phase lock"]
    #[inline(always)]
    pub fn dsp_ddr_phase(&self) -> DspDdrPhaseR {
        DspDdrPhaseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interlace display enable"]
    #[inline(always)]
    pub fn dsp_interlace(&self) -> DspInterlaceR {
        DspInterlaceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - field polarity when interlace dsp"]
    #[inline(always)]
    pub fn dsp_field_pol(&self) -> DspFieldPolR {
        DspFieldPolR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Display output blue and green swap enable"]
    #[inline(always)]
    pub fn dsp_bg_swap(&self) -> DspBgSwapR {
        DspBgSwapR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Display output red and blue swap enable"]
    #[inline(always)]
    pub fn dsp_rb_swap(&self) -> DspRbSwapR {
        DspRbSwapR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Display output red and green swap enable"]
    #[inline(always)]
    pub fn dsp_rg_swap(&self) -> DspRgSwapR {
        DspRgSwapR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Display delta swap enable"]
    #[inline(always)]
    pub fn dsp_delta_swap(&self) -> DspDeltaSwapR {
        DspDeltaSwapR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Display dummy swap enable"]
    #[inline(always)]
    pub fn dsp_dummy_swap(&self) -> DspDummySwapR {
        DspDummySwapR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Hsync/Vsync/Den output software ctrl"]
    #[inline(always)]
    pub fn dsp_out_zero(&self) -> DspOutZeroR {
        DspOutZeroR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Blank display mode\n\nWhen this bit enable, the Hsync/Vsync/Den output is blank"]
    #[inline(always)]
    pub fn dsp_blank_en(&self) -> DspBlankEnR {
        DspBlankEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Black display mode\n\nWhen this bit enable, the pixel data output is all black\n\n(0x000000)"]
    #[inline(always)]
    pub fn dsp_black_en(&self) -> DspBlackEnR {
        DspBlackEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Cb-Cr filter in CCIR656 mode"]
    #[inline(always)]
    pub fn dsp_ccir656_avg(&self) -> DspCcir656AvgR {
        DspCcir656AvgR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - YCrCb clip"]
    #[inline(always)]
    pub fn dsp_yuv_clip(&self) -> DspYuvClipR {
        DspYuvClipR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dsp_x_mir_en(&self) -> DspXMirEnR {
        DspXMirEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dsp_y_mir_en(&self) -> DspYMirEnR {
        DspYMirEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - sw_tve_output_sel"]
    #[inline(always)]
    pub fn sw_tve_output_sel(&self) -> SwTveOutputSelR {
        SwTveOutputSelR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - dsp_field"]
    #[inline(always)]
    pub fn dsp_field(&self) -> DspFieldR {
        DspFieldR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Display output format"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_out_mode(&mut self) -> DspOutModeW<DspCtrl0Spec> {
        DspOutModeW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn sw_core_dclk_sel(&mut self) -> SwCoreDclkSelW<DspCtrl0Spec> {
        SwCoreDclkSelW::new(self, 4)
    }
    #[doc = "Bit 5 - p2i_en"]
    #[inline(always)]
    #[must_use]
    pub fn p2i_en(&mut self) -> P2iEnW<DspCtrl0Spec> {
        P2iEnW::new(self, 5)
    }
    #[doc = "Bit 8 - dclk output mode"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_dclk_ddr(&mut self) -> DspDclkDdrW<DspCtrl0Spec> {
        DspDclkDdrW::new(self, 8)
    }
    #[doc = "Bit 9 - dclk phase lock"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_ddr_phase(&mut self) -> DspDdrPhaseW<DspCtrl0Spec> {
        DspDdrPhaseW::new(self, 9)
    }
    #[doc = "Bit 10 - Interlace display enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_interlace(&mut self) -> DspInterlaceW<DspCtrl0Spec> {
        DspInterlaceW::new(self, 10)
    }
    #[doc = "Bit 11 - field polarity when interlace dsp"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_field_pol(&mut self) -> DspFieldPolW<DspCtrl0Spec> {
        DspFieldPolW::new(self, 11)
    }
    #[doc = "Bit 12 - Display output blue and green swap enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_bg_swap(&mut self) -> DspBgSwapW<DspCtrl0Spec> {
        DspBgSwapW::new(self, 12)
    }
    #[doc = "Bit 13 - Display output red and blue swap enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_rb_swap(&mut self) -> DspRbSwapW<DspCtrl0Spec> {
        DspRbSwapW::new(self, 13)
    }
    #[doc = "Bit 14 - Display output red and green swap enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_rg_swap(&mut self) -> DspRgSwapW<DspCtrl0Spec> {
        DspRgSwapW::new(self, 14)
    }
    #[doc = "Bit 15 - Display delta swap enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_delta_swap(&mut self) -> DspDeltaSwapW<DspCtrl0Spec> {
        DspDeltaSwapW::new(self, 15)
    }
    #[doc = "Bit 16 - Display dummy swap enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_dummy_swap(&mut self) -> DspDummySwapW<DspCtrl0Spec> {
        DspDummySwapW::new(self, 16)
    }
    #[doc = "Bit 17 - Hsync/Vsync/Den output software ctrl"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_out_zero(&mut self) -> DspOutZeroW<DspCtrl0Spec> {
        DspOutZeroW::new(self, 17)
    }
    #[doc = "Bit 18 - Blank display mode\n\nWhen this bit enable, the Hsync/Vsync/Den output is blank"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_blank_en(&mut self) -> DspBlankEnW<DspCtrl0Spec> {
        DspBlankEnW::new(self, 18)
    }
    #[doc = "Bit 19 - Black display mode\n\nWhen this bit enable, the pixel data output is all black\n\n(0x000000)"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_black_en(&mut self) -> DspBlackEnW<DspCtrl0Spec> {
        DspBlackEnW::new(self, 19)
    }
    #[doc = "Bit 20 - Cb-Cr filter in CCIR656 mode"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_ccir656_avg(&mut self) -> DspCcir656AvgW<DspCtrl0Spec> {
        DspCcir656AvgW::new(self, 20)
    }
    #[doc = "Bit 21 - YCrCb clip"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_yuv_clip(&mut self) -> DspYuvClipW<DspCtrl0Spec> {
        DspYuvClipW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_x_mir_en(&mut self) -> DspXMirEnW<DspCtrl0Spec> {
        DspXMirEnW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_y_mir_en(&mut self) -> DspYMirEnW<DspCtrl0Spec> {
        DspYMirEnW::new(self, 23)
    }
    #[doc = "Bit 25 - sw_tve_output_sel"]
    #[inline(always)]
    #[must_use]
    pub fn sw_tve_output_sel(&mut self) -> SwTveOutputSelW<DspCtrl0Spec> {
        SwTveOutputSelW::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspCtrl0Spec;
impl crate::RegisterSpec for DspCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_ctrl0::R`](R) reader structure"]
impl crate::Readable for DspCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`dsp_ctrl0::W`](W) writer structure"]
impl crate::Writable for DspCtrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP_CTRL0 to value 0"]
impl crate::Resettable for DspCtrl0Spec {
    const RESET_VALUE: u32 = 0;
}
