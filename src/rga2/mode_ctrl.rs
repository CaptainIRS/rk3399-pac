#[doc = "Register `MODE_CTRL` reader"]
pub type R = crate::R<ModeCtrlSpec>;
#[doc = "Register `MODE_CTRL` writer"]
pub type W = crate::W<ModeCtrlSpec>;
#[doc = "RGA 2D render mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwRenderMode {
    #[doc = "0: Bitblt"]
    B000 = 0,
    #[doc = "1: Color palette"]
    B001 = 1,
    #[doc = "2: Rectangle fill"]
    B010 = 2,
    #[doc = "3: Update palette LUT/pattern ram"]
    B011 = 3,
}
impl From<SwRenderMode> for u8 {
    #[inline(always)]
    fn from(variant: SwRenderMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwRenderMode {
    type Ux = u8;
}
#[doc = "Field `SW_RENDER_MODE` reader - RGA 2D render mode"]
pub type SwRenderModeR = crate::FieldReader<SwRenderMode>;
impl SwRenderModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwRenderMode> {
        match self.bits {
            0 => Some(SwRenderMode::B000),
            1 => Some(SwRenderMode::B001),
            2 => Some(SwRenderMode::B010),
            3 => Some(SwRenderMode::B011),
            _ => None,
        }
    }
    #[doc = "Bitblt"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == SwRenderMode::B000
    }
    #[doc = "Color palette"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == SwRenderMode::B001
    }
    #[doc = "Rectangle fill"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == SwRenderMode::B010
    }
    #[doc = "Update palette LUT/pattern ram"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == SwRenderMode::B011
    }
}
#[doc = "Field `SW_RENDER_MODE` writer - RGA 2D render mode"]
pub type SwRenderModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, SwRenderMode>;
impl<'a, REG> SwRenderModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bitblt"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(SwRenderMode::B000)
    }
    #[doc = "Color palette"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(SwRenderMode::B001)
    }
    #[doc = "Rectangle fill"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(SwRenderMode::B010)
    }
    #[doc = "Update palette LUT/pattern ram"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(SwRenderMode::B011)
    }
}
#[doc = "Bitblt mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwBbMode {
    #[doc = "0: SRC + DST => DST"]
    B0 = 0,
    #[doc = "1: SRC + SRC1 => DST"]
    B1 = 1,
}
impl From<SwBbMode> for bool {
    #[inline(always)]
    fn from(variant: SwBbMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_BB_MODE` reader - Bitblt mode"]
pub type SwBbModeR = crate::BitReader<SwBbMode>;
impl SwBbModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwBbMode {
        match self.bits {
            false => SwBbMode::B0,
            true => SwBbMode::B1,
        }
    }
    #[doc = "SRC + DST => DST"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwBbMode::B0
    }
    #[doc = "SRC + SRC1 => DST"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwBbMode::B1
    }
}
#[doc = "Field `SW_BB_MODE` writer - Bitblt mode"]
pub type SwBbModeW<'a, REG> = crate::BitWriter<'a, REG, SwBbMode>;
impl<'a, REG> SwBbModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRC + DST => DST"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwBbMode::B0)
    }
    #[doc = "SRC + SRC1 => DST"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwBbMode::B1)
    }
}
#[doc = "Color fill/ROP4 pattern\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwCfRop4Pat {
    #[doc = "0: solid color"]
    B0 = 0,
    #[doc = "1: pattern color"]
    B1 = 1,
}
impl From<SwCfRop4Pat> for bool {
    #[inline(always)]
    fn from(variant: SwCfRop4Pat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_CF_ROP4_PAT` reader - Color fill/ROP4 pattern"]
pub type SwCfRop4PatR = crate::BitReader<SwCfRop4Pat>;
impl SwCfRop4PatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwCfRop4Pat {
        match self.bits {
            false => SwCfRop4Pat::B0,
            true => SwCfRop4Pat::B1,
        }
    }
    #[doc = "solid color"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwCfRop4Pat::B0
    }
    #[doc = "pattern color"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwCfRop4Pat::B1
    }
}
#[doc = "Field `SW_CF_ROP4_PAT` writer - Color fill/ROP4 pattern"]
pub type SwCfRop4PatW<'a, REG> = crate::BitWriter<'a, REG, SwCfRop4Pat>;
impl<'a, REG> SwCfRop4PatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "solid color"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwCfRop4Pat::B0)
    }
    #[doc = "pattern color"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwCfRop4Pat::B1)
    }
}
#[doc = "ARGB888 alpha zero key mode\n\n0x000000 would be changed to\n\n0x000100(RGB888)/0x0020(RGB565)for ARGB888 to\n\nRGBX/RGB565 color key\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwAlphaZeroKey {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwAlphaZeroKey> for bool {
    #[inline(always)]
    fn from(variant: SwAlphaZeroKey) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_ALPHA_ZERO_KEY` reader - ARGB888 alpha zero key mode\n\n0x000000 would be changed to\n\n0x000100(RGB888)/0x0020(RGB565)for ARGB888 to\n\nRGBX/RGB565 color key"]
pub type SwAlphaZeroKeyR = crate::BitReader<SwAlphaZeroKey>;
impl SwAlphaZeroKeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwAlphaZeroKey {
        match self.bits {
            false => SwAlphaZeroKey::B0,
            true => SwAlphaZeroKey::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwAlphaZeroKey::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwAlphaZeroKey::B1
    }
}
#[doc = "Field `SW_ALPHA_ZERO_KEY` writer - ARGB888 alpha zero key mode\n\n0x000000 would be changed to\n\n0x000100(RGB888)/0x0020(RGB565)for ARGB888 to\n\nRGBX/RGB565 color key"]
pub type SwAlphaZeroKeyW<'a, REG> = crate::BitWriter<'a, REG, SwAlphaZeroKey>;
impl<'a, REG> SwAlphaZeroKeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwAlphaZeroKey::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwAlphaZeroKey::B1)
    }
}
#[doc = "Gradient saturation calculation mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwGradientSat {
    #[doc = "0: clip"]
    B0 = 0,
    #[doc = "1: not-clip"]
    B1 = 1,
}
impl From<SwGradientSat> for bool {
    #[inline(always)]
    fn from(variant: SwGradientSat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_GRADIENT_SAT` reader - Gradient saturation calculation mode"]
pub type SwGradientSatR = crate::BitReader<SwGradientSat>;
impl SwGradientSatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwGradientSat {
        match self.bits {
            false => SwGradientSat::B0,
            true => SwGradientSat::B1,
        }
    }
    #[doc = "clip"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwGradientSat::B0
    }
    #[doc = "not-clip"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwGradientSat::B1
    }
}
#[doc = "Field `SW_GRADIENT_SAT` writer - Gradient saturation calculation mode"]
pub type SwGradientSatW<'a, REG> = crate::BitWriter<'a, REG, SwGradientSat>;
impl<'a, REG> SwGradientSatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clip"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwGradientSat::B0)
    }
    #[doc = "not-clip"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwGradientSat::B1)
    }
}
#[doc = "Field `SW_INTR_CF_E` reader - Current command finished interrupt enable"]
pub type SwIntrCfER = crate::BitReader;
#[doc = "Field `SW_INTR_CF_E` writer - Current command finished interrupt enable"]
pub type SwIntrCfEW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - RGA 2D render mode"]
    #[inline(always)]
    pub fn sw_render_mode(&self) -> SwRenderModeR {
        SwRenderModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Bitblt mode"]
    #[inline(always)]
    pub fn sw_bb_mode(&self) -> SwBbModeR {
        SwBbModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Color fill/ROP4 pattern"]
    #[inline(always)]
    pub fn sw_cf_rop4_pat(&self) -> SwCfRop4PatR {
        SwCfRop4PatR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ARGB888 alpha zero key mode\n\n0x000000 would be changed to\n\n0x000100(RGB888)/0x0020(RGB565)for ARGB888 to\n\nRGBX/RGB565 color key"]
    #[inline(always)]
    pub fn sw_alpha_zero_key(&self) -> SwAlphaZeroKeyR {
        SwAlphaZeroKeyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Gradient saturation calculation mode"]
    #[inline(always)]
    pub fn sw_gradient_sat(&self) -> SwGradientSatR {
        SwGradientSatR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Current command finished interrupt enable"]
    #[inline(always)]
    pub fn sw_intr_cf_e(&self) -> SwIntrCfER {
        SwIntrCfER::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - RGA 2D render mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_render_mode(&mut self) -> SwRenderModeW<ModeCtrlSpec> {
        SwRenderModeW::new(self, 0)
    }
    #[doc = "Bit 3 - Bitblt mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_bb_mode(&mut self) -> SwBbModeW<ModeCtrlSpec> {
        SwBbModeW::new(self, 3)
    }
    #[doc = "Bit 4 - Color fill/ROP4 pattern"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cf_rop4_pat(&mut self) -> SwCfRop4PatW<ModeCtrlSpec> {
        SwCfRop4PatW::new(self, 4)
    }
    #[doc = "Bit 5 - ARGB888 alpha zero key mode\n\n0x000000 would be changed to\n\n0x000100(RGB888)/0x0020(RGB565)for ARGB888 to\n\nRGBX/RGB565 color key"]
    #[inline(always)]
    #[must_use]
    pub fn sw_alpha_zero_key(&mut self) -> SwAlphaZeroKeyW<ModeCtrlSpec> {
        SwAlphaZeroKeyW::new(self, 5)
    }
    #[doc = "Bit 6 - Gradient saturation calculation mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_gradient_sat(&mut self) -> SwGradientSatW<ModeCtrlSpec> {
        SwGradientSatW::new(self, 6)
    }
    #[doc = "Bit 7 - Current command finished interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_intr_cf_e(&mut self) -> SwIntrCfEW<ModeCtrlSpec> {
        SwIntrCfEW::new(self, 7)
    }
}
#[doc = "RGA mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeCtrlSpec;
impl crate::RegisterSpec for ModeCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode_ctrl::R`](R) reader structure"]
impl crate::Readable for ModeCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mode_ctrl::W`](W) writer structure"]
impl crate::Writable for ModeCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODE_CTRL to value 0"]
impl crate::Resettable for ModeCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
