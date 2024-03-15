#[doc = "Register `PMU_PWRDN_CON` reader"]
pub type R = crate::R<PmuPwrdnConSpec>;
#[doc = "Register `PMU_PWRDN_CON` writer"]
pub type W = crate::W<PmuPwrdnConSpec>;
#[doc = "pd_a53_l0 power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdA53L0PwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdA53L0PwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdA53L0PwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_A53_L0_PWRDWN_EN` reader - pd_a53_l0 power down enable"]
pub type PdA53L0PwrdwnEnR = crate::BitReader<PdA53L0PwrdwnEn>;
impl PdA53L0PwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdA53L0PwrdwnEn {
        match self.bits {
            false => PdA53L0PwrdwnEn::B0,
            true => PdA53L0PwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdA53L0PwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdA53L0PwrdwnEn::B1
    }
}
#[doc = "Field `PD_A53_L0_PWRDWN_EN` writer - pd_a53_l0 power down enable"]
pub type PdA53L0PwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdA53L0PwrdwnEn>;
impl<'a, REG> PdA53L0PwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdA53L0PwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdA53L0PwrdwnEn::B1)
    }
}
#[doc = "pd_a53_l1 power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdA53L1Pwrdwn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdA53L1Pwrdwn> for bool {
    #[inline(always)]
    fn from(variant: PdA53L1Pwrdwn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_A53_L1_PWRDWN` reader - pd_a53_l1 power down enable"]
pub type PdA53L1PwrdwnR = crate::BitReader<PdA53L1Pwrdwn>;
impl PdA53L1PwrdwnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdA53L1Pwrdwn {
        match self.bits {
            false => PdA53L1Pwrdwn::B0,
            true => PdA53L1Pwrdwn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdA53L1Pwrdwn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdA53L1Pwrdwn::B1
    }
}
#[doc = "Field `PD_A53_L1_PWRDWN` writer - pd_a53_l1 power down enable"]
pub type PdA53L1PwrdwnW<'a, REG> = crate::BitWriter<'a, REG, PdA53L1Pwrdwn>;
impl<'a, REG> PdA53L1PwrdwnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdA53L1Pwrdwn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdA53L1Pwrdwn::B1)
    }
}
#[doc = "pd_a53_l2 power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdA53L2Pwrdwn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdA53L2Pwrdwn> for bool {
    #[inline(always)]
    fn from(variant: PdA53L2Pwrdwn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_A53_L2_PWRDWN` reader - pd_a53_l2 power down enable"]
pub type PdA53L2PwrdwnR = crate::BitReader<PdA53L2Pwrdwn>;
impl PdA53L2PwrdwnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdA53L2Pwrdwn {
        match self.bits {
            false => PdA53L2Pwrdwn::B0,
            true => PdA53L2Pwrdwn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdA53L2Pwrdwn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdA53L2Pwrdwn::B1
    }
}
#[doc = "Field `PD_A53_L2_PWRDWN` writer - pd_a53_l2 power down enable"]
pub type PdA53L2PwrdwnW<'a, REG> = crate::BitWriter<'a, REG, PdA53L2Pwrdwn>;
impl<'a, REG> PdA53L2PwrdwnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdA53L2Pwrdwn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdA53L2Pwrdwn::B1)
    }
}
#[doc = "pd_a53_l3 power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdA53L3Pwrdwn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdA53L3Pwrdwn> for bool {
    #[inline(always)]
    fn from(variant: PdA53L3Pwrdwn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_A53_L3_PWRDWN` reader - pd_a53_l3 power down enable"]
pub type PdA53L3PwrdwnR = crate::BitReader<PdA53L3Pwrdwn>;
impl PdA53L3PwrdwnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdA53L3Pwrdwn {
        match self.bits {
            false => PdA53L3Pwrdwn::B0,
            true => PdA53L3Pwrdwn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdA53L3Pwrdwn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdA53L3Pwrdwn::B1
    }
}
#[doc = "Field `PD_A53_L3_PWRDWN` writer - pd_a53_l3 power down enable"]
pub type PdA53L3PwrdwnW<'a, REG> = crate::BitWriter<'a, REG, PdA53L3Pwrdwn>;
impl<'a, REG> PdA53L3PwrdwnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdA53L3Pwrdwn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdA53L3Pwrdwn::B1)
    }
}
#[doc = "pd_a72_b0 power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdA72B0PwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdA72B0PwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdA72B0PwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_A72_B0_PWRDWN_EN` reader - pd_a72_b0 power down enable"]
pub type PdA72B0PwrdwnEnR = crate::BitReader<PdA72B0PwrdwnEn>;
impl PdA72B0PwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdA72B0PwrdwnEn {
        match self.bits {
            false => PdA72B0PwrdwnEn::B0,
            true => PdA72B0PwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdA72B0PwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdA72B0PwrdwnEn::B1
    }
}
#[doc = "Field `PD_A72_B0_PWRDWN_EN` writer - pd_a72_b0 power down enable"]
pub type PdA72B0PwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdA72B0PwrdwnEn>;
impl<'a, REG> PdA72B0PwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdA72B0PwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdA72B0PwrdwnEn::B1)
    }
}
#[doc = "pd_a72_b0 power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdA72B1PwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdA72B1PwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdA72B1PwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_A72_B1_PWRDWN_EN` reader - pd_a72_b0 power down enable"]
pub type PdA72B1PwrdwnEnR = crate::BitReader<PdA72B1PwrdwnEn>;
impl PdA72B1PwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdA72B1PwrdwnEn {
        match self.bits {
            false => PdA72B1PwrdwnEn::B0,
            true => PdA72B1PwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdA72B1PwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdA72B1PwrdwnEn::B1
    }
}
#[doc = "Field `PD_A72_B1_PWRDWN_EN` writer - pd_a72_b0 power down enable"]
pub type PdA72B1PwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdA72B1PwrdwnEn>;
impl<'a, REG> PdA72B1PwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdA72B1PwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdA72B1PwrdwnEn::B1)
    }
}
#[doc = "pd_scu_l power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdScuLPwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdScuLPwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdScuLPwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_SCU_L_PWRDWN_EN` reader - pd_scu_l power down enable"]
pub type PdScuLPwrdwnEnR = crate::BitReader<PdScuLPwrdwnEn>;
impl PdScuLPwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdScuLPwrdwnEn {
        match self.bits {
            false => PdScuLPwrdwnEn::B0,
            true => PdScuLPwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdScuLPwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdScuLPwrdwnEn::B1
    }
}
#[doc = "Field `PD_SCU_L_PWRDWN_EN` writer - pd_scu_l power down enable"]
pub type PdScuLPwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdScuLPwrdwnEn>;
impl<'a, REG> PdScuLPwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdScuLPwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdScuLPwrdwnEn::B1)
    }
}
#[doc = "pd_scu_b power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdScuBPwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdScuBPwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdScuBPwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_SCU_B_PWRDWN_EN` reader - pd_scu_b power down enable"]
pub type PdScuBPwrdwnEnR = crate::BitReader<PdScuBPwrdwnEn>;
impl PdScuBPwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdScuBPwrdwnEn {
        match self.bits {
            false => PdScuBPwrdwnEn::B0,
            true => PdScuBPwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdScuBPwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdScuBPwrdwnEn::B1
    }
}
#[doc = "Field `PD_SCU_B_PWRDWN_EN` writer - pd_scu_b power down enable"]
pub type PdScuBPwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdScuBPwrdwnEn>;
impl<'a, REG> PdScuBPwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdScuBPwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdScuBPwrdwnEn::B1)
    }
}
#[doc = "pd_tcpd0 power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdTcpd0PwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdTcpd0PwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdTcpd0PwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_TCPD0_PWRDWN_EN` reader - pd_tcpd0 power down enable"]
pub type PdTcpd0PwrdwnEnR = crate::BitReader<PdTcpd0PwrdwnEn>;
impl PdTcpd0PwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdTcpd0PwrdwnEn {
        match self.bits {
            false => PdTcpd0PwrdwnEn::B0,
            true => PdTcpd0PwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdTcpd0PwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdTcpd0PwrdwnEn::B1
    }
}
#[doc = "Field `PD_TCPD0_PWRDWN_EN` writer - pd_tcpd0 power down enable"]
pub type PdTcpd0PwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdTcpd0PwrdwnEn>;
impl<'a, REG> PdTcpd0PwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdTcpd0PwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdTcpd0PwrdwnEn::B1)
    }
}
#[doc = "pd_tcpd1 power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdTcpd1PwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdTcpd1PwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdTcpd1PwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_TCPD1_PWRDWN_EN` reader - pd_tcpd1 power down enable"]
pub type PdTcpd1PwrdwnEnR = crate::BitReader<PdTcpd1PwrdwnEn>;
impl PdTcpd1PwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdTcpd1PwrdwnEn {
        match self.bits {
            false => PdTcpd1PwrdwnEn::B0,
            true => PdTcpd1PwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdTcpd1PwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdTcpd1PwrdwnEn::B1
    }
}
#[doc = "Field `PD_TCPD1_PWRDWN_EN` writer - pd_tcpd1 power down enable"]
pub type PdTcpd1PwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdTcpd1PwrdwnEn>;
impl<'a, REG> PdTcpd1PwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdTcpd1PwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdTcpd1PwrdwnEn::B1)
    }
}
#[doc = "pd_cci power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdCciPwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdCciPwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdCciPwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_CCI_PWRDWN_EN` reader - pd_cci power down enable"]
pub type PdCciPwrdwnEnR = crate::BitReader<PdCciPwrdwnEn>;
impl PdCciPwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdCciPwrdwnEn {
        match self.bits {
            false => PdCciPwrdwnEn::B0,
            true => PdCciPwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdCciPwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdCciPwrdwnEn::B1
    }
}
#[doc = "Field `PD_CCI_PWRDWN_EN` writer - pd_cci power down enable"]
pub type PdCciPwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdCciPwrdwnEn>;
impl<'a, REG> PdCciPwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdCciPwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdCciPwrdwnEn::B1)
    }
}
#[doc = "pd_perilp power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdPerilpPwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdPerilpPwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdPerilpPwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_PERILP_PWRDWN_EN` reader - pd_perilp power down enable"]
pub type PdPerilpPwrdwnEnR = crate::BitReader<PdPerilpPwrdwnEn>;
impl PdPerilpPwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdPerilpPwrdwnEn {
        match self.bits {
            false => PdPerilpPwrdwnEn::B0,
            true => PdPerilpPwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdPerilpPwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdPerilpPwrdwnEn::B1
    }
}
#[doc = "Field `PD_PERILP_PWRDWN_EN` writer - pd_perilp power down enable"]
pub type PdPerilpPwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdPerilpPwrdwnEn>;
impl<'a, REG> PdPerilpPwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdPerilpPwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdPerilpPwrdwnEn::B1)
    }
}
#[doc = "pd_perihp power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdPerihpPwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdPerihpPwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdPerihpPwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_PERIHP_PWRDWN_EN` reader - pd_perihp power down enable"]
pub type PdPerihpPwrdwnEnR = crate::BitReader<PdPerihpPwrdwnEn>;
impl PdPerihpPwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdPerihpPwrdwnEn {
        match self.bits {
            false => PdPerihpPwrdwnEn::B0,
            true => PdPerihpPwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdPerihpPwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdPerihpPwrdwnEn::B1
    }
}
#[doc = "Field `PD_PERIHP_PWRDWN_EN` writer - pd_perihp power down enable"]
pub type PdPerihpPwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdPerihpPwrdwnEn>;
impl<'a, REG> PdPerihpPwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdPerihpPwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdPerihpPwrdwnEn::B1)
    }
}
#[doc = "pd_center power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdCenterPwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdCenterPwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdCenterPwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_CENTER_PWRDWN_EN` reader - pd_center power down enable"]
pub type PdCenterPwrdwnEnR = crate::BitReader<PdCenterPwrdwnEn>;
impl PdCenterPwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdCenterPwrdwnEn {
        match self.bits {
            false => PdCenterPwrdwnEn::B0,
            true => PdCenterPwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdCenterPwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdCenterPwrdwnEn::B1
    }
}
#[doc = "Field `PD_CENTER_PWRDWN_EN` writer - pd_center power down enable"]
pub type PdCenterPwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdCenterPwrdwnEn>;
impl<'a, REG> PdCenterPwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdCenterPwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdCenterPwrdwnEn::B1)
    }
}
#[doc = "pd_vio power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdVioPwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdVioPwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdVioPwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_VIO_PWRDWN_EN` reader - pd_vio power down enable"]
pub type PdVioPwrdwnEnR = crate::BitReader<PdVioPwrdwnEn>;
impl PdVioPwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdVioPwrdwnEn {
        match self.bits {
            false => PdVioPwrdwnEn::B0,
            true => PdVioPwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdVioPwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdVioPwrdwnEn::B1
    }
}
#[doc = "Field `PD_VIO_PWRDWN_EN` writer - pd_vio power down enable"]
pub type PdVioPwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdVioPwrdwnEn>;
impl<'a, REG> PdVioPwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdVioPwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdVioPwrdwnEn::B1)
    }
}
#[doc = "pd_gpu power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdGpuPwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdGpuPwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdGpuPwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_GPU_PWRDWN_EN` reader - pd_gpu power down enable"]
pub type PdGpuPwrdwnEnR = crate::BitReader<PdGpuPwrdwnEn>;
impl PdGpuPwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdGpuPwrdwnEn {
        match self.bits {
            false => PdGpuPwrdwnEn::B0,
            true => PdGpuPwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdGpuPwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdGpuPwrdwnEn::B1
    }
}
#[doc = "Field `PD_GPU_PWRDWN_EN` writer - pd_gpu power down enable"]
pub type PdGpuPwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdGpuPwrdwnEn>;
impl<'a, REG> PdGpuPwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdGpuPwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdGpuPwrdwnEn::B1)
    }
}
#[doc = "pd_perihp power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdVcodecPwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdVcodecPwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdVcodecPwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_VCODEC_PWRDWN_EN` reader - pd_perihp power down enable"]
pub type PdVcodecPwrdwnEnR = crate::BitReader<PdVcodecPwrdwnEn>;
impl PdVcodecPwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdVcodecPwrdwnEn {
        match self.bits {
            false => PdVcodecPwrdwnEn::B0,
            true => PdVcodecPwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdVcodecPwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdVcodecPwrdwnEn::B1
    }
}
#[doc = "Field `PD_VCODEC_PWRDWN_EN` writer - pd_perihp power down enable"]
pub type PdVcodecPwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdVcodecPwrdwnEn>;
impl<'a, REG> PdVcodecPwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdVcodecPwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdVcodecPwrdwnEn::B1)
    }
}
#[doc = "pd_vdu power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdVduPwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdVduPwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdVduPwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_VDU_PWRDWN_EN` reader - pd_vdu power down enable"]
pub type PdVduPwrdwnEnR = crate::BitReader<PdVduPwrdwnEn>;
impl PdVduPwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdVduPwrdwnEn {
        match self.bits {
            false => PdVduPwrdwnEn::B0,
            true => PdVduPwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdVduPwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdVduPwrdwnEn::B1
    }
}
#[doc = "Field `PD_VDU_PWRDWN_EN` writer - pd_vdu power down enable"]
pub type PdVduPwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdVduPwrdwnEn>;
impl<'a, REG> PdVduPwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdVduPwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdVduPwrdwnEn::B1)
    }
}
#[doc = "pd_rga power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdRgaPwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdRgaPwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdRgaPwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_RGA_PWRDWN_EN` reader - pd_rga power down enable"]
pub type PdRgaPwrdwnEnR = crate::BitReader<PdRgaPwrdwnEn>;
impl PdRgaPwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdRgaPwrdwnEn {
        match self.bits {
            false => PdRgaPwrdwnEn::B0,
            true => PdRgaPwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdRgaPwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdRgaPwrdwnEn::B1
    }
}
#[doc = "Field `PD_RGA_PWRDWN_EN` writer - pd_rga power down enable"]
pub type PdRgaPwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdRgaPwrdwnEn>;
impl<'a, REG> PdRgaPwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdRgaPwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdRgaPwrdwnEn::B1)
    }
}
#[doc = "pd_perihp power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdIepPwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdIepPwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdIepPwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_IEP_PWRDWN_EN` reader - pd_perihp power down enable"]
pub type PdIepPwrdwnEnR = crate::BitReader<PdIepPwrdwnEn>;
impl PdIepPwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdIepPwrdwnEn {
        match self.bits {
            false => PdIepPwrdwnEn::B0,
            true => PdIepPwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdIepPwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdIepPwrdwnEn::B1
    }
}
#[doc = "Field `PD_IEP_PWRDWN_EN` writer - pd_perihp power down enable"]
pub type PdIepPwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdIepPwrdwnEn>;
impl<'a, REG> PdIepPwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdIepPwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdIepPwrdwnEn::B1)
    }
}
#[doc = "pd_vo power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdVoPwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdVoPwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdVoPwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_VO_PWRDWN_EN` reader - pd_vo power down enable"]
pub type PdVoPwrdwnEnR = crate::BitReader<PdVoPwrdwnEn>;
impl PdVoPwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdVoPwrdwnEn {
        match self.bits {
            false => PdVoPwrdwnEn::B0,
            true => PdVoPwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdVoPwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdVoPwrdwnEn::B1
    }
}
#[doc = "Field `PD_VO_PWRDWN_EN` writer - pd_vo power down enable"]
pub type PdVoPwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdVoPwrdwnEn>;
impl<'a, REG> PdVoPwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdVoPwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdVoPwrdwnEn::B1)
    }
}
#[doc = "pd_isp0 power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdIsp0PwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdIsp0PwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdIsp0PwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_ISP0_PWRDWN_EN` reader - pd_isp0 power down enable"]
pub type PdIsp0PwrdwnEnR = crate::BitReader<PdIsp0PwrdwnEn>;
impl PdIsp0PwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdIsp0PwrdwnEn {
        match self.bits {
            false => PdIsp0PwrdwnEn::B0,
            true => PdIsp0PwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdIsp0PwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdIsp0PwrdwnEn::B1
    }
}
#[doc = "Field `PD_ISP0_PWRDWN_EN` writer - pd_isp0 power down enable"]
pub type PdIsp0PwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdIsp0PwrdwnEn>;
impl<'a, REG> PdIsp0PwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdIsp0PwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdIsp0PwrdwnEn::B1)
    }
}
#[doc = "pd_isp1 power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdIsp1PwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdIsp1PwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdIsp1PwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_ISP1_PWRDWN_EN` reader - pd_isp1 power down enable"]
pub type PdIsp1PwrdwnEnR = crate::BitReader<PdIsp1PwrdwnEn>;
impl PdIsp1PwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdIsp1PwrdwnEn {
        match self.bits {
            false => PdIsp1PwrdwnEn::B0,
            true => PdIsp1PwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdIsp1PwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdIsp1PwrdwnEn::B1
    }
}
#[doc = "Field `PD_ISP1_PWRDWN_EN` writer - pd_isp1 power down enable"]
pub type PdIsp1PwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdIsp1PwrdwnEn>;
impl<'a, REG> PdIsp1PwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdIsp1PwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdIsp1PwrdwnEn::B1)
    }
}
#[doc = "pd_hdcp power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdHdcpPwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdHdcpPwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdHdcpPwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_HDCP_PWRDWN_EN` reader - pd_hdcp power down enable"]
pub type PdHdcpPwrdwnEnR = crate::BitReader<PdHdcpPwrdwnEn>;
impl PdHdcpPwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdHdcpPwrdwnEn {
        match self.bits {
            false => PdHdcpPwrdwnEn::B0,
            true => PdHdcpPwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdHdcpPwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdHdcpPwrdwnEn::B1
    }
}
#[doc = "Field `PD_HDCP_PWRDWN_EN` writer - pd_hdcp power down enable"]
pub type PdHdcpPwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdHdcpPwrdwnEn>;
impl<'a, REG> PdHdcpPwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdHdcpPwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdHdcpPwrdwnEn::B1)
    }
}
#[doc = "pd_gmac power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdGmacPwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdGmacPwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdGmacPwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_GMAC_PWRDWN_EN` reader - pd_gmac power down enable"]
pub type PdGmacPwrdwnEnR = crate::BitReader<PdGmacPwrdwnEn>;
impl PdGmacPwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdGmacPwrdwnEn {
        match self.bits {
            false => PdGmacPwrdwnEn::B0,
            true => PdGmacPwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdGmacPwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdGmacPwrdwnEn::B1
    }
}
#[doc = "Field `PD_GMAC_PWRDWN_EN` writer - pd_gmac power down enable"]
pub type PdGmacPwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdGmacPwrdwnEn>;
impl<'a, REG> PdGmacPwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdGmacPwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdGmacPwrdwnEn::B1)
    }
}
#[doc = "pd_emmc power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdEmmcPwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdEmmcPwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdEmmcPwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_EMMC_PWRDWN_EN` reader - pd_emmc power down enable"]
pub type PdEmmcPwrdwnEnR = crate::BitReader<PdEmmcPwrdwnEn>;
impl PdEmmcPwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdEmmcPwrdwnEn {
        match self.bits {
            false => PdEmmcPwrdwnEn::B0,
            true => PdEmmcPwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdEmmcPwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdEmmcPwrdwnEn::B1
    }
}
#[doc = "Field `PD_EMMC_PWRDWN_EN` writer - pd_emmc power down enable"]
pub type PdEmmcPwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdEmmcPwrdwnEn>;
impl<'a, REG> PdEmmcPwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdEmmcPwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdEmmcPwrdwnEn::B1)
    }
}
#[doc = "pd_usb3 power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdUsb3PwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdUsb3PwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdUsb3PwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_USB3_PWRDWN_EN` reader - pd_usb3 power down enable"]
pub type PdUsb3PwrdwnEnR = crate::BitReader<PdUsb3PwrdwnEn>;
impl PdUsb3PwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdUsb3PwrdwnEn {
        match self.bits {
            false => PdUsb3PwrdwnEn::B0,
            true => PdUsb3PwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdUsb3PwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdUsb3PwrdwnEn::B1
    }
}
#[doc = "Field `PD_USB3_PWRDWN_EN` writer - pd_usb3 power down enable"]
pub type PdUsb3PwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdUsb3PwrdwnEn>;
impl<'a, REG> PdUsb3PwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdUsb3PwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdUsb3PwrdwnEn::B1)
    }
}
#[doc = "pd_edp power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdEdpPwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdEdpPwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdEdpPwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_EDP_PWRDWN_EN` reader - pd_edp power down enable"]
pub type PdEdpPwrdwnEnR = crate::BitReader<PdEdpPwrdwnEn>;
impl PdEdpPwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdEdpPwrdwnEn {
        match self.bits {
            false => PdEdpPwrdwnEn::B0,
            true => PdEdpPwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdEdpPwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdEdpPwrdwnEn::B1
    }
}
#[doc = "Field `PD_EDP_PWRDWN_EN` writer - pd_edp power down enable"]
pub type PdEdpPwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdEdpPwrdwnEn>;
impl<'a, REG> PdEdpPwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdEdpPwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdEdpPwrdwnEn::B1)
    }
}
#[doc = "pd_gic power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdGicPwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdGicPwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdGicPwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_GIC_PWRDWN_EN` reader - pd_gic power down enable"]
pub type PdGicPwrdwnEnR = crate::BitReader<PdGicPwrdwnEn>;
impl PdGicPwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdGicPwrdwnEn {
        match self.bits {
            false => PdGicPwrdwnEn::B0,
            true => PdGicPwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdGicPwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdGicPwrdwnEn::B1
    }
}
#[doc = "Field `PD_GIC_PWRDWN_EN` writer - pd_gic power down enable"]
pub type PdGicPwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdGicPwrdwnEn>;
impl<'a, REG> PdGicPwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdGicPwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdGicPwrdwnEn::B1)
    }
}
#[doc = "pd_sd power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdSdPwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdSdPwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdSdPwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_SD_PWRDWN_EN` reader - pd_sd power down enable"]
pub type PdSdPwrdwnEnR = crate::BitReader<PdSdPwrdwnEn>;
impl PdSdPwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdSdPwrdwnEn {
        match self.bits {
            false => PdSdPwrdwnEn::B0,
            true => PdSdPwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdSdPwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdSdPwrdwnEn::B1
    }
}
#[doc = "Field `PD_SD_PWRDWN_EN` writer - pd_sd power down enable"]
pub type PdSdPwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdSdPwrdwnEn>;
impl<'a, REG> PdSdPwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdSdPwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdSdPwrdwnEn::B1)
    }
}
#[doc = "pd_sdioaudio power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdSdioaudioPwrdwnEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PdSdioaudioPwrdwnEn> for bool {
    #[inline(always)]
    fn from(variant: PdSdioaudioPwrdwnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_SDIOAUDIO_PWRDWN_EN` reader - pd_sdioaudio power down enable"]
pub type PdSdioaudioPwrdwnEnR = crate::BitReader<PdSdioaudioPwrdwnEn>;
impl PdSdioaudioPwrdwnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdSdioaudioPwrdwnEn {
        match self.bits {
            false => PdSdioaudioPwrdwnEn::B0,
            true => PdSdioaudioPwrdwnEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdSdioaudioPwrdwnEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdSdioaudioPwrdwnEn::B1
    }
}
#[doc = "Field `PD_SDIOAUDIO_PWRDWN_EN` writer - pd_sdioaudio power down enable"]
pub type PdSdioaudioPwrdwnEnW<'a, REG> = crate::BitWriter<'a, REG, PdSdioaudioPwrdwnEn>;
impl<'a, REG> PdSdioaudioPwrdwnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdSdioaudioPwrdwnEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdSdioaudioPwrdwnEn::B1)
    }
}
impl R {
    #[doc = "Bit 0 - pd_a53_l0 power down enable"]
    #[inline(always)]
    pub fn pd_a53_l0_pwrdwn_en(&self) -> PdA53L0PwrdwnEnR {
        PdA53L0PwrdwnEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pd_a53_l1 power down enable"]
    #[inline(always)]
    pub fn pd_a53_l1_pwrdwn(&self) -> PdA53L1PwrdwnR {
        PdA53L1PwrdwnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pd_a53_l2 power down enable"]
    #[inline(always)]
    pub fn pd_a53_l2_pwrdwn(&self) -> PdA53L2PwrdwnR {
        PdA53L2PwrdwnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - pd_a53_l3 power down enable"]
    #[inline(always)]
    pub fn pd_a53_l3_pwrdwn(&self) -> PdA53L3PwrdwnR {
        PdA53L3PwrdwnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - pd_a72_b0 power down enable"]
    #[inline(always)]
    pub fn pd_a72_b0_pwrdwn_en(&self) -> PdA72B0PwrdwnEnR {
        PdA72B0PwrdwnEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - pd_a72_b0 power down enable"]
    #[inline(always)]
    pub fn pd_a72_b1_pwrdwn_en(&self) -> PdA72B1PwrdwnEnR {
        PdA72B1PwrdwnEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pd_scu_l power down enable"]
    #[inline(always)]
    pub fn pd_scu_l_pwrdwn_en(&self) -> PdScuLPwrdwnEnR {
        PdScuLPwrdwnEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - pd_scu_b power down enable"]
    #[inline(always)]
    pub fn pd_scu_b_pwrdwn_en(&self) -> PdScuBPwrdwnEnR {
        PdScuBPwrdwnEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - pd_tcpd0 power down enable"]
    #[inline(always)]
    pub fn pd_tcpd0_pwrdwn_en(&self) -> PdTcpd0PwrdwnEnR {
        PdTcpd0PwrdwnEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - pd_tcpd1 power down enable"]
    #[inline(always)]
    pub fn pd_tcpd1_pwrdwn_en(&self) -> PdTcpd1PwrdwnEnR {
        PdTcpd1PwrdwnEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - pd_cci power down enable"]
    #[inline(always)]
    pub fn pd_cci_pwrdwn_en(&self) -> PdCciPwrdwnEnR {
        PdCciPwrdwnEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - pd_perilp power down enable"]
    #[inline(always)]
    pub fn pd_perilp_pwrdwn_en(&self) -> PdPerilpPwrdwnEnR {
        PdPerilpPwrdwnEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - pd_perihp power down enable"]
    #[inline(always)]
    pub fn pd_perihp_pwrdwn_en(&self) -> PdPerihpPwrdwnEnR {
        PdPerihpPwrdwnEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - pd_center power down enable"]
    #[inline(always)]
    pub fn pd_center_pwrdwn_en(&self) -> PdCenterPwrdwnEnR {
        PdCenterPwrdwnEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - pd_vio power down enable"]
    #[inline(always)]
    pub fn pd_vio_pwrdwn_en(&self) -> PdVioPwrdwnEnR {
        PdVioPwrdwnEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - pd_gpu power down enable"]
    #[inline(always)]
    pub fn pd_gpu_pwrdwn_en(&self) -> PdGpuPwrdwnEnR {
        PdGpuPwrdwnEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - pd_perihp power down enable"]
    #[inline(always)]
    pub fn pd_vcodec_pwrdwn_en(&self) -> PdVcodecPwrdwnEnR {
        PdVcodecPwrdwnEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - pd_vdu power down enable"]
    #[inline(always)]
    pub fn pd_vdu_pwrdwn_en(&self) -> PdVduPwrdwnEnR {
        PdVduPwrdwnEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - pd_rga power down enable"]
    #[inline(always)]
    pub fn pd_rga_pwrdwn_en(&self) -> PdRgaPwrdwnEnR {
        PdRgaPwrdwnEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - pd_perihp power down enable"]
    #[inline(always)]
    pub fn pd_iep_pwrdwn_en(&self) -> PdIepPwrdwnEnR {
        PdIepPwrdwnEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - pd_vo power down enable"]
    #[inline(always)]
    pub fn pd_vo_pwrdwn_en(&self) -> PdVoPwrdwnEnR {
        PdVoPwrdwnEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - pd_isp0 power down enable"]
    #[inline(always)]
    pub fn pd_isp0_pwrdwn_en(&self) -> PdIsp0PwrdwnEnR {
        PdIsp0PwrdwnEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - pd_isp1 power down enable"]
    #[inline(always)]
    pub fn pd_isp1_pwrdwn_en(&self) -> PdIsp1PwrdwnEnR {
        PdIsp1PwrdwnEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - pd_hdcp power down enable"]
    #[inline(always)]
    pub fn pd_hdcp_pwrdwn_en(&self) -> PdHdcpPwrdwnEnR {
        PdHdcpPwrdwnEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - pd_gmac power down enable"]
    #[inline(always)]
    pub fn pd_gmac_pwrdwn_en(&self) -> PdGmacPwrdwnEnR {
        PdGmacPwrdwnEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - pd_emmc power down enable"]
    #[inline(always)]
    pub fn pd_emmc_pwrdwn_en(&self) -> PdEmmcPwrdwnEnR {
        PdEmmcPwrdwnEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - pd_usb3 power down enable"]
    #[inline(always)]
    pub fn pd_usb3_pwrdwn_en(&self) -> PdUsb3PwrdwnEnR {
        PdUsb3PwrdwnEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - pd_edp power down enable"]
    #[inline(always)]
    pub fn pd_edp_pwrdwn_en(&self) -> PdEdpPwrdwnEnR {
        PdEdpPwrdwnEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - pd_gic power down enable"]
    #[inline(always)]
    pub fn pd_gic_pwrdwn_en(&self) -> PdGicPwrdwnEnR {
        PdGicPwrdwnEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - pd_sd power down enable"]
    #[inline(always)]
    pub fn pd_sd_pwrdwn_en(&self) -> PdSdPwrdwnEnR {
        PdSdPwrdwnEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - pd_sdioaudio power down enable"]
    #[inline(always)]
    pub fn pd_sdioaudio_pwrdwn_en(&self) -> PdSdioaudioPwrdwnEnR {
        PdSdioaudioPwrdwnEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - pd_a53_l0 power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_a53_l0_pwrdwn_en(&mut self) -> PdA53L0PwrdwnEnW<PmuPwrdnConSpec> {
        PdA53L0PwrdwnEnW::new(self, 0)
    }
    #[doc = "Bit 1 - pd_a53_l1 power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_a53_l1_pwrdwn(&mut self) -> PdA53L1PwrdwnW<PmuPwrdnConSpec> {
        PdA53L1PwrdwnW::new(self, 1)
    }
    #[doc = "Bit 2 - pd_a53_l2 power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_a53_l2_pwrdwn(&mut self) -> PdA53L2PwrdwnW<PmuPwrdnConSpec> {
        PdA53L2PwrdwnW::new(self, 2)
    }
    #[doc = "Bit 3 - pd_a53_l3 power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_a53_l3_pwrdwn(&mut self) -> PdA53L3PwrdwnW<PmuPwrdnConSpec> {
        PdA53L3PwrdwnW::new(self, 3)
    }
    #[doc = "Bit 4 - pd_a72_b0 power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_a72_b0_pwrdwn_en(&mut self) -> PdA72B0PwrdwnEnW<PmuPwrdnConSpec> {
        PdA72B0PwrdwnEnW::new(self, 4)
    }
    #[doc = "Bit 5 - pd_a72_b0 power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_a72_b1_pwrdwn_en(&mut self) -> PdA72B1PwrdwnEnW<PmuPwrdnConSpec> {
        PdA72B1PwrdwnEnW::new(self, 5)
    }
    #[doc = "Bit 6 - pd_scu_l power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_scu_l_pwrdwn_en(&mut self) -> PdScuLPwrdwnEnW<PmuPwrdnConSpec> {
        PdScuLPwrdwnEnW::new(self, 6)
    }
    #[doc = "Bit 7 - pd_scu_b power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_scu_b_pwrdwn_en(&mut self) -> PdScuBPwrdwnEnW<PmuPwrdnConSpec> {
        PdScuBPwrdwnEnW::new(self, 7)
    }
    #[doc = "Bit 8 - pd_tcpd0 power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_tcpd0_pwrdwn_en(&mut self) -> PdTcpd0PwrdwnEnW<PmuPwrdnConSpec> {
        PdTcpd0PwrdwnEnW::new(self, 8)
    }
    #[doc = "Bit 9 - pd_tcpd1 power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_tcpd1_pwrdwn_en(&mut self) -> PdTcpd1PwrdwnEnW<PmuPwrdnConSpec> {
        PdTcpd1PwrdwnEnW::new(self, 9)
    }
    #[doc = "Bit 10 - pd_cci power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_cci_pwrdwn_en(&mut self) -> PdCciPwrdwnEnW<PmuPwrdnConSpec> {
        PdCciPwrdwnEnW::new(self, 10)
    }
    #[doc = "Bit 11 - pd_perilp power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_perilp_pwrdwn_en(&mut self) -> PdPerilpPwrdwnEnW<PmuPwrdnConSpec> {
        PdPerilpPwrdwnEnW::new(self, 11)
    }
    #[doc = "Bit 12 - pd_perihp power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_perihp_pwrdwn_en(&mut self) -> PdPerihpPwrdwnEnW<PmuPwrdnConSpec> {
        PdPerihpPwrdwnEnW::new(self, 12)
    }
    #[doc = "Bit 13 - pd_center power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_center_pwrdwn_en(&mut self) -> PdCenterPwrdwnEnW<PmuPwrdnConSpec> {
        PdCenterPwrdwnEnW::new(self, 13)
    }
    #[doc = "Bit 14 - pd_vio power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_vio_pwrdwn_en(&mut self) -> PdVioPwrdwnEnW<PmuPwrdnConSpec> {
        PdVioPwrdwnEnW::new(self, 14)
    }
    #[doc = "Bit 15 - pd_gpu power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_gpu_pwrdwn_en(&mut self) -> PdGpuPwrdwnEnW<PmuPwrdnConSpec> {
        PdGpuPwrdwnEnW::new(self, 15)
    }
    #[doc = "Bit 16 - pd_perihp power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_vcodec_pwrdwn_en(&mut self) -> PdVcodecPwrdwnEnW<PmuPwrdnConSpec> {
        PdVcodecPwrdwnEnW::new(self, 16)
    }
    #[doc = "Bit 17 - pd_vdu power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_vdu_pwrdwn_en(&mut self) -> PdVduPwrdwnEnW<PmuPwrdnConSpec> {
        PdVduPwrdwnEnW::new(self, 17)
    }
    #[doc = "Bit 18 - pd_rga power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_rga_pwrdwn_en(&mut self) -> PdRgaPwrdwnEnW<PmuPwrdnConSpec> {
        PdRgaPwrdwnEnW::new(self, 18)
    }
    #[doc = "Bit 19 - pd_perihp power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_iep_pwrdwn_en(&mut self) -> PdIepPwrdwnEnW<PmuPwrdnConSpec> {
        PdIepPwrdwnEnW::new(self, 19)
    }
    #[doc = "Bit 20 - pd_vo power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_vo_pwrdwn_en(&mut self) -> PdVoPwrdwnEnW<PmuPwrdnConSpec> {
        PdVoPwrdwnEnW::new(self, 20)
    }
    #[doc = "Bit 22 - pd_isp0 power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_isp0_pwrdwn_en(&mut self) -> PdIsp0PwrdwnEnW<PmuPwrdnConSpec> {
        PdIsp0PwrdwnEnW::new(self, 22)
    }
    #[doc = "Bit 23 - pd_isp1 power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_isp1_pwrdwn_en(&mut self) -> PdIsp1PwrdwnEnW<PmuPwrdnConSpec> {
        PdIsp1PwrdwnEnW::new(self, 23)
    }
    #[doc = "Bit 24 - pd_hdcp power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_hdcp_pwrdwn_en(&mut self) -> PdHdcpPwrdwnEnW<PmuPwrdnConSpec> {
        PdHdcpPwrdwnEnW::new(self, 24)
    }
    #[doc = "Bit 25 - pd_gmac power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_gmac_pwrdwn_en(&mut self) -> PdGmacPwrdwnEnW<PmuPwrdnConSpec> {
        PdGmacPwrdwnEnW::new(self, 25)
    }
    #[doc = "Bit 26 - pd_emmc power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_emmc_pwrdwn_en(&mut self) -> PdEmmcPwrdwnEnW<PmuPwrdnConSpec> {
        PdEmmcPwrdwnEnW::new(self, 26)
    }
    #[doc = "Bit 27 - pd_usb3 power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_usb3_pwrdwn_en(&mut self) -> PdUsb3PwrdwnEnW<PmuPwrdnConSpec> {
        PdUsb3PwrdwnEnW::new(self, 27)
    }
    #[doc = "Bit 28 - pd_edp power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_edp_pwrdwn_en(&mut self) -> PdEdpPwrdwnEnW<PmuPwrdnConSpec> {
        PdEdpPwrdwnEnW::new(self, 28)
    }
    #[doc = "Bit 29 - pd_gic power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_gic_pwrdwn_en(&mut self) -> PdGicPwrdwnEnW<PmuPwrdnConSpec> {
        PdGicPwrdwnEnW::new(self, 29)
    }
    #[doc = "Bit 30 - pd_sd power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_sd_pwrdwn_en(&mut self) -> PdSdPwrdwnEnW<PmuPwrdnConSpec> {
        PdSdPwrdwnEnW::new(self, 30)
    }
    #[doc = "Bit 31 - pd_sdioaudio power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd_sdioaudio_pwrdwn_en(&mut self) -> PdSdioaudioPwrdwnEnW<PmuPwrdnConSpec> {
        PdSdioaudioPwrdwnEnW::new(self, 31)
    }
}
#[doc = "pmu power down configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_pwrdn_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_pwrdn_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuPwrdnConSpec;
impl crate::RegisterSpec for PmuPwrdnConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_pwrdn_con::R`](R) reader structure"]
impl crate::Readable for PmuPwrdnConSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_pwrdn_con::W`](W) writer structure"]
impl crate::Writable for PmuPwrdnConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_PWRDN_CON to value 0"]
impl crate::Resettable for PmuPwrdnConSpec {
    const RESET_VALUE: u32 = 0;
}
