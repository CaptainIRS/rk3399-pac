#[doc = "Register `PMU_PWRDN_ST` reader"]
pub type R = crate::R<PmuPwrdnStSpec>;
#[doc = "Register `PMU_PWRDN_ST` writer"]
pub type W = crate::W<PmuPwrdnStSpec>;
#[doc = "pd_a53_l0 power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdA53L0PwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdA53L0PwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdA53L0PwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_A53_L0_PWR_STAT` reader - pd_a53_l0 power state"]
pub type PdA53L0PwrStatR = crate::BitReader<PdA53L0PwrStat>;
impl PdA53L0PwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdA53L0PwrStat {
        match self.bits {
            false => PdA53L0PwrStat::B0,
            true => PdA53L0PwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdA53L0PwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdA53L0PwrStat::B1
    }
}
#[doc = "pd_a53_l1 power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdA53L1PwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdA53L1PwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdA53L1PwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_A53_L1_PWR_STAT` reader - pd_a53_l1 power state"]
pub type PdA53L1PwrStatR = crate::BitReader<PdA53L1PwrStat>;
impl PdA53L1PwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdA53L1PwrStat {
        match self.bits {
            false => PdA53L1PwrStat::B0,
            true => PdA53L1PwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdA53L1PwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdA53L1PwrStat::B1
    }
}
#[doc = "pd_a53_l2 power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdA53L2PwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdA53L2PwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdA53L2PwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_A53_L2_PWR_STAT` reader - pd_a53_l2 power state"]
pub type PdA53L2PwrStatR = crate::BitReader<PdA53L2PwrStat>;
impl PdA53L2PwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdA53L2PwrStat {
        match self.bits {
            false => PdA53L2PwrStat::B0,
            true => PdA53L2PwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdA53L2PwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdA53L2PwrStat::B1
    }
}
#[doc = "pd_a53_l3 power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdA53L3PwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdA53L3PwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdA53L3PwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_A53_L3_PWR_STAT` reader - pd_a53_l3 power state"]
pub type PdA53L3PwrStatR = crate::BitReader<PdA53L3PwrStat>;
impl PdA53L3PwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdA53L3PwrStat {
        match self.bits {
            false => PdA53L3PwrStat::B0,
            true => PdA53L3PwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdA53L3PwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdA53L3PwrStat::B1
    }
}
#[doc = "pd_a72_b0 power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdA72B0PwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdA72B0PwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdA72B0PwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_A72_B0_PWR_STAT` reader - pd_a72_b0 power state"]
pub type PdA72B0PwrStatR = crate::BitReader<PdA72B0PwrStat>;
impl PdA72B0PwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdA72B0PwrStat {
        match self.bits {
            false => PdA72B0PwrStat::B0,
            true => PdA72B0PwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdA72B0PwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdA72B0PwrStat::B1
    }
}
#[doc = "pd_a72_b1 power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdA72B1PwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdA72B1PwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdA72B1PwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_A72_B1_PWR_STAT` reader - pd_a72_b1 power state"]
pub type PdA72B1PwrStatR = crate::BitReader<PdA72B1PwrStat>;
impl PdA72B1PwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdA72B1PwrStat {
        match self.bits {
            false => PdA72B1PwrStat::B0,
            true => PdA72B1PwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdA72B1PwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdA72B1PwrStat::B1
    }
}
#[doc = "pd_scu_l power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdScuLPwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdScuLPwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdScuLPwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_SCU_L_PWR_STAT` reader - pd_scu_l power state"]
pub type PdScuLPwrStatR = crate::BitReader<PdScuLPwrStat>;
impl PdScuLPwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdScuLPwrStat {
        match self.bits {
            false => PdScuLPwrStat::B0,
            true => PdScuLPwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdScuLPwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdScuLPwrStat::B1
    }
}
#[doc = "pd_scu_b power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdScuBPwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdScuBPwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdScuBPwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_SCU_B_PWR_STAT` reader - pd_scu_b power state"]
pub type PdScuBPwrStatR = crate::BitReader<PdScuBPwrStat>;
impl PdScuBPwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdScuBPwrStat {
        match self.bits {
            false => PdScuBPwrStat::B0,
            true => PdScuBPwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdScuBPwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdScuBPwrStat::B1
    }
}
#[doc = "pd_tcpd0 power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdTcpd0PwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdTcpd0PwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdTcpd0PwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_TCPD0_PWR_STAT` reader - pd_tcpd0 power state"]
pub type PdTcpd0PwrStatR = crate::BitReader<PdTcpd0PwrStat>;
impl PdTcpd0PwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdTcpd0PwrStat {
        match self.bits {
            false => PdTcpd0PwrStat::B0,
            true => PdTcpd0PwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdTcpd0PwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdTcpd0PwrStat::B1
    }
}
#[doc = "Field `PD_TCPD0_PWR_STAT` writer - pd_tcpd0 power state"]
pub type PdTcpd0PwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdTcpd0PwrStat>;
impl<'a, REG> PdTcpd0PwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdTcpd0PwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdTcpd0PwrStat::B1)
    }
}
#[doc = "pd_tcpd1 power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdTcpd1PwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdTcpd1PwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdTcpd1PwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_TCPD1_PWR_STAT` reader - pd_tcpd1 power state"]
pub type PdTcpd1PwrStatR = crate::BitReader<PdTcpd1PwrStat>;
impl PdTcpd1PwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdTcpd1PwrStat {
        match self.bits {
            false => PdTcpd1PwrStat::B0,
            true => PdTcpd1PwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdTcpd1PwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdTcpd1PwrStat::B1
    }
}
#[doc = "Field `PD_TCPD1_PWR_STAT` writer - pd_tcpd1 power state"]
pub type PdTcpd1PwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdTcpd1PwrStat>;
impl<'a, REG> PdTcpd1PwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdTcpd1PwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdTcpd1PwrStat::B1)
    }
}
#[doc = "pd_core power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdCciPwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdCciPwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdCciPwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_CCI_PWR_STAT` reader - pd_core power state"]
pub type PdCciPwrStatR = crate::BitReader<PdCciPwrStat>;
impl PdCciPwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdCciPwrStat {
        match self.bits {
            false => PdCciPwrStat::B0,
            true => PdCciPwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdCciPwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdCciPwrStat::B1
    }
}
#[doc = "Field `PD_CCI_PWR_STAT` writer - pd_core power state"]
pub type PdCciPwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdCciPwrStat>;
impl<'a, REG> PdCciPwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdCciPwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdCciPwrStat::B1)
    }
}
#[doc = "pd_bus power stat\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdPerilpPwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdPerilpPwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdPerilpPwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_PERILP_PWR_STAT` reader - pd_bus power stat"]
pub type PdPerilpPwrStatR = crate::BitReader<PdPerilpPwrStat>;
impl PdPerilpPwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdPerilpPwrStat {
        match self.bits {
            false => PdPerilpPwrStat::B0,
            true => PdPerilpPwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdPerilpPwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdPerilpPwrStat::B1
    }
}
#[doc = "Field `PD_PERILP_PWR_STAT` writer - pd_bus power stat"]
pub type PdPerilpPwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdPerilpPwrStat>;
impl<'a, REG> PdPerilpPwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdPerilpPwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdPerilpPwrStat::B1)
    }
}
#[doc = "pd_peri power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdPerihpPwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdPerihpPwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdPerihpPwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_PERIHP_PWR_STAT` reader - pd_peri power state"]
pub type PdPerihpPwrStatR = crate::BitReader<PdPerihpPwrStat>;
impl PdPerihpPwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdPerihpPwrStat {
        match self.bits {
            false => PdPerihpPwrStat::B0,
            true => PdPerihpPwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdPerihpPwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdPerihpPwrStat::B1
    }
}
#[doc = "Field `PD_PERIHP_PWR_STAT` writer - pd_peri power state"]
pub type PdPerihpPwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdPerihpPwrStat>;
impl<'a, REG> PdPerihpPwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdPerihpPwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdPerihpPwrStat::B1)
    }
}
#[doc = "pd_center power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdCenterPwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdCenterPwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdCenterPwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_CENTER_PWR_STAT` reader - pd_center power state"]
pub type PdCenterPwrStatR = crate::BitReader<PdCenterPwrStat>;
impl PdCenterPwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdCenterPwrStat {
        match self.bits {
            false => PdCenterPwrStat::B0,
            true => PdCenterPwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdCenterPwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdCenterPwrStat::B1
    }
}
#[doc = "Field `PD_CENTER_PWR_STAT` writer - pd_center power state"]
pub type PdCenterPwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdCenterPwrStat>;
impl<'a, REG> PdCenterPwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdCenterPwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdCenterPwrStat::B1)
    }
}
#[doc = "pd_vio power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdVioPwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdVioPwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdVioPwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_VIO_PWR_STAT` reader - pd_vio power state"]
pub type PdVioPwrStatR = crate::BitReader<PdVioPwrStat>;
impl PdVioPwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdVioPwrStat {
        match self.bits {
            false => PdVioPwrStat::B0,
            true => PdVioPwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdVioPwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdVioPwrStat::B1
    }
}
#[doc = "Field `PD_VIO_PWR_STAT` writer - pd_vio power state"]
pub type PdVioPwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdVioPwrStat>;
impl<'a, REG> PdVioPwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdVioPwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdVioPwrStat::B1)
    }
}
#[doc = "pd_gpu power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdGpuPwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdGpuPwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdGpuPwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_GPU_PWR_STAT` reader - pd_gpu power state"]
pub type PdGpuPwrStatR = crate::BitReader<PdGpuPwrStat>;
impl PdGpuPwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdGpuPwrStat {
        match self.bits {
            false => PdGpuPwrStat::B0,
            true => PdGpuPwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdGpuPwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdGpuPwrStat::B1
    }
}
#[doc = "Field `PD_GPU_PWR_STAT` writer - pd_gpu power state"]
pub type PdGpuPwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdGpuPwrStat>;
impl<'a, REG> PdGpuPwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdGpuPwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdGpuPwrStat::B1)
    }
}
#[doc = "pd_vcodec power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdVcodecPwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdVcodecPwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdVcodecPwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_VCODEC_PWR_STAT` reader - pd_vcodec power state"]
pub type PdVcodecPwrStatR = crate::BitReader<PdVcodecPwrStat>;
impl PdVcodecPwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdVcodecPwrStat {
        match self.bits {
            false => PdVcodecPwrStat::B0,
            true => PdVcodecPwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdVcodecPwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdVcodecPwrStat::B1
    }
}
#[doc = "Field `PD_VCODEC_PWR_STAT` writer - pd_vcodec power state"]
pub type PdVcodecPwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdVcodecPwrStat>;
impl<'a, REG> PdVcodecPwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdVcodecPwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdVcodecPwrStat::B1)
    }
}
#[doc = "pd_vdu power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdVduPwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdVduPwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdVduPwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_VDU_PWR_STAT` reader - pd_vdu power state"]
pub type PdVduPwrStatR = crate::BitReader<PdVduPwrStat>;
impl PdVduPwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdVduPwrStat {
        match self.bits {
            false => PdVduPwrStat::B0,
            true => PdVduPwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdVduPwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdVduPwrStat::B1
    }
}
#[doc = "Field `PD_VDU_PWR_STAT` writer - pd_vdu power state"]
pub type PdVduPwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdVduPwrStat>;
impl<'a, REG> PdVduPwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdVduPwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdVduPwrStat::B1)
    }
}
#[doc = "pd_rga power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdRgaPwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdRgaPwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdRgaPwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_RGA_PWR_STAT` reader - pd_rga power state"]
pub type PdRgaPwrStatR = crate::BitReader<PdRgaPwrStat>;
impl PdRgaPwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdRgaPwrStat {
        match self.bits {
            false => PdRgaPwrStat::B0,
            true => PdRgaPwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdRgaPwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdRgaPwrStat::B1
    }
}
#[doc = "Field `PD_RGA_PWR_STAT` writer - pd_rga power state"]
pub type PdRgaPwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdRgaPwrStat>;
impl<'a, REG> PdRgaPwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdRgaPwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdRgaPwrStat::B1)
    }
}
#[doc = "pd_iep power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdIepPwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdIepPwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdIepPwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_IEP_PWR_STAT` reader - pd_iep power state"]
pub type PdIepPwrStatR = crate::BitReader<PdIepPwrStat>;
impl PdIepPwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdIepPwrStat {
        match self.bits {
            false => PdIepPwrStat::B0,
            true => PdIepPwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdIepPwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdIepPwrStat::B1
    }
}
#[doc = "Field `PD_IEP_PWR_STAT` writer - pd_iep power state"]
pub type PdIepPwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdIepPwrStat>;
impl<'a, REG> PdIepPwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdIepPwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdIepPwrStat::B1)
    }
}
#[doc = "pd_vo power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdVoPwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdVoPwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdVoPwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_VO_PWR_STAT` reader - pd_vo power state"]
pub type PdVoPwrStatR = crate::BitReader<PdVoPwrStat>;
impl PdVoPwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdVoPwrStat {
        match self.bits {
            false => PdVoPwrStat::B0,
            true => PdVoPwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdVoPwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdVoPwrStat::B1
    }
}
#[doc = "Field `PD_VO_PWR_STAT` writer - pd_vo power state"]
pub type PdVoPwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdVoPwrStat>;
impl<'a, REG> PdVoPwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdVoPwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdVoPwrStat::B1)
    }
}
#[doc = "pd_isp0 power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdIsp0PwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdIsp0PwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdIsp0PwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_ISP0_PWR_STAT` reader - pd_isp0 power state"]
pub type PdIsp0PwrStatR = crate::BitReader<PdIsp0PwrStat>;
impl PdIsp0PwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdIsp0PwrStat {
        match self.bits {
            false => PdIsp0PwrStat::B0,
            true => PdIsp0PwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdIsp0PwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdIsp0PwrStat::B1
    }
}
#[doc = "Field `PD_ISP0_PWR_STAT` writer - pd_isp0 power state"]
pub type PdIsp0PwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdIsp0PwrStat>;
impl<'a, REG> PdIsp0PwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdIsp0PwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdIsp0PwrStat::B1)
    }
}
#[doc = "pd_isp1 power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdIsp1PwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdIsp1PwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdIsp1PwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_ISP1_PWR_STAT` reader - pd_isp1 power state"]
pub type PdIsp1PwrStatR = crate::BitReader<PdIsp1PwrStat>;
impl PdIsp1PwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdIsp1PwrStat {
        match self.bits {
            false => PdIsp1PwrStat::B0,
            true => PdIsp1PwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdIsp1PwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdIsp1PwrStat::B1
    }
}
#[doc = "Field `PD_ISP1_PWR_STAT` writer - pd_isp1 power state"]
pub type PdIsp1PwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdIsp1PwrStat>;
impl<'a, REG> PdIsp1PwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdIsp1PwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdIsp1PwrStat::B1)
    }
}
#[doc = "pd_hdcp power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdHdcpPwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdHdcpPwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdHdcpPwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_HDCP_PWR_STAT` reader - pd_hdcp power state"]
pub type PdHdcpPwrStatR = crate::BitReader<PdHdcpPwrStat>;
impl PdHdcpPwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdHdcpPwrStat {
        match self.bits {
            false => PdHdcpPwrStat::B0,
            true => PdHdcpPwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdHdcpPwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdHdcpPwrStat::B1
    }
}
#[doc = "Field `PD_HDCP_PWR_STAT` writer - pd_hdcp power state"]
pub type PdHdcpPwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdHdcpPwrStat>;
impl<'a, REG> PdHdcpPwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdHdcpPwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdHdcpPwrStat::B1)
    }
}
#[doc = "pd_gmac power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdGmacPwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdGmacPwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdGmacPwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_GMAC_PWR_STAT` reader - pd_gmac power state"]
pub type PdGmacPwrStatR = crate::BitReader<PdGmacPwrStat>;
impl PdGmacPwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdGmacPwrStat {
        match self.bits {
            false => PdGmacPwrStat::B0,
            true => PdGmacPwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdGmacPwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdGmacPwrStat::B1
    }
}
#[doc = "Field `PD_GMAC_PWR_STAT` writer - pd_gmac power state"]
pub type PdGmacPwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdGmacPwrStat>;
impl<'a, REG> PdGmacPwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdGmacPwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdGmacPwrStat::B1)
    }
}
#[doc = "pd_emmc power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdEmmcPwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdEmmcPwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdEmmcPwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_EMMC_PWR_STAT` reader - pd_emmc power state"]
pub type PdEmmcPwrStatR = crate::BitReader<PdEmmcPwrStat>;
impl PdEmmcPwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdEmmcPwrStat {
        match self.bits {
            false => PdEmmcPwrStat::B0,
            true => PdEmmcPwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdEmmcPwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdEmmcPwrStat::B1
    }
}
#[doc = "Field `PD_EMMC_PWR_STAT` writer - pd_emmc power state"]
pub type PdEmmcPwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdEmmcPwrStat>;
impl<'a, REG> PdEmmcPwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdEmmcPwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdEmmcPwrStat::B1)
    }
}
#[doc = "pd_usb3 power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdUsb3PwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdUsb3PwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdUsb3PwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_USB3_PWR_STAT` reader - pd_usb3 power state"]
pub type PdUsb3PwrStatR = crate::BitReader<PdUsb3PwrStat>;
impl PdUsb3PwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdUsb3PwrStat {
        match self.bits {
            false => PdUsb3PwrStat::B0,
            true => PdUsb3PwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdUsb3PwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdUsb3PwrStat::B1
    }
}
#[doc = "Field `PD_USB3_PWR_STAT` writer - pd_usb3 power state"]
pub type PdUsb3PwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdUsb3PwrStat>;
impl<'a, REG> PdUsb3PwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdUsb3PwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdUsb3PwrStat::B1)
    }
}
#[doc = "pd_edp power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdEdpPwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdEdpPwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdEdpPwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_EDP_PWR_STAT` reader - pd_edp power state"]
pub type PdEdpPwrStatR = crate::BitReader<PdEdpPwrStat>;
impl PdEdpPwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdEdpPwrStat {
        match self.bits {
            false => PdEdpPwrStat::B0,
            true => PdEdpPwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdEdpPwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdEdpPwrStat::B1
    }
}
#[doc = "Field `PD_EDP_PWR_STAT` writer - pd_edp power state"]
pub type PdEdpPwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdEdpPwrStat>;
impl<'a, REG> PdEdpPwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdEdpPwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdEdpPwrStat::B1)
    }
}
#[doc = "pd_gic power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdGicPwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdGicPwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdGicPwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_GIC_PWR_STAT` reader - pd_gic power state"]
pub type PdGicPwrStatR = crate::BitReader<PdGicPwrStat>;
impl PdGicPwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdGicPwrStat {
        match self.bits {
            false => PdGicPwrStat::B0,
            true => PdGicPwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdGicPwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdGicPwrStat::B1
    }
}
#[doc = "Field `PD_GIC_PWR_STAT` writer - pd_gic power state"]
pub type PdGicPwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdGicPwrStat>;
impl<'a, REG> PdGicPwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdGicPwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdGicPwrStat::B1)
    }
}
#[doc = "pd_sd power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdSdPwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdSdPwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdSdPwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_SD_PWR_STAT` reader - pd_sd power state"]
pub type PdSdPwrStatR = crate::BitReader<PdSdPwrStat>;
impl PdSdPwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdSdPwrStat {
        match self.bits {
            false => PdSdPwrStat::B0,
            true => PdSdPwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdSdPwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdSdPwrStat::B1
    }
}
#[doc = "Field `PD_SD_PWR_STAT` writer - pd_sd power state"]
pub type PdSdPwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdSdPwrStat>;
impl<'a, REG> PdSdPwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdSdPwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdSdPwrStat::B1)
    }
}
#[doc = "pd_sdioaudio power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdSdioaudioPwrStat {
    #[doc = "0: powered up"]
    B0 = 0,
    #[doc = "1: powered down"]
    B1 = 1,
}
impl From<PdSdioaudioPwrStat> for bool {
    #[inline(always)]
    fn from(variant: PdSdioaudioPwrStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_SDIOAUDIO_PWR_STAT` reader - pd_sdioaudio power state"]
pub type PdSdioaudioPwrStatR = crate::BitReader<PdSdioaudioPwrStat>;
impl PdSdioaudioPwrStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdSdioaudioPwrStat {
        match self.bits {
            false => PdSdioaudioPwrStat::B0,
            true => PdSdioaudioPwrStat::B1,
        }
    }
    #[doc = "powered up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdSdioaudioPwrStat::B0
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdSdioaudioPwrStat::B1
    }
}
#[doc = "Field `PD_SDIOAUDIO_PWR_STAT` writer - pd_sdioaudio power state"]
pub type PdSdioaudioPwrStatW<'a, REG> = crate::BitWriter<'a, REG, PdSdioaudioPwrStat>;
impl<'a, REG> PdSdioaudioPwrStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "powered up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdSdioaudioPwrStat::B0)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdSdioaudioPwrStat::B1)
    }
}
impl R {
    #[doc = "Bit 0 - pd_a53_l0 power state"]
    #[inline(always)]
    pub fn pd_a53_l0_pwr_stat(&self) -> PdA53L0PwrStatR {
        PdA53L0PwrStatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pd_a53_l1 power state"]
    #[inline(always)]
    pub fn pd_a53_l1_pwr_stat(&self) -> PdA53L1PwrStatR {
        PdA53L1PwrStatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pd_a53_l2 power state"]
    #[inline(always)]
    pub fn pd_a53_l2_pwr_stat(&self) -> PdA53L2PwrStatR {
        PdA53L2PwrStatR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - pd_a53_l3 power state"]
    #[inline(always)]
    pub fn pd_a53_l3_pwr_stat(&self) -> PdA53L3PwrStatR {
        PdA53L3PwrStatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - pd_a72_b0 power state"]
    #[inline(always)]
    pub fn pd_a72_b0_pwr_stat(&self) -> PdA72B0PwrStatR {
        PdA72B0PwrStatR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - pd_a72_b1 power state"]
    #[inline(always)]
    pub fn pd_a72_b1_pwr_stat(&self) -> PdA72B1PwrStatR {
        PdA72B1PwrStatR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pd_scu_l power state"]
    #[inline(always)]
    pub fn pd_scu_l_pwr_stat(&self) -> PdScuLPwrStatR {
        PdScuLPwrStatR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - pd_scu_b power state"]
    #[inline(always)]
    pub fn pd_scu_b_pwr_stat(&self) -> PdScuBPwrStatR {
        PdScuBPwrStatR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - pd_tcpd0 power state"]
    #[inline(always)]
    pub fn pd_tcpd0_pwr_stat(&self) -> PdTcpd0PwrStatR {
        PdTcpd0PwrStatR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - pd_tcpd1 power state"]
    #[inline(always)]
    pub fn pd_tcpd1_pwr_stat(&self) -> PdTcpd1PwrStatR {
        PdTcpd1PwrStatR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - pd_core power state"]
    #[inline(always)]
    pub fn pd_cci_pwr_stat(&self) -> PdCciPwrStatR {
        PdCciPwrStatR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - pd_bus power stat"]
    #[inline(always)]
    pub fn pd_perilp_pwr_stat(&self) -> PdPerilpPwrStatR {
        PdPerilpPwrStatR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - pd_peri power state"]
    #[inline(always)]
    pub fn pd_perihp_pwr_stat(&self) -> PdPerihpPwrStatR {
        PdPerihpPwrStatR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - pd_center power state"]
    #[inline(always)]
    pub fn pd_center_pwr_stat(&self) -> PdCenterPwrStatR {
        PdCenterPwrStatR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - pd_vio power state"]
    #[inline(always)]
    pub fn pd_vio_pwr_stat(&self) -> PdVioPwrStatR {
        PdVioPwrStatR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - pd_gpu power state"]
    #[inline(always)]
    pub fn pd_gpu_pwr_stat(&self) -> PdGpuPwrStatR {
        PdGpuPwrStatR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - pd_vcodec power state"]
    #[inline(always)]
    pub fn pd_vcodec_pwr_stat(&self) -> PdVcodecPwrStatR {
        PdVcodecPwrStatR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - pd_vdu power state"]
    #[inline(always)]
    pub fn pd_vdu_pwr_stat(&self) -> PdVduPwrStatR {
        PdVduPwrStatR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - pd_rga power state"]
    #[inline(always)]
    pub fn pd_rga_pwr_stat(&self) -> PdRgaPwrStatR {
        PdRgaPwrStatR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - pd_iep power state"]
    #[inline(always)]
    pub fn pd_iep_pwr_stat(&self) -> PdIepPwrStatR {
        PdIepPwrStatR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - pd_vo power state"]
    #[inline(always)]
    pub fn pd_vo_pwr_stat(&self) -> PdVoPwrStatR {
        PdVoPwrStatR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - pd_isp0 power state"]
    #[inline(always)]
    pub fn pd_isp0_pwr_stat(&self) -> PdIsp0PwrStatR {
        PdIsp0PwrStatR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - pd_isp1 power state"]
    #[inline(always)]
    pub fn pd_isp1_pwr_stat(&self) -> PdIsp1PwrStatR {
        PdIsp1PwrStatR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - pd_hdcp power state"]
    #[inline(always)]
    pub fn pd_hdcp_pwr_stat(&self) -> PdHdcpPwrStatR {
        PdHdcpPwrStatR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - pd_gmac power state"]
    #[inline(always)]
    pub fn pd_gmac_pwr_stat(&self) -> PdGmacPwrStatR {
        PdGmacPwrStatR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - pd_emmc power state"]
    #[inline(always)]
    pub fn pd_emmc_pwr_stat(&self) -> PdEmmcPwrStatR {
        PdEmmcPwrStatR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - pd_usb3 power state"]
    #[inline(always)]
    pub fn pd_usb3_pwr_stat(&self) -> PdUsb3PwrStatR {
        PdUsb3PwrStatR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - pd_edp power state"]
    #[inline(always)]
    pub fn pd_edp_pwr_stat(&self) -> PdEdpPwrStatR {
        PdEdpPwrStatR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - pd_gic power state"]
    #[inline(always)]
    pub fn pd_gic_pwr_stat(&self) -> PdGicPwrStatR {
        PdGicPwrStatR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - pd_sd power state"]
    #[inline(always)]
    pub fn pd_sd_pwr_stat(&self) -> PdSdPwrStatR {
        PdSdPwrStatR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - pd_sdioaudio power state"]
    #[inline(always)]
    pub fn pd_sdioaudio_pwr_stat(&self) -> PdSdioaudioPwrStatR {
        PdSdioaudioPwrStatR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - pd_tcpd0 power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_tcpd0_pwr_stat(&mut self) -> PdTcpd0PwrStatW<PmuPwrdnStSpec> {
        PdTcpd0PwrStatW::new(self, 8)
    }
    #[doc = "Bit 9 - pd_tcpd1 power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_tcpd1_pwr_stat(&mut self) -> PdTcpd1PwrStatW<PmuPwrdnStSpec> {
        PdTcpd1PwrStatW::new(self, 9)
    }
    #[doc = "Bit 10 - pd_core power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_cci_pwr_stat(&mut self) -> PdCciPwrStatW<PmuPwrdnStSpec> {
        PdCciPwrStatW::new(self, 10)
    }
    #[doc = "Bit 11 - pd_bus power stat"]
    #[inline(always)]
    #[must_use]
    pub fn pd_perilp_pwr_stat(&mut self) -> PdPerilpPwrStatW<PmuPwrdnStSpec> {
        PdPerilpPwrStatW::new(self, 11)
    }
    #[doc = "Bit 12 - pd_peri power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_perihp_pwr_stat(&mut self) -> PdPerihpPwrStatW<PmuPwrdnStSpec> {
        PdPerihpPwrStatW::new(self, 12)
    }
    #[doc = "Bit 13 - pd_center power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_center_pwr_stat(&mut self) -> PdCenterPwrStatW<PmuPwrdnStSpec> {
        PdCenterPwrStatW::new(self, 13)
    }
    #[doc = "Bit 14 - pd_vio power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_vio_pwr_stat(&mut self) -> PdVioPwrStatW<PmuPwrdnStSpec> {
        PdVioPwrStatW::new(self, 14)
    }
    #[doc = "Bit 15 - pd_gpu power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_gpu_pwr_stat(&mut self) -> PdGpuPwrStatW<PmuPwrdnStSpec> {
        PdGpuPwrStatW::new(self, 15)
    }
    #[doc = "Bit 16 - pd_vcodec power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_vcodec_pwr_stat(&mut self) -> PdVcodecPwrStatW<PmuPwrdnStSpec> {
        PdVcodecPwrStatW::new(self, 16)
    }
    #[doc = "Bit 17 - pd_vdu power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_vdu_pwr_stat(&mut self) -> PdVduPwrStatW<PmuPwrdnStSpec> {
        PdVduPwrStatW::new(self, 17)
    }
    #[doc = "Bit 18 - pd_rga power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_rga_pwr_stat(&mut self) -> PdRgaPwrStatW<PmuPwrdnStSpec> {
        PdRgaPwrStatW::new(self, 18)
    }
    #[doc = "Bit 19 - pd_iep power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_iep_pwr_stat(&mut self) -> PdIepPwrStatW<PmuPwrdnStSpec> {
        PdIepPwrStatW::new(self, 19)
    }
    #[doc = "Bit 20 - pd_vo power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_vo_pwr_stat(&mut self) -> PdVoPwrStatW<PmuPwrdnStSpec> {
        PdVoPwrStatW::new(self, 20)
    }
    #[doc = "Bit 22 - pd_isp0 power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_isp0_pwr_stat(&mut self) -> PdIsp0PwrStatW<PmuPwrdnStSpec> {
        PdIsp0PwrStatW::new(self, 22)
    }
    #[doc = "Bit 23 - pd_isp1 power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_isp1_pwr_stat(&mut self) -> PdIsp1PwrStatW<PmuPwrdnStSpec> {
        PdIsp1PwrStatW::new(self, 23)
    }
    #[doc = "Bit 24 - pd_hdcp power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_hdcp_pwr_stat(&mut self) -> PdHdcpPwrStatW<PmuPwrdnStSpec> {
        PdHdcpPwrStatW::new(self, 24)
    }
    #[doc = "Bit 25 - pd_gmac power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_gmac_pwr_stat(&mut self) -> PdGmacPwrStatW<PmuPwrdnStSpec> {
        PdGmacPwrStatW::new(self, 25)
    }
    #[doc = "Bit 26 - pd_emmc power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_emmc_pwr_stat(&mut self) -> PdEmmcPwrStatW<PmuPwrdnStSpec> {
        PdEmmcPwrStatW::new(self, 26)
    }
    #[doc = "Bit 27 - pd_usb3 power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_usb3_pwr_stat(&mut self) -> PdUsb3PwrStatW<PmuPwrdnStSpec> {
        PdUsb3PwrStatW::new(self, 27)
    }
    #[doc = "Bit 28 - pd_edp power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_edp_pwr_stat(&mut self) -> PdEdpPwrStatW<PmuPwrdnStSpec> {
        PdEdpPwrStatW::new(self, 28)
    }
    #[doc = "Bit 29 - pd_gic power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_gic_pwr_stat(&mut self) -> PdGicPwrStatW<PmuPwrdnStSpec> {
        PdGicPwrStatW::new(self, 29)
    }
    #[doc = "Bit 30 - pd_sd power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_sd_pwr_stat(&mut self) -> PdSdPwrStatW<PmuPwrdnStSpec> {
        PdSdPwrStatW::new(self, 30)
    }
    #[doc = "Bit 31 - pd_sdioaudio power state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_sdioaudio_pwr_stat(&mut self) -> PdSdioaudioPwrStatW<PmuPwrdnStSpec> {
        PdSdioaudioPwrStatW::new(self, 31)
    }
}
#[doc = "pmu power down status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_pwrdn_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_pwrdn_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuPwrdnStSpec;
impl crate::RegisterSpec for PmuPwrdnStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_pwrdn_st::R`](R) reader structure"]
impl crate::Readable for PmuPwrdnStSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_pwrdn_st::W`](W) writer structure"]
impl crate::Writable for PmuPwrdnStSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_PWRDN_ST to value 0"]
impl crate::Resettable for PmuPwrdnStSpec {
    const RESET_VALUE: u32 = 0;
}
