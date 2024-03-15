#[doc = "Register `CIC_CTRL1` reader"]
pub type R = crate::R<CicCtrl1Spec>;
#[doc = "Register `CIC_CTRL1` writer"]
pub type W = crate::W<CicCtrl1Spec>;
#[doc = "Channel 0 standby mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StdbyEnCh0 {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<StdbyEnCh0> for bool {
    #[inline(always)]
    fn from(variant: StdbyEnCh0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STDBY_EN_CH0` reader - Channel 0 standby mode enable"]
pub type StdbyEnCh0R = crate::BitReader<StdbyEnCh0>;
impl StdbyEnCh0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StdbyEnCh0 {
        match self.bits {
            false => StdbyEnCh0::B0,
            true => StdbyEnCh0::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == StdbyEnCh0::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == StdbyEnCh0::B1
    }
}
#[doc = "Field `STDBY_EN_CH0` writer - Channel 0 standby mode enable"]
pub type StdbyEnCh0W<'a, REG> = crate::BitWriter<'a, REG, StdbyEnCh0>;
impl<'a, REG> StdbyEnCh0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(StdbyEnCh0::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(StdbyEnCh0::B1)
    }
}
#[doc = "Channel 1 standby mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StdbyEnCh1 {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<StdbyEnCh1> for bool {
    #[inline(always)]
    fn from(variant: StdbyEnCh1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STDBY_EN_CH1` reader - Channel 1 standby mode enable"]
pub type StdbyEnCh1R = crate::BitReader<StdbyEnCh1>;
impl StdbyEnCh1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StdbyEnCh1 {
        match self.bits {
            false => StdbyEnCh1::B0,
            true => StdbyEnCh1::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == StdbyEnCh1::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == StdbyEnCh1::B1
    }
}
#[doc = "Field `STDBY_EN_CH1` writer - Channel 1 standby mode enable"]
pub type StdbyEnCh1W<'a, REG> = crate::BitWriter<'a, REG, StdbyEnCh1>;
impl<'a, REG> StdbyEnCh1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(StdbyEnCh1::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(StdbyEnCh1::B1)
    }
}
#[doc = "Channel 0 memory clock gating in standby mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StdbyMemcgCh0 {
    #[doc = "0: memory is clock gated when in standby mode"]
    B0 = 0,
    #[doc = "1: memory is clock gated when in standby mode"]
    B1 = 1,
}
impl From<StdbyMemcgCh0> for bool {
    #[inline(always)]
    fn from(variant: StdbyMemcgCh0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STDBY_MEMCG_CH0` reader - Channel 0 memory clock gating in standby mode"]
pub type StdbyMemcgCh0R = crate::BitReader<StdbyMemcgCh0>;
impl StdbyMemcgCh0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StdbyMemcgCh0 {
        match self.bits {
            false => StdbyMemcgCh0::B0,
            true => StdbyMemcgCh0::B1,
        }
    }
    #[doc = "memory is clock gated when in standby mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == StdbyMemcgCh0::B0
    }
    #[doc = "memory is clock gated when in standby mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == StdbyMemcgCh0::B1
    }
}
#[doc = "Field `STDBY_MEMCG_CH0` writer - Channel 0 memory clock gating in standby mode"]
pub type StdbyMemcgCh0W<'a, REG> = crate::BitWriter<'a, REG, StdbyMemcgCh0>;
impl<'a, REG> StdbyMemcgCh0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "memory is clock gated when in standby mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(StdbyMemcgCh0::B0)
    }
    #[doc = "memory is clock gated when in standby mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(StdbyMemcgCh0::B1)
    }
}
#[doc = "Channel 1 memory clock gating in standby mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StdbyMemcgCh1 {
    #[doc = "0: memory is clock gated when in standby mode"]
    B0 = 0,
    #[doc = "1: memory is clock gated when in standby mode"]
    B1 = 1,
}
impl From<StdbyMemcgCh1> for bool {
    #[inline(always)]
    fn from(variant: StdbyMemcgCh1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STDBY_MEMCG_CH1` reader - Channel 1 memory clock gating in standby mode"]
pub type StdbyMemcgCh1R = crate::BitReader<StdbyMemcgCh1>;
impl StdbyMemcgCh1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StdbyMemcgCh1 {
        match self.bits {
            false => StdbyMemcgCh1::B0,
            true => StdbyMemcgCh1::B1,
        }
    }
    #[doc = "memory is clock gated when in standby mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == StdbyMemcgCh1::B0
    }
    #[doc = "memory is clock gated when in standby mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == StdbyMemcgCh1::B1
    }
}
#[doc = "Field `STDBY_MEMCG_CH1` writer - Channel 1 memory clock gating in standby mode"]
pub type StdbyMemcgCh1W<'a, REG> = crate::BitWriter<'a, REG, StdbyMemcgCh1>;
impl<'a, REG> StdbyMemcgCh1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "memory is clock gated when in standby mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(StdbyMemcgCh1::B0)
    }
    #[doc = "memory is clock gated when in standby mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(StdbyMemcgCh1::B1)
    }
}
#[doc = "Channel 0 LP command priority in standby mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StdbyCmdPrioCh0 {
    #[doc = "0: issue priority request when enter standby mode"]
    B0 = 0,
    #[doc = "1: issue priority request when enter standby mode"]
    B1 = 1,
}
impl From<StdbyCmdPrioCh0> for bool {
    #[inline(always)]
    fn from(variant: StdbyCmdPrioCh0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STDBY_CMD_PRIO_CH0` reader - Channel 0 LP command priority in standby mode"]
pub type StdbyCmdPrioCh0R = crate::BitReader<StdbyCmdPrioCh0>;
impl StdbyCmdPrioCh0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StdbyCmdPrioCh0 {
        match self.bits {
            false => StdbyCmdPrioCh0::B0,
            true => StdbyCmdPrioCh0::B1,
        }
    }
    #[doc = "issue priority request when enter standby mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == StdbyCmdPrioCh0::B0
    }
    #[doc = "issue priority request when enter standby mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == StdbyCmdPrioCh0::B1
    }
}
#[doc = "Field `STDBY_CMD_PRIO_CH0` writer - Channel 0 LP command priority in standby mode"]
pub type StdbyCmdPrioCh0W<'a, REG> = crate::BitWriter<'a, REG, StdbyCmdPrioCh0>;
impl<'a, REG> StdbyCmdPrioCh0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "issue priority request when enter standby mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(StdbyCmdPrioCh0::B0)
    }
    #[doc = "issue priority request when enter standby mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(StdbyCmdPrioCh0::B1)
    }
}
#[doc = "Channel 1 LP command priority in standby mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StdbyCmdPrioCh1 {
    #[doc = "0: issue priority request when enter standby mode"]
    B0 = 0,
    #[doc = "1: issue priority request when enter standby mode"]
    B1 = 1,
}
impl From<StdbyCmdPrioCh1> for bool {
    #[inline(always)]
    fn from(variant: StdbyCmdPrioCh1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STDBY_CMD_PRIO_CH1` reader - Channel 1 LP command priority in standby mode"]
pub type StdbyCmdPrioCh1R = crate::BitReader<StdbyCmdPrioCh1>;
impl StdbyCmdPrioCh1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StdbyCmdPrioCh1 {
        match self.bits {
            false => StdbyCmdPrioCh1::B0,
            true => StdbyCmdPrioCh1::B1,
        }
    }
    #[doc = "issue priority request when enter standby mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == StdbyCmdPrioCh1::B0
    }
    #[doc = "issue priority request when enter standby mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == StdbyCmdPrioCh1::B1
    }
}
#[doc = "Field `STDBY_CMD_PRIO_CH1` writer - Channel 1 LP command priority in standby mode"]
pub type StdbyCmdPrioCh1W<'a, REG> = crate::BitWriter<'a, REG, StdbyCmdPrioCh1>;
impl<'a, REG> StdbyCmdPrioCh1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "issue priority request when enter standby mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(StdbyCmdPrioCh1::B0)
    }
    #[doc = "issue priority request when enter standby mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(StdbyCmdPrioCh1::B1)
    }
}
#[doc = "Channel 0 LP command priority in external self-refresh mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LpCmdPrioCh0 {
    #[doc = "0: issue priority request when enter external self-refresh"]
    B0 = 0,
    #[doc = "1: issue priority request when enter external self-refresh"]
    B1 = 1,
}
impl From<LpCmdPrioCh0> for bool {
    #[inline(always)]
    fn from(variant: LpCmdPrioCh0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP_CMD_PRIO_CH0` reader - Channel 0 LP command priority in external self-refresh mode"]
pub type LpCmdPrioCh0R = crate::BitReader<LpCmdPrioCh0>;
impl LpCmdPrioCh0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LpCmdPrioCh0 {
        match self.bits {
            false => LpCmdPrioCh0::B0,
            true => LpCmdPrioCh0::B1,
        }
    }
    #[doc = "issue priority request when enter external self-refresh"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LpCmdPrioCh0::B0
    }
    #[doc = "issue priority request when enter external self-refresh"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LpCmdPrioCh0::B1
    }
}
#[doc = "Field `LP_CMD_PRIO_CH0` writer - Channel 0 LP command priority in external self-refresh mode"]
pub type LpCmdPrioCh0W<'a, REG> = crate::BitWriter<'a, REG, LpCmdPrioCh0>;
impl<'a, REG> LpCmdPrioCh0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "issue priority request when enter external self-refresh"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LpCmdPrioCh0::B0)
    }
    #[doc = "issue priority request when enter external self-refresh"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LpCmdPrioCh0::B1)
    }
}
#[doc = "Channel 1 LP command priority in external self-refresh mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LpCmdPrioCh1 {
    #[doc = "0: issue priority request when enter external self-refresh"]
    B0 = 0,
    #[doc = "1: issue priority request when enter external self-refresh"]
    B1 = 1,
}
impl From<LpCmdPrioCh1> for bool {
    #[inline(always)]
    fn from(variant: LpCmdPrioCh1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP_CMD_PRIO_CH1` reader - Channel 1 LP command priority in external self-refresh mode"]
pub type LpCmdPrioCh1R = crate::BitReader<LpCmdPrioCh1>;
impl LpCmdPrioCh1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LpCmdPrioCh1 {
        match self.bits {
            false => LpCmdPrioCh1::B0,
            true => LpCmdPrioCh1::B1,
        }
    }
    #[doc = "issue priority request when enter external self-refresh"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LpCmdPrioCh1::B0
    }
    #[doc = "issue priority request when enter external self-refresh"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LpCmdPrioCh1::B1
    }
}
#[doc = "Field `LP_CMD_PRIO_CH1` writer - Channel 1 LP command priority in external self-refresh mode"]
pub type LpCmdPrioCh1W<'a, REG> = crate::BitWriter<'a, REG, LpCmdPrioCh1>;
impl<'a, REG> LpCmdPrioCh1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "issue priority request when enter external self-refresh"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LpCmdPrioCh1::B0)
    }
    #[doc = "issue priority request when enter external self-refresh"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LpCmdPrioCh1::B1)
    }
}
#[doc = "Channel 0 memory clock gating in self-refresh\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SrefMemcgCh0 {
    #[doc = "0: memory is clock gated in external self-refresh"]
    B0 = 0,
    #[doc = "1: memory is clock gated in external self-refresh"]
    B1 = 1,
}
impl From<SrefMemcgCh0> for bool {
    #[inline(always)]
    fn from(variant: SrefMemcgCh0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SREF_MEMCG_CH0` reader - Channel 0 memory clock gating in self-refresh"]
pub type SrefMemcgCh0R = crate::BitReader<SrefMemcgCh0>;
impl SrefMemcgCh0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SrefMemcgCh0 {
        match self.bits {
            false => SrefMemcgCh0::B0,
            true => SrefMemcgCh0::B1,
        }
    }
    #[doc = "memory is clock gated in external self-refresh"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SrefMemcgCh0::B0
    }
    #[doc = "memory is clock gated in external self-refresh"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SrefMemcgCh0::B1
    }
}
#[doc = "Field `SREF_MEMCG_CH0` writer - Channel 0 memory clock gating in self-refresh"]
pub type SrefMemcgCh0W<'a, REG> = crate::BitWriter<'a, REG, SrefMemcgCh0>;
impl<'a, REG> SrefMemcgCh0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "memory is clock gated in external self-refresh"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SrefMemcgCh0::B0)
    }
    #[doc = "memory is clock gated in external self-refresh"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SrefMemcgCh0::B1)
    }
}
#[doc = "Channel 1 memory clock gating in self-refresh\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SrefMemcgCh1 {
    #[doc = "0: memory is clock gated in external self-refresh"]
    B0 = 0,
    #[doc = "1: memory is clock gated in external self-refresh"]
    B1 = 1,
}
impl From<SrefMemcgCh1> for bool {
    #[inline(always)]
    fn from(variant: SrefMemcgCh1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SREF_MEMCG_CH1` reader - Channel 1 memory clock gating in self-refresh"]
pub type SrefMemcgCh1R = crate::BitReader<SrefMemcgCh1>;
impl SrefMemcgCh1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SrefMemcgCh1 {
        match self.bits {
            false => SrefMemcgCh1::B0,
            true => SrefMemcgCh1::B1,
        }
    }
    #[doc = "memory is clock gated in external self-refresh"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SrefMemcgCh1::B0
    }
    #[doc = "memory is clock gated in external self-refresh"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SrefMemcgCh1::B1
    }
}
#[doc = "Field `SREF_MEMCG_CH1` writer - Channel 1 memory clock gating in self-refresh"]
pub type SrefMemcgCh1W<'a, REG> = crate::BitWriter<'a, REG, SrefMemcgCh1>;
impl<'a, REG> SrefMemcgCh1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "memory is clock gated in external self-refresh"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SrefMemcgCh1::B0)
    }
    #[doc = "memory is clock gated in external self-refresh"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SrefMemcgCh1::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software. When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software. When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software. When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software. When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software. When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software. When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Channel 0 standby mode enable"]
    #[inline(always)]
    pub fn stdby_en_ch0(&self) -> StdbyEnCh0R {
        StdbyEnCh0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 standby mode enable"]
    #[inline(always)]
    pub fn stdby_en_ch1(&self) -> StdbyEnCh1R {
        StdbyEnCh1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 0 memory clock gating in standby mode"]
    #[inline(always)]
    pub fn stdby_memcg_ch0(&self) -> StdbyMemcgCh0R {
        StdbyMemcgCh0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 1 memory clock gating in standby mode"]
    #[inline(always)]
    pub fn stdby_memcg_ch1(&self) -> StdbyMemcgCh1R {
        StdbyMemcgCh1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 0 LP command priority in standby mode"]
    #[inline(always)]
    pub fn stdby_cmd_prio_ch0(&self) -> StdbyCmdPrioCh0R {
        StdbyCmdPrioCh0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 LP command priority in standby mode"]
    #[inline(always)]
    pub fn stdby_cmd_prio_ch1(&self) -> StdbyCmdPrioCh1R {
        StdbyCmdPrioCh1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 0 LP command priority in external self-refresh mode"]
    #[inline(always)]
    pub fn lp_cmd_prio_ch0(&self) -> LpCmdPrioCh0R {
        LpCmdPrioCh0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 LP command priority in external self-refresh mode"]
    #[inline(always)]
    pub fn lp_cmd_prio_ch1(&self) -> LpCmdPrioCh1R {
        LpCmdPrioCh1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 0 memory clock gating in self-refresh"]
    #[inline(always)]
    pub fn sref_memcg_ch0(&self) -> SrefMemcgCh0R {
        SrefMemcgCh0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 1 memory clock gating in self-refresh"]
    #[inline(always)]
    pub fn sref_memcg_ch1(&self) -> SrefMemcgCh1R {
        SrefMemcgCh1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software. When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software. When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software. When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 standby mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn stdby_en_ch0(&mut self) -> StdbyEnCh0W<CicCtrl1Spec> {
        StdbyEnCh0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 standby mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn stdby_en_ch1(&mut self) -> StdbyEnCh1W<CicCtrl1Spec> {
        StdbyEnCh1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 0 memory clock gating in standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn stdby_memcg_ch0(&mut self) -> StdbyMemcgCh0W<CicCtrl1Spec> {
        StdbyMemcgCh0W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 1 memory clock gating in standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn stdby_memcg_ch1(&mut self) -> StdbyMemcgCh1W<CicCtrl1Spec> {
        StdbyMemcgCh1W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 0 LP command priority in standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn stdby_cmd_prio_ch0(&mut self) -> StdbyCmdPrioCh0W<CicCtrl1Spec> {
        StdbyCmdPrioCh0W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 1 LP command priority in standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn stdby_cmd_prio_ch1(&mut self) -> StdbyCmdPrioCh1W<CicCtrl1Spec> {
        StdbyCmdPrioCh1W::new(self, 5)
    }
    #[doc = "Bit 8 - Channel 0 LP command priority in external self-refresh mode"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cmd_prio_ch0(&mut self) -> LpCmdPrioCh0W<CicCtrl1Spec> {
        LpCmdPrioCh0W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 1 LP command priority in external self-refresh mode"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cmd_prio_ch1(&mut self) -> LpCmdPrioCh1W<CicCtrl1Spec> {
        LpCmdPrioCh1W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 0 memory clock gating in self-refresh"]
    #[inline(always)]
    #[must_use]
    pub fn sref_memcg_ch0(&mut self) -> SrefMemcgCh0W<CicCtrl1Spec> {
        SrefMemcgCh0W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 1 memory clock gating in self-refresh"]
    #[inline(always)]
    #[must_use]
    pub fn sref_memcg_ch1(&mut self) -> SrefMemcgCh1W<CicCtrl1Spec> {
        SrefMemcgCh1W::new(self, 11)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software. When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software. When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software. When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<CicCtrl1Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "DDR Controller LP Interface Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cic_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cic_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CicCtrl1Spec;
impl crate::RegisterSpec for CicCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cic_ctrl1::R`](R) reader structure"]
impl crate::Readable for CicCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`cic_ctrl1::W`](W) writer structure"]
impl crate::Writable for CicCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CIC_CTRL1 to value 0"]
impl crate::Resettable for CicCtrl1Spec {
    const RESET_VALUE: u32 = 0;
}
