#[doc = "Register `MI_XTD_FORMAT_CTRL` reader"]
pub type R = crate::R<MiXtdFormatCtrlSpec>;
#[doc = "Register `MI_XTD_FORMAT_CTRL` writer"]
pub type W = crate::W<MiXtdFormatCtrlSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nv21Main {
    #[doc = "0: main path: use NV12 storage format for semi-planar YCbCr 4:2:x mode, Cb is located on even addresses."]
    B0 = 0,
    #[doc = "1: main path: use NV21 storage format for semi-planar YCbCr 4:2:x mode, Cr is located on even"]
    B1 = 1,
}
impl From<Nv21Main> for bool {
    #[inline(always)]
    fn from(variant: Nv21Main) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `nv21_main` reader - "]
pub type Nv21MainR = crate::BitReader<Nv21Main>;
impl Nv21MainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nv21Main {
        match self.bits {
            false => Nv21Main::B0,
            true => Nv21Main::B1,
        }
    }
    #[doc = "main path: use NV12 storage format for semi-planar YCbCr 4:2:x mode, Cb is located on even addresses."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Nv21Main::B0
    }
    #[doc = "main path: use NV21 storage format for semi-planar YCbCr 4:2:x mode, Cr is located on even"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Nv21Main::B1
    }
}
#[doc = "Field `nv21_main` writer - "]
pub type Nv21MainW<'a, REG> = crate::BitWriter<'a, REG, Nv21Main>;
impl<'a, REG> Nv21MainW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "main path: use NV12 storage format for semi-planar YCbCr 4:2:x mode, Cb is located on even addresses."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Nv21Main::B0)
    }
    #[doc = "main path: use NV21 storage format for semi-planar YCbCr 4:2:x mode, Cr is located on even"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Nv21Main::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nv21Self {
    #[doc = "0: self path: use NV12 storage format for semi-planar YCbCr 4:2:x mode, Cb is located on even addresses."]
    B0 = 0,
    #[doc = "1: self path: use NV21 storage format for semi-planar YCbCr 4:2:x mode, Cr is located on even addresses."]
    B1 = 1,
}
impl From<Nv21Self> for bool {
    #[inline(always)]
    fn from(variant: Nv21Self) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `nv21_self` reader - "]
pub type Nv21SelfR = crate::BitReader<Nv21Self>;
impl Nv21SelfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nv21Self {
        match self.bits {
            false => Nv21Self::B0,
            true => Nv21Self::B1,
        }
    }
    #[doc = "self path: use NV12 storage format for semi-planar YCbCr 4:2:x mode, Cb is located on even addresses."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Nv21Self::B0
    }
    #[doc = "self path: use NV21 storage format for semi-planar YCbCr 4:2:x mode, Cr is located on even addresses."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Nv21Self::B1
    }
}
#[doc = "Field `nv21_self` writer - "]
pub type Nv21SelfW<'a, REG> = crate::BitWriter<'a, REG, Nv21Self>;
impl<'a, REG> Nv21SelfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "self path: use NV12 storage format for semi-planar YCbCr 4:2:x mode, Cb is located on even addresses."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Nv21Self::B0)
    }
    #[doc = "self path: use NV21 storage format for semi-planar YCbCr 4:2:x mode, Cr is located on even addresses."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Nv21Self::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nv21DmaRead {
    #[doc = "0: dma read path: use NV12 storage format for semi-planar YCbCr 4:2:x mode, Cb is located on even addresses."]
    B0 = 0,
    #[doc = "1: dma read path: use NV21 storage format for semi-planar YCbCr 4:2:x mode, Cr is located on even"]
    B1 = 1,
}
impl From<Nv21DmaRead> for bool {
    #[inline(always)]
    fn from(variant: Nv21DmaRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `nv21_dma_read` reader - "]
pub type Nv21DmaReadR = crate::BitReader<Nv21DmaRead>;
impl Nv21DmaReadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nv21DmaRead {
        match self.bits {
            false => Nv21DmaRead::B0,
            true => Nv21DmaRead::B1,
        }
    }
    #[doc = "dma read path: use NV12 storage format for semi-planar YCbCr 4:2:x mode, Cb is located on even addresses."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Nv21DmaRead::B0
    }
    #[doc = "dma read path: use NV21 storage format for semi-planar YCbCr 4:2:x mode, Cr is located on even"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Nv21DmaRead::B1
    }
}
#[doc = "Field `nv21_dma_read` writer - "]
pub type Nv21DmaReadW<'a, REG> = crate::BitWriter<'a, REG, Nv21DmaRead>;
impl<'a, REG> Nv21DmaReadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "dma read path: use NV12 storage format for semi-planar YCbCr 4:2:x mode, Cb is located on even addresses."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Nv21DmaRead::B0)
    }
    #[doc = "dma read path: use NV21 storage format for semi-planar YCbCr 4:2:x mode, Cr is located on even"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Nv21DmaRead::B1)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn nv21_main(&self) -> Nv21MainR {
        Nv21MainR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn nv21_self(&self) -> Nv21SelfR {
        Nv21SelfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn nv21_dma_read(&self) -> Nv21DmaReadR {
        Nv21DmaReadR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn nv21_main(&mut self) -> Nv21MainW<MiXtdFormatCtrlSpec> {
        Nv21MainW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn nv21_self(&mut self) -> Nv21SelfW<MiXtdFormatCtrlSpec> {
        Nv21SelfW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn nv21_dma_read(&mut self) -> Nv21DmaReadW<MiXtdFormatCtrlSpec> {
        Nv21DmaReadW::new(self, 2)
    }
}
#[doc = "Extended Storage Format Control for main, self and dma read path\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_xtd_format_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_xtd_format_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiXtdFormatCtrlSpec;
impl crate::RegisterSpec for MiXtdFormatCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_xtd_format_ctrl::R`](R) reader structure"]
impl crate::Readable for MiXtdFormatCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mi_xtd_format_ctrl::W`](W) writer structure"]
impl crate::Writable for MiXtdFormatCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_XTD_FORMAT_CTRL to value 0"]
impl crate::Resettable for MiXtdFormatCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
