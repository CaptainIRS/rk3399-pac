#[doc = "Register `PWRMODE_CON` reader"]
pub type R = crate::R<PwrmodeConSpec>;
#[doc = "Register `PWRMODE_CON` writer"]
pub type W = crate::W<PwrmodeConSpec>;
#[doc = "enter power mode enable, will auto self-clear when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerModeEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PowerModeEn> for bool {
    #[inline(always)]
    fn from(variant: PowerModeEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POWER_MODE_EN` reader - enter power mode enable, will auto self-clear when in power mode"]
pub type PowerModeEnR = crate::BitReader<PowerModeEn>;
impl PowerModeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PowerModeEn {
        match self.bits {
            false => PowerModeEn::B0,
            true => PowerModeEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PowerModeEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PowerModeEn::B1
    }
}
#[doc = "Field `POWER_MODE_EN` writer - enter power mode enable, will auto self-clear when in power mode"]
pub type PowerModeEnW<'a, REG> = crate::BitWriter<'a, REG, PowerModeEn>;
impl<'a, REG> PowerModeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PowerModeEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PowerModeEn::B1)
    }
}
#[doc = "wakeup reset enable when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupResetEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<WakeupResetEn> for bool {
    #[inline(always)]
    fn from(variant: WakeupResetEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_RESET_EN` reader - wakeup reset enable when in power mode"]
pub type WakeupResetEnR = crate::BitReader<WakeupResetEn>;
impl WakeupResetEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupResetEn {
        match self.bits {
            false => WakeupResetEn::B0,
            true => WakeupResetEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupResetEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupResetEn::B1
    }
}
#[doc = "Field `WAKEUP_RESET_EN` writer - wakeup reset enable when in power mode"]
pub type WakeupResetEnW<'a, REG> = crate::BitWriter<'a, REG, WakeupResetEn>;
impl<'a, REG> WakeupResetEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupResetEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupResetEn::B1)
    }
}
#[doc = "clamp vd_logic when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputClampEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<InputClampEn> for bool {
    #[inline(always)]
    fn from(variant: InputClampEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPUT_CLAMP_EN` reader - clamp vd_logic when in power mode"]
pub type InputClampEnR = crate::BitReader<InputClampEn>;
impl InputClampEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InputClampEn {
        match self.bits {
            false => InputClampEn::B0,
            true => InputClampEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == InputClampEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == InputClampEn::B1
    }
}
#[doc = "Field `INPUT_CLAMP_EN` writer - clamp vd_logic when in power mode"]
pub type InputClampEnW<'a, REG> = crate::BitWriter<'a, REG, InputClampEn>;
impl<'a, REG> InputClampEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(InputClampEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(InputClampEn::B1)
    }
}
#[doc = "osc disable when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OscDisable {
    #[doc = "1: disable"]
    B1 = 1,
    #[doc = "0: enable"]
    B0 = 0,
}
impl From<OscDisable> for bool {
    #[inline(always)]
    fn from(variant: OscDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSC_DISABLE` reader - osc disable when in power mode"]
pub type OscDisableR = crate::BitReader<OscDisable>;
impl OscDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OscDisable {
        match self.bits {
            true => OscDisable::B1,
            false => OscDisable::B0,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == OscDisable::B1
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == OscDisable::B0
    }
}
#[doc = "Field `OSC_DISABLE` writer - osc disable when in power mode"]
pub type OscDisableW<'a, REG> = crate::BitWriter<'a, REG, OscDisable>;
impl<'a, REG> OscDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(OscDisable::B1)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(OscDisable::B0)
    }
}
#[doc = "alive low frequency mode when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AliveUseLf {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<AliveUseLf> for bool {
    #[inline(always)]
    fn from(variant: AliveUseLf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIVE_USE_LF` reader - alive low frequency mode when in power mode"]
pub type AliveUseLfR = crate::BitReader<AliveUseLf>;
impl AliveUseLfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AliveUseLf {
        match self.bits {
            false => AliveUseLf::B0,
            true => AliveUseLf::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AliveUseLf::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AliveUseLf::B1
    }
}
#[doc = "Field `ALIVE_USE_LF` writer - alive low frequency mode when in power mode"]
pub type AliveUseLfW<'a, REG> = crate::BitWriter<'a, REG, AliveUseLf>;
impl<'a, REG> AliveUseLfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AliveUseLf::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AliveUseLf::B1)
    }
}
#[doc = "pmu low frequency mode enable when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmuUseLf {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PmuUseLf> for bool {
    #[inline(always)]
    fn from(variant: PmuUseLf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMU_USE_LF` reader - pmu low frequency mode enable when in power mode"]
pub type PmuUseLfR = crate::BitReader<PmuUseLf>;
impl PmuUseLfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PmuUseLf {
        match self.bits {
            false => PmuUseLf::B0,
            true => PmuUseLf::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PmuUseLf::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PmuUseLf::B1
    }
}
#[doc = "Field `PMU_USE_LF` writer - pmu low frequency mode enable when in power mode"]
pub type PmuUseLfW<'a, REG> = crate::BitWriter<'a, REG, PmuUseLf>;
impl<'a, REG> PmuUseLfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PmuUseLf::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PmuUseLf::B1)
    }
}
#[doc = "send power off request to PMIC when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerOffReqCfg {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PowerOffReqCfg> for bool {
    #[inline(always)]
    fn from(variant: PowerOffReqCfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POWER_OFF_REQ_CFG` reader - send power off request to PMIC when in power mode"]
pub type PowerOffReqCfgR = crate::BitReader<PowerOffReqCfg>;
impl PowerOffReqCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PowerOffReqCfg {
        match self.bits {
            false => PowerOffReqCfg::B0,
            true => PowerOffReqCfg::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PowerOffReqCfg::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PowerOffReqCfg::B1
    }
}
#[doc = "Field `POWER_OFF_REQ_CFG` writer - send power off request to PMIC when in power mode"]
pub type PowerOffReqCfgW<'a, REG> = crate::BitWriter<'a, REG, PowerOffReqCfg>;
impl<'a, REG> PowerOffReqCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PowerOffReqCfg::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PowerOffReqCfg::B1)
    }
}
#[doc = "chip power down enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChipPdEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ChipPdEn> for bool {
    #[inline(always)]
    fn from(variant: ChipPdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHIP_PD_EN` reader - chip power down enable"]
pub type ChipPdEnR = crate::BitReader<ChipPdEn>;
impl ChipPdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ChipPdEn {
        match self.bits {
            false => ChipPdEn::B0,
            true => ChipPdEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ChipPdEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ChipPdEn::B1
    }
}
#[doc = "Field `CHIP_PD_EN` writer - chip power down enable"]
pub type ChipPdEnW<'a, REG> = crate::BitWriter<'a, REG, ChipPdEn>;
impl<'a, REG> ChipPdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ChipPdEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ChipPdEn::B1)
    }
}
#[doc = "power down pll when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PllPdEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PllPdEn> for bool {
    #[inline(always)]
    fn from(variant: PllPdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL_PD_EN` reader - power down pll when in power mode"]
pub type PllPdEnR = crate::BitReader<PllPdEn>;
impl PllPdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PllPdEn {
        match self.bits {
            false => PllPdEn::B0,
            true => PllPdEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PllPdEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PllPdEn::B1
    }
}
#[doc = "Field `PLL_PD_EN` writer - power down pll when in power mode"]
pub type PllPdEnW<'a, REG> = crate::BitWriter<'a, REG, PllPdEn>;
impl<'a, REG> PllPdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PllPdEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PllPdEn::B1)
    }
}
#[doc = "power down core0 of cluster_l in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu0PdEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Cpu0PdEn> for bool {
    #[inline(always)]
    fn from(variant: Cpu0PdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU0_PD_EN` reader - power down core0 of cluster_l in power mode"]
pub type Cpu0PdEnR = crate::BitReader<Cpu0PdEn>;
impl Cpu0PdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu0PdEn {
        match self.bits {
            false => Cpu0PdEn::B0,
            true => Cpu0PdEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cpu0PdEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cpu0PdEn::B1
    }
}
#[doc = "Field `CPU0_PD_EN` writer - power down core0 of cluster_l in power mode"]
pub type Cpu0PdEnW<'a, REG> = crate::BitWriter<'a, REG, Cpu0PdEn>;
impl<'a, REG> Cpu0PdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0PdEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0PdEn::B1)
    }
}
#[doc = "flush l2 by hardware when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L2FlushEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<L2FlushEn> for bool {
    #[inline(always)]
    fn from(variant: L2FlushEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L2_FLUSH_EN` reader - flush l2 by hardware when in power mode"]
pub type L2FlushEnR = crate::BitReader<L2FlushEn>;
impl L2FlushEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L2FlushEn {
        match self.bits {
            false => L2FlushEn::B0,
            true => L2FlushEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == L2FlushEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == L2FlushEn::B1
    }
}
#[doc = "Field `L2_FLUSH_EN` writer - flush l2 by hardware when in power mode"]
pub type L2FlushEnW<'a, REG> = crate::BitWriter<'a, REG, L2FlushEn>;
impl<'a, REG> L2FlushEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(L2FlushEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(L2FlushEn::B1)
    }
}
#[doc = "wait l2 idle when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L2IdleEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<L2IdleEn> for bool {
    #[inline(always)]
    fn from(variant: L2IdleEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L2_IDLE_EN` reader - wait l2 idle when in power mode"]
pub type L2IdleEnR = crate::BitReader<L2IdleEn>;
impl L2IdleEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L2IdleEn {
        match self.bits {
            false => L2IdleEn::B0,
            true => L2IdleEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == L2IdleEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == L2IdleEn::B1
    }
}
#[doc = "Field `L2_IDLE_EN` writer - wait l2 idle when in power mode"]
pub type L2IdleEnW<'a, REG> = crate::BitWriter<'a, REG, L2IdleEn>;
impl<'a, REG> L2IdleEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(L2IdleEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(L2IdleEn::B1)
    }
}
#[doc = "power down main cluster scu when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScuPdEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ScuPdEn> for bool {
    #[inline(always)]
    fn from(variant: ScuPdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCU_PD_EN` reader - power down main cluster scu when in power mode"]
pub type ScuPdEnR = crate::BitReader<ScuPdEn>;
impl ScuPdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScuPdEn {
        match self.bits {
            false => ScuPdEn::B0,
            true => ScuPdEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ScuPdEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ScuPdEn::B1
    }
}
#[doc = "Field `SCU_PD_EN` writer - power down main cluster scu when in power mode"]
pub type ScuPdEnW<'a, REG> = crate::BitWriter<'a, REG, ScuPdEn>;
impl<'a, REG> ScuPdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ScuPdEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ScuPdEn::B1)
    }
}
#[doc = "power down pd_cci when power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CciPdEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<CciPdEn> for bool {
    #[inline(always)]
    fn from(variant: CciPdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCI_PD_EN` reader - power down pd_cci when power mode"]
pub type CciPdEnR = crate::BitReader<CciPdEn>;
impl CciPdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CciPdEn {
        match self.bits {
            false => CciPdEn::B0,
            true => CciPdEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CciPdEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CciPdEn::B1
    }
}
#[doc = "Field `CCI_PD_EN` writer - power down pd_cci when power mode"]
pub type CciPdEnW<'a, REG> = crate::BitWriter<'a, REG, CciPdEn>;
impl<'a, REG> CciPdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CciPdEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CciPdEn::B1)
    }
}
#[doc = "power down pd_perilp when power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerilpPdEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PerilpPdEn> for bool {
    #[inline(always)]
    fn from(variant: PerilpPdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERILP_PD_EN` reader - power down pd_perilp when power mode"]
pub type PerilpPdEnR = crate::BitReader<PerilpPdEn>;
impl PerilpPdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerilpPdEn {
        match self.bits {
            false => PerilpPdEn::B0,
            true => PerilpPdEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PerilpPdEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PerilpPdEn::B1
    }
}
#[doc = "Field `PERILP_PD_EN` writer - power down pd_perilp when power mode"]
pub type PerilpPdEnW<'a, REG> = crate::BitWriter<'a, REG, PerilpPdEn>;
impl<'a, REG> PerilpPdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpPdEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpPdEn::B1)
    }
}
#[doc = "power down pd_center when power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CenterPdEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<CenterPdEn> for bool {
    #[inline(always)]
    fn from(variant: CenterPdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER_PD_EN` reader - power down pd_center when power mode"]
pub type CenterPdEnR = crate::BitReader<CenterPdEn>;
impl CenterPdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CenterPdEn {
        match self.bits {
            false => CenterPdEn::B0,
            true => CenterPdEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CenterPdEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CenterPdEn::B1
    }
}
#[doc = "Field `CENTER_PD_EN` writer - power down pd_center when power mode"]
pub type CenterPdEnW<'a, REG> = crate::BitWriter<'a, REG, CenterPdEn>;
impl<'a, REG> CenterPdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CenterPdEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CenterPdEn::B1)
    }
}
#[doc = "ddr0 self_refresh by hardware when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sref0EnterEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Sref0EnterEn> for bool {
    #[inline(always)]
    fn from(variant: Sref0EnterEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SREF0_ENTER_EN` reader - ddr0 self_refresh by hardware when in power mode"]
pub type Sref0EnterEnR = crate::BitReader<Sref0EnterEn>;
impl Sref0EnterEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sref0EnterEn {
        match self.bits {
            false => Sref0EnterEn::B0,
            true => Sref0EnterEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sref0EnterEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sref0EnterEn::B1
    }
}
#[doc = "Field `SREF0_ENTER_EN` writer - ddr0 self_refresh by hardware when in power mode"]
pub type Sref0EnterEnW<'a, REG> = crate::BitWriter<'a, REG, Sref0EnterEn>;
impl<'a, REG> Sref0EnterEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sref0EnterEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sref0EnterEn::B1)
    }
}
#[doc = "ddr0 controller auto gating when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddrc0GatingEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Ddrc0GatingEn> for bool {
    #[inline(always)]
    fn from(variant: Ddrc0GatingEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDRC0_GATING_EN` reader - ddr0 controller auto gating when in power mode"]
pub type Ddrc0GatingEnR = crate::BitReader<Ddrc0GatingEn>;
impl Ddrc0GatingEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddrc0GatingEn {
        match self.bits {
            false => Ddrc0GatingEn::B0,
            true => Ddrc0GatingEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ddrc0GatingEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ddrc0GatingEn::B1
    }
}
#[doc = "Field `DDRC0_GATING_EN` writer - ddr0 controller auto gating when in power mode"]
pub type Ddrc0GatingEnW<'a, REG> = crate::BitWriter<'a, REG, Ddrc0GatingEn>;
impl<'a, REG> Ddrc0GatingEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ddrc0GatingEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ddrc0GatingEn::B1)
    }
}
#[doc = "ddrio0 retention enable when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddrio0RetEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Ddrio0RetEn> for bool {
    #[inline(always)]
    fn from(variant: Ddrio0RetEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDRIO0_RET_EN` reader - ddrio0 retention enable when in power mode"]
pub type Ddrio0RetEnR = crate::BitReader<Ddrio0RetEn>;
impl Ddrio0RetEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddrio0RetEn {
        match self.bits {
            false => Ddrio0RetEn::B0,
            true => Ddrio0RetEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ddrio0RetEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ddrio0RetEn::B1
    }
}
#[doc = "Field `DDRIO0_RET_EN` writer - ddrio0 retention enable when in power mode"]
pub type Ddrio0RetEnW<'a, REG> = crate::BitWriter<'a, REG, Ddrio0RetEn>;
impl<'a, REG> Ddrio0RetEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ddrio0RetEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ddrio0RetEn::B1)
    }
}
#[doc = "Field `DDRIO0_RET_DE_REQ` reader - ddrio0 retention de-assert request\n\nwrite one clear"]
pub type Ddrio0RetDeReqR = crate::BitReader;
#[doc = "Field `DDRIO0_RET_DE_REQ` writer - ddrio0 retention de-assert request\n\nwrite one clear"]
pub type Ddrio0RetDeReqW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "ddr1 self_refresh by hardware when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sref1EnterEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Sref1EnterEn> for bool {
    #[inline(always)]
    fn from(variant: Sref1EnterEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SREF1_ENTER_EN` reader - ddr1 self_refresh by hardware when in power mode"]
pub type Sref1EnterEnR = crate::BitReader<Sref1EnterEn>;
impl Sref1EnterEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sref1EnterEn {
        match self.bits {
            false => Sref1EnterEn::B0,
            true => Sref1EnterEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sref1EnterEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sref1EnterEn::B1
    }
}
#[doc = "Field `SREF1_ENTER_EN` writer - ddr1 self_refresh by hardware when in power mode"]
pub type Sref1EnterEnW<'a, REG> = crate::BitWriter<'a, REG, Sref1EnterEn>;
impl<'a, REG> Sref1EnterEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sref1EnterEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sref1EnterEn::B1)
    }
}
#[doc = "ddr1 controller auto gating when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddrc1GatingEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Ddrc1GatingEn> for bool {
    #[inline(always)]
    fn from(variant: Ddrc1GatingEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDRC1_GATING_EN` reader - ddr1 controller auto gating when in power mode"]
pub type Ddrc1GatingEnR = crate::BitReader<Ddrc1GatingEn>;
impl Ddrc1GatingEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddrc1GatingEn {
        match self.bits {
            false => Ddrc1GatingEn::B0,
            true => Ddrc1GatingEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ddrc1GatingEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ddrc1GatingEn::B1
    }
}
#[doc = "Field `DDRC1_GATING_EN` writer - ddr1 controller auto gating when in power mode"]
pub type Ddrc1GatingEnW<'a, REG> = crate::BitWriter<'a, REG, Ddrc1GatingEn>;
impl<'a, REG> Ddrc1GatingEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ddrc1GatingEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ddrc1GatingEn::B1)
    }
}
#[doc = "ddrio1 retention enable when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddrio1RetEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Ddrio1RetEn> for bool {
    #[inline(always)]
    fn from(variant: Ddrio1RetEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDRIO1_RET_EN` reader - ddrio1 retention enable when in power mode"]
pub type Ddrio1RetEnR = crate::BitReader<Ddrio1RetEn>;
impl Ddrio1RetEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddrio1RetEn {
        match self.bits {
            false => Ddrio1RetEn::B0,
            true => Ddrio1RetEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ddrio1RetEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ddrio1RetEn::B1
    }
}
#[doc = "Field `DDRIO1_RET_EN` writer - ddrio1 retention enable when in power mode"]
pub type Ddrio1RetEnW<'a, REG> = crate::BitWriter<'a, REG, Ddrio1RetEn>;
impl<'a, REG> Ddrio1RetEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ddrio1RetEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ddrio1RetEn::B1)
    }
}
#[doc = "Field `DDRIO1_RET_DE_REQ` reader - ddrio1 retention de-assert request\n\nwrite one clear"]
pub type Ddrio1RetDeReqR = crate::BitReader;
#[doc = "Field `DDRIO1_RET_DE_REQ` writer - ddrio1 retention de-assert request\n\nwrite one clear"]
pub type Ddrio1RetDeReqW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "pd_center clock gate enable when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkCenterSrcGateEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClkCenterSrcGateEn> for bool {
    #[inline(always)]
    fn from(variant: ClkCenterSrcGateEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_CENTER_SRC_GATE_EN` reader - pd_center clock gate enable when in power mode"]
pub type ClkCenterSrcGateEnR = crate::BitReader<ClkCenterSrcGateEn>;
impl ClkCenterSrcGateEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkCenterSrcGateEn {
        match self.bits {
            false => ClkCenterSrcGateEn::B0,
            true => ClkCenterSrcGateEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkCenterSrcGateEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkCenterSrcGateEn::B1
    }
}
#[doc = "Field `CLK_CENTER_SRC_GATE_EN` writer - pd_center clock gate enable when in power mode"]
pub type ClkCenterSrcGateEnW<'a, REG> = crate::BitWriter<'a, REG, ClkCenterSrcGateEn>;
impl<'a, REG> ClkCenterSrcGateEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCenterSrcGateEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCenterSrcGateEn::B1)
    }
}
#[doc = "pd_perilp clock gate enable when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkPerilpSrcGateEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClkPerilpSrcGateEn> for bool {
    #[inline(always)]
    fn from(variant: ClkPerilpSrcGateEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_PERILP_SRC_GATE_EN` reader - pd_perilp clock gate enable when in power mode"]
pub type ClkPerilpSrcGateEnR = crate::BitReader<ClkPerilpSrcGateEn>;
impl ClkPerilpSrcGateEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkPerilpSrcGateEn {
        match self.bits {
            false => ClkPerilpSrcGateEn::B0,
            true => ClkPerilpSrcGateEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkPerilpSrcGateEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkPerilpSrcGateEn::B1
    }
}
#[doc = "Field `CLK_PERILP_SRC_GATE_EN` writer - pd_perilp clock gate enable when in power mode"]
pub type ClkPerilpSrcGateEnW<'a, REG> = crate::BitWriter<'a, REG, ClkPerilpSrcGateEn>;
impl<'a, REG> ClkPerilpSrcGateEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkPerilpSrcGateEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkPerilpSrcGateEn::B1)
    }
}
#[doc = "cpu clock gate enable when in power mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkCoreSrcGateEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClkCoreSrcGateEn> for bool {
    #[inline(always)]
    fn from(variant: ClkCoreSrcGateEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_CORE_SRC_GATE_EN` reader - cpu clock gate enable when in power mode"]
pub type ClkCoreSrcGateEnR = crate::BitReader<ClkCoreSrcGateEn>;
impl ClkCoreSrcGateEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkCoreSrcGateEn {
        match self.bits {
            false => ClkCoreSrcGateEn::B0,
            true => ClkCoreSrcGateEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkCoreSrcGateEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkCoreSrcGateEn::B1
    }
}
#[doc = "Field `CLK_CORE_SRC_GATE_EN` writer - cpu clock gate enable when in power mode"]
pub type ClkCoreSrcGateEnW<'a, REG> = crate::BitWriter<'a, REG, ClkCoreSrcGateEn>;
impl<'a, REG> ClkCoreSrcGateEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCoreSrcGateEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCoreSrcGateEn::B1)
    }
}
#[doc = "Field `DDRIO_RET_HW_DE_REQ` reader - hardware ddrio retention de-assert request"]
pub type DdrioRetHwDeReqR = crate::BitReader;
#[doc = "Field `DDRIO_RET_HW_DE_REQ` writer - hardware ddrio retention de-assert request"]
pub type DdrioRetHwDeReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_OUTPUT_CFG` reader - output pmu_sleep instead of ap_pwroff to IO."]
pub type SleepOutputCfgR = crate::BitReader;
#[doc = "Field `SLEEP_OUTPUT_CFG` writer - output pmu_sleep instead of ap_pwroff to IO."]
pub type SleepOutputCfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "use core big for main cluster.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MainCluster {
    #[doc = "0: core_l ;"]
    B0 = 0,
    #[doc = "1: core_b."]
    B1 = 1,
}
impl From<MainCluster> for bool {
    #[inline(always)]
    fn from(variant: MainCluster) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAIN_CLUSTER` reader - use core big for main cluster."]
pub type MainClusterR = crate::BitReader<MainCluster>;
impl MainClusterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MainCluster {
        match self.bits {
            false => MainCluster::B0,
            true => MainCluster::B1,
        }
    }
    #[doc = "core_l ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == MainCluster::B0
    }
    #[doc = "core_b."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == MainCluster::B1
    }
}
#[doc = "Field `MAIN_CLUSTER` writer - use core big for main cluster."]
pub type MainClusterW<'a, REG> = crate::BitWriter<'a, REG, MainCluster>;
impl<'a, REG> MainClusterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "core_l ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(MainCluster::B0)
    }
    #[doc = "core_b."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(MainCluster::B1)
    }
}
impl R {
    #[doc = "Bit 0 - enter power mode enable, will auto self-clear when in power mode"]
    #[inline(always)]
    pub fn power_mode_en(&self) -> PowerModeEnR {
        PowerModeEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - wakeup reset enable when in power mode"]
    #[inline(always)]
    pub fn wakeup_reset_en(&self) -> WakeupResetEnR {
        WakeupResetEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - clamp vd_logic when in power mode"]
    #[inline(always)]
    pub fn input_clamp_en(&self) -> InputClampEnR {
        InputClampEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - osc disable when in power mode"]
    #[inline(always)]
    pub fn osc_disable(&self) -> OscDisableR {
        OscDisableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - alive low frequency mode when in power mode"]
    #[inline(always)]
    pub fn alive_use_lf(&self) -> AliveUseLfR {
        AliveUseLfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - pmu low frequency mode enable when in power mode"]
    #[inline(always)]
    pub fn pmu_use_lf(&self) -> PmuUseLfR {
        PmuUseLfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - send power off request to PMIC when in power mode"]
    #[inline(always)]
    pub fn power_off_req_cfg(&self) -> PowerOffReqCfgR {
        PowerOffReqCfgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - chip power down enable"]
    #[inline(always)]
    pub fn chip_pd_en(&self) -> ChipPdEnR {
        ChipPdEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - power down pll when in power mode"]
    #[inline(always)]
    pub fn pll_pd_en(&self) -> PllPdEnR {
        PllPdEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - power down core0 of cluster_l in power mode"]
    #[inline(always)]
    pub fn cpu0_pd_en(&self) -> Cpu0PdEnR {
        Cpu0PdEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - flush l2 by hardware when in power mode"]
    #[inline(always)]
    pub fn l2_flush_en(&self) -> L2FlushEnR {
        L2FlushEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - wait l2 idle when in power mode"]
    #[inline(always)]
    pub fn l2_idle_en(&self) -> L2IdleEnR {
        L2IdleEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - power down main cluster scu when in power mode"]
    #[inline(always)]
    pub fn scu_pd_en(&self) -> ScuPdEnR {
        ScuPdEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - power down pd_cci when power mode"]
    #[inline(always)]
    pub fn cci_pd_en(&self) -> CciPdEnR {
        CciPdEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - power down pd_perilp when power mode"]
    #[inline(always)]
    pub fn perilp_pd_en(&self) -> PerilpPdEnR {
        PerilpPdEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - power down pd_center when power mode"]
    #[inline(always)]
    pub fn center_pd_en(&self) -> CenterPdEnR {
        CenterPdEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ddr0 self_refresh by hardware when in power mode"]
    #[inline(always)]
    pub fn sref0_enter_en(&self) -> Sref0EnterEnR {
        Sref0EnterEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ddr0 controller auto gating when in power mode"]
    #[inline(always)]
    pub fn ddrc0_gating_en(&self) -> Ddrc0GatingEnR {
        Ddrc0GatingEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ddrio0 retention enable when in power mode"]
    #[inline(always)]
    pub fn ddrio0_ret_en(&self) -> Ddrio0RetEnR {
        Ddrio0RetEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ddrio0 retention de-assert request\n\nwrite one clear"]
    #[inline(always)]
    pub fn ddrio0_ret_de_req(&self) -> Ddrio0RetDeReqR {
        Ddrio0RetDeReqR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ddr1 self_refresh by hardware when in power mode"]
    #[inline(always)]
    pub fn sref1_enter_en(&self) -> Sref1EnterEnR {
        Sref1EnterEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ddr1 controller auto gating when in power mode"]
    #[inline(always)]
    pub fn ddrc1_gating_en(&self) -> Ddrc1GatingEnR {
        Ddrc1GatingEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ddrio1 retention enable when in power mode"]
    #[inline(always)]
    pub fn ddrio1_ret_en(&self) -> Ddrio1RetEnR {
        Ddrio1RetEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ddrio1 retention de-assert request\n\nwrite one clear"]
    #[inline(always)]
    pub fn ddrio1_ret_de_req(&self) -> Ddrio1RetDeReqR {
        Ddrio1RetDeReqR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - pd_center clock gate enable when in power mode"]
    #[inline(always)]
    pub fn clk_center_src_gate_en(&self) -> ClkCenterSrcGateEnR {
        ClkCenterSrcGateEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - pd_perilp clock gate enable when in power mode"]
    #[inline(always)]
    pub fn clk_perilp_src_gate_en(&self) -> ClkPerilpSrcGateEnR {
        ClkPerilpSrcGateEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - cpu clock gate enable when in power mode"]
    #[inline(always)]
    pub fn clk_core_src_gate_en(&self) -> ClkCoreSrcGateEnR {
        ClkCoreSrcGateEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - hardware ddrio retention de-assert request"]
    #[inline(always)]
    pub fn ddrio_ret_hw_de_req(&self) -> DdrioRetHwDeReqR {
        DdrioRetHwDeReqR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - output pmu_sleep instead of ap_pwroff to IO."]
    #[inline(always)]
    pub fn sleep_output_cfg(&self) -> SleepOutputCfgR {
        SleepOutputCfgR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - use core big for main cluster."]
    #[inline(always)]
    pub fn main_cluster(&self) -> MainClusterR {
        MainClusterR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enter power mode enable, will auto self-clear when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn power_mode_en(&mut self) -> PowerModeEnW<PwrmodeConSpec> {
        PowerModeEnW::new(self, 0)
    }
    #[doc = "Bit 1 - wakeup reset enable when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_reset_en(&mut self) -> WakeupResetEnW<PwrmodeConSpec> {
        WakeupResetEnW::new(self, 1)
    }
    #[doc = "Bit 2 - clamp vd_logic when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn input_clamp_en(&mut self) -> InputClampEnW<PwrmodeConSpec> {
        InputClampEnW::new(self, 2)
    }
    #[doc = "Bit 3 - osc disable when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn osc_disable(&mut self) -> OscDisableW<PwrmodeConSpec> {
        OscDisableW::new(self, 3)
    }
    #[doc = "Bit 4 - alive low frequency mode when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn alive_use_lf(&mut self) -> AliveUseLfW<PwrmodeConSpec> {
        AliveUseLfW::new(self, 4)
    }
    #[doc = "Bit 5 - pmu low frequency mode enable when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_use_lf(&mut self) -> PmuUseLfW<PwrmodeConSpec> {
        PmuUseLfW::new(self, 5)
    }
    #[doc = "Bit 6 - send power off request to PMIC when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn power_off_req_cfg(&mut self) -> PowerOffReqCfgW<PwrmodeConSpec> {
        PowerOffReqCfgW::new(self, 6)
    }
    #[doc = "Bit 7 - chip power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn chip_pd_en(&mut self) -> ChipPdEnW<PwrmodeConSpec> {
        ChipPdEnW::new(self, 7)
    }
    #[doc = "Bit 8 - power down pll when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn pll_pd_en(&mut self) -> PllPdEnW<PwrmodeConSpec> {
        PllPdEnW::new(self, 8)
    }
    #[doc = "Bit 9 - power down core0 of cluster_l in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_pd_en(&mut self) -> Cpu0PdEnW<PwrmodeConSpec> {
        Cpu0PdEnW::new(self, 9)
    }
    #[doc = "Bit 10 - flush l2 by hardware when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn l2_flush_en(&mut self) -> L2FlushEnW<PwrmodeConSpec> {
        L2FlushEnW::new(self, 10)
    }
    #[doc = "Bit 11 - wait l2 idle when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn l2_idle_en(&mut self) -> L2IdleEnW<PwrmodeConSpec> {
        L2IdleEnW::new(self, 11)
    }
    #[doc = "Bit 12 - power down main cluster scu when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn scu_pd_en(&mut self) -> ScuPdEnW<PwrmodeConSpec> {
        ScuPdEnW::new(self, 12)
    }
    #[doc = "Bit 13 - power down pd_cci when power mode"]
    #[inline(always)]
    #[must_use]
    pub fn cci_pd_en(&mut self) -> CciPdEnW<PwrmodeConSpec> {
        CciPdEnW::new(self, 13)
    }
    #[doc = "Bit 14 - power down pd_perilp when power mode"]
    #[inline(always)]
    #[must_use]
    pub fn perilp_pd_en(&mut self) -> PerilpPdEnW<PwrmodeConSpec> {
        PerilpPdEnW::new(self, 14)
    }
    #[doc = "Bit 15 - power down pd_center when power mode"]
    #[inline(always)]
    #[must_use]
    pub fn center_pd_en(&mut self) -> CenterPdEnW<PwrmodeConSpec> {
        CenterPdEnW::new(self, 15)
    }
    #[doc = "Bit 16 - ddr0 self_refresh by hardware when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn sref0_enter_en(&mut self) -> Sref0EnterEnW<PwrmodeConSpec> {
        Sref0EnterEnW::new(self, 16)
    }
    #[doc = "Bit 17 - ddr0 controller auto gating when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn ddrc0_gating_en(&mut self) -> Ddrc0GatingEnW<PwrmodeConSpec> {
        Ddrc0GatingEnW::new(self, 17)
    }
    #[doc = "Bit 18 - ddrio0 retention enable when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn ddrio0_ret_en(&mut self) -> Ddrio0RetEnW<PwrmodeConSpec> {
        Ddrio0RetEnW::new(self, 18)
    }
    #[doc = "Bit 19 - ddrio0 retention de-assert request\n\nwrite one clear"]
    #[inline(always)]
    #[must_use]
    pub fn ddrio0_ret_de_req(&mut self) -> Ddrio0RetDeReqW<PwrmodeConSpec> {
        Ddrio0RetDeReqW::new(self, 19)
    }
    #[doc = "Bit 20 - ddr1 self_refresh by hardware when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn sref1_enter_en(&mut self) -> Sref1EnterEnW<PwrmodeConSpec> {
        Sref1EnterEnW::new(self, 20)
    }
    #[doc = "Bit 21 - ddr1 controller auto gating when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn ddrc1_gating_en(&mut self) -> Ddrc1GatingEnW<PwrmodeConSpec> {
        Ddrc1GatingEnW::new(self, 21)
    }
    #[doc = "Bit 22 - ddrio1 retention enable when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn ddrio1_ret_en(&mut self) -> Ddrio1RetEnW<PwrmodeConSpec> {
        Ddrio1RetEnW::new(self, 22)
    }
    #[doc = "Bit 23 - ddrio1 retention de-assert request\n\nwrite one clear"]
    #[inline(always)]
    #[must_use]
    pub fn ddrio1_ret_de_req(&mut self) -> Ddrio1RetDeReqW<PwrmodeConSpec> {
        Ddrio1RetDeReqW::new(self, 23)
    }
    #[doc = "Bit 26 - pd_center clock gate enable when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn clk_center_src_gate_en(&mut self) -> ClkCenterSrcGateEnW<PwrmodeConSpec> {
        ClkCenterSrcGateEnW::new(self, 26)
    }
    #[doc = "Bit 27 - pd_perilp clock gate enable when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn clk_perilp_src_gate_en(&mut self) -> ClkPerilpSrcGateEnW<PwrmodeConSpec> {
        ClkPerilpSrcGateEnW::new(self, 27)
    }
    #[doc = "Bit 28 - cpu clock gate enable when in power mode"]
    #[inline(always)]
    #[must_use]
    pub fn clk_core_src_gate_en(&mut self) -> ClkCoreSrcGateEnW<PwrmodeConSpec> {
        ClkCoreSrcGateEnW::new(self, 28)
    }
    #[doc = "Bit 29 - hardware ddrio retention de-assert request"]
    #[inline(always)]
    #[must_use]
    pub fn ddrio_ret_hw_de_req(&mut self) -> DdrioRetHwDeReqW<PwrmodeConSpec> {
        DdrioRetHwDeReqW::new(self, 29)
    }
    #[doc = "Bit 30 - output pmu_sleep instead of ap_pwroff to IO."]
    #[inline(always)]
    #[must_use]
    pub fn sleep_output_cfg(&mut self) -> SleepOutputCfgW<PwrmodeConSpec> {
        SleepOutputCfgW::new(self, 30)
    }
    #[doc = "Bit 31 - use core big for main cluster."]
    #[inline(always)]
    #[must_use]
    pub fn main_cluster(&mut self) -> MainClusterW<PwrmodeConSpec> {
        MainClusterW::new(self, 31)
    }
}
#[doc = "pmu power mode configure register of common resource\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrmode_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrmode_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrmodeConSpec;
impl crate::RegisterSpec for PwrmodeConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrmode_con::R`](R) reader structure"]
impl crate::Readable for PwrmodeConSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrmode_con::W`](W) writer structure"]
impl crate::Writable for PwrmodeConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0088_0000;
}
#[doc = "`reset()` method sets PWRMODE_CON to value 0"]
impl crate::Resettable for PwrmodeConSpec {
    const RESET_VALUE: u32 = 0;
}
