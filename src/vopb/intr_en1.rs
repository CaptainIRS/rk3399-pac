#[doc = "Register `INTR_EN1` reader"]
pub type R = crate::R<IntrEn1Spec>;
#[doc = "Register `INTR_EN1` writer"]
pub type W = crate::W<IntrEn1Spec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEnFbcd0 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntEnFbcd0> for bool {
    #[inline(always)]
    fn from(variant: IntEnFbcd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EN_FBCD0` reader - "]
pub type IntEnFbcd0R = crate::BitReader<IntEnFbcd0>;
impl IntEnFbcd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEnFbcd0 {
        match self.bits {
            false => IntEnFbcd0::B0,
            true => IntEnFbcd0::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntEnFbcd0::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntEnFbcd0::B1
    }
}
#[doc = "Field `INT_EN_FBCD0` writer - "]
pub type IntEnFbcd0W<'a, REG> = crate::BitWriter<'a, REG, IntEnFbcd0>;
impl<'a, REG> IntEnFbcd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnFbcd0::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnFbcd0::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEnFbcd1 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntEnFbcd1> for bool {
    #[inline(always)]
    fn from(variant: IntEnFbcd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EN_FBCD1` reader - "]
pub type IntEnFbcd1R = crate::BitReader<IntEnFbcd1>;
impl IntEnFbcd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEnFbcd1 {
        match self.bits {
            false => IntEnFbcd1::B0,
            true => IntEnFbcd1::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntEnFbcd1::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntEnFbcd1::B1
    }
}
#[doc = "Field `INT_EN_FBCD1` writer - "]
pub type IntEnFbcd1W<'a, REG> = crate::BitWriter<'a, REG, IntEnFbcd1>;
impl<'a, REG> IntEnFbcd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnFbcd1::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnFbcd1::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEnFbcd2 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntEnFbcd2> for bool {
    #[inline(always)]
    fn from(variant: IntEnFbcd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EN_FBCD2` reader - "]
pub type IntEnFbcd2R = crate::BitReader<IntEnFbcd2>;
impl IntEnFbcd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEnFbcd2 {
        match self.bits {
            false => IntEnFbcd2::B0,
            true => IntEnFbcd2::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntEnFbcd2::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntEnFbcd2::B1
    }
}
#[doc = "Field `INT_EN_FBCD2` writer - "]
pub type IntEnFbcd2W<'a, REG> = crate::BitWriter<'a, REG, IntEnFbcd2>;
impl<'a, REG> IntEnFbcd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnFbcd2::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnFbcd2::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEnFbcd3 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntEnFbcd3> for bool {
    #[inline(always)]
    fn from(variant: IntEnFbcd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EN_FBCD3` reader - "]
pub type IntEnFbcd3R = crate::BitReader<IntEnFbcd3>;
impl IntEnFbcd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEnFbcd3 {
        match self.bits {
            false => IntEnFbcd3::B0,
            true => IntEnFbcd3::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntEnFbcd3::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntEnFbcd3::B1
    }
}
#[doc = "Field `INT_EN_FBCD3` writer - "]
pub type IntEnFbcd3W<'a, REG> = crate::BitWriter<'a, REG, IntEnFbcd3>;
impl<'a, REG> IntEnFbcd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnFbcd3::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnFbcd3::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEnAfbcd0HregDecResp {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntEnAfbcd0HregDecResp> for bool {
    #[inline(always)]
    fn from(variant: IntEnAfbcd0HregDecResp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EN_AFBCD0_HREG_DEC_RESP` reader - "]
pub type IntEnAfbcd0HregDecRespR = crate::BitReader<IntEnAfbcd0HregDecResp>;
impl IntEnAfbcd0HregDecRespR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEnAfbcd0HregDecResp {
        match self.bits {
            false => IntEnAfbcd0HregDecResp::B0,
            true => IntEnAfbcd0HregDecResp::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntEnAfbcd0HregDecResp::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntEnAfbcd0HregDecResp::B1
    }
}
#[doc = "Field `INT_EN_AFBCD0_HREG_DEC_RESP` writer - "]
pub type IntEnAfbcd0HregDecRespW<'a, REG> = crate::BitWriter<'a, REG, IntEnAfbcd0HregDecResp>;
impl<'a, REG> IntEnAfbcd0HregDecRespW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnAfbcd0HregDecResp::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnAfbcd0HregDecResp::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEnAfbcd0HregAxiRresp {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntEnAfbcd0HregAxiRresp> for bool {
    #[inline(always)]
    fn from(variant: IntEnAfbcd0HregAxiRresp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EN_AFBCD0_HREG_AXI_RRESP` reader - "]
pub type IntEnAfbcd0HregAxiRrespR = crate::BitReader<IntEnAfbcd0HregAxiRresp>;
impl IntEnAfbcd0HregAxiRrespR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEnAfbcd0HregAxiRresp {
        match self.bits {
            false => IntEnAfbcd0HregAxiRresp::B0,
            true => IntEnAfbcd0HregAxiRresp::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntEnAfbcd0HregAxiRresp::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntEnAfbcd0HregAxiRresp::B1
    }
}
#[doc = "Field `INT_EN_AFBCD0_HREG_AXI_RRESP` writer - "]
pub type IntEnAfbcd0HregAxiRrespW<'a, REG> = crate::BitWriter<'a, REG, IntEnAfbcd0HregAxiRresp>;
impl<'a, REG> IntEnAfbcd0HregAxiRrespW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnAfbcd0HregAxiRresp::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnAfbcd0HregAxiRresp::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEnAfbcd1HregDecResp {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntEnAfbcd1HregDecResp> for bool {
    #[inline(always)]
    fn from(variant: IntEnAfbcd1HregDecResp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EN_AFBCD1_HREG_DEC_RESP` reader - "]
pub type IntEnAfbcd1HregDecRespR = crate::BitReader<IntEnAfbcd1HregDecResp>;
impl IntEnAfbcd1HregDecRespR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEnAfbcd1HregDecResp {
        match self.bits {
            false => IntEnAfbcd1HregDecResp::B0,
            true => IntEnAfbcd1HregDecResp::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntEnAfbcd1HregDecResp::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntEnAfbcd1HregDecResp::B1
    }
}
#[doc = "Field `INT_EN_AFBCD1_HREG_DEC_RESP` writer - "]
pub type IntEnAfbcd1HregDecRespW<'a, REG> = crate::BitWriter<'a, REG, IntEnAfbcd1HregDecResp>;
impl<'a, REG> IntEnAfbcd1HregDecRespW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnAfbcd1HregDecResp::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnAfbcd1HregDecResp::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEnAfbcd1HregAxiRresp {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntEnAfbcd1HregAxiRresp> for bool {
    #[inline(always)]
    fn from(variant: IntEnAfbcd1HregAxiRresp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EN_AFBCD1_HREG_AXI_RRESP` reader - "]
pub type IntEnAfbcd1HregAxiRrespR = crate::BitReader<IntEnAfbcd1HregAxiRresp>;
impl IntEnAfbcd1HregAxiRrespR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEnAfbcd1HregAxiRresp {
        match self.bits {
            false => IntEnAfbcd1HregAxiRresp::B0,
            true => IntEnAfbcd1HregAxiRresp::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntEnAfbcd1HregAxiRresp::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntEnAfbcd1HregAxiRresp::B1
    }
}
#[doc = "Field `INT_EN_AFBCD1_HREG_AXI_RRESP` writer - "]
pub type IntEnAfbcd1HregAxiRrespW<'a, REG> = crate::BitWriter<'a, REG, IntEnAfbcd1HregAxiRresp>;
impl<'a, REG> IntEnAfbcd1HregAxiRrespW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnAfbcd1HregAxiRresp::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnAfbcd1HregAxiRresp::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEnAfbcd2HregDecResp {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntEnAfbcd2HregDecResp> for bool {
    #[inline(always)]
    fn from(variant: IntEnAfbcd2HregDecResp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EN_AFBCD2_HREG_DEC_RESP` reader - "]
pub type IntEnAfbcd2HregDecRespR = crate::BitReader<IntEnAfbcd2HregDecResp>;
impl IntEnAfbcd2HregDecRespR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEnAfbcd2HregDecResp {
        match self.bits {
            false => IntEnAfbcd2HregDecResp::B0,
            true => IntEnAfbcd2HregDecResp::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntEnAfbcd2HregDecResp::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntEnAfbcd2HregDecResp::B1
    }
}
#[doc = "Field `INT_EN_AFBCD2_HREG_DEC_RESP` writer - "]
pub type IntEnAfbcd2HregDecRespW<'a, REG> = crate::BitWriter<'a, REG, IntEnAfbcd2HregDecResp>;
impl<'a, REG> IntEnAfbcd2HregDecRespW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnAfbcd2HregDecResp::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnAfbcd2HregDecResp::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEnAfbcd2HregAxiRresp {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntEnAfbcd2HregAxiRresp> for bool {
    #[inline(always)]
    fn from(variant: IntEnAfbcd2HregAxiRresp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EN_AFBCD2_HREG_AXI_RRESP` reader - "]
pub type IntEnAfbcd2HregAxiRrespR = crate::BitReader<IntEnAfbcd2HregAxiRresp>;
impl IntEnAfbcd2HregAxiRrespR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEnAfbcd2HregAxiRresp {
        match self.bits {
            false => IntEnAfbcd2HregAxiRresp::B0,
            true => IntEnAfbcd2HregAxiRresp::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntEnAfbcd2HregAxiRresp::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntEnAfbcd2HregAxiRresp::B1
    }
}
#[doc = "Field `INT_EN_AFBCD2_HREG_AXI_RRESP` writer - "]
pub type IntEnAfbcd2HregAxiRrespW<'a, REG> = crate::BitWriter<'a, REG, IntEnAfbcd2HregAxiRresp>;
impl<'a, REG> IntEnAfbcd2HregAxiRrespW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnAfbcd2HregAxiRresp::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnAfbcd2HregAxiRresp::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEnAfbcd3HregDecResp {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntEnAfbcd3HregDecResp> for bool {
    #[inline(always)]
    fn from(variant: IntEnAfbcd3HregDecResp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EN_AFBCD3_HREG_DEC_RESP` reader - "]
pub type IntEnAfbcd3HregDecRespR = crate::BitReader<IntEnAfbcd3HregDecResp>;
impl IntEnAfbcd3HregDecRespR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEnAfbcd3HregDecResp {
        match self.bits {
            false => IntEnAfbcd3HregDecResp::B0,
            true => IntEnAfbcd3HregDecResp::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntEnAfbcd3HregDecResp::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntEnAfbcd3HregDecResp::B1
    }
}
#[doc = "Field `INT_EN_AFBCD3_HREG_DEC_RESP` writer - "]
pub type IntEnAfbcd3HregDecRespW<'a, REG> = crate::BitWriter<'a, REG, IntEnAfbcd3HregDecResp>;
impl<'a, REG> IntEnAfbcd3HregDecRespW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnAfbcd3HregDecResp::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnAfbcd3HregDecResp::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEnAfbcd3HregAxiRresp {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntEnAfbcd3HregAxiRresp> for bool {
    #[inline(always)]
    fn from(variant: IntEnAfbcd3HregAxiRresp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EN_AFBCD3_HREG_AXI_RRESP` reader - "]
pub type IntEnAfbcd3HregAxiRrespR = crate::BitReader<IntEnAfbcd3HregAxiRresp>;
impl IntEnAfbcd3HregAxiRrespR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEnAfbcd3HregAxiRresp {
        match self.bits {
            false => IntEnAfbcd3HregAxiRresp::B0,
            true => IntEnAfbcd3HregAxiRresp::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntEnAfbcd3HregAxiRresp::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntEnAfbcd3HregAxiRresp::B1
    }
}
#[doc = "Field `INT_EN_AFBCD3_HREG_AXI_RRESP` writer - "]
pub type IntEnAfbcd3HregAxiRrespW<'a, REG> = crate::BitWriter<'a, REG, IntEnAfbcd3HregAxiRresp>;
impl<'a, REG> IntEnAfbcd3HregAxiRrespW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnAfbcd3HregAxiRresp::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnAfbcd3HregAxiRresp::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEnWbYrgbFifoFull {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntEnWbYrgbFifoFull> for bool {
    #[inline(always)]
    fn from(variant: IntEnWbYrgbFifoFull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EN_WB_YRGB_FIFO_FULL` reader - "]
pub type IntEnWbYrgbFifoFullR = crate::BitReader<IntEnWbYrgbFifoFull>;
impl IntEnWbYrgbFifoFullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEnWbYrgbFifoFull {
        match self.bits {
            false => IntEnWbYrgbFifoFull::B0,
            true => IntEnWbYrgbFifoFull::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntEnWbYrgbFifoFull::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntEnWbYrgbFifoFull::B1
    }
}
#[doc = "Field `INT_EN_WB_YRGB_FIFO_FULL` writer - "]
pub type IntEnWbYrgbFifoFullW<'a, REG> = crate::BitWriter<'a, REG, IntEnWbYrgbFifoFull>;
impl<'a, REG> IntEnWbYrgbFifoFullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnWbYrgbFifoFull::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnWbYrgbFifoFull::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEnWbUvFifoFull {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntEnWbUvFifoFull> for bool {
    #[inline(always)]
    fn from(variant: IntEnWbUvFifoFull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EN_WB_UV_FIFO_FULL` reader - "]
pub type IntEnWbUvFifoFullR = crate::BitReader<IntEnWbUvFifoFull>;
impl IntEnWbUvFifoFullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEnWbUvFifoFull {
        match self.bits {
            false => IntEnWbUvFifoFull::B0,
            true => IntEnWbUvFifoFull::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntEnWbUvFifoFull::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntEnWbUvFifoFull::B1
    }
}
#[doc = "Field `INT_EN_WB_UV_FIFO_FULL` writer - "]
pub type IntEnWbUvFifoFullW<'a, REG> = crate::BitWriter<'a, REG, IntEnWbUvFifoFull>;
impl<'a, REG> IntEnWbUvFifoFullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnWbUvFifoFull::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnWbUvFifoFull::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEnWbFinish {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntEnWbFinish> for bool {
    #[inline(always)]
    fn from(variant: IntEnWbFinish) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EN_WB_FINISH` reader - "]
pub type IntEnWbFinishR = crate::BitReader<IntEnWbFinish>;
impl IntEnWbFinishR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEnWbFinish {
        match self.bits {
            false => IntEnWbFinish::B0,
            true => IntEnWbFinish::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntEnWbFinish::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntEnWbFinish::B1
    }
}
#[doc = "Field `INT_EN_WB_FINISH` writer - "]
pub type IntEnWbFinishW<'a, REG> = crate::BitWriter<'a, REG, IntEnWbFinish>;
impl<'a, REG> IntEnWbFinishW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnWbFinish::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnWbFinish::B1)
    }
}
#[doc = "Field `INT_EN_VFP` reader - int_en_vfp"]
pub type IntEnVfpR = crate::BitReader;
#[doc = "Field `INT_EN_VFP` writer - int_en_vfp"]
pub type IntEnVfpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` reader - write_mask"]
pub type WriteMaskR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_MASK` writer - write_mask"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn int_en_fbcd0(&self) -> IntEnFbcd0R {
        IntEnFbcd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn int_en_fbcd1(&self) -> IntEnFbcd1R {
        IntEnFbcd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn int_en_fbcd2(&self) -> IntEnFbcd2R {
        IntEnFbcd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn int_en_fbcd3(&self) -> IntEnFbcd3R {
        IntEnFbcd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn int_en_afbcd0_hreg_dec_resp(&self) -> IntEnAfbcd0HregDecRespR {
        IntEnAfbcd0HregDecRespR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn int_en_afbcd0_hreg_axi_rresp(&self) -> IntEnAfbcd0HregAxiRrespR {
        IntEnAfbcd0HregAxiRrespR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn int_en_afbcd1_hreg_dec_resp(&self) -> IntEnAfbcd1HregDecRespR {
        IntEnAfbcd1HregDecRespR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn int_en_afbcd1_hreg_axi_rresp(&self) -> IntEnAfbcd1HregAxiRrespR {
        IntEnAfbcd1HregAxiRrespR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn int_en_afbcd2_hreg_dec_resp(&self) -> IntEnAfbcd2HregDecRespR {
        IntEnAfbcd2HregDecRespR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn int_en_afbcd2_hreg_axi_rresp(&self) -> IntEnAfbcd2HregAxiRrespR {
        IntEnAfbcd2HregAxiRrespR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn int_en_afbcd3_hreg_dec_resp(&self) -> IntEnAfbcd3HregDecRespR {
        IntEnAfbcd3HregDecRespR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn int_en_afbcd3_hreg_axi_rresp(&self) -> IntEnAfbcd3HregAxiRrespR {
        IntEnAfbcd3HregAxiRrespR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn int_en_wb_yrgb_fifo_full(&self) -> IntEnWbYrgbFifoFullR {
        IntEnWbYrgbFifoFullR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn int_en_wb_uv_fifo_full(&self) -> IntEnWbUvFifoFullR {
        IntEnWbUvFifoFullR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn int_en_wb_finish(&self) -> IntEnWbFinishR {
        IntEnWbFinishR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - int_en_vfp"]
    #[inline(always)]
    pub fn int_en_vfp(&self) -> IntEnVfpR {
        IntEnVfpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - write_mask"]
    #[inline(always)]
    pub fn write_mask(&self) -> WriteMaskR {
        WriteMaskR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn int_en_fbcd0(&mut self) -> IntEnFbcd0W<IntrEn1Spec> {
        IntEnFbcd0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn int_en_fbcd1(&mut self) -> IntEnFbcd1W<IntrEn1Spec> {
        IntEnFbcd1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn int_en_fbcd2(&mut self) -> IntEnFbcd2W<IntrEn1Spec> {
        IntEnFbcd2W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn int_en_fbcd3(&mut self) -> IntEnFbcd3W<IntrEn1Spec> {
        IntEnFbcd3W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn int_en_afbcd0_hreg_dec_resp(&mut self) -> IntEnAfbcd0HregDecRespW<IntrEn1Spec> {
        IntEnAfbcd0HregDecRespW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn int_en_afbcd0_hreg_axi_rresp(&mut self) -> IntEnAfbcd0HregAxiRrespW<IntrEn1Spec> {
        IntEnAfbcd0HregAxiRrespW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn int_en_afbcd1_hreg_dec_resp(&mut self) -> IntEnAfbcd1HregDecRespW<IntrEn1Spec> {
        IntEnAfbcd1HregDecRespW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn int_en_afbcd1_hreg_axi_rresp(&mut self) -> IntEnAfbcd1HregAxiRrespW<IntrEn1Spec> {
        IntEnAfbcd1HregAxiRrespW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn int_en_afbcd2_hreg_dec_resp(&mut self) -> IntEnAfbcd2HregDecRespW<IntrEn1Spec> {
        IntEnAfbcd2HregDecRespW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn int_en_afbcd2_hreg_axi_rresp(&mut self) -> IntEnAfbcd2HregAxiRrespW<IntrEn1Spec> {
        IntEnAfbcd2HregAxiRrespW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn int_en_afbcd3_hreg_dec_resp(&mut self) -> IntEnAfbcd3HregDecRespW<IntrEn1Spec> {
        IntEnAfbcd3HregDecRespW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn int_en_afbcd3_hreg_axi_rresp(&mut self) -> IntEnAfbcd3HregAxiRrespW<IntrEn1Spec> {
        IntEnAfbcd3HregAxiRrespW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn int_en_wb_yrgb_fifo_full(&mut self) -> IntEnWbYrgbFifoFullW<IntrEn1Spec> {
        IntEnWbYrgbFifoFullW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn int_en_wb_uv_fifo_full(&mut self) -> IntEnWbUvFifoFullW<IntrEn1Spec> {
        IntEnWbUvFifoFullW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn int_en_wb_finish(&mut self) -> IntEnWbFinishW<IntrEn1Spec> {
        IntEnWbFinishW::new(self, 14)
    }
    #[doc = "Bit 15 - int_en_vfp"]
    #[inline(always)]
    #[must_use]
    pub fn int_en_vfp(&mut self) -> IntEnVfpW<IntrEn1Spec> {
        IntEnVfpW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write_mask"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<IntrEn1Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_en1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_en1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrEn1Spec;
impl crate::RegisterSpec for IntrEn1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_en1::R`](R) reader structure"]
impl crate::Readable for IntrEn1Spec {}
#[doc = "`write(|w| ..)` method takes [`intr_en1::W`](W) writer structure"]
impl crate::Writable for IntrEn1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_EN1 to value 0"]
impl crate::Resettable for IntrEn1Spec {
    const RESET_VALUE: u32 = 0;
}
