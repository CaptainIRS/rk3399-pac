#[doc = "Register `ALPHA_CTRL0` reader"]
pub type R = crate::R<AlphaCtrl0Spec>;
#[doc = "Register `ALPHA_CTRL0` writer"]
pub type W = crate::W<AlphaCtrl0Spec>;
#[doc = "Alpha or ROP enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwAlphaRopE {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwAlphaRopE> for bool {
    #[inline(always)]
    fn from(variant: SwAlphaRopE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_ALPHA_ROP_E` reader - Alpha or ROP enable"]
pub type SwAlphaRopER = crate::BitReader<SwAlphaRopE>;
impl SwAlphaRopER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwAlphaRopE {
        match self.bits {
            false => SwAlphaRopE::B0,
            true => SwAlphaRopE::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwAlphaRopE::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwAlphaRopE::B1
    }
}
#[doc = "Field `SW_ALPHA_ROP_E` writer - Alpha or ROP enable"]
pub type SwAlphaRopEW<'a, REG> = crate::BitWriter<'a, REG, SwAlphaRopE>;
impl<'a, REG> SwAlphaRopEW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwAlphaRopE::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwAlphaRopE::B1)
    }
}
#[doc = "Alpha or ROP select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwAlphaRopSel {
    #[doc = "0: alpha"]
    B0 = 0,
    #[doc = "1: ROP"]
    B1 = 1,
}
impl From<SwAlphaRopSel> for bool {
    #[inline(always)]
    fn from(variant: SwAlphaRopSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_ALPHA_ROP_SEL` reader - Alpha or ROP select"]
pub type SwAlphaRopSelR = crate::BitReader<SwAlphaRopSel>;
impl SwAlphaRopSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwAlphaRopSel {
        match self.bits {
            false => SwAlphaRopSel::B0,
            true => SwAlphaRopSel::B1,
        }
    }
    #[doc = "alpha"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwAlphaRopSel::B0
    }
    #[doc = "ROP"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwAlphaRopSel::B1
    }
}
#[doc = "Field `SW_ALPHA_ROP_SEL` writer - Alpha or ROP select"]
pub type SwAlphaRopSelW<'a, REG> = crate::BitWriter<'a, REG, SwAlphaRopSel>;
impl<'a, REG> SwAlphaRopSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "alpha"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwAlphaRopSel::B0)
    }
    #[doc = "ROP"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwAlphaRopSel::B1)
    }
}
#[doc = "ROP mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwRopMode {
    #[doc = "0: ROP 2"]
    B00 = 0,
    #[doc = "1: ROP 3"]
    B01 = 1,
    #[doc = "2: ROP 4"]
    B10 = 2,
}
impl From<SwRopMode> for u8 {
    #[inline(always)]
    fn from(variant: SwRopMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwRopMode {
    type Ux = u8;
}
#[doc = "Field `SW_ROP_MODE` reader - ROP mode select"]
pub type SwRopModeR = crate::FieldReader<SwRopMode>;
impl SwRopModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwRopMode> {
        match self.bits {
            0 => Some(SwRopMode::B00),
            1 => Some(SwRopMode::B01),
            2 => Some(SwRopMode::B10),
            _ => None,
        }
    }
    #[doc = "ROP 2"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SwRopMode::B00
    }
    #[doc = "ROP 3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SwRopMode::B01
    }
    #[doc = "ROP 4"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SwRopMode::B10
    }
}
#[doc = "Field `SW_ROP_MODE` writer - ROP mode select"]
pub type SwRopModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, SwRopMode>;
impl<'a, REG> SwRopModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ROP 2"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SwRopMode::B00)
    }
    #[doc = "ROP 3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SwRopMode::B01)
    }
    #[doc = "ROP 4"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SwRopMode::B10)
    }
}
#[doc = "Field `SW_SRC_GLOBAL_ALPHA` reader - global alpha value of SRC(Ags)\n\nfading value in fading mod"]
pub type SwSrcGlobalAlphaR = crate::FieldReader;
#[doc = "Field `SW_SRC_GLOBAL_ALPHA` writer - global alpha value of SRC(Ags)\n\nfading value in fading mod"]
pub type SwSrcGlobalAlphaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_DST_GLOBAL_ALPHA` reader - global alpha value of DST(Agd)"]
pub type SwDstGlobalAlphaR = crate::FieldReader;
#[doc = "Field `SW_DST_GLOBAL_ALPHA` writer - global alpha value of DST(Agd)"]
pub type SwDstGlobalAlphaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "ROP4 mask endian swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwMaskEndian {
    #[doc = "0: big endian"]
    B0 = 0,
    #[doc = "1: little endian"]
    B1 = 1,
}
impl From<SwMaskEndian> for bool {
    #[inline(always)]
    fn from(variant: SwMaskEndian) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_MASK_ENDIAN` reader - ROP4 mask endian swap"]
pub type SwMaskEndianR = crate::BitReader<SwMaskEndian>;
impl SwMaskEndianR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwMaskEndian {
        match self.bits {
            false => SwMaskEndian::B0,
            true => SwMaskEndian::B1,
        }
    }
    #[doc = "big endian"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwMaskEndian::B0
    }
    #[doc = "little endian"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwMaskEndian::B1
    }
}
#[doc = "Field `SW_MASK_ENDIAN` writer - ROP4 mask endian swap"]
pub type SwMaskEndianW<'a, REG> = crate::BitWriter<'a, REG, SwMaskEndian>;
impl<'a, REG> SwMaskEndianW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "big endian"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwMaskEndian::B0)
    }
    #[doc = "little endian"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwMaskEndian::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Alpha or ROP enable"]
    #[inline(always)]
    pub fn sw_alpha_rop_e(&self) -> SwAlphaRopER {
        SwAlphaRopER::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alpha or ROP select"]
    #[inline(always)]
    pub fn sw_alpha_rop_sel(&self) -> SwAlphaRopSelR {
        SwAlphaRopSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - ROP mode select"]
    #[inline(always)]
    pub fn sw_rop_mode(&self) -> SwRopModeR {
        SwRopModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:11 - global alpha value of SRC(Ags)\n\nfading value in fading mod"]
    #[inline(always)]
    pub fn sw_src_global_alpha(&self) -> SwSrcGlobalAlphaR {
        SwSrcGlobalAlphaR::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:19 - global alpha value of DST(Agd)"]
    #[inline(always)]
    pub fn sw_dst_global_alpha(&self) -> SwDstGlobalAlphaR {
        SwDstGlobalAlphaR::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bit 20 - ROP4 mask endian swap"]
    #[inline(always)]
    pub fn sw_mask_endian(&self) -> SwMaskEndianR {
        SwMaskEndianR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alpha or ROP enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_alpha_rop_e(&mut self) -> SwAlphaRopEW<AlphaCtrl0Spec> {
        SwAlphaRopEW::new(self, 0)
    }
    #[doc = "Bit 1 - Alpha or ROP select"]
    #[inline(always)]
    #[must_use]
    pub fn sw_alpha_rop_sel(&mut self) -> SwAlphaRopSelW<AlphaCtrl0Spec> {
        SwAlphaRopSelW::new(self, 1)
    }
    #[doc = "Bits 2:3 - ROP mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rop_mode(&mut self) -> SwRopModeW<AlphaCtrl0Spec> {
        SwRopModeW::new(self, 2)
    }
    #[doc = "Bits 4:11 - global alpha value of SRC(Ags)\n\nfading value in fading mod"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_global_alpha(&mut self) -> SwSrcGlobalAlphaW<AlphaCtrl0Spec> {
        SwSrcGlobalAlphaW::new(self, 4)
    }
    #[doc = "Bits 12:19 - global alpha value of DST(Agd)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_global_alpha(&mut self) -> SwDstGlobalAlphaW<AlphaCtrl0Spec> {
        SwDstGlobalAlphaW::new(self, 12)
    }
    #[doc = "Bit 20 - ROP4 mask endian swap"]
    #[inline(always)]
    #[must_use]
    pub fn sw_mask_endian(&mut self) -> SwMaskEndianW<AlphaCtrl0Spec> {
        SwMaskEndianW::new(self, 20)
    }
}
#[doc = "Alpha control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alpha_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alpha_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlphaCtrl0Spec;
impl crate::RegisterSpec for AlphaCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alpha_ctrl0::R`](R) reader structure"]
impl crate::Readable for AlphaCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`alpha_ctrl0::W`](W) writer structure"]
impl crate::Writable for AlphaCtrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALPHA_CTRL0 to value 0"]
impl crate::Resettable for AlphaCtrl0Spec {
    const RESET_VALUE: u32 = 0;
}
