#[doc = "Register `VIDEO_CTL_4` reader"]
pub type R = crate::R<VideoCtl4Spec>;
#[doc = "Register `VIDEO_CTL_4` writer"]
pub type W = crate::W<VideoCtl4Spec>;
#[doc = "Display BIST type.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BistType {
    #[doc = "0: Color bar,"]
    B00 = 0,
    #[doc = "1: White, gray and black bar,"]
    B01 = 1,
    #[doc = "2: Mobile white bar,"]
    B10 = 2,
    #[doc = "3: Reserved."]
    B11 = 3,
}
impl From<BistType> for u8 {
    #[inline(always)]
    fn from(variant: BistType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BistType {
    type Ux = u8;
}
#[doc = "Field `BIST_TYPE` reader - Display BIST type."]
pub type BistTypeR = crate::FieldReader<BistType>;
impl BistTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BistType {
        match self.bits {
            0 => BistType::B00,
            1 => BistType::B01,
            2 => BistType::B10,
            3 => BistType::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Color bar,"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == BistType::B00
    }
    #[doc = "White, gray and black bar,"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == BistType::B01
    }
    #[doc = "Mobile white bar,"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == BistType::B10
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == BistType::B11
    }
}
#[doc = "Field `BIST_TYPE` writer - Display BIST type."]
pub type BistTypeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, BistType>;
impl<'a, REG> BistTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Color bar,"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(BistType::B00)
    }
    #[doc = "White, gray and black bar,"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(BistType::B01)
    }
    #[doc = "Mobile white bar,"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(BistType::B10)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(BistType::B11)
    }
}
#[doc = "Control display BIST color bar width.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BistWidth {
    #[doc = "1: Each bar is 64 pixel width,"]
    B1 = 1,
    #[doc = "0: Each bar is 32 pixel width."]
    B0 = 0,
}
impl From<BistWidth> for bool {
    #[inline(always)]
    fn from(variant: BistWidth) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIST_WIDTH` reader - Control display BIST color bar width."]
pub type BistWidthR = crate::BitReader<BistWidth>;
impl BistWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BistWidth {
        match self.bits {
            true => BistWidth::B1,
            false => BistWidth::B0,
        }
    }
    #[doc = "Each bar is 64 pixel width,"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BistWidth::B1
    }
    #[doc = "Each bar is 32 pixel width."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BistWidth::B0
    }
}
#[doc = "Field `BIST_WIDTH` writer - Control display BIST color bar width."]
pub type BistWidthW<'a, REG> = crate::BitWriter<'a, REG, BistWidth>;
impl<'a, REG> BistWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Each bar is 64 pixel width,"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BistWidth::B1)
    }
    #[doc = "Each bar is 32 pixel width."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BistWidth::B0)
    }
}
#[doc = "Video BIST enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BistEn {
    #[doc = "1: Enable video BIST,"]
    B1 = 1,
    #[doc = "0: Normal operation mode."]
    B0 = 0,
}
impl From<BistEn> for bool {
    #[inline(always)]
    fn from(variant: BistEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIST_EN` reader - Video BIST enable."]
pub type BistEnR = crate::BitReader<BistEn>;
impl BistEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BistEn {
        match self.bits {
            true => BistEn::B1,
            false => BistEn::B0,
        }
    }
    #[doc = "Enable video BIST,"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BistEn::B1
    }
    #[doc = "Normal operation mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BistEn::B0
    }
}
#[doc = "Field `BIST_EN` writer - Video BIST enable."]
pub type BistEnW<'a, REG> = crate::BitWriter<'a, REG, BistEn>;
impl<'a, REG> BistEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable video BIST,"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BistEn::B1)
    }
    #[doc = "Normal operation mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BistEn::B0)
    }
}
impl R {
    #[doc = "Bits 0:1 - Display BIST type."]
    #[inline(always)]
    pub fn bist_type(&self) -> BistTypeR {
        BistTypeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Control display BIST color bar width."]
    #[inline(always)]
    pub fn bist_width(&self) -> BistWidthR {
        BistWidthR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Video BIST enable."]
    #[inline(always)]
    pub fn bist_en(&self) -> BistEnR {
        BistEnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Display BIST type."]
    #[inline(always)]
    #[must_use]
    pub fn bist_type(&mut self) -> BistTypeW<VideoCtl4Spec> {
        BistTypeW::new(self, 0)
    }
    #[doc = "Bit 2 - Control display BIST color bar width."]
    #[inline(always)]
    #[must_use]
    pub fn bist_width(&mut self) -> BistWidthW<VideoCtl4Spec> {
        BistWidthW::new(self, 2)
    }
    #[doc = "Bit 3 - Video BIST enable."]
    #[inline(always)]
    #[must_use]
    pub fn bist_en(&mut self) -> BistEnW<VideoCtl4Spec> {
        BistEnW::new(self, 3)
    }
}
#[doc = "Video Control 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`video_ctl_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`video_ctl_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VideoCtl4Spec;
impl crate::RegisterSpec for VideoCtl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`video_ctl_4::R`](R) reader structure"]
impl crate::Readable for VideoCtl4Spec {}
#[doc = "`write(|w| ..)` method takes [`video_ctl_4::W`](W) writer structure"]
impl crate::Writable for VideoCtl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VIDEO_CTL_4 to value 0"]
impl crate::Resettable for VideoCtl4Spec {
    const RESET_VALUE: u32 = 0;
}
