#[doc = "Register `BCSH_CTRL` reader"]
pub type R = crate::R<BcshCtrlSpec>;
#[doc = "Register `BCSH_CTRL` writer"]
pub type W = crate::W<BcshCtrlSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BcshY2rEn {
    #[doc = "0: bypass"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<BcshY2rEn> for bool {
    #[inline(always)]
    fn from(variant: BcshY2rEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCSH_Y2R_EN` reader - "]
pub type BcshY2rEnR = crate::BitReader<BcshY2rEn>;
impl BcshY2rEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BcshY2rEn {
        match self.bits {
            false => BcshY2rEn::B0,
            true => BcshY2rEn::B1,
        }
    }
    #[doc = "bypass"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BcshY2rEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BcshY2rEn::B1
    }
}
#[doc = "Field `BCSH_Y2R_EN` writer - "]
pub type BcshY2rEnW<'a, REG> = crate::BitWriter<'a, REG, BcshY2rEn>;
impl<'a, REG> BcshY2rEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bypass"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BcshY2rEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BcshY2rEn::B1)
    }
}
#[doc = "Color space conversion(YUV2RGB):\n\n2'b00/01 : mpeg\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BcshY2rCscMode {
    #[doc = "2: jpeg"]
    B10 = 2,
    #[doc = "3: hd"]
    B11 = 3,
}
impl From<BcshY2rCscMode> for u8 {
    #[inline(always)]
    fn from(variant: BcshY2rCscMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BcshY2rCscMode {
    type Ux = u8;
}
#[doc = "Field `BCSH_Y2R_CSC_MODE` reader - Color space conversion(YUV2RGB):\n\n2'b00/01 : mpeg"]
pub type BcshY2rCscModeR = crate::FieldReader<BcshY2rCscMode>;
impl BcshY2rCscModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BcshY2rCscMode> {
        match self.bits {
            2 => Some(BcshY2rCscMode::B10),
            3 => Some(BcshY2rCscMode::B11),
            _ => None,
        }
    }
    #[doc = "jpeg"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == BcshY2rCscMode::B10
    }
    #[doc = "hd"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == BcshY2rCscMode::B11
    }
}
#[doc = "Field `BCSH_Y2R_CSC_MODE` writer - Color space conversion(YUV2RGB):\n\n2'b00/01 : mpeg"]
pub type BcshY2rCscModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, BcshY2rCscMode>;
impl<'a, REG> BcshY2rCscModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "jpeg"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(BcshY2rCscMode::B10)
    }
    #[doc = "hd"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(BcshY2rCscMode::B11)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BcshR2yEn {
    #[doc = "0: bypass"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<BcshR2yEn> for bool {
    #[inline(always)]
    fn from(variant: BcshR2yEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCSH_R2Y_EN` reader - "]
pub type BcshR2yEnR = crate::BitReader<BcshR2yEn>;
impl BcshR2yEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BcshR2yEn {
        match self.bits {
            false => BcshR2yEn::B0,
            true => BcshR2yEn::B1,
        }
    }
    #[doc = "bypass"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BcshR2yEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BcshR2yEn::B1
    }
}
#[doc = "Field `BCSH_R2Y_EN` writer - "]
pub type BcshR2yEnW<'a, REG> = crate::BitWriter<'a, REG, BcshR2yEn>;
impl<'a, REG> BcshR2yEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bypass"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BcshR2yEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BcshR2yEn::B1)
    }
}
#[doc = "Color space conversion:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BcshR2yCscMode {
    #[doc = "0: BT601"]
    B0 = 0,
    #[doc = "1: BT709"]
    B1 = 1,
}
impl From<BcshR2yCscMode> for bool {
    #[inline(always)]
    fn from(variant: BcshR2yCscMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCSH_R2Y_CSC_MODE` reader - Color space conversion:"]
pub type BcshR2yCscModeR = crate::BitReader<BcshR2yCscMode>;
impl BcshR2yCscModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BcshR2yCscMode {
        match self.bits {
            false => BcshR2yCscMode::B0,
            true => BcshR2yCscMode::B1,
        }
    }
    #[doc = "BT601"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BcshR2yCscMode::B0
    }
    #[doc = "BT709"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BcshR2yCscMode::B1
    }
}
#[doc = "Field `BCSH_R2Y_CSC_MODE` writer - Color space conversion:"]
pub type BcshR2yCscModeW<'a, REG> = crate::BitWriter<'a, REG, BcshR2yCscMode>;
impl<'a, REG> BcshR2yCscModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BT601"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BcshR2yCscMode::B0)
    }
    #[doc = "BT709"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BcshR2yCscMode::B1)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bcsh_y2r_en(&self) -> BcshY2rEnR {
        BcshY2rEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Color space conversion(YUV2RGB):\n\n2'b00/01 : mpeg"]
    #[inline(always)]
    pub fn bcsh_y2r_csc_mode(&self) -> BcshY2rCscModeR {
        BcshY2rCscModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bcsh_r2y_en(&self) -> BcshR2yEnR {
        BcshR2yEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Color space conversion:"]
    #[inline(always)]
    pub fn bcsh_r2y_csc_mode(&self) -> BcshR2yCscModeR {
        BcshR2yCscModeR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn bcsh_y2r_en(&mut self) -> BcshY2rEnW<BcshCtrlSpec> {
        BcshY2rEnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Color space conversion(YUV2RGB):\n\n2'b00/01 : mpeg"]
    #[inline(always)]
    #[must_use]
    pub fn bcsh_y2r_csc_mode(&mut self) -> BcshY2rCscModeW<BcshCtrlSpec> {
        BcshY2rCscModeW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn bcsh_r2y_en(&mut self) -> BcshR2yEnW<BcshCtrlSpec> {
        BcshR2yEnW::new(self, 4)
    }
    #[doc = "Bit 6 - Color space conversion:"]
    #[inline(always)]
    #[must_use]
    pub fn bcsh_r2y_csc_mode(&mut self) -> BcshR2yCscModeW<BcshCtrlSpec> {
        BcshR2yCscModeW::new(self, 6)
    }
}
#[doc = "BCSH contrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcsh_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcsh_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcshCtrlSpec;
impl crate::RegisterSpec for BcshCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcsh_ctrl::R`](R) reader structure"]
impl crate::Readable for BcshCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bcsh_ctrl::W`](W) writer structure"]
impl crate::Writable for BcshCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCSH_CTRL to value 0"]
impl crate::Resettable for BcshCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
