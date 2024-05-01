#[doc = "Register `MMU_CTRL1` reader"]
pub type R = crate::R<MmuCtrl1Spec>;
#[doc = "Register `MMU_CTRL1` writer"]
pub type W = crate::W<MmuCtrl1Spec>;
#[doc = "RGA SRC channel MMU enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrcMmuEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwSrcMmuEn> for bool {
    #[inline(always)]
    fn from(variant: SwSrcMmuEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC_MMU_EN` reader - RGA SRC channel MMU enable"]
pub type SwSrcMmuEnR = crate::BitReader<SwSrcMmuEn>;
impl SwSrcMmuEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrcMmuEn {
        match self.bits {
            false => SwSrcMmuEn::B0,
            true => SwSrcMmuEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrcMmuEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrcMmuEn::B1
    }
}
#[doc = "Field `SW_SRC_MMU_EN` writer - RGA SRC channel MMU enable"]
pub type SwSrcMmuEnW<'a, REG> = crate::BitWriter<'a, REG, SwSrcMmuEn>;
impl<'a, REG> SwSrcMmuEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcMmuEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcMmuEn::B1)
    }
}
#[doc = "Field `SW_SRC_MMU_FLUSH` reader - RGA SRC channel MMU TLB flush:\n\nSet 1 to this bit to flush MMU TLB, auto clear"]
pub type SwSrcMmuFlushR = crate::BitReader;
#[doc = "Field `SW_SRC_MMU_FLUSH` writer - RGA SRC channel MMU TLB flush:\n\nSet 1 to this bit to flush MMU TLB, auto clear"]
pub type SwSrcMmuFlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrcMmuPrefetchEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwSrcMmuPrefetchEn> for bool {
    #[inline(always)]
    fn from(variant: SwSrcMmuPrefetchEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC_MMU_PREFETCH_EN` reader - "]
pub type SwSrcMmuPrefetchEnR = crate::BitReader<SwSrcMmuPrefetchEn>;
impl SwSrcMmuPrefetchEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrcMmuPrefetchEn {
        match self.bits {
            false => SwSrcMmuPrefetchEn::B0,
            true => SwSrcMmuPrefetchEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrcMmuPrefetchEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrcMmuPrefetchEn::B1
    }
}
#[doc = "Field `SW_SRC_MMU_PREFETCH_EN` writer - "]
pub type SwSrcMmuPrefetchEnW<'a, REG> = crate::BitWriter<'a, REG, SwSrcMmuPrefetchEn>;
impl<'a, REG> SwSrcMmuPrefetchEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcMmuPrefetchEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcMmuPrefetchEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrcMmuPrefetchDir {
    #[doc = "0: forward"]
    B0 = 0,
    #[doc = "1: backward"]
    B1 = 1,
}
impl From<SwSrcMmuPrefetchDir> for bool {
    #[inline(always)]
    fn from(variant: SwSrcMmuPrefetchDir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC_MMU_PREFETCH_DIR` reader - "]
pub type SwSrcMmuPrefetchDirR = crate::BitReader<SwSrcMmuPrefetchDir>;
impl SwSrcMmuPrefetchDirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrcMmuPrefetchDir {
        match self.bits {
            false => SwSrcMmuPrefetchDir::B0,
            true => SwSrcMmuPrefetchDir::B1,
        }
    }
    #[doc = "forward"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrcMmuPrefetchDir::B0
    }
    #[doc = "backward"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrcMmuPrefetchDir::B1
    }
}
#[doc = "Field `SW_SRC_MMU_PREFETCH_DIR` writer - "]
pub type SwSrcMmuPrefetchDirW<'a, REG> = crate::BitWriter<'a, REG, SwSrcMmuPrefetchDir>;
impl<'a, REG> SwSrcMmuPrefetchDirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "forward"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcMmuPrefetchDir::B0)
    }
    #[doc = "backward"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcMmuPrefetchDir::B1)
    }
}
#[doc = "RGA SRC1 channel MMU enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrc1MmuEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwSrc1MmuEn> for bool {
    #[inline(always)]
    fn from(variant: SwSrc1MmuEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC1_MMU_EN` reader - RGA SRC1 channel MMU enable"]
pub type SwSrc1MmuEnR = crate::BitReader<SwSrc1MmuEn>;
impl SwSrc1MmuEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrc1MmuEn {
        match self.bits {
            false => SwSrc1MmuEn::B0,
            true => SwSrc1MmuEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrc1MmuEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrc1MmuEn::B1
    }
}
#[doc = "Field `SW_SRC1_MMU_EN` writer - RGA SRC1 channel MMU enable"]
pub type SwSrc1MmuEnW<'a, REG> = crate::BitWriter<'a, REG, SwSrc1MmuEn>;
impl<'a, REG> SwSrc1MmuEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrc1MmuEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrc1MmuEn::B1)
    }
}
#[doc = "Field `SW_SRC1_MMU_FLUSH` reader - RGA SRC1 channel MMU TLB flush:\n\nSet 1 to this bit to flush MMU TLB, auto clear"]
pub type SwSrc1MmuFlushR = crate::BitReader;
#[doc = "Field `SW_SRC1_MMU_FLUSH` writer - RGA SRC1 channel MMU TLB flush:\n\nSet 1 to this bit to flush MMU TLB, auto clear"]
pub type SwSrc1MmuFlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrc1MmuPrefetchEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwSrc1MmuPrefetchEn> for bool {
    #[inline(always)]
    fn from(variant: SwSrc1MmuPrefetchEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC1_MMU_PREFETCH_EN` reader - "]
pub type SwSrc1MmuPrefetchEnR = crate::BitReader<SwSrc1MmuPrefetchEn>;
impl SwSrc1MmuPrefetchEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrc1MmuPrefetchEn {
        match self.bits {
            false => SwSrc1MmuPrefetchEn::B0,
            true => SwSrc1MmuPrefetchEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrc1MmuPrefetchEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrc1MmuPrefetchEn::B1
    }
}
#[doc = "Field `SW_SRC1_MMU_PREFETCH_EN` writer - "]
pub type SwSrc1MmuPrefetchEnW<'a, REG> = crate::BitWriter<'a, REG, SwSrc1MmuPrefetchEn>;
impl<'a, REG> SwSrc1MmuPrefetchEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrc1MmuPrefetchEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrc1MmuPrefetchEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrc1MmuPrefetchDir {
    #[doc = "0: forward"]
    B0 = 0,
    #[doc = "1: backward"]
    B1 = 1,
}
impl From<SwSrc1MmuPrefetchDir> for bool {
    #[inline(always)]
    fn from(variant: SwSrc1MmuPrefetchDir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC1_MMU_PREFETCH_DIR` reader - "]
pub type SwSrc1MmuPrefetchDirR = crate::BitReader<SwSrc1MmuPrefetchDir>;
impl SwSrc1MmuPrefetchDirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrc1MmuPrefetchDir {
        match self.bits {
            false => SwSrc1MmuPrefetchDir::B0,
            true => SwSrc1MmuPrefetchDir::B1,
        }
    }
    #[doc = "forward"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrc1MmuPrefetchDir::B0
    }
    #[doc = "backward"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrc1MmuPrefetchDir::B1
    }
}
#[doc = "Field `SW_SRC1_MMU_PREFETCH_DIR` writer - "]
pub type SwSrc1MmuPrefetchDirW<'a, REG> = crate::BitWriter<'a, REG, SwSrc1MmuPrefetchDir>;
impl<'a, REG> SwSrc1MmuPrefetchDirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "forward"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrc1MmuPrefetchDir::B0)
    }
    #[doc = "backward"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrc1MmuPrefetchDir::B1)
    }
}
#[doc = "RGA DST channel MMU enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDstMmuEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwDstMmuEn> for bool {
    #[inline(always)]
    fn from(variant: SwDstMmuEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DST_MMU_EN` reader - RGA DST channel MMU enable"]
pub type SwDstMmuEnR = crate::BitReader<SwDstMmuEn>;
impl SwDstMmuEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDstMmuEn {
        match self.bits {
            false => SwDstMmuEn::B0,
            true => SwDstMmuEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDstMmuEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDstMmuEn::B1
    }
}
#[doc = "Field `SW_DST_MMU_EN` writer - RGA DST channel MMU enable"]
pub type SwDstMmuEnW<'a, REG> = crate::BitWriter<'a, REG, SwDstMmuEn>;
impl<'a, REG> SwDstMmuEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstMmuEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstMmuEn::B1)
    }
}
#[doc = "Field `SW_DST_MMU_FLUSH` reader - RGA DST channel MMU TLB flush:\n\nSet 1 to this bit to flush MMU TLB, auto clear"]
pub type SwDstMmuFlushR = crate::BitReader;
#[doc = "Field `SW_DST_MMU_FLUSH` writer - RGA DST channel MMU TLB flush:\n\nSet 1 to this bit to flush MMU TLB, auto clear"]
pub type SwDstMmuFlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDstMmuPrefetchEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwDstMmuPrefetchEn> for bool {
    #[inline(always)]
    fn from(variant: SwDstMmuPrefetchEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DST_MMU_PREFETCH_EN` reader - "]
pub type SwDstMmuPrefetchEnR = crate::BitReader<SwDstMmuPrefetchEn>;
impl SwDstMmuPrefetchEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDstMmuPrefetchEn {
        match self.bits {
            false => SwDstMmuPrefetchEn::B0,
            true => SwDstMmuPrefetchEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDstMmuPrefetchEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDstMmuPrefetchEn::B1
    }
}
#[doc = "Field `SW_DST_MMU_PREFETCH_EN` writer - "]
pub type SwDstMmuPrefetchEnW<'a, REG> = crate::BitWriter<'a, REG, SwDstMmuPrefetchEn>;
impl<'a, REG> SwDstMmuPrefetchEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstMmuPrefetchEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstMmuPrefetchEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDstMmuPrefetchDir {
    #[doc = "0: forward"]
    B0 = 0,
    #[doc = "1: backward"]
    B1 = 1,
}
impl From<SwDstMmuPrefetchDir> for bool {
    #[inline(always)]
    fn from(variant: SwDstMmuPrefetchDir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DST_MMU_PREFETCH_DIR` reader - "]
pub type SwDstMmuPrefetchDirR = crate::BitReader<SwDstMmuPrefetchDir>;
impl SwDstMmuPrefetchDirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDstMmuPrefetchDir {
        match self.bits {
            false => SwDstMmuPrefetchDir::B0,
            true => SwDstMmuPrefetchDir::B1,
        }
    }
    #[doc = "forward"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDstMmuPrefetchDir::B0
    }
    #[doc = "backward"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDstMmuPrefetchDir::B1
    }
}
#[doc = "Field `SW_DST_MMU_PREFETCH_DIR` writer - "]
pub type SwDstMmuPrefetchDirW<'a, REG> = crate::BitWriter<'a, REG, SwDstMmuPrefetchDir>;
impl<'a, REG> SwDstMmuPrefetchDirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "forward"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstMmuPrefetchDir::B0)
    }
    #[doc = "backward"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstMmuPrefetchDir::B1)
    }
}
#[doc = "RGA ELSE channel MMU enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwElsMmuEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwElsMmuEn> for bool {
    #[inline(always)]
    fn from(variant: SwElsMmuEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_ELS_MMU_EN` reader - RGA ELSE channel MMU enable"]
pub type SwElsMmuEnR = crate::BitReader<SwElsMmuEn>;
impl SwElsMmuEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwElsMmuEn {
        match self.bits {
            false => SwElsMmuEn::B0,
            true => SwElsMmuEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwElsMmuEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwElsMmuEn::B1
    }
}
#[doc = "Field `SW_ELS_MMU_EN` writer - RGA ELSE channel MMU enable"]
pub type SwElsMmuEnW<'a, REG> = crate::BitWriter<'a, REG, SwElsMmuEn>;
impl<'a, REG> SwElsMmuEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwElsMmuEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwElsMmuEn::B1)
    }
}
#[doc = "Field `SW_ELS_MMU_FLUSH` reader - RGA ELSE channel MMU TLB flush:\n\nSet 1 to this bit to flush MMU TLB, auto clear"]
pub type SwElsMmuFlushR = crate::BitReader;
#[doc = "Field `SW_ELS_MMU_FLUSH` writer - RGA ELSE channel MMU TLB flush:\n\nSet 1 to this bit to flush MMU TLB, auto clear"]
pub type SwElsMmuFlushW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RGA SRC channel MMU enable"]
    #[inline(always)]
    pub fn sw_src_mmu_en(&self) -> SwSrcMmuEnR {
        SwSrcMmuEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RGA SRC channel MMU TLB flush:\n\nSet 1 to this bit to flush MMU TLB, auto clear"]
    #[inline(always)]
    pub fn sw_src_mmu_flush(&self) -> SwSrcMmuFlushR {
        SwSrcMmuFlushR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sw_src_mmu_prefetch_en(&self) -> SwSrcMmuPrefetchEnR {
        SwSrcMmuPrefetchEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sw_src_mmu_prefetch_dir(&self) -> SwSrcMmuPrefetchDirR {
        SwSrcMmuPrefetchDirR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RGA SRC1 channel MMU enable"]
    #[inline(always)]
    pub fn sw_src1_mmu_en(&self) -> SwSrc1MmuEnR {
        SwSrc1MmuEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RGA SRC1 channel MMU TLB flush:\n\nSet 1 to this bit to flush MMU TLB, auto clear"]
    #[inline(always)]
    pub fn sw_src1_mmu_flush(&self) -> SwSrc1MmuFlushR {
        SwSrc1MmuFlushR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sw_src1_mmu_prefetch_en(&self) -> SwSrc1MmuPrefetchEnR {
        SwSrc1MmuPrefetchEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sw_src1_mmu_prefetch_dir(&self) -> SwSrc1MmuPrefetchDirR {
        SwSrc1MmuPrefetchDirR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RGA DST channel MMU enable"]
    #[inline(always)]
    pub fn sw_dst_mmu_en(&self) -> SwDstMmuEnR {
        SwDstMmuEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RGA DST channel MMU TLB flush:\n\nSet 1 to this bit to flush MMU TLB, auto clear"]
    #[inline(always)]
    pub fn sw_dst_mmu_flush(&self) -> SwDstMmuFlushR {
        SwDstMmuFlushR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sw_dst_mmu_prefetch_en(&self) -> SwDstMmuPrefetchEnR {
        SwDstMmuPrefetchEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sw_dst_mmu_prefetch_dir(&self) -> SwDstMmuPrefetchDirR {
        SwDstMmuPrefetchDirR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RGA ELSE channel MMU enable"]
    #[inline(always)]
    pub fn sw_els_mmu_en(&self) -> SwElsMmuEnR {
        SwElsMmuEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RGA ELSE channel MMU TLB flush:\n\nSet 1 to this bit to flush MMU TLB, auto clear"]
    #[inline(always)]
    pub fn sw_els_mmu_flush(&self) -> SwElsMmuFlushR {
        SwElsMmuFlushR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RGA SRC channel MMU enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_mmu_en(&mut self) -> SwSrcMmuEnW<MmuCtrl1Spec> {
        SwSrcMmuEnW::new(self, 0)
    }
    #[doc = "Bit 1 - RGA SRC channel MMU TLB flush:\n\nSet 1 to this bit to flush MMU TLB, auto clear"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_mmu_flush(&mut self) -> SwSrcMmuFlushW<MmuCtrl1Spec> {
        SwSrcMmuFlushW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_mmu_prefetch_en(&mut self) -> SwSrcMmuPrefetchEnW<MmuCtrl1Spec> {
        SwSrcMmuPrefetchEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_mmu_prefetch_dir(&mut self) -> SwSrcMmuPrefetchDirW<MmuCtrl1Spec> {
        SwSrcMmuPrefetchDirW::new(self, 3)
    }
    #[doc = "Bit 4 - RGA SRC1 channel MMU enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src1_mmu_en(&mut self) -> SwSrc1MmuEnW<MmuCtrl1Spec> {
        SwSrc1MmuEnW::new(self, 4)
    }
    #[doc = "Bit 5 - RGA SRC1 channel MMU TLB flush:\n\nSet 1 to this bit to flush MMU TLB, auto clear"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src1_mmu_flush(&mut self) -> SwSrc1MmuFlushW<MmuCtrl1Spec> {
        SwSrc1MmuFlushW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src1_mmu_prefetch_en(&mut self) -> SwSrc1MmuPrefetchEnW<MmuCtrl1Spec> {
        SwSrc1MmuPrefetchEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src1_mmu_prefetch_dir(&mut self) -> SwSrc1MmuPrefetchDirW<MmuCtrl1Spec> {
        SwSrc1MmuPrefetchDirW::new(self, 7)
    }
    #[doc = "Bit 8 - RGA DST channel MMU enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_mmu_en(&mut self) -> SwDstMmuEnW<MmuCtrl1Spec> {
        SwDstMmuEnW::new(self, 8)
    }
    #[doc = "Bit 9 - RGA DST channel MMU TLB flush:\n\nSet 1 to this bit to flush MMU TLB, auto clear"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_mmu_flush(&mut self) -> SwDstMmuFlushW<MmuCtrl1Spec> {
        SwDstMmuFlushW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_mmu_prefetch_en(&mut self) -> SwDstMmuPrefetchEnW<MmuCtrl1Spec> {
        SwDstMmuPrefetchEnW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_mmu_prefetch_dir(&mut self) -> SwDstMmuPrefetchDirW<MmuCtrl1Spec> {
        SwDstMmuPrefetchDirW::new(self, 11)
    }
    #[doc = "Bit 12 - RGA ELSE channel MMU enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_els_mmu_en(&mut self) -> SwElsMmuEnW<MmuCtrl1Spec> {
        SwElsMmuEnW::new(self, 12)
    }
    #[doc = "Bit 13 - RGA ELSE channel MMU TLB flush:\n\nSet 1 to this bit to flush MMU TLB, auto clear"]
    #[inline(always)]
    #[must_use]
    pub fn sw_els_mmu_flush(&mut self) -> SwElsMmuFlushW<MmuCtrl1Spec> {
        SwElsMmuFlushW::new(self, 13)
    }
}
#[doc = "RGA MMU control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmuCtrl1Spec;
impl crate::RegisterSpec for MmuCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmu_ctrl1::R`](R) reader structure"]
impl crate::Readable for MmuCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`mmu_ctrl1::W`](W) writer structure"]
impl crate::Writable for MmuCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMU_CTRL1 to value 0"]
impl crate::Resettable for MmuCtrl1Spec {
    const RESET_VALUE: u32 = 0;
}
