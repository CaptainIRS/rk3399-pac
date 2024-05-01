#[doc = "Register `INTR_EN0` reader"]
pub type R = crate::R<IntrEn0Spec>;
#[doc = "Register `INTR_EN0` writer"]
pub type W = crate::W<IntrEn0Spec>;
#[doc = "Frame start interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrEnFs {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntrEnFs> for bool {
    #[inline(always)]
    fn from(variant: IntrEnFs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTR_EN_FS` reader - Frame start interrupt enable"]
pub type IntrEnFsR = crate::BitReader<IntrEnFs>;
impl IntrEnFsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntrEnFs {
        match self.bits {
            false => IntrEnFs::B0,
            true => IntrEnFs::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntrEnFs::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntrEnFs::B1
    }
}
#[doc = "Field `INTR_EN_FS` writer - Frame start interrupt enable"]
pub type IntrEnFsW<'a, REG> = crate::BitWriter<'a, REG, IntrEnFs>;
impl<'a, REG> IntrEnFsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnFs::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnFs::B1)
    }
}
#[doc = "Frame new start interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrEnFsNew {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntrEnFsNew> for bool {
    #[inline(always)]
    fn from(variant: IntrEnFsNew) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTR_EN_FS_NEW` reader - Frame new start interrupt enable"]
pub type IntrEnFsNewR = crate::BitReader<IntrEnFsNew>;
impl IntrEnFsNewR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntrEnFsNew {
        match self.bits {
            false => IntrEnFsNew::B0,
            true => IntrEnFsNew::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntrEnFsNew::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntrEnFsNew::B1
    }
}
#[doc = "Field `INTR_EN_FS_NEW` writer - Frame new start interrupt enable"]
pub type IntrEnFsNewW<'a, REG> = crate::BitWriter<'a, REG, IntrEnFsNew>;
impl<'a, REG> IntrEnFsNewW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnFsNew::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnFsNew::B1)
    }
}
#[doc = "memory start addr same interruption enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrEnAddrSame {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntrEnAddrSame> for bool {
    #[inline(always)]
    fn from(variant: IntrEnAddrSame) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTR_EN_ADDR_SAME` reader - memory start addr same interruption enable"]
pub type IntrEnAddrSameR = crate::BitReader<IntrEnAddrSame>;
impl IntrEnAddrSameR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntrEnAddrSame {
        match self.bits {
            false => IntrEnAddrSame::B0,
            true => IntrEnAddrSame::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntrEnAddrSame::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntrEnAddrSame::B1
    }
}
#[doc = "Field `INTR_EN_ADDR_SAME` writer - memory start addr same interruption enable"]
pub type IntrEnAddrSameW<'a, REG> = crate::BitWriter<'a, REG, IntrEnAddrSame>;
impl<'a, REG> IntrEnAddrSameW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnAddrSame::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnAddrSame::B1)
    }
}
#[doc = "Line flag 0 Interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrEnLineFlag0 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntrEnLineFlag0> for bool {
    #[inline(always)]
    fn from(variant: IntrEnLineFlag0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTR_EN_LINE_FLAG0` reader - Line flag 0 Interrupt enable"]
pub type IntrEnLineFlag0R = crate::BitReader<IntrEnLineFlag0>;
impl IntrEnLineFlag0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntrEnLineFlag0 {
        match self.bits {
            false => IntrEnLineFlag0::B0,
            true => IntrEnLineFlag0::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntrEnLineFlag0::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntrEnLineFlag0::B1
    }
}
#[doc = "Field `INTR_EN_LINE_FLAG0` writer - Line flag 0 Interrupt enable"]
pub type IntrEnLineFlag0W<'a, REG> = crate::BitWriter<'a, REG, IntrEnLineFlag0>;
impl<'a, REG> IntrEnLineFlag0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnLineFlag0::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnLineFlag0::B1)
    }
}
#[doc = "Line flag 1 Interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrEnLineFlag1 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntrEnLineFlag1> for bool {
    #[inline(always)]
    fn from(variant: IntrEnLineFlag1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTR_EN_LINE_FLAG1` reader - Line flag 1 Interrupt enable"]
pub type IntrEnLineFlag1R = crate::BitReader<IntrEnLineFlag1>;
impl IntrEnLineFlag1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntrEnLineFlag1 {
        match self.bits {
            false => IntrEnLineFlag1::B0,
            true => IntrEnLineFlag1::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntrEnLineFlag1::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntrEnLineFlag1::B1
    }
}
#[doc = "Field `INTR_EN_LINE_FLAG1` writer - Line flag 1 Interrupt enable"]
pub type IntrEnLineFlag1W<'a, REG> = crate::BitWriter<'a, REG, IntrEnLineFlag1>;
impl<'a, REG> IntrEnLineFlag1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnLineFlag1::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnLineFlag1::B1)
    }
}
#[doc = "Bus error Interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrEnBusError {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntrEnBusError> for bool {
    #[inline(always)]
    fn from(variant: IntrEnBusError) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTR_EN_BUS_ERROR` reader - Bus error Interrupt enable"]
pub type IntrEnBusErrorR = crate::BitReader<IntrEnBusError>;
impl IntrEnBusErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntrEnBusError {
        match self.bits {
            false => IntrEnBusError::B0,
            true => IntrEnBusError::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntrEnBusError::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntrEnBusError::B1
    }
}
#[doc = "Field `INTR_EN_BUS_ERROR` writer - Bus error Interrupt enable"]
pub type IntrEnBusErrorW<'a, REG> = crate::BitWriter<'a, REG, IntrEnBusError>;
impl<'a, REG> IntrEnBusErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnBusError::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnBusError::B1)
    }
}
#[doc = "win0 data empty interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrEnWin0Empty {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntrEnWin0Empty> for bool {
    #[inline(always)]
    fn from(variant: IntrEnWin0Empty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTR_EN_WIN0_EMPTY` reader - win0 data empty interrupt enable"]
pub type IntrEnWin0EmptyR = crate::BitReader<IntrEnWin0Empty>;
impl IntrEnWin0EmptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntrEnWin0Empty {
        match self.bits {
            false => IntrEnWin0Empty::B0,
            true => IntrEnWin0Empty::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntrEnWin0Empty::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntrEnWin0Empty::B1
    }
}
#[doc = "Field `INTR_EN_WIN0_EMPTY` writer - win0 data empty interrupt enable"]
pub type IntrEnWin0EmptyW<'a, REG> = crate::BitWriter<'a, REG, IntrEnWin0Empty>;
impl<'a, REG> IntrEnWin0EmptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnWin0Empty::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnWin0Empty::B1)
    }
}
#[doc = "win1 data empty interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrEnWin1Empty {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntrEnWin1Empty> for bool {
    #[inline(always)]
    fn from(variant: IntrEnWin1Empty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTR_EN_WIN1_EMPTY` reader - win1 data empty interrupt enable"]
pub type IntrEnWin1EmptyR = crate::BitReader<IntrEnWin1Empty>;
impl IntrEnWin1EmptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntrEnWin1Empty {
        match self.bits {
            false => IntrEnWin1Empty::B0,
            true => IntrEnWin1Empty::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntrEnWin1Empty::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntrEnWin1Empty::B1
    }
}
#[doc = "Field `INTR_EN_WIN1_EMPTY` writer - win1 data empty interrupt enable"]
pub type IntrEnWin1EmptyW<'a, REG> = crate::BitWriter<'a, REG, IntrEnWin1Empty>;
impl<'a, REG> IntrEnWin1EmptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnWin1Empty::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnWin1Empty::B1)
    }
}
#[doc = "win2 data empty interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrEnWin2Empty {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntrEnWin2Empty> for bool {
    #[inline(always)]
    fn from(variant: IntrEnWin2Empty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTR_EN_WIN2_EMPTY` reader - win2 data empty interrupt enable"]
pub type IntrEnWin2EmptyR = crate::BitReader<IntrEnWin2Empty>;
impl IntrEnWin2EmptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntrEnWin2Empty {
        match self.bits {
            false => IntrEnWin2Empty::B0,
            true => IntrEnWin2Empty::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntrEnWin2Empty::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntrEnWin2Empty::B1
    }
}
#[doc = "Field `INTR_EN_WIN2_EMPTY` writer - win2 data empty interrupt enable"]
pub type IntrEnWin2EmptyW<'a, REG> = crate::BitWriter<'a, REG, IntrEnWin2Empty>;
impl<'a, REG> IntrEnWin2EmptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnWin2Empty::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnWin2Empty::B1)
    }
}
#[doc = "win3 data empty interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrEnWin3Empty {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntrEnWin3Empty> for bool {
    #[inline(always)]
    fn from(variant: IntrEnWin3Empty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTR_EN_WIN3_EMPTY` reader - win3 data empty interrupt enable"]
pub type IntrEnWin3EmptyR = crate::BitReader<IntrEnWin3Empty>;
impl IntrEnWin3EmptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntrEnWin3Empty {
        match self.bits {
            false => IntrEnWin3Empty::B0,
            true => IntrEnWin3Empty::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntrEnWin3Empty::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntrEnWin3Empty::B1
    }
}
#[doc = "Field `INTR_EN_WIN3_EMPTY` writer - win3 data empty interrupt enable"]
pub type IntrEnWin3EmptyW<'a, REG> = crate::BitWriter<'a, REG, IntrEnWin3Empty>;
impl<'a, REG> IntrEnWin3EmptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnWin3Empty::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnWin3Empty::B1)
    }
}
#[doc = "hwc data empty interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrEnHwcEmpty {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntrEnHwcEmpty> for bool {
    #[inline(always)]
    fn from(variant: IntrEnHwcEmpty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTR_EN_HWC_EMPTY` reader - hwc data empty interrupt enable"]
pub type IntrEnHwcEmptyR = crate::BitReader<IntrEnHwcEmpty>;
impl IntrEnHwcEmptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntrEnHwcEmpty {
        match self.bits {
            false => IntrEnHwcEmpty::B0,
            true => IntrEnHwcEmpty::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntrEnHwcEmpty::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntrEnHwcEmpty::B1
    }
}
#[doc = "Field `INTR_EN_HWC_EMPTY` writer - hwc data empty interrupt enable"]
pub type IntrEnHwcEmptyW<'a, REG> = crate::BitWriter<'a, REG, IntrEnHwcEmpty>;
impl<'a, REG> IntrEnHwcEmptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnHwcEmpty::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnHwcEmpty::B1)
    }
}
#[doc = "post buffer empty interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrEnPostBufEmpty {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntrEnPostBufEmpty> for bool {
    #[inline(always)]
    fn from(variant: IntrEnPostBufEmpty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTR_EN_POST_BUF_EMPTY` reader - post buffer empty interrupt enable"]
pub type IntrEnPostBufEmptyR = crate::BitReader<IntrEnPostBufEmpty>;
impl IntrEnPostBufEmptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntrEnPostBufEmpty {
        match self.bits {
            false => IntrEnPostBufEmpty::B0,
            true => IntrEnPostBufEmpty::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntrEnPostBufEmpty::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntrEnPostBufEmpty::B1
    }
}
#[doc = "Field `INTR_EN_POST_BUF_EMPTY` writer - post buffer empty interrupt enable"]
pub type IntrEnPostBufEmptyW<'a, REG> = crate::BitWriter<'a, REG, IntrEnPostBufEmpty>;
impl<'a, REG> IntrEnPostBufEmptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnPostBufEmpty::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnPostBufEmpty::B1)
    }
}
#[doc = "field interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrEnFsField {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntrEnFsField> for bool {
    #[inline(always)]
    fn from(variant: IntrEnFsField) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTR_EN_FS_FIELD` reader - field interrupt enable"]
pub type IntrEnFsFieldR = crate::BitReader<IntrEnFsField>;
impl IntrEnFsFieldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntrEnFsField {
        match self.bits {
            false => IntrEnFsField::B0,
            true => IntrEnFsField::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntrEnFsField::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntrEnFsField::B1
    }
}
#[doc = "Field `INTR_EN_FS_FIELD` writer - field interrupt enable"]
pub type IntrEnFsFieldW<'a, REG> = crate::BitWriter<'a, REG, IntrEnFsField>;
impl<'a, REG> IntrEnFsFieldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnFsField::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnFsField::B1)
    }
}
#[doc = "display hold valid interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrEnDspHoldValid {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntrEnDspHoldValid> for bool {
    #[inline(always)]
    fn from(variant: IntrEnDspHoldValid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTR_EN_DSP_HOLD_VALID` reader - display hold valid interrupt enable"]
pub type IntrEnDspHoldValidR = crate::BitReader<IntrEnDspHoldValid>;
impl IntrEnDspHoldValidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntrEnDspHoldValid {
        match self.bits {
            false => IntrEnDspHoldValid::B0,
            true => IntrEnDspHoldValid::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntrEnDspHoldValid::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntrEnDspHoldValid::B1
    }
}
#[doc = "Field `INTR_EN_DSP_HOLD_VALID` writer - display hold valid interrupt enable"]
pub type IntrEnDspHoldValidW<'a, REG> = crate::BitWriter<'a, REG, IntrEnDspHoldValid>;
impl<'a, REG> IntrEnDspHoldValidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnDspHoldValid::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnDspHoldValid::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrEnMmu {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntrEnMmu> for bool {
    #[inline(always)]
    fn from(variant: IntrEnMmu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTR_EN_MMU` reader - "]
pub type IntrEnMmuR = crate::BitReader<IntrEnMmu>;
impl IntrEnMmuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntrEnMmu {
        match self.bits {
            false => IntrEnMmu::B0,
            true => IntrEnMmu::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntrEnMmu::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntrEnMmu::B1
    }
}
#[doc = "Field `INTR_EN_MMU` writer - "]
pub type IntrEnMmuW<'a, REG> = crate::BitWriter<'a, REG, IntrEnMmu>;
impl<'a, REG> IntrEnMmuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnMmu::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnMmu::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrEnDmaFinish {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntrEnDmaFinish> for bool {
    #[inline(always)]
    fn from(variant: IntrEnDmaFinish) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTR_EN_DMA_FINISH` reader - "]
pub type IntrEnDmaFinishR = crate::BitReader<IntrEnDmaFinish>;
impl IntrEnDmaFinishR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntrEnDmaFinish {
        match self.bits {
            false => IntrEnDmaFinish::B0,
            true => IntrEnDmaFinish::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntrEnDmaFinish::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntrEnDmaFinish::B1
    }
}
#[doc = "Field `INTR_EN_DMA_FINISH` writer - "]
pub type IntrEnDmaFinishW<'a, REG> = crate::BitWriter<'a, REG, IntrEnDmaFinish>;
impl<'a, REG> IntrEnDmaFinishW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnDmaFinish::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntrEnDmaFinish::B1)
    }
}
#[doc = "Field `WRITE_MASK` reader - When every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_MASK` writer - When every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Frame start interrupt enable"]
    #[inline(always)]
    pub fn intr_en_fs(&self) -> IntrEnFsR {
        IntrEnFsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame new start interrupt enable"]
    #[inline(always)]
    pub fn intr_en_fs_new(&self) -> IntrEnFsNewR {
        IntrEnFsNewR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - memory start addr same interruption enable"]
    #[inline(always)]
    pub fn intr_en_addr_same(&self) -> IntrEnAddrSameR {
        IntrEnAddrSameR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Line flag 0 Interrupt enable"]
    #[inline(always)]
    pub fn intr_en_line_flag0(&self) -> IntrEnLineFlag0R {
        IntrEnLineFlag0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line flag 1 Interrupt enable"]
    #[inline(always)]
    pub fn intr_en_line_flag1(&self) -> IntrEnLineFlag1R {
        IntrEnLineFlag1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus error Interrupt enable"]
    #[inline(always)]
    pub fn intr_en_bus_error(&self) -> IntrEnBusErrorR {
        IntrEnBusErrorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - win0 data empty interrupt enable"]
    #[inline(always)]
    pub fn intr_en_win0_empty(&self) -> IntrEnWin0EmptyR {
        IntrEnWin0EmptyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - win1 data empty interrupt enable"]
    #[inline(always)]
    pub fn intr_en_win1_empty(&self) -> IntrEnWin1EmptyR {
        IntrEnWin1EmptyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - win2 data empty interrupt enable"]
    #[inline(always)]
    pub fn intr_en_win2_empty(&self) -> IntrEnWin2EmptyR {
        IntrEnWin2EmptyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - win3 data empty interrupt enable"]
    #[inline(always)]
    pub fn intr_en_win3_empty(&self) -> IntrEnWin3EmptyR {
        IntrEnWin3EmptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - hwc data empty interrupt enable"]
    #[inline(always)]
    pub fn intr_en_hwc_empty(&self) -> IntrEnHwcEmptyR {
        IntrEnHwcEmptyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - post buffer empty interrupt enable"]
    #[inline(always)]
    pub fn intr_en_post_buf_empty(&self) -> IntrEnPostBufEmptyR {
        IntrEnPostBufEmptyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - field interrupt enable"]
    #[inline(always)]
    pub fn intr_en_fs_field(&self) -> IntrEnFsFieldR {
        IntrEnFsFieldR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - display hold valid interrupt enable"]
    #[inline(always)]
    pub fn intr_en_dsp_hold_valid(&self) -> IntrEnDspHoldValidR {
        IntrEnDspHoldValidR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn intr_en_mmu(&self) -> IntrEnMmuR {
        IntrEnMmuR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn intr_en_dma_finish(&self) -> IntrEnDmaFinishR {
        IntrEnDmaFinishR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - When every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    pub fn write_mask(&self) -> WriteMaskR {
        WriteMaskR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Frame start interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn intr_en_fs(&mut self) -> IntrEnFsW<IntrEn0Spec> {
        IntrEnFsW::new(self, 0)
    }
    #[doc = "Bit 1 - Frame new start interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn intr_en_fs_new(&mut self) -> IntrEnFsNewW<IntrEn0Spec> {
        IntrEnFsNewW::new(self, 1)
    }
    #[doc = "Bit 2 - memory start addr same interruption enable"]
    #[inline(always)]
    #[must_use]
    pub fn intr_en_addr_same(&mut self) -> IntrEnAddrSameW<IntrEn0Spec> {
        IntrEnAddrSameW::new(self, 2)
    }
    #[doc = "Bit 3 - Line flag 0 Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn intr_en_line_flag0(&mut self) -> IntrEnLineFlag0W<IntrEn0Spec> {
        IntrEnLineFlag0W::new(self, 3)
    }
    #[doc = "Bit 4 - Line flag 1 Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn intr_en_line_flag1(&mut self) -> IntrEnLineFlag1W<IntrEn0Spec> {
        IntrEnLineFlag1W::new(self, 4)
    }
    #[doc = "Bit 5 - Bus error Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn intr_en_bus_error(&mut self) -> IntrEnBusErrorW<IntrEn0Spec> {
        IntrEnBusErrorW::new(self, 5)
    }
    #[doc = "Bit 6 - win0 data empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn intr_en_win0_empty(&mut self) -> IntrEnWin0EmptyW<IntrEn0Spec> {
        IntrEnWin0EmptyW::new(self, 6)
    }
    #[doc = "Bit 7 - win1 data empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn intr_en_win1_empty(&mut self) -> IntrEnWin1EmptyW<IntrEn0Spec> {
        IntrEnWin1EmptyW::new(self, 7)
    }
    #[doc = "Bit 8 - win2 data empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn intr_en_win2_empty(&mut self) -> IntrEnWin2EmptyW<IntrEn0Spec> {
        IntrEnWin2EmptyW::new(self, 8)
    }
    #[doc = "Bit 9 - win3 data empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn intr_en_win3_empty(&mut self) -> IntrEnWin3EmptyW<IntrEn0Spec> {
        IntrEnWin3EmptyW::new(self, 9)
    }
    #[doc = "Bit 10 - hwc data empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn intr_en_hwc_empty(&mut self) -> IntrEnHwcEmptyW<IntrEn0Spec> {
        IntrEnHwcEmptyW::new(self, 10)
    }
    #[doc = "Bit 11 - post buffer empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn intr_en_post_buf_empty(&mut self) -> IntrEnPostBufEmptyW<IntrEn0Spec> {
        IntrEnPostBufEmptyW::new(self, 11)
    }
    #[doc = "Bit 12 - field interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn intr_en_fs_field(&mut self) -> IntrEnFsFieldW<IntrEn0Spec> {
        IntrEnFsFieldW::new(self, 12)
    }
    #[doc = "Bit 13 - display hold valid interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn intr_en_dsp_hold_valid(&mut self) -> IntrEnDspHoldValidW<IntrEn0Spec> {
        IntrEnDspHoldValidW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn intr_en_mmu(&mut self) -> IntrEnMmuW<IntrEn0Spec> {
        IntrEnMmuW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn intr_en_dma_finish(&mut self) -> IntrEnDmaFinishW<IntrEn0Spec> {
        IntrEnDmaFinishW::new(self, 15)
    }
    #[doc = "Bits 16:31 - When every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<IntrEn0Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_en0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_en0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrEn0Spec;
impl crate::RegisterSpec for IntrEn0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_en0::R`](R) reader structure"]
impl crate::Readable for IntrEn0Spec {}
#[doc = "`write(|w| ..)` method takes [`intr_en0::W`](W) writer structure"]
impl crate::Writable for IntrEn0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_EN0 to value 0"]
impl crate::Resettable for IntrEn0Spec {
    const RESET_VALUE: u32 = 0;
}
