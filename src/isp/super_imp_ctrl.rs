#[doc = "Register `SUPER_IMP_CTRL` reader"]
pub type R = crate::R<SuperImpCtrlSpec>;
#[doc = "Register `SUPER_IMP_CTRL` writer"]
pub type W = crate::W<SuperImpCtrlSpec>;
#[doc = "Bypass mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BypassMode {
    #[doc = "0: bypass mode"]
    B0 = 0,
    #[doc = "1: normal operation"]
    B1 = 1,
}
impl From<BypassMode> for bool {
    #[inline(always)]
    fn from(variant: BypassMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `bypass_mode` reader - Bypass mode"]
pub type BypassModeR = crate::BitReader<BypassMode>;
impl BypassModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BypassMode {
        match self.bits {
            false => BypassMode::B0,
            true => BypassMode::B1,
        }
    }
    #[doc = "bypass mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BypassMode::B0
    }
    #[doc = "normal operation"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BypassMode::B1
    }
}
#[doc = "Field `bypass_mode` writer - Bypass mode"]
pub type BypassModeW<'a, REG> = crate::BitWriter<'a, REG, BypassMode>;
impl<'a, REG> BypassModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bypass mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BypassMode::B0)
    }
    #[doc = "normal operation"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BypassMode::B1)
    }
}
#[doc = "Define the reference image\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefImage {
    #[doc = "1: superimpose bitmap from main memory"]
    B1 = 1,
    #[doc = "0: image from the Image Effect module Note: the reference image defines the size of the output image"]
    B0 = 0,
}
impl From<RefImage> for bool {
    #[inline(always)]
    fn from(variant: RefImage) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ref_image` reader - Define the reference image"]
pub type RefImageR = crate::BitReader<RefImage>;
impl RefImageR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RefImage {
        match self.bits {
            true => RefImage::B1,
            false => RefImage::B0,
        }
    }
    #[doc = "superimpose bitmap from main memory"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RefImage::B1
    }
    #[doc = "image from the Image Effect module Note: the reference image defines the size of the output image"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RefImage::B0
    }
}
#[doc = "Field `ref_image` writer - Define the reference image"]
pub type RefImageW<'a, REG> = crate::BitWriter<'a, REG, RefImage>;
impl<'a, REG> RefImageW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "superimpose bitmap from main memory"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RefImage::B1)
    }
    #[doc = "image from the Image Effect module Note: the reference image defines the size of the output image"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RefImage::B0)
    }
}
#[doc = "transparency mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransparencyMode {
    #[doc = "1: transparency mode disabled"]
    B1 = 1,
    #[doc = "0: transparency mode enabled"]
    B0 = 0,
}
impl From<TransparencyMode> for bool {
    #[inline(always)]
    fn from(variant: TransparencyMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `transparency_mode` reader - transparency mode"]
pub type TransparencyModeR = crate::BitReader<TransparencyMode>;
impl TransparencyModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TransparencyMode {
        match self.bits {
            true => TransparencyMode::B1,
            false => TransparencyMode::B0,
        }
    }
    #[doc = "transparency mode disabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TransparencyMode::B1
    }
    #[doc = "transparency mode enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TransparencyMode::B0
    }
}
#[doc = "Field `transparency_mode` writer - transparency mode"]
pub type TransparencyModeW<'a, REG> = crate::BitWriter<'a, REG, TransparencyMode>;
impl<'a, REG> TransparencyModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "transparency mode disabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TransparencyMode::B1)
    }
    #[doc = "transparency mode enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TransparencyMode::B0)
    }
}
impl R {
    #[doc = "Bit 0 - Bypass mode"]
    #[inline(always)]
    pub fn bypass_mode(&self) -> BypassModeR {
        BypassModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Define the reference image"]
    #[inline(always)]
    pub fn ref_image(&self) -> RefImageR {
        RefImageR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - transparency mode"]
    #[inline(always)]
    pub fn transparency_mode(&self) -> TransparencyModeR {
        TransparencyModeR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass mode"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_mode(&mut self) -> BypassModeW<SuperImpCtrlSpec> {
        BypassModeW::new(self, 0)
    }
    #[doc = "Bit 1 - Define the reference image"]
    #[inline(always)]
    #[must_use]
    pub fn ref_image(&mut self) -> RefImageW<SuperImpCtrlSpec> {
        RefImageW::new(self, 1)
    }
    #[doc = "Bit 2 - transparency mode"]
    #[inline(always)]
    #[must_use]
    pub fn transparency_mode(&mut self) -> TransparencyModeW<SuperImpCtrlSpec> {
        TransparencyModeW::new(self, 2)
    }
}
#[doc = "Global control register\n\nNote: in the bypass mode the data stream from Image \n\n\n\nEffect is transmitted to MUX module without overlaying \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`super_imp_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`super_imp_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SuperImpCtrlSpec;
impl crate::RegisterSpec for SuperImpCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`super_imp_ctrl::R`](R) reader structure"]
impl crate::Readable for SuperImpCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`super_imp_ctrl::W`](W) writer structure"]
impl crate::Writable for SuperImpCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUPER_IMP_CTRL to value 0"]
impl crate::Resettable for SuperImpCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
