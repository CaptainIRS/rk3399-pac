#[doc = "Register `PMU_PWRDN_INTEN` reader"]
pub type R = crate::R<PmuPwrdnIntenSpec>;
#[doc = "Register `PMU_PWRDN_INTEN` writer"]
pub type W = crate::W<PmuPwrdnIntenSpec>;
#[doc = "pd_a53_l0 power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdA53L0PwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdA53L0PwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdA53L0PwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_A53_L0_PWR_SWITCH_INT_EN` reader - pd_a53_l0 power switch interrupt enable"]
pub type PdA53L0PwrSwitchIntEnR = crate::BitReader<PdA53L0PwrSwitchIntEn>;
impl PdA53L0PwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdA53L0PwrSwitchIntEn {
        match self.bits {
            false => PdA53L0PwrSwitchIntEn::B0,
            true => PdA53L0PwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdA53L0PwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdA53L0PwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_A53_L0_PWR_SWITCH_INT_EN` writer - pd_a53_l0 power switch interrupt enable"]
pub type PdA53L0PwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdA53L0PwrSwitchIntEn>;
impl<'a, REG> PdA53L0PwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdA53L0PwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdA53L0PwrSwitchIntEn::B1)
    }
}
#[doc = "pd_a53_l1 power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdA53L1PwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdA53L1PwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdA53L1PwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_A53_L1_PWR_SWITCH_INT_EN` reader - pd_a53_l1 power switch interrupt enable"]
pub type PdA53L1PwrSwitchIntEnR = crate::BitReader<PdA53L1PwrSwitchIntEn>;
impl PdA53L1PwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdA53L1PwrSwitchIntEn {
        match self.bits {
            false => PdA53L1PwrSwitchIntEn::B0,
            true => PdA53L1PwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdA53L1PwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdA53L1PwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_A53_L1_PWR_SWITCH_INT_EN` writer - pd_a53_l1 power switch interrupt enable"]
pub type PdA53L1PwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdA53L1PwrSwitchIntEn>;
impl<'a, REG> PdA53L1PwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdA53L1PwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdA53L1PwrSwitchIntEn::B1)
    }
}
#[doc = "pd_a53_l2 power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdA53L2PwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdA53L2PwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdA53L2PwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_A53_L2_PWR_SWITCH_INT_EN` reader - pd_a53_l2 power switch interrupt enable"]
pub type PdA53L2PwrSwitchIntEnR = crate::BitReader<PdA53L2PwrSwitchIntEn>;
impl PdA53L2PwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdA53L2PwrSwitchIntEn {
        match self.bits {
            false => PdA53L2PwrSwitchIntEn::B0,
            true => PdA53L2PwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdA53L2PwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdA53L2PwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_A53_L2_PWR_SWITCH_INT_EN` writer - pd_a53_l2 power switch interrupt enable"]
pub type PdA53L2PwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdA53L2PwrSwitchIntEn>;
impl<'a, REG> PdA53L2PwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdA53L2PwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdA53L2PwrSwitchIntEn::B1)
    }
}
#[doc = "pd_a53_l3 power switch int enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdA53L3PwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdA53L3PwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdA53L3PwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_A53_L3_PWR_SWITCH_INT_EN` reader - pd_a53_l3 power switch int enable"]
pub type PdA53L3PwrSwitchIntEnR = crate::BitReader<PdA53L3PwrSwitchIntEn>;
impl PdA53L3PwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdA53L3PwrSwitchIntEn {
        match self.bits {
            false => PdA53L3PwrSwitchIntEn::B0,
            true => PdA53L3PwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdA53L3PwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdA53L3PwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_A53_L3_PWR_SWITCH_INT_EN` writer - pd_a53_l3 power switch int enable"]
pub type PdA53L3PwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdA53L3PwrSwitchIntEn>;
impl<'a, REG> PdA53L3PwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdA53L3PwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdA53L3PwrSwitchIntEn::B1)
    }
}
#[doc = "pd_a72_b0 power enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdA72B0PwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdA72B0PwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdA72B0PwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_A72_B0_PWR_SWITCH_INT_EN` reader - pd_a72_b0 power enable"]
pub type PdA72B0PwrSwitchIntEnR = crate::BitReader<PdA72B0PwrSwitchIntEn>;
impl PdA72B0PwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdA72B0PwrSwitchIntEn {
        match self.bits {
            false => PdA72B0PwrSwitchIntEn::B0,
            true => PdA72B0PwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdA72B0PwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdA72B0PwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_A72_B0_PWR_SWITCH_INT_EN` writer - pd_a72_b0 power enable"]
pub type PdA72B0PwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdA72B0PwrSwitchIntEn>;
impl<'a, REG> PdA72B0PwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdA72B0PwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdA72B0PwrSwitchIntEn::B1)
    }
}
#[doc = "pd_a72_b1 power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdA72B1PwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdA72B1PwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdA72B1PwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_A72_B1_PWR_SWITCH_INT_EN` reader - pd_a72_b1 power switch interrupt enable"]
pub type PdA72B1PwrSwitchIntEnR = crate::BitReader<PdA72B1PwrSwitchIntEn>;
impl PdA72B1PwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdA72B1PwrSwitchIntEn {
        match self.bits {
            false => PdA72B1PwrSwitchIntEn::B0,
            true => PdA72B1PwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdA72B1PwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdA72B1PwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_A72_B1_PWR_SWITCH_INT_EN` writer - pd_a72_b1 power switch interrupt enable"]
pub type PdA72B1PwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdA72B1PwrSwitchIntEn>;
impl<'a, REG> PdA72B1PwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdA72B1PwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdA72B1PwrSwitchIntEn::B1)
    }
}
#[doc = "pd_scu_l power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdScuLPwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdScuLPwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdScuLPwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_SCU_L_PWR_SWITCH_INT_EN` reader - pd_scu_l power switch interrupt enable"]
pub type PdScuLPwrSwitchIntEnR = crate::BitReader<PdScuLPwrSwitchIntEn>;
impl PdScuLPwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdScuLPwrSwitchIntEn {
        match self.bits {
            false => PdScuLPwrSwitchIntEn::B0,
            true => PdScuLPwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdScuLPwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdScuLPwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_SCU_L_PWR_SWITCH_INT_EN` writer - pd_scu_l power switch interrupt enable"]
pub type PdScuLPwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdScuLPwrSwitchIntEn>;
impl<'a, REG> PdScuLPwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdScuLPwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdScuLPwrSwitchIntEn::B1)
    }
}
#[doc = "pd_scu_b power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdScuBPwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdScuBPwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdScuBPwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_SCU_B_PWR_SWITCH_INT_EN` reader - pd_scu_b power switch interrupt enable"]
pub type PdScuBPwrSwitchIntEnR = crate::BitReader<PdScuBPwrSwitchIntEn>;
impl PdScuBPwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdScuBPwrSwitchIntEn {
        match self.bits {
            false => PdScuBPwrSwitchIntEn::B0,
            true => PdScuBPwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdScuBPwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdScuBPwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_SCU_B_PWR_SWITCH_INT_EN` writer - pd_scu_b power switch interrupt enable"]
pub type PdScuBPwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdScuBPwrSwitchIntEn>;
impl<'a, REG> PdScuBPwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdScuBPwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdScuBPwrSwitchIntEn::B1)
    }
}
#[doc = "pd_tcpd0 power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdTcpd0PwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdTcpd0PwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdTcpd0PwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_TCPD0_PWR_SWITCH_INT_EN` reader - pd_tcpd0 power switch interrupt enable"]
pub type PdTcpd0PwrSwitchIntEnR = crate::BitReader<PdTcpd0PwrSwitchIntEn>;
impl PdTcpd0PwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdTcpd0PwrSwitchIntEn {
        match self.bits {
            false => PdTcpd0PwrSwitchIntEn::B0,
            true => PdTcpd0PwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdTcpd0PwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdTcpd0PwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_TCPD0_PWR_SWITCH_INT_EN` writer - pd_tcpd0 power switch interrupt enable"]
pub type PdTcpd0PwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdTcpd0PwrSwitchIntEn>;
impl<'a, REG> PdTcpd0PwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdTcpd0PwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdTcpd0PwrSwitchIntEn::B1)
    }
}
#[doc = "pd_tcpd1 power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdTcpd1PwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdTcpd1PwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdTcpd1PwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_TCPD1_PWR_SWITCH_INT_EN` reader - pd_tcpd1 power switch interrupt enable"]
pub type PdTcpd1PwrSwitchIntEnR = crate::BitReader<PdTcpd1PwrSwitchIntEn>;
impl PdTcpd1PwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdTcpd1PwrSwitchIntEn {
        match self.bits {
            false => PdTcpd1PwrSwitchIntEn::B0,
            true => PdTcpd1PwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdTcpd1PwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdTcpd1PwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_TCPD1_PWR_SWITCH_INT_EN` writer - pd_tcpd1 power switch interrupt enable"]
pub type PdTcpd1PwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdTcpd1PwrSwitchIntEn>;
impl<'a, REG> PdTcpd1PwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdTcpd1PwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdTcpd1PwrSwitchIntEn::B1)
    }
}
#[doc = "pd_cci power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdCciPwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdCciPwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdCciPwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_CCI_PWR_SWITCH_INT_EN` reader - pd_cci power switch interrupt enable"]
pub type PdCciPwrSwitchIntEnR = crate::BitReader<PdCciPwrSwitchIntEn>;
impl PdCciPwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdCciPwrSwitchIntEn {
        match self.bits {
            false => PdCciPwrSwitchIntEn::B0,
            true => PdCciPwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdCciPwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdCciPwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_CCI_PWR_SWITCH_INT_EN` writer - pd_cci power switch interrupt enable"]
pub type PdCciPwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdCciPwrSwitchIntEn>;
impl<'a, REG> PdCciPwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdCciPwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdCciPwrSwitchIntEn::B1)
    }
}
#[doc = "pd_perilp power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdPerilpPwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdPerilpPwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdPerilpPwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_PERILP_PWR_SWITCH_INT_EN` reader - pd_perilp power switch interrupt enable"]
pub type PdPerilpPwrSwitchIntEnR = crate::BitReader<PdPerilpPwrSwitchIntEn>;
impl PdPerilpPwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdPerilpPwrSwitchIntEn {
        match self.bits {
            false => PdPerilpPwrSwitchIntEn::B0,
            true => PdPerilpPwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdPerilpPwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdPerilpPwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_PERILP_PWR_SWITCH_INT_EN` writer - pd_perilp power switch interrupt enable"]
pub type PdPerilpPwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdPerilpPwrSwitchIntEn>;
impl<'a, REG> PdPerilpPwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdPerilpPwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdPerilpPwrSwitchIntEn::B1)
    }
}
#[doc = "pd_perihp power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdPerihpPwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdPerihpPwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdPerihpPwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_PERIHP_PWR_SWITCH_INT_EN` reader - pd_perihp power switch interrupt enable"]
pub type PdPerihpPwrSwitchIntEnR = crate::BitReader<PdPerihpPwrSwitchIntEn>;
impl PdPerihpPwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdPerihpPwrSwitchIntEn {
        match self.bits {
            false => PdPerihpPwrSwitchIntEn::B0,
            true => PdPerihpPwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdPerihpPwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdPerihpPwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_PERIHP_PWR_SWITCH_INT_EN` writer - pd_perihp power switch interrupt enable"]
pub type PdPerihpPwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdPerihpPwrSwitchIntEn>;
impl<'a, REG> PdPerihpPwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdPerihpPwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdPerihpPwrSwitchIntEn::B1)
    }
}
#[doc = "pd_center power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdCenterPwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdCenterPwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdCenterPwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_CENTER_PWR_SWITCH_INT_EN` reader - pd_center power switch interrupt enable"]
pub type PdCenterPwrSwitchIntEnR = crate::BitReader<PdCenterPwrSwitchIntEn>;
impl PdCenterPwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdCenterPwrSwitchIntEn {
        match self.bits {
            false => PdCenterPwrSwitchIntEn::B0,
            true => PdCenterPwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdCenterPwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdCenterPwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_CENTER_PWR_SWITCH_INT_EN` writer - pd_center power switch interrupt enable"]
pub type PdCenterPwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdCenterPwrSwitchIntEn>;
impl<'a, REG> PdCenterPwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdCenterPwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdCenterPwrSwitchIntEn::B1)
    }
}
#[doc = "pd_vio power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdVioPwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdVioPwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdVioPwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_VIO_PWR_SWITCH_INT_EN` reader - pd_vio power switch interrupt enable"]
pub type PdVioPwrSwitchIntEnR = crate::BitReader<PdVioPwrSwitchIntEn>;
impl PdVioPwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdVioPwrSwitchIntEn {
        match self.bits {
            false => PdVioPwrSwitchIntEn::B0,
            true => PdVioPwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdVioPwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdVioPwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_VIO_PWR_SWITCH_INT_EN` writer - pd_vio power switch interrupt enable"]
pub type PdVioPwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdVioPwrSwitchIntEn>;
impl<'a, REG> PdVioPwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdVioPwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdVioPwrSwitchIntEn::B1)
    }
}
#[doc = "pd_gpu power interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdGpuPwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdGpuPwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdGpuPwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_GPU_PWR_SWITCH_INT_EN` reader - pd_gpu power interrupt enable"]
pub type PdGpuPwrSwitchIntEnR = crate::BitReader<PdGpuPwrSwitchIntEn>;
impl PdGpuPwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdGpuPwrSwitchIntEn {
        match self.bits {
            false => PdGpuPwrSwitchIntEn::B0,
            true => PdGpuPwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdGpuPwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdGpuPwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_GPU_PWR_SWITCH_INT_EN` writer - pd_gpu power interrupt enable"]
pub type PdGpuPwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdGpuPwrSwitchIntEn>;
impl<'a, REG> PdGpuPwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdGpuPwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdGpuPwrSwitchIntEn::B1)
    }
}
#[doc = "pd_perihp power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdVcodecPwrSwitchInten {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdVcodecPwrSwitchInten> for bool {
    #[inline(always)]
    fn from(variant: PdVcodecPwrSwitchInten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_VCODEC_PWR_SWITCH_INTEN` reader - pd_perihp power switch interrupt enable"]
pub type PdVcodecPwrSwitchIntenR = crate::BitReader<PdVcodecPwrSwitchInten>;
impl PdVcodecPwrSwitchIntenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdVcodecPwrSwitchInten {
        match self.bits {
            false => PdVcodecPwrSwitchInten::B0,
            true => PdVcodecPwrSwitchInten::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdVcodecPwrSwitchInten::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdVcodecPwrSwitchInten::B1
    }
}
#[doc = "Field `PD_VCODEC_PWR_SWITCH_INTEN` writer - pd_perihp power switch interrupt enable"]
pub type PdVcodecPwrSwitchIntenW<'a, REG> = crate::BitWriter<'a, REG, PdVcodecPwrSwitchInten>;
impl<'a, REG> PdVcodecPwrSwitchIntenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdVcodecPwrSwitchInten::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdVcodecPwrSwitchInten::B1)
    }
}
#[doc = "pd_vdu power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdVduPwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdVduPwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdVduPwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_VDU_PWR_SWITCH_INT_EN` reader - pd_vdu power switch interrupt enable"]
pub type PdVduPwrSwitchIntEnR = crate::BitReader<PdVduPwrSwitchIntEn>;
impl PdVduPwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdVduPwrSwitchIntEn {
        match self.bits {
            false => PdVduPwrSwitchIntEn::B0,
            true => PdVduPwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdVduPwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdVduPwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_VDU_PWR_SWITCH_INT_EN` writer - pd_vdu power switch interrupt enable"]
pub type PdVduPwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdVduPwrSwitchIntEn>;
impl<'a, REG> PdVduPwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdVduPwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdVduPwrSwitchIntEn::B1)
    }
}
#[doc = "pd_rga power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdRgaPwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdRgaPwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdRgaPwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_RGA_PWR_SWITCH_INT_EN` reader - pd_rga power switch interrupt enable"]
pub type PdRgaPwrSwitchIntEnR = crate::BitReader<PdRgaPwrSwitchIntEn>;
impl PdRgaPwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdRgaPwrSwitchIntEn {
        match self.bits {
            false => PdRgaPwrSwitchIntEn::B0,
            true => PdRgaPwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdRgaPwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdRgaPwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_RGA_PWR_SWITCH_INT_EN` writer - pd_rga power switch interrupt enable"]
pub type PdRgaPwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdRgaPwrSwitchIntEn>;
impl<'a, REG> PdRgaPwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdRgaPwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdRgaPwrSwitchIntEn::B1)
    }
}
#[doc = "pd_perihp power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdIepPwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdIepPwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdIepPwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_IEP_PWR_SWITCH_INT_EN` reader - pd_perihp power switch interrupt enable"]
pub type PdIepPwrSwitchIntEnR = crate::BitReader<PdIepPwrSwitchIntEn>;
impl PdIepPwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdIepPwrSwitchIntEn {
        match self.bits {
            false => PdIepPwrSwitchIntEn::B0,
            true => PdIepPwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdIepPwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdIepPwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_IEP_PWR_SWITCH_INT_EN` writer - pd_perihp power switch interrupt enable"]
pub type PdIepPwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdIepPwrSwitchIntEn>;
impl<'a, REG> PdIepPwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdIepPwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdIepPwrSwitchIntEn::B1)
    }
}
#[doc = "pd_vo power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdVoPwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdVoPwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdVoPwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_VO_PWR_SWITCH_INT_EN` reader - pd_vo power switch interrupt enable"]
pub type PdVoPwrSwitchIntEnR = crate::BitReader<PdVoPwrSwitchIntEn>;
impl PdVoPwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdVoPwrSwitchIntEn {
        match self.bits {
            false => PdVoPwrSwitchIntEn::B0,
            true => PdVoPwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdVoPwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdVoPwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_VO_PWR_SWITCH_INT_EN` writer - pd_vo power switch interrupt enable"]
pub type PdVoPwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdVoPwrSwitchIntEn>;
impl<'a, REG> PdVoPwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdVoPwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdVoPwrSwitchIntEn::B1)
    }
}
#[doc = "pd_isp0 power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdIsp0PwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdIsp0PwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdIsp0PwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_ISP0_PWR_SWITCH_INT_EN` reader - pd_isp0 power switch interrupt enable"]
pub type PdIsp0PwrSwitchIntEnR = crate::BitReader<PdIsp0PwrSwitchIntEn>;
impl PdIsp0PwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdIsp0PwrSwitchIntEn {
        match self.bits {
            false => PdIsp0PwrSwitchIntEn::B0,
            true => PdIsp0PwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdIsp0PwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdIsp0PwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_ISP0_PWR_SWITCH_INT_EN` writer - pd_isp0 power switch interrupt enable"]
pub type PdIsp0PwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdIsp0PwrSwitchIntEn>;
impl<'a, REG> PdIsp0PwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdIsp0PwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdIsp0PwrSwitchIntEn::B1)
    }
}
#[doc = "pd_isp1 power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdIsp1PwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdIsp1PwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdIsp1PwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_ISP1_PWR_SWITCH_INT_EN` reader - pd_isp1 power switch interrupt enable"]
pub type PdIsp1PwrSwitchIntEnR = crate::BitReader<PdIsp1PwrSwitchIntEn>;
impl PdIsp1PwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdIsp1PwrSwitchIntEn {
        match self.bits {
            false => PdIsp1PwrSwitchIntEn::B0,
            true => PdIsp1PwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdIsp1PwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdIsp1PwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_ISP1_PWR_SWITCH_INT_EN` writer - pd_isp1 power switch interrupt enable"]
pub type PdIsp1PwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdIsp1PwrSwitchIntEn>;
impl<'a, REG> PdIsp1PwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdIsp1PwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdIsp1PwrSwitchIntEn::B1)
    }
}
#[doc = "pd_hdcp power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdHdcpPwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdHdcpPwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdHdcpPwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_HDCP_PWR_SWITCH_INT_EN` reader - pd_hdcp power switch interrupt enable"]
pub type PdHdcpPwrSwitchIntEnR = crate::BitReader<PdHdcpPwrSwitchIntEn>;
impl PdHdcpPwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdHdcpPwrSwitchIntEn {
        match self.bits {
            false => PdHdcpPwrSwitchIntEn::B0,
            true => PdHdcpPwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdHdcpPwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdHdcpPwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_HDCP_PWR_SWITCH_INT_EN` writer - pd_hdcp power switch interrupt enable"]
pub type PdHdcpPwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdHdcpPwrSwitchIntEn>;
impl<'a, REG> PdHdcpPwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdHdcpPwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdHdcpPwrSwitchIntEn::B1)
    }
}
#[doc = "pd_gmac power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdGmacPwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdGmacPwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdGmacPwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_GMAC_PWR_SWITCH_INT_EN` reader - pd_gmac power switch interrupt enable"]
pub type PdGmacPwrSwitchIntEnR = crate::BitReader<PdGmacPwrSwitchIntEn>;
impl PdGmacPwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdGmacPwrSwitchIntEn {
        match self.bits {
            false => PdGmacPwrSwitchIntEn::B0,
            true => PdGmacPwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdGmacPwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdGmacPwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_GMAC_PWR_SWITCH_INT_EN` writer - pd_gmac power switch interrupt enable"]
pub type PdGmacPwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdGmacPwrSwitchIntEn>;
impl<'a, REG> PdGmacPwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdGmacPwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdGmacPwrSwitchIntEn::B1)
    }
}
#[doc = "pd_emmc power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdEmmcPwrSwitchInterruptEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdEmmcPwrSwitchInterruptEn> for bool {
    #[inline(always)]
    fn from(variant: PdEmmcPwrSwitchInterruptEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_EMMC_PWR_SWITCH_INTERRUPT_EN` reader - pd_emmc power switch interrupt enable"]
pub type PdEmmcPwrSwitchInterruptEnR = crate::BitReader<PdEmmcPwrSwitchInterruptEn>;
impl PdEmmcPwrSwitchInterruptEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdEmmcPwrSwitchInterruptEn {
        match self.bits {
            false => PdEmmcPwrSwitchInterruptEn::B0,
            true => PdEmmcPwrSwitchInterruptEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdEmmcPwrSwitchInterruptEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdEmmcPwrSwitchInterruptEn::B1
    }
}
#[doc = "Field `PD_EMMC_PWR_SWITCH_INTERRUPT_EN` writer - pd_emmc power switch interrupt enable"]
pub type PdEmmcPwrSwitchInterruptEnW<'a, REG> =
    crate::BitWriter<'a, REG, PdEmmcPwrSwitchInterruptEn>;
impl<'a, REG> PdEmmcPwrSwitchInterruptEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdEmmcPwrSwitchInterruptEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdEmmcPwrSwitchInterruptEn::B1)
    }
}
#[doc = "pd_usb3 power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdUsb3PwrSwitchInterruptEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdUsb3PwrSwitchInterruptEn> for bool {
    #[inline(always)]
    fn from(variant: PdUsb3PwrSwitchInterruptEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_USB3_PWR_SWITCH_INTERRUPT_EN` reader - pd_usb3 power switch interrupt enable"]
pub type PdUsb3PwrSwitchInterruptEnR = crate::BitReader<PdUsb3PwrSwitchInterruptEn>;
impl PdUsb3PwrSwitchInterruptEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdUsb3PwrSwitchInterruptEn {
        match self.bits {
            false => PdUsb3PwrSwitchInterruptEn::B0,
            true => PdUsb3PwrSwitchInterruptEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdUsb3PwrSwitchInterruptEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdUsb3PwrSwitchInterruptEn::B1
    }
}
#[doc = "Field `PD_USB3_PWR_SWITCH_INTERRUPT_EN` writer - pd_usb3 power switch interrupt enable"]
pub type PdUsb3PwrSwitchInterruptEnW<'a, REG> =
    crate::BitWriter<'a, REG, PdUsb3PwrSwitchInterruptEn>;
impl<'a, REG> PdUsb3PwrSwitchInterruptEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdUsb3PwrSwitchInterruptEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdUsb3PwrSwitchInterruptEn::B1)
    }
}
#[doc = "pd_edp power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdEdpPwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdEdpPwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdEdpPwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_EDP_PWR_SWITCH_INT_EN` reader - pd_edp power switch interrupt enable"]
pub type PdEdpPwrSwitchIntEnR = crate::BitReader<PdEdpPwrSwitchIntEn>;
impl PdEdpPwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdEdpPwrSwitchIntEn {
        match self.bits {
            false => PdEdpPwrSwitchIntEn::B0,
            true => PdEdpPwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdEdpPwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdEdpPwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_EDP_PWR_SWITCH_INT_EN` writer - pd_edp power switch interrupt enable"]
pub type PdEdpPwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdEdpPwrSwitchIntEn>;
impl<'a, REG> PdEdpPwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdEdpPwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdEdpPwrSwitchIntEn::B1)
    }
}
#[doc = "pd_gic power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdGicPwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdGicPwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdGicPwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_GIC_PWR_SWITCH_INT_EN` reader - pd_gic power switch interrupt enable"]
pub type PdGicPwrSwitchIntEnR = crate::BitReader<PdGicPwrSwitchIntEn>;
impl PdGicPwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdGicPwrSwitchIntEn {
        match self.bits {
            false => PdGicPwrSwitchIntEn::B0,
            true => PdGicPwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdGicPwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdGicPwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_GIC_PWR_SWITCH_INT_EN` writer - pd_gic power switch interrupt enable"]
pub type PdGicPwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdGicPwrSwitchIntEn>;
impl<'a, REG> PdGicPwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdGicPwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdGicPwrSwitchIntEn::B1)
    }
}
#[doc = "pd_sd power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdSdPwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdSdPwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdSdPwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_SD_PWR_SWITCH_INT_EN` reader - pd_sd power switch interrupt enable"]
pub type PdSdPwrSwitchIntEnR = crate::BitReader<PdSdPwrSwitchIntEn>;
impl PdSdPwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdSdPwrSwitchIntEn {
        match self.bits {
            false => PdSdPwrSwitchIntEn::B0,
            true => PdSdPwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdSdPwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdSdPwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_SD_PWR_SWITCH_INT_EN` writer - pd_sd power switch interrupt enable"]
pub type PdSdPwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdSdPwrSwitchIntEn>;
impl<'a, REG> PdSdPwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdSdPwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdSdPwrSwitchIntEn::B1)
    }
}
#[doc = "pd_sdioaudio power switch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdSdioaudioPwrSwitchIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdSdioaudioPwrSwitchIntEn> for bool {
    #[inline(always)]
    fn from(variant: PdSdioaudioPwrSwitchIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_SDIOAUDIO_PWR_SWITCH_INT_EN` reader - pd_sdioaudio power switch interrupt enable"]
pub type PdSdioaudioPwrSwitchIntEnR = crate::BitReader<PdSdioaudioPwrSwitchIntEn>;
impl PdSdioaudioPwrSwitchIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdSdioaudioPwrSwitchIntEn {
        match self.bits {
            false => PdSdioaudioPwrSwitchIntEn::B0,
            true => PdSdioaudioPwrSwitchIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdSdioaudioPwrSwitchIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdSdioaudioPwrSwitchIntEn::B1
    }
}
#[doc = "Field `PD_SDIOAUDIO_PWR_SWITCH_INT_EN` writer - pd_sdioaudio power switch interrupt enable"]
pub type PdSdioaudioPwrSwitchIntEnW<'a, REG> = crate::BitWriter<'a, REG, PdSdioaudioPwrSwitchIntEn>;
impl<'a, REG> PdSdioaudioPwrSwitchIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdSdioaudioPwrSwitchIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdSdioaudioPwrSwitchIntEn::B1)
    }
}
impl R {
    #[doc = "Bit 0 - pd_a53_l0 power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_a53_l0_pwr_switch_int_en(&self) -> PdA53L0PwrSwitchIntEnR {
        PdA53L0PwrSwitchIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pd_a53_l1 power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_a53_l1_pwr_switch_int_en(&self) -> PdA53L1PwrSwitchIntEnR {
        PdA53L1PwrSwitchIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pd_a53_l2 power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_a53_l2_pwr_switch_int_en(&self) -> PdA53L2PwrSwitchIntEnR {
        PdA53L2PwrSwitchIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - pd_a53_l3 power switch int enable"]
    #[inline(always)]
    pub fn pd_a53_l3_pwr_switch_int_en(&self) -> PdA53L3PwrSwitchIntEnR {
        PdA53L3PwrSwitchIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - pd_a72_b0 power enable"]
    #[inline(always)]
    pub fn pd_a72_b0_pwr_switch_int_en(&self) -> PdA72B0PwrSwitchIntEnR {
        PdA72B0PwrSwitchIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - pd_a72_b1 power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_a72_b1_pwr_switch_int_en(&self) -> PdA72B1PwrSwitchIntEnR {
        PdA72B1PwrSwitchIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pd_scu_l power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_scu_l_pwr_switch_int_en(&self) -> PdScuLPwrSwitchIntEnR {
        PdScuLPwrSwitchIntEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - pd_scu_b power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_scu_b_pwr_switch_int_en(&self) -> PdScuBPwrSwitchIntEnR {
        PdScuBPwrSwitchIntEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - pd_tcpd0 power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_tcpd0_pwr_switch_int_en(&self) -> PdTcpd0PwrSwitchIntEnR {
        PdTcpd0PwrSwitchIntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - pd_tcpd1 power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_tcpd1_pwr_switch_int_en(&self) -> PdTcpd1PwrSwitchIntEnR {
        PdTcpd1PwrSwitchIntEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - pd_cci power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_cci_pwr_switch_int_en(&self) -> PdCciPwrSwitchIntEnR {
        PdCciPwrSwitchIntEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - pd_perilp power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_perilp_pwr_switch_int_en(&self) -> PdPerilpPwrSwitchIntEnR {
        PdPerilpPwrSwitchIntEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - pd_perihp power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_perihp_pwr_switch_int_en(&self) -> PdPerihpPwrSwitchIntEnR {
        PdPerihpPwrSwitchIntEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - pd_center power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_center_pwr_switch_int_en(&self) -> PdCenterPwrSwitchIntEnR {
        PdCenterPwrSwitchIntEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - pd_vio power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_vio_pwr_switch_int_en(&self) -> PdVioPwrSwitchIntEnR {
        PdVioPwrSwitchIntEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - pd_gpu power interrupt enable"]
    #[inline(always)]
    pub fn pd_gpu_pwr_switch_int_en(&self) -> PdGpuPwrSwitchIntEnR {
        PdGpuPwrSwitchIntEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - pd_perihp power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_vcodec_pwr_switch_inten(&self) -> PdVcodecPwrSwitchIntenR {
        PdVcodecPwrSwitchIntenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - pd_vdu power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_vdu_pwr_switch_int_en(&self) -> PdVduPwrSwitchIntEnR {
        PdVduPwrSwitchIntEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - pd_rga power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_rga_pwr_switch_int_en(&self) -> PdRgaPwrSwitchIntEnR {
        PdRgaPwrSwitchIntEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - pd_perihp power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_iep_pwr_switch_int_en(&self) -> PdIepPwrSwitchIntEnR {
        PdIepPwrSwitchIntEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - pd_vo power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_vo_pwr_switch_int_en(&self) -> PdVoPwrSwitchIntEnR {
        PdVoPwrSwitchIntEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - pd_isp0 power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_isp0_pwr_switch_int_en(&self) -> PdIsp0PwrSwitchIntEnR {
        PdIsp0PwrSwitchIntEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - pd_isp1 power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_isp1_pwr_switch_int_en(&self) -> PdIsp1PwrSwitchIntEnR {
        PdIsp1PwrSwitchIntEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - pd_hdcp power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_hdcp_pwr_switch_int_en(&self) -> PdHdcpPwrSwitchIntEnR {
        PdHdcpPwrSwitchIntEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - pd_gmac power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_gmac_pwr_switch_int_en(&self) -> PdGmacPwrSwitchIntEnR {
        PdGmacPwrSwitchIntEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - pd_emmc power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_emmc_pwr_switch_interrupt_en(&self) -> PdEmmcPwrSwitchInterruptEnR {
        PdEmmcPwrSwitchInterruptEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - pd_usb3 power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_usb3_pwr_switch_interrupt_en(&self) -> PdUsb3PwrSwitchInterruptEnR {
        PdUsb3PwrSwitchInterruptEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - pd_edp power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_edp_pwr_switch_int_en(&self) -> PdEdpPwrSwitchIntEnR {
        PdEdpPwrSwitchIntEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - pd_gic power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_gic_pwr_switch_int_en(&self) -> PdGicPwrSwitchIntEnR {
        PdGicPwrSwitchIntEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - pd_sd power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_sd_pwr_switch_int_en(&self) -> PdSdPwrSwitchIntEnR {
        PdSdPwrSwitchIntEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - pd_sdioaudio power switch interrupt enable"]
    #[inline(always)]
    pub fn pd_sdioaudio_pwr_switch_int_en(&self) -> PdSdioaudioPwrSwitchIntEnR {
        PdSdioaudioPwrSwitchIntEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - pd_a53_l0 power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_a53_l0_pwr_switch_int_en(&mut self) -> PdA53L0PwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdA53L0PwrSwitchIntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - pd_a53_l1 power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_a53_l1_pwr_switch_int_en(&mut self) -> PdA53L1PwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdA53L1PwrSwitchIntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - pd_a53_l2 power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_a53_l2_pwr_switch_int_en(&mut self) -> PdA53L2PwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdA53L2PwrSwitchIntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - pd_a53_l3 power switch int enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_a53_l3_pwr_switch_int_en(&mut self) -> PdA53L3PwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdA53L3PwrSwitchIntEnW::new(self, 3)
    }
    #[doc = "Bit 4 - pd_a72_b0 power enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_a72_b0_pwr_switch_int_en(&mut self) -> PdA72B0PwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdA72B0PwrSwitchIntEnW::new(self, 4)
    }
    #[doc = "Bit 5 - pd_a72_b1 power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_a72_b1_pwr_switch_int_en(&mut self) -> PdA72B1PwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdA72B1PwrSwitchIntEnW::new(self, 5)
    }
    #[doc = "Bit 6 - pd_scu_l power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_scu_l_pwr_switch_int_en(&mut self) -> PdScuLPwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdScuLPwrSwitchIntEnW::new(self, 6)
    }
    #[doc = "Bit 7 - pd_scu_b power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_scu_b_pwr_switch_int_en(&mut self) -> PdScuBPwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdScuBPwrSwitchIntEnW::new(self, 7)
    }
    #[doc = "Bit 8 - pd_tcpd0 power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_tcpd0_pwr_switch_int_en(&mut self) -> PdTcpd0PwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdTcpd0PwrSwitchIntEnW::new(self, 8)
    }
    #[doc = "Bit 9 - pd_tcpd1 power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_tcpd1_pwr_switch_int_en(&mut self) -> PdTcpd1PwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdTcpd1PwrSwitchIntEnW::new(self, 9)
    }
    #[doc = "Bit 10 - pd_cci power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_cci_pwr_switch_int_en(&mut self) -> PdCciPwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdCciPwrSwitchIntEnW::new(self, 10)
    }
    #[doc = "Bit 11 - pd_perilp power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_perilp_pwr_switch_int_en(&mut self) -> PdPerilpPwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdPerilpPwrSwitchIntEnW::new(self, 11)
    }
    #[doc = "Bit 12 - pd_perihp power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_perihp_pwr_switch_int_en(&mut self) -> PdPerihpPwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdPerihpPwrSwitchIntEnW::new(self, 12)
    }
    #[doc = "Bit 13 - pd_center power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_center_pwr_switch_int_en(&mut self) -> PdCenterPwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdCenterPwrSwitchIntEnW::new(self, 13)
    }
    #[doc = "Bit 14 - pd_vio power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_vio_pwr_switch_int_en(&mut self) -> PdVioPwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdVioPwrSwitchIntEnW::new(self, 14)
    }
    #[doc = "Bit 15 - pd_gpu power interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_gpu_pwr_switch_int_en(&mut self) -> PdGpuPwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdGpuPwrSwitchIntEnW::new(self, 15)
    }
    #[doc = "Bit 16 - pd_perihp power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_vcodec_pwr_switch_inten(&mut self) -> PdVcodecPwrSwitchIntenW<PmuPwrdnIntenSpec> {
        PdVcodecPwrSwitchIntenW::new(self, 16)
    }
    #[doc = "Bit 17 - pd_vdu power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_vdu_pwr_switch_int_en(&mut self) -> PdVduPwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdVduPwrSwitchIntEnW::new(self, 17)
    }
    #[doc = "Bit 18 - pd_rga power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_rga_pwr_switch_int_en(&mut self) -> PdRgaPwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdRgaPwrSwitchIntEnW::new(self, 18)
    }
    #[doc = "Bit 19 - pd_perihp power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_iep_pwr_switch_int_en(&mut self) -> PdIepPwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdIepPwrSwitchIntEnW::new(self, 19)
    }
    #[doc = "Bit 20 - pd_vo power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_vo_pwr_switch_int_en(&mut self) -> PdVoPwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdVoPwrSwitchIntEnW::new(self, 20)
    }
    #[doc = "Bit 22 - pd_isp0 power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_isp0_pwr_switch_int_en(&mut self) -> PdIsp0PwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdIsp0PwrSwitchIntEnW::new(self, 22)
    }
    #[doc = "Bit 23 - pd_isp1 power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_isp1_pwr_switch_int_en(&mut self) -> PdIsp1PwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdIsp1PwrSwitchIntEnW::new(self, 23)
    }
    #[doc = "Bit 24 - pd_hdcp power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_hdcp_pwr_switch_int_en(&mut self) -> PdHdcpPwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdHdcpPwrSwitchIntEnW::new(self, 24)
    }
    #[doc = "Bit 25 - pd_gmac power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_gmac_pwr_switch_int_en(&mut self) -> PdGmacPwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdGmacPwrSwitchIntEnW::new(self, 25)
    }
    #[doc = "Bit 26 - pd_emmc power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_emmc_pwr_switch_interrupt_en(
        &mut self,
    ) -> PdEmmcPwrSwitchInterruptEnW<PmuPwrdnIntenSpec> {
        PdEmmcPwrSwitchInterruptEnW::new(self, 26)
    }
    #[doc = "Bit 27 - pd_usb3 power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_usb3_pwr_switch_interrupt_en(
        &mut self,
    ) -> PdUsb3PwrSwitchInterruptEnW<PmuPwrdnIntenSpec> {
        PdUsb3PwrSwitchInterruptEnW::new(self, 27)
    }
    #[doc = "Bit 28 - pd_edp power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_edp_pwr_switch_int_en(&mut self) -> PdEdpPwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdEdpPwrSwitchIntEnW::new(self, 28)
    }
    #[doc = "Bit 29 - pd_gic power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_gic_pwr_switch_int_en(&mut self) -> PdGicPwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdGicPwrSwitchIntEnW::new(self, 29)
    }
    #[doc = "Bit 30 - pd_sd power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_sd_pwr_switch_int_en(&mut self) -> PdSdPwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdSdPwrSwitchIntEnW::new(self, 30)
    }
    #[doc = "Bit 31 - pd_sdioaudio power switch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_sdioaudio_pwr_switch_int_en(
        &mut self,
    ) -> PdSdioaudioPwrSwitchIntEnW<PmuPwrdnIntenSpec> {
        PdSdioaudioPwrSwitchIntEnW::new(self, 31)
    }
}
#[doc = "pmu power down interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_pwrdn_inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_pwrdn_inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuPwrdnIntenSpec;
impl crate::RegisterSpec for PmuPwrdnIntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_pwrdn_inten::R`](R) reader structure"]
impl crate::Readable for PmuPwrdnIntenSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_pwrdn_inten::W`](W) writer structure"]
impl crate::Writable for PmuPwrdnIntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_PWRDN_INTEN to value 0"]
impl crate::Resettable for PmuPwrdnIntenSpec {
    const RESET_VALUE: u32 = 0;
}
