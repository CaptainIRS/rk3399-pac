#[doc = "Register `PMU_SFT_CON` reader"]
pub type R = crate::R<PmuSftConSpec>;
#[doc = "Register `PMU_SFT_CON` writer"]
pub type W = crate::W<PmuSftConSpec>;
#[doc = "Field `WAKEUP_SFT` reader - software wakeup request bit\n\nA 0 to 1 pulse posedge will wakeup pmu when in low power mode"]
pub type WakeupSftR = crate::BitReader;
#[doc = "Field `WAKEUP_SFT` writer - software wakeup request bit\n\nA 0 to 1 pulse posedge will wakeup pmu when in low power mode"]
pub type WakeupSftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUT_CLAMP_CFG` reader - software control of input clamp signal"]
pub type InputClampCfgR = crate::BitReader;
#[doc = "Field `INPUT_CLAMP_CFG` writer - software control of input clamp signal"]
pub type InputClampCfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "osc disable configure by software\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OscDisableCfg {
    #[doc = "1: disable osc"]
    B1 = 1,
    #[doc = "0: enable psc"]
    B0 = 0,
}
impl From<OscDisableCfg> for bool {
    #[inline(always)]
    fn from(variant: OscDisableCfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSC_DISABLE_CFG` reader - osc disable configure by software"]
pub type OscDisableCfgR = crate::BitReader<OscDisableCfg>;
impl OscDisableCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OscDisableCfg {
        match self.bits {
            true => OscDisableCfg::B1,
            false => OscDisableCfg::B0,
        }
    }
    #[doc = "disable osc"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == OscDisableCfg::B1
    }
    #[doc = "enable psc"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == OscDisableCfg::B0
    }
}
#[doc = "Field `OSC_DISABLE_CFG` writer - osc disable configure by software"]
pub type OscDisableCfgW<'a, REG> = crate::BitWriter<'a, REG, OscDisableCfg>;
impl<'a, REG> OscDisableCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable osc"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(OscDisableCfg::B1)
    }
    #[doc = "enable psc"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(OscDisableCfg::B0)
    }
}
#[doc = "pd_pmu low frequency mode configure by software\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmuLfEnaCfg {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PmuLfEnaCfg> for bool {
    #[inline(always)]
    fn from(variant: PmuLfEnaCfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMU_LF_ENA_CFG` reader - pd_pmu low frequency mode configure by software"]
pub type PmuLfEnaCfgR = crate::BitReader<PmuLfEnaCfg>;
impl PmuLfEnaCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PmuLfEnaCfg {
        match self.bits {
            false => PmuLfEnaCfg::B0,
            true => PmuLfEnaCfg::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PmuLfEnaCfg::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PmuLfEnaCfg::B1
    }
}
#[doc = "Field `PMU_LF_ENA_CFG` writer - pd_pmu low frequency mode configure by software"]
pub type PmuLfEnaCfgW<'a, REG> = crate::BitWriter<'a, REG, PmuLfEnaCfg>;
impl<'a, REG> PmuLfEnaCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PmuLfEnaCfg::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PmuLfEnaCfg::B1)
    }
}
#[doc = "pd_alive low frequency mode configure by software\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AliveLfEnaCfg {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<AliveLfEnaCfg> for bool {
    #[inline(always)]
    fn from(variant: AliveLfEnaCfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIVE_LF_ENA_CFG` reader - pd_alive low frequency mode configure by software"]
pub type AliveLfEnaCfgR = crate::BitReader<AliveLfEnaCfg>;
impl AliveLfEnaCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AliveLfEnaCfg {
        match self.bits {
            false => AliveLfEnaCfg::B0,
            true => AliveLfEnaCfg::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AliveLfEnaCfg::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AliveLfEnaCfg::B1
    }
}
#[doc = "Field `ALIVE_LF_ENA_CFG` writer - pd_alive low frequency mode configure by software"]
pub type AliveLfEnaCfgW<'a, REG> = crate::BitWriter<'a, REG, AliveLfEnaCfg>;
impl<'a, REG> AliveLfEnaCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AliveLfEnaCfg::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AliveLfEnaCfg::B1)
    }
}
#[doc = "Field `PMU_24M_ENA_CFG` reader - configure PD_PMU use 24M clock"]
pub type Pmu24mEnaCfgR = crate::BitReader;
#[doc = "Field `PMU_24M_ENA_CFG` writer - configure PD_PMU use 24M clock"]
pub type Pmu24mEnaCfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "dbg powered up of pd_a53_l0 enable when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgpwrdupL0Cfg {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<DbgpwrdupL0Cfg> for bool {
    #[inline(always)]
    fn from(variant: DbgpwrdupL0Cfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGPWRDUP_L0_CFG` reader - dbg powered up of pd_a53_l0 enable when in power mode"]
pub type DbgpwrdupL0CfgR = crate::BitReader<DbgpwrdupL0Cfg>;
impl DbgpwrdupL0CfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgpwrdupL0Cfg {
        match self.bits {
            false => DbgpwrdupL0Cfg::B0,
            true => DbgpwrdupL0Cfg::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DbgpwrdupL0Cfg::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DbgpwrdupL0Cfg::B1
    }
}
#[doc = "Field `DBGPWRDUP_L0_CFG` writer - dbg powered up of pd_a53_l0 enable when in power mode"]
pub type DbgpwrdupL0CfgW<'a, REG> = crate::BitWriter<'a, REG, DbgpwrdupL0Cfg>;
impl<'a, REG> DbgpwrdupL0CfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgpwrdupL0Cfg::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgpwrdupL0Cfg::B1)
    }
}
#[doc = "Field `WAKEUP_SFT_M0` reader - m0 configure this bit to wakeup PMU state machine."]
pub type WakeupSftM0R = crate::BitReader;
#[doc = "Field `WAKEUP_SFT_M0` writer - m0 configure this bit to wakeup PMU state machine."]
pub type WakeupSftM0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ddrctl idle request configure\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddrctl0CSysreqCfg {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Ddrctl0CSysreqCfg> for bool {
    #[inline(always)]
    fn from(variant: Ddrctl0CSysreqCfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDRCTL0_C_SYSREQ_CFG` reader - ddrctl idle request configure"]
pub type Ddrctl0CSysreqCfgR = crate::BitReader<Ddrctl0CSysreqCfg>;
impl Ddrctl0CSysreqCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddrctl0CSysreqCfg {
        match self.bits {
            false => Ddrctl0CSysreqCfg::B0,
            true => Ddrctl0CSysreqCfg::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ddrctl0CSysreqCfg::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ddrctl0CSysreqCfg::B1
    }
}
#[doc = "Field `DDRCTL0_C_SYSREQ_CFG` writer - ddrctl idle request configure"]
pub type Ddrctl0CSysreqCfgW<'a, REG> = crate::BitWriter<'a, REG, Ddrctl0CSysreqCfg>;
impl<'a, REG> Ddrctl0CSysreqCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ddrctl0CSysreqCfg::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ddrctl0CSysreqCfg::B1)
    }
}
#[doc = "ddr0 io retention configure by software\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddr0IoRetCfg {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Ddr0IoRetCfg> for bool {
    #[inline(always)]
    fn from(variant: Ddr0IoRetCfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDR0_IO_RET_CFG` reader - ddr0 io retention configure by software"]
pub type Ddr0IoRetCfgR = crate::BitReader<Ddr0IoRetCfg>;
impl Ddr0IoRetCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddr0IoRetCfg {
        match self.bits {
            false => Ddr0IoRetCfg::B0,
            true => Ddr0IoRetCfg::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ddr0IoRetCfg::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ddr0IoRetCfg::B1
    }
}
#[doc = "Field `DDR0_IO_RET_CFG` writer - ddr0 io retention configure by software"]
pub type Ddr0IoRetCfgW<'a, REG> = crate::BitWriter<'a, REG, Ddr0IoRetCfg>;
impl<'a, REG> Ddr0IoRetCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ddr0IoRetCfg::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ddr0IoRetCfg::B1)
    }
}
#[doc = "ddrctl1 idle request configure\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddrctl1CSysreqCfg {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Ddrctl1CSysreqCfg> for bool {
    #[inline(always)]
    fn from(variant: Ddrctl1CSysreqCfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDRCTL1_C_SYSREQ_CFG` reader - ddrctl1 idle request configure"]
pub type Ddrctl1CSysreqCfgR = crate::BitReader<Ddrctl1CSysreqCfg>;
impl Ddrctl1CSysreqCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddrctl1CSysreqCfg {
        match self.bits {
            false => Ddrctl1CSysreqCfg::B0,
            true => Ddrctl1CSysreqCfg::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ddrctl1CSysreqCfg::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ddrctl1CSysreqCfg::B1
    }
}
#[doc = "Field `DDRCTL1_C_SYSREQ_CFG` writer - ddrctl1 idle request configure"]
pub type Ddrctl1CSysreqCfgW<'a, REG> = crate::BitWriter<'a, REG, Ddrctl1CSysreqCfg>;
impl<'a, REG> Ddrctl1CSysreqCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ddrctl1CSysreqCfg::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ddrctl1CSysreqCfg::B1)
    }
}
#[doc = "ddr1 io retention configure by software\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddr1IoRetCfg {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Ddr1IoRetCfg> for bool {
    #[inline(always)]
    fn from(variant: Ddr1IoRetCfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDR1_IO_RET_CFG` reader - ddr1 io retention configure by software"]
pub type Ddr1IoRetCfgR = crate::BitReader<Ddr1IoRetCfg>;
impl Ddr1IoRetCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddr1IoRetCfg {
        match self.bits {
            false => Ddr1IoRetCfg::B0,
            true => Ddr1IoRetCfg::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ddr1IoRetCfg::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ddr1IoRetCfg::B1
    }
}
#[doc = "Field `DDR1_IO_RET_CFG` writer - ddr1 io retention configure by software"]
pub type Ddr1IoRetCfgW<'a, REG> = crate::BitWriter<'a, REG, Ddr1IoRetCfg>;
impl<'a, REG> Ddr1IoRetCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ddr1IoRetCfg::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ddr1IoRetCfg::B1)
    }
}
#[doc = "dbg powered up of pd_a72_b0 enable when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgpwrdupB0Cfg {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<DbgpwrdupB0Cfg> for bool {
    #[inline(always)]
    fn from(variant: DbgpwrdupB0Cfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGPWRDUP_B0_CFG` reader - dbg powered up of pd_a72_b0 enable when in power mode"]
pub type DbgpwrdupB0CfgR = crate::BitReader<DbgpwrdupB0Cfg>;
impl DbgpwrdupB0CfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgpwrdupB0Cfg {
        match self.bits {
            false => DbgpwrdupB0Cfg::B0,
            true => DbgpwrdupB0Cfg::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DbgpwrdupB0Cfg::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DbgpwrdupB0Cfg::B1
    }
}
#[doc = "Field `DBGPWRDUP_B0_CFG` writer - dbg powered up of pd_a72_b0 enable when in power mode"]
pub type DbgpwrdupB0CfgW<'a, REG> = crate::BitWriter<'a, REG, DbgpwrdupB0Cfg>;
impl<'a, REG> DbgpwrdupB0CfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgpwrdupB0Cfg::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgpwrdupB0Cfg::B1)
    }
}
#[doc = "dbgnopowerdown function of cluster_l enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DbgnopwrdwnLEnable {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<DbgnopwrdwnLEnable> for u8 {
    #[inline(always)]
    fn from(variant: DbgnopwrdwnLEnable) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DbgnopwrdwnLEnable {
    type Ux = u8;
}
#[doc = "Field `DBGNOPWRDWN_L_ENABLE` reader - dbgnopowerdown function of cluster_l enable"]
pub type DbgnopwrdwnLEnableR = crate::FieldReader<DbgnopwrdwnLEnable>;
impl DbgnopwrdwnLEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DbgnopwrdwnLEnable> {
        match self.bits {
            0 => Some(DbgnopwrdwnLEnable::B0),
            1 => Some(DbgnopwrdwnLEnable::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DbgnopwrdwnLEnable::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DbgnopwrdwnLEnable::B1
    }
}
#[doc = "Field `DBGNOPWRDWN_L_ENABLE` writer - dbgnopowerdown function of cluster_l enable"]
pub type DbgnopwrdwnLEnableW<'a, REG> = crate::FieldWriter<'a, REG, 4, DbgnopwrdwnLEnable>;
impl<'a, REG> DbgnopwrdwnLEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgnopwrdwnLEnable::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgnopwrdwnLEnable::B1)
    }
}
#[doc = "dbg powered up request function of cluster_l enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgpwrupreqLEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<DbgpwrupreqLEn> for bool {
    #[inline(always)]
    fn from(variant: DbgpwrupreqLEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGPWRUPREQ_L_EN` reader - dbg powered up request function of cluster_l enable"]
pub type DbgpwrupreqLEnR = crate::BitReader<DbgpwrupreqLEn>;
impl DbgpwrupreqLEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgpwrupreqLEn {
        match self.bits {
            false => DbgpwrupreqLEn::B0,
            true => DbgpwrupreqLEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DbgpwrupreqLEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DbgpwrupreqLEn::B1
    }
}
#[doc = "Field `DBGPWRUPREQ_L_EN` writer - dbg powered up request function of cluster_l enable"]
pub type DbgpwrupreqLEnW<'a, REG> = crate::BitWriter<'a, REG, DbgpwrupreqLEn>;
impl<'a, REG> DbgpwrupreqLEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgpwrupreqLEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgpwrupreqLEn::B1)
    }
}
#[doc = "cluster_l clock source gating configure\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClusterLClkSrcGatingCfg {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClusterLClkSrcGatingCfg> for bool {
    #[inline(always)]
    fn from(variant: ClusterLClkSrcGatingCfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLUSTER_L_CLK_SRC_GATING_CFG` reader - cluster_l clock source gating configure"]
pub type ClusterLClkSrcGatingCfgR = crate::BitReader<ClusterLClkSrcGatingCfg>;
impl ClusterLClkSrcGatingCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClusterLClkSrcGatingCfg {
        match self.bits {
            false => ClusterLClkSrcGatingCfg::B0,
            true => ClusterLClkSrcGatingCfg::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClusterLClkSrcGatingCfg::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClusterLClkSrcGatingCfg::B1
    }
}
#[doc = "Field `CLUSTER_L_CLK_SRC_GATING_CFG` writer - cluster_l clock source gating configure"]
pub type ClusterLClkSrcGatingCfgW<'a, REG> = crate::BitWriter<'a, REG, ClusterLClkSrcGatingCfg>;
impl<'a, REG> ClusterLClkSrcGatingCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClusterLClkSrcGatingCfg::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClusterLClkSrcGatingCfg::B1)
    }
}
#[doc = "send l2 flush request to cluster_l by software\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L2flushreqClusterL {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<L2flushreqClusterL> for bool {
    #[inline(always)]
    fn from(variant: L2flushreqClusterL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L2FLUSHREQ_CLUSTER_L` reader - send l2 flush request to cluster_l by software"]
pub type L2flushreqClusterLR = crate::BitReader<L2flushreqClusterL>;
impl L2flushreqClusterLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L2flushreqClusterL {
        match self.bits {
            false => L2flushreqClusterL::B0,
            true => L2flushreqClusterL::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == L2flushreqClusterL::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == L2flushreqClusterL::B1
    }
}
#[doc = "Field `L2FLUSHREQ_CLUSTER_L` writer - send l2 flush request to cluster_l by software"]
pub type L2flushreqClusterLW<'a, REG> = crate::BitWriter<'a, REG, L2flushreqClusterL>;
impl<'a, REG> L2flushreqClusterLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(L2flushreqClusterL::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(L2flushreqClusterL::B1)
    }
}
#[doc = "acinactm indicate to cluster_l\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcinactmClusterLCfg {
    #[doc = "0: acinactm to cluster_l is 0"]
    B0 = 0,
    #[doc = "1: acinactm to cluster_l is 1"]
    B1 = 1,
}
impl From<AcinactmClusterLCfg> for bool {
    #[inline(always)]
    fn from(variant: AcinactmClusterLCfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACINACTM_CLUSTER_L_CFG` reader - acinactm indicate to cluster_l"]
pub type AcinactmClusterLCfgR = crate::BitReader<AcinactmClusterLCfg>;
impl AcinactmClusterLCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AcinactmClusterLCfg {
        match self.bits {
            false => AcinactmClusterLCfg::B0,
            true => AcinactmClusterLCfg::B1,
        }
    }
    #[doc = "acinactm to cluster_l is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AcinactmClusterLCfg::B0
    }
    #[doc = "acinactm to cluster_l is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AcinactmClusterLCfg::B1
    }
}
#[doc = "Field `ACINACTM_CLUSTER_L_CFG` writer - acinactm indicate to cluster_l"]
pub type AcinactmClusterLCfgW<'a, REG> = crate::BitWriter<'a, REG, AcinactmClusterLCfg>;
impl<'a, REG> AcinactmClusterLCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "acinactm to cluster_l is 0"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AcinactmClusterLCfg::B0)
    }
    #[doc = "acinactm to cluster_l is 1"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AcinactmClusterLCfg::B1)
    }
}
#[doc = "dbgnopowerdown function of cluster_b enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DbgnopwrdwnBEnable {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<DbgnopwrdwnBEnable> for u8 {
    #[inline(always)]
    fn from(variant: DbgnopwrdwnBEnable) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DbgnopwrdwnBEnable {
    type Ux = u8;
}
#[doc = "Field `DBGNOPWRDWN_B_ENABLE` reader - dbgnopowerdown function of cluster_b enable"]
pub type DbgnopwrdwnBEnableR = crate::FieldReader<DbgnopwrdwnBEnable>;
impl DbgnopwrdwnBEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DbgnopwrdwnBEnable> {
        match self.bits {
            0 => Some(DbgnopwrdwnBEnable::B0),
            1 => Some(DbgnopwrdwnBEnable::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DbgnopwrdwnBEnable::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DbgnopwrdwnBEnable::B1
    }
}
#[doc = "Field `DBGNOPWRDWN_B_ENABLE` writer - dbgnopowerdown function of cluster_b enable"]
pub type DbgnopwrdwnBEnableW<'a, REG> = crate::FieldWriter<'a, REG, 2, DbgnopwrdwnBEnable>;
impl<'a, REG> DbgnopwrdwnBEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgnopwrdwnBEnable::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgnopwrdwnBEnable::B1)
    }
}
#[doc = "dbg powered up request function of cluster_b enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgpwrupreqBEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<DbgpwrupreqBEn> for bool {
    #[inline(always)]
    fn from(variant: DbgpwrupreqBEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGPWRUPREQ_B_EN` reader - dbg powered up request function of cluster_b enable"]
pub type DbgpwrupreqBEnR = crate::BitReader<DbgpwrupreqBEn>;
impl DbgpwrupreqBEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgpwrupreqBEn {
        match self.bits {
            false => DbgpwrupreqBEn::B0,
            true => DbgpwrupreqBEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DbgpwrupreqBEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DbgpwrupreqBEn::B1
    }
}
#[doc = "Field `DBGPWRUPREQ_B_EN` writer - dbg powered up request function of cluster_b enable"]
pub type DbgpwrupreqBEnW<'a, REG> = crate::BitWriter<'a, REG, DbgpwrupreqBEn>;
impl<'a, REG> DbgpwrupreqBEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgpwrupreqBEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgpwrupreqBEn::B1)
    }
}
#[doc = "cluster_b clock source gating configure\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClusterBClkSrcGatingCfg {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClusterBClkSrcGatingCfg> for bool {
    #[inline(always)]
    fn from(variant: ClusterBClkSrcGatingCfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLUSTER_B_CLK_SRC_GATING_CFG` reader - cluster_b clock source gating configure"]
pub type ClusterBClkSrcGatingCfgR = crate::BitReader<ClusterBClkSrcGatingCfg>;
impl ClusterBClkSrcGatingCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClusterBClkSrcGatingCfg {
        match self.bits {
            false => ClusterBClkSrcGatingCfg::B0,
            true => ClusterBClkSrcGatingCfg::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClusterBClkSrcGatingCfg::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClusterBClkSrcGatingCfg::B1
    }
}
#[doc = "Field `CLUSTER_B_CLK_SRC_GATING_CFG` writer - cluster_b clock source gating configure"]
pub type ClusterBClkSrcGatingCfgW<'a, REG> = crate::BitWriter<'a, REG, ClusterBClkSrcGatingCfg>;
impl<'a, REG> ClusterBClkSrcGatingCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClusterBClkSrcGatingCfg::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClusterBClkSrcGatingCfg::B1)
    }
}
#[doc = "send l2 flush request to cluster_l by software\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L2flushreqClusterB {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<L2flushreqClusterB> for bool {
    #[inline(always)]
    fn from(variant: L2flushreqClusterB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L2FLUSHREQ_CLUSTER_B` reader - send l2 flush request to cluster_l by software"]
pub type L2flushreqClusterBR = crate::BitReader<L2flushreqClusterB>;
impl L2flushreqClusterBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L2flushreqClusterB {
        match self.bits {
            false => L2flushreqClusterB::B0,
            true => L2flushreqClusterB::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == L2flushreqClusterB::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == L2flushreqClusterB::B1
    }
}
#[doc = "Field `L2FLUSHREQ_CLUSTER_B` writer - send l2 flush request to cluster_l by software"]
pub type L2flushreqClusterBW<'a, REG> = crate::BitWriter<'a, REG, L2flushreqClusterB>;
impl<'a, REG> L2flushreqClusterBW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(L2flushreqClusterB::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(L2flushreqClusterB::B1)
    }
}
#[doc = "acinactm indicate to cluster_b\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcinactmClusterBCfg {
    #[doc = "0: acinactm to cluster_b is 0"]
    B0 = 0,
    #[doc = "1: acinactm to cluster_b is 1"]
    B1 = 1,
}
impl From<AcinactmClusterBCfg> for bool {
    #[inline(always)]
    fn from(variant: AcinactmClusterBCfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACINACTM_CLUSTER_B_CFG` reader - acinactm indicate to cluster_b"]
pub type AcinactmClusterBCfgR = crate::BitReader<AcinactmClusterBCfg>;
impl AcinactmClusterBCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AcinactmClusterBCfg {
        match self.bits {
            false => AcinactmClusterBCfg::B0,
            true => AcinactmClusterBCfg::B1,
        }
    }
    #[doc = "acinactm to cluster_b is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AcinactmClusterBCfg::B0
    }
    #[doc = "acinactm to cluster_b is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AcinactmClusterBCfg::B1
    }
}
#[doc = "Field `ACINACTM_CLUSTER_B_CFG` writer - acinactm indicate to cluster_b"]
pub type AcinactmClusterBCfgW<'a, REG> = crate::BitWriter<'a, REG, AcinactmClusterBCfg>;
impl<'a, REG> AcinactmClusterBCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "acinactm to cluster_b is 0"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AcinactmClusterBCfg::B0)
    }
    #[doc = "acinactm to cluster_b is 1"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AcinactmClusterBCfg::B1)
    }
}
impl R {
    #[doc = "Bit 0 - software wakeup request bit\n\nA 0 to 1 pulse posedge will wakeup pmu when in low power mode"]
    #[inline(always)]
    pub fn wakeup_sft(&self) -> WakeupSftR {
        WakeupSftR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - software control of input clamp signal"]
    #[inline(always)]
    pub fn input_clamp_cfg(&self) -> InputClampCfgR {
        InputClampCfgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - osc disable configure by software"]
    #[inline(always)]
    pub fn osc_disable_cfg(&self) -> OscDisableCfgR {
        OscDisableCfgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - pd_pmu low frequency mode configure by software"]
    #[inline(always)]
    pub fn pmu_lf_ena_cfg(&self) -> PmuLfEnaCfgR {
        PmuLfEnaCfgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - pd_alive low frequency mode configure by software"]
    #[inline(always)]
    pub fn alive_lf_ena_cfg(&self) -> AliveLfEnaCfgR {
        AliveLfEnaCfgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - configure PD_PMU use 24M clock"]
    #[inline(always)]
    pub fn pmu_24m_ena_cfg(&self) -> Pmu24mEnaCfgR {
        Pmu24mEnaCfgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - dbg powered up of pd_a53_l0 enable when in power mode"]
    #[inline(always)]
    pub fn dbgpwrdup_l0_cfg(&self) -> DbgpwrdupL0CfgR {
        DbgpwrdupL0CfgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - m0 configure this bit to wakeup PMU state machine."]
    #[inline(always)]
    pub fn wakeup_sft_m0(&self) -> WakeupSftM0R {
        WakeupSftM0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ddrctl idle request configure"]
    #[inline(always)]
    pub fn ddrctl0_c_sysreq_cfg(&self) -> Ddrctl0CSysreqCfgR {
        Ddrctl0CSysreqCfgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ddr0 io retention configure by software"]
    #[inline(always)]
    pub fn ddr0_io_ret_cfg(&self) -> Ddr0IoRetCfgR {
        Ddr0IoRetCfgR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - ddrctl1 idle request configure"]
    #[inline(always)]
    pub fn ddrctl1_c_sysreq_cfg(&self) -> Ddrctl1CSysreqCfgR {
        Ddrctl1CSysreqCfgR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ddr1 io retention configure by software"]
    #[inline(always)]
    pub fn ddr1_io_ret_cfg(&self) -> Ddr1IoRetCfgR {
        Ddr1IoRetCfgR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - dbg powered up of pd_a72_b0 enable when in power mode"]
    #[inline(always)]
    pub fn dbgpwrdup_b0_cfg(&self) -> DbgpwrdupB0CfgR {
        DbgpwrdupB0CfgR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - dbgnopowerdown function of cluster_l enable"]
    #[inline(always)]
    pub fn dbgnopwrdwn_l_enable(&self) -> DbgnopwrdwnLEnableR {
        DbgnopwrdwnLEnableR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - dbg powered up request function of cluster_l enable"]
    #[inline(always)]
    pub fn dbgpwrupreq_l_en(&self) -> DbgpwrupreqLEnR {
        DbgpwrupreqLEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - cluster_l clock source gating configure"]
    #[inline(always)]
    pub fn cluster_l_clk_src_gating_cfg(&self) -> ClusterLClkSrcGatingCfgR {
        ClusterLClkSrcGatingCfgR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - send l2 flush request to cluster_l by software"]
    #[inline(always)]
    pub fn l2flushreq_cluster_l(&self) -> L2flushreqClusterLR {
        L2flushreqClusterLR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - acinactm indicate to cluster_l"]
    #[inline(always)]
    pub fn acinactm_cluster_l_cfg(&self) -> AcinactmClusterLCfgR {
        AcinactmClusterLCfgR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - dbgnopowerdown function of cluster_b enable"]
    #[inline(always)]
    pub fn dbgnopwrdwn_b_enable(&self) -> DbgnopwrdwnBEnableR {
        DbgnopwrdwnBEnableR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - dbg powered up request function of cluster_b enable"]
    #[inline(always)]
    pub fn dbgpwrupreq_b_en(&self) -> DbgpwrupreqBEnR {
        DbgpwrupreqBEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - cluster_b clock source gating configure"]
    #[inline(always)]
    pub fn cluster_b_clk_src_gating_cfg(&self) -> ClusterBClkSrcGatingCfgR {
        ClusterBClkSrcGatingCfgR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - send l2 flush request to cluster_l by software"]
    #[inline(always)]
    pub fn l2flushreq_cluster_b(&self) -> L2flushreqClusterBR {
        L2flushreqClusterBR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - acinactm indicate to cluster_b"]
    #[inline(always)]
    pub fn acinactm_cluster_b_cfg(&self) -> AcinactmClusterBCfgR {
        AcinactmClusterBCfgR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - software wakeup request bit\n\nA 0 to 1 pulse posedge will wakeup pmu when in low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_sft(&mut self) -> WakeupSftW<PmuSftConSpec> {
        WakeupSftW::new(self, 0)
    }
    #[doc = "Bit 1 - software control of input clamp signal"]
    #[inline(always)]
    #[must_use]
    pub fn input_clamp_cfg(&mut self) -> InputClampCfgW<PmuSftConSpec> {
        InputClampCfgW::new(self, 1)
    }
    #[doc = "Bit 2 - osc disable configure by software"]
    #[inline(always)]
    #[must_use]
    pub fn osc_disable_cfg(&mut self) -> OscDisableCfgW<PmuSftConSpec> {
        OscDisableCfgW::new(self, 2)
    }
    #[doc = "Bit 3 - pd_pmu low frequency mode configure by software"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_lf_ena_cfg(&mut self) -> PmuLfEnaCfgW<PmuSftConSpec> {
        PmuLfEnaCfgW::new(self, 3)
    }
    #[doc = "Bit 4 - pd_alive low frequency mode configure by software"]
    #[inline(always)]
    #[must_use]
    pub fn alive_lf_ena_cfg(&mut self) -> AliveLfEnaCfgW<PmuSftConSpec> {
        AliveLfEnaCfgW::new(self, 4)
    }
    #[doc = "Bit 5 - configure PD_PMU use 24M clock"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_24m_ena_cfg(&mut self) -> Pmu24mEnaCfgW<PmuSftConSpec> {
        Pmu24mEnaCfgW::new(self, 5)
    }
    #[doc = "Bit 6 - dbg powered up of pd_a53_l0 enable when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgpwrdup_l0_cfg(&mut self) -> DbgpwrdupL0CfgW<PmuSftConSpec> {
        DbgpwrdupL0CfgW::new(self, 6)
    }
    #[doc = "Bit 7 - m0 configure this bit to wakeup PMU state machine."]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_sft_m0(&mut self) -> WakeupSftM0W<PmuSftConSpec> {
        WakeupSftM0W::new(self, 7)
    }
    #[doc = "Bit 8 - ddrctl idle request configure"]
    #[inline(always)]
    #[must_use]
    pub fn ddrctl0_c_sysreq_cfg(&mut self) -> Ddrctl0CSysreqCfgW<PmuSftConSpec> {
        Ddrctl0CSysreqCfgW::new(self, 8)
    }
    #[doc = "Bit 9 - ddr0 io retention configure by software"]
    #[inline(always)]
    #[must_use]
    pub fn ddr0_io_ret_cfg(&mut self) -> Ddr0IoRetCfgW<PmuSftConSpec> {
        Ddr0IoRetCfgW::new(self, 9)
    }
    #[doc = "Bit 12 - ddrctl1 idle request configure"]
    #[inline(always)]
    #[must_use]
    pub fn ddrctl1_c_sysreq_cfg(&mut self) -> Ddrctl1CSysreqCfgW<PmuSftConSpec> {
        Ddrctl1CSysreqCfgW::new(self, 12)
    }
    #[doc = "Bit 13 - ddr1 io retention configure by software"]
    #[inline(always)]
    #[must_use]
    pub fn ddr1_io_ret_cfg(&mut self) -> Ddr1IoRetCfgW<PmuSftConSpec> {
        Ddr1IoRetCfgW::new(self, 13)
    }
    #[doc = "Bit 15 - dbg powered up of pd_a72_b0 enable when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgpwrdup_b0_cfg(&mut self) -> DbgpwrdupB0CfgW<PmuSftConSpec> {
        DbgpwrdupB0CfgW::new(self, 15)
    }
    #[doc = "Bits 16:19 - dbgnopowerdown function of cluster_l enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbgnopwrdwn_l_enable(&mut self) -> DbgnopwrdwnLEnableW<PmuSftConSpec> {
        DbgnopwrdwnLEnableW::new(self, 16)
    }
    #[doc = "Bit 20 - dbg powered up request function of cluster_l enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbgpwrupreq_l_en(&mut self) -> DbgpwrupreqLEnW<PmuSftConSpec> {
        DbgpwrupreqLEnW::new(self, 20)
    }
    #[doc = "Bit 21 - cluster_l clock source gating configure"]
    #[inline(always)]
    #[must_use]
    pub fn cluster_l_clk_src_gating_cfg(&mut self) -> ClusterLClkSrcGatingCfgW<PmuSftConSpec> {
        ClusterLClkSrcGatingCfgW::new(self, 21)
    }
    #[doc = "Bit 22 - send l2 flush request to cluster_l by software"]
    #[inline(always)]
    #[must_use]
    pub fn l2flushreq_cluster_l(&mut self) -> L2flushreqClusterLW<PmuSftConSpec> {
        L2flushreqClusterLW::new(self, 22)
    }
    #[doc = "Bit 23 - acinactm indicate to cluster_l"]
    #[inline(always)]
    #[must_use]
    pub fn acinactm_cluster_l_cfg(&mut self) -> AcinactmClusterLCfgW<PmuSftConSpec> {
        AcinactmClusterLCfgW::new(self, 23)
    }
    #[doc = "Bits 24:25 - dbgnopowerdown function of cluster_b enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbgnopwrdwn_b_enable(&mut self) -> DbgnopwrdwnBEnableW<PmuSftConSpec> {
        DbgnopwrdwnBEnableW::new(self, 24)
    }
    #[doc = "Bit 28 - dbg powered up request function of cluster_b enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbgpwrupreq_b_en(&mut self) -> DbgpwrupreqBEnW<PmuSftConSpec> {
        DbgpwrupreqBEnW::new(self, 28)
    }
    #[doc = "Bit 29 - cluster_b clock source gating configure"]
    #[inline(always)]
    #[must_use]
    pub fn cluster_b_clk_src_gating_cfg(&mut self) -> ClusterBClkSrcGatingCfgW<PmuSftConSpec> {
        ClusterBClkSrcGatingCfgW::new(self, 29)
    }
    #[doc = "Bit 30 - send l2 flush request to cluster_l by software"]
    #[inline(always)]
    #[must_use]
    pub fn l2flushreq_cluster_b(&mut self) -> L2flushreqClusterBW<PmuSftConSpec> {
        L2flushreqClusterBW::new(self, 30)
    }
    #[doc = "Bit 31 - acinactm indicate to cluster_b"]
    #[inline(always)]
    #[must_use]
    pub fn acinactm_cluster_b_cfg(&mut self) -> AcinactmClusterBCfgW<PmuSftConSpec> {
        AcinactmClusterBCfgW::new(self, 31)
    }
}
#[doc = "pmu software configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_sft_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_sft_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuSftConSpec;
impl crate::RegisterSpec for PmuSftConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_sft_con::R`](R) reader structure"]
impl crate::Readable for PmuSftConSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_sft_con::W`](W) writer structure"]
impl crate::Writable for PmuSftConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_SFT_CON to value 0"]
impl crate::Resettable for PmuSftConSpec {
    const RESET_VALUE: u32 = 0;
}
