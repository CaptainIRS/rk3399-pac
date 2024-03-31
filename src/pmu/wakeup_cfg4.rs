#[doc = "Register `WAKEUP_CFG4` reader"]
pub type R = crate::R<WakeupCfg4Spec>;
#[doc = "Register `WAKEUP_CFG4` writer"]
pub type W = crate::W<WakeupCfg4Spec>;
#[doc = "cluster_l interrupt wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntClusterLEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntClusterLEn> for bool {
    #[inline(always)]
    fn from(variant: IntClusterLEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_CLUSTER_L_EN` reader - cluster_l interrupt wakeup enable"]
pub type IntClusterLEnR = crate::BitReader<IntClusterLEn>;
impl IntClusterLEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntClusterLEn {
        match self.bits {
            false => IntClusterLEn::B0,
            true => IntClusterLEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntClusterLEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntClusterLEn::B1
    }
}
#[doc = "Field `INT_CLUSTER_L_EN` writer - cluster_l interrupt wakeup enable"]
pub type IntClusterLEnW<'a, REG> = crate::BitWriter<'a, REG, IntClusterLEn>;
impl<'a, REG> IntClusterLEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntClusterLEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntClusterLEn::B1)
    }
}
#[doc = "cluster_b interrupt wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntClusterBEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IntClusterBEn> for bool {
    #[inline(always)]
    fn from(variant: IntClusterBEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_CLUSTER_B_EN` reader - cluster_b interrupt wakeup enable"]
pub type IntClusterBEnR = crate::BitReader<IntClusterBEn>;
impl IntClusterBEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntClusterBEn {
        match self.bits {
            false => IntClusterBEn::B0,
            true => IntClusterBEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntClusterBEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntClusterBEn::B1
    }
}
#[doc = "Field `INT_CLUSTER_B_EN` writer - cluster_b interrupt wakeup enable"]
pub type IntClusterBEnW<'a, REG> = crate::BitWriter<'a, REG, IntClusterBEn>;
impl<'a, REG> IntClusterBEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntClusterBEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntClusterBEn::B1)
    }
}
#[doc = "gpio interrupt wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<GpioIntEn> for bool {
    #[inline(always)]
    fn from(variant: GpioIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT_EN` reader - gpio interrupt wakeup enable"]
pub type GpioIntEnR = crate::BitReader<GpioIntEn>;
impl GpioIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioIntEn {
        match self.bits {
            false => GpioIntEn::B0,
            true => GpioIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GpioIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GpioIntEn::B1
    }
}
#[doc = "Field `GPIO_INT_EN` writer - gpio interrupt wakeup enable"]
pub type GpioIntEnW<'a, REG> = crate::BitWriter<'a, REG, GpioIntEn>;
impl<'a, REG> GpioIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GpioIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GpioIntEn::B1)
    }
}
#[doc = "sdio detect wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdioEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SdioEn> for bool {
    #[inline(always)]
    fn from(variant: SdioEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO_EN` reader - sdio detect wakeup enable"]
pub type SdioEnR = crate::BitReader<SdioEn>;
impl SdioEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SdioEn {
        match self.bits {
            false => SdioEn::B0,
            true => SdioEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SdioEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SdioEn::B1
    }
}
#[doc = "Field `SDIO_EN` writer - sdio detect wakeup enable"]
pub type SdioEnW<'a, REG> = crate::BitWriter<'a, REG, SdioEn>;
impl<'a, REG> SdioEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SdioEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SdioEn::B1)
    }
}
#[doc = "sdmmc detect wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdmmcEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SdmmcEn> for bool {
    #[inline(always)]
    fn from(variant: SdmmcEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMMC_EN` reader - sdmmc detect wakeup enable"]
pub type SdmmcEnR = crate::BitReader<SdmmcEn>;
impl SdmmcEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SdmmcEn {
        match self.bits {
            false => SdmmcEn::B0,
            true => SdmmcEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SdmmcEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SdmmcEn::B1
    }
}
#[doc = "Field `SDMMC_EN` writer - sdmmc detect wakeup enable"]
pub type SdmmcEnW<'a, REG> = crate::BitWriter<'a, REG, SdmmcEn>;
impl<'a, REG> SdmmcEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SdmmcEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SdmmcEn::B1)
    }
}
#[doc = "timer wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<TimerEn> for bool {
    #[inline(always)]
    fn from(variant: TimerEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER_EN` reader - timer wakeup enable"]
pub type TimerEnR = crate::BitReader<TimerEn>;
impl TimerEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimerEn {
        match self.bits {
            false => TimerEn::B0,
            true => TimerEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TimerEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TimerEn::B1
    }
}
#[doc = "Field `TIMER_EN` writer - timer wakeup enable"]
pub type TimerEnW<'a, REG> = crate::BitWriter<'a, REG, TimerEn>;
impl<'a, REG> TimerEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEn::B1)
    }
}
#[doc = "usb device detect wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbdevEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<UsbdevEn> for bool {
    #[inline(always)]
    fn from(variant: UsbdevEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBDEV_EN` reader - usb device detect wakeup enable"]
pub type UsbdevEnR = crate::BitReader<UsbdevEn>;
impl UsbdevEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbdevEn {
        match self.bits {
            false => UsbdevEn::B0,
            true => UsbdevEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == UsbdevEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == UsbdevEn::B1
    }
}
#[doc = "Field `USBDEV_EN` writer - usb device detect wakeup enable"]
pub type UsbdevEnW<'a, REG> = crate::BitWriter<'a, REG, UsbdevEn>;
impl<'a, REG> UsbdevEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(UsbdevEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(UsbdevEn::B1)
    }
}
#[doc = "software wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SftEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SftEn> for bool {
    #[inline(always)]
    fn from(variant: SftEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFT_EN` reader - software wakeup enable"]
pub type SftEnR = crate::BitReader<SftEn>;
impl SftEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SftEn {
        match self.bits {
            false => SftEn::B0,
            true => SftEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SftEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SftEn::B1
    }
}
#[doc = "Field `SFT_EN` writer - software wakeup enable"]
pub type SftEnW<'a, REG> = crate::BitWriter<'a, REG, SftEn>;
impl<'a, REG> SftEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SftEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SftEn::B1)
    }
}
#[doc = "m3 watch dog wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdtM0En {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<WdtM0En> for bool {
    #[inline(always)]
    fn from(variant: WdtM0En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT_M0_EN` reader - m3 watch dog wakeup enable"]
pub type WdtM0EnR = crate::BitReader<WdtM0En>;
impl WdtM0EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtM0En {
        match self.bits {
            false => WdtM0En::B0,
            true => WdtM0En::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WdtM0En::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WdtM0En::B1
    }
}
#[doc = "Field `WDT_M0_EN` writer - m3 watch dog wakeup enable"]
pub type WdtM0EnW<'a, REG> = crate::BitWriter<'a, REG, WdtM0En>;
impl<'a, REG> WdtM0EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WdtM0En::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WdtM0En::B1)
    }
}
#[doc = "pmu time out wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeoutEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<TimeoutEn> for bool {
    #[inline(always)]
    fn from(variant: TimeoutEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT_EN` reader - pmu time out wakeup enable"]
pub type TimeoutEnR = crate::BitReader<TimeoutEn>;
impl TimeoutEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimeoutEn {
        match self.bits {
            false => TimeoutEn::B0,
            true => TimeoutEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TimeoutEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TimeoutEn::B1
    }
}
#[doc = "Field `TIMEOUT_EN` writer - pmu time out wakeup enable"]
pub type TimeoutEnW<'a, REG> = crate::BitWriter<'a, REG, TimeoutEn>;
impl<'a, REG> TimeoutEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutEn::B1)
    }
}
#[doc = "pwm interrupt wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwmEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PwmEn> for bool {
    #[inline(always)]
    fn from(variant: PwmEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM_EN` reader - pwm interrupt wakeup enable"]
pub type PwmEnR = crate::BitReader<PwmEn>;
impl PwmEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwmEn {
        match self.bits {
            false => PwmEn::B0,
            true => PwmEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwmEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwmEn::B1
    }
}
#[doc = "Field `PWM_EN` writer - pwm interrupt wakeup enable"]
pub type PwmEnW<'a, REG> = crate::BitWriter<'a, REG, PwmEn>;
impl<'a, REG> PwmEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PwmEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PwmEn::B1)
    }
}
#[doc = "pcie interrupt wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcieEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PcieEn> for bool {
    #[inline(always)]
    fn from(variant: PcieEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCIE_EN` reader - pcie interrupt wakeup enable"]
pub type PcieEnR = crate::BitReader<PcieEn>;
impl PcieEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PcieEn {
        match self.bits {
            false => PcieEn::B0,
            true => PcieEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PcieEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PcieEn::B1
    }
}
#[doc = "Field `PCIE_EN` writer - pcie interrupt wakeup enable"]
pub type PcieEnW<'a, REG> = crate::BitWriter<'a, REG, PcieEn>;
impl<'a, REG> PcieEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PcieEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PcieEn::B1)
    }
}
impl R {
    #[doc = "Bit 0 - cluster_l interrupt wakeup enable"]
    #[inline(always)]
    pub fn int_cluster_l_en(&self) -> IntClusterLEnR {
        IntClusterLEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - cluster_b interrupt wakeup enable"]
    #[inline(always)]
    pub fn int_cluster_b_en(&self) -> IntClusterBEnR {
        IntClusterBEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - gpio interrupt wakeup enable"]
    #[inline(always)]
    pub fn gpio_int_en(&self) -> GpioIntEnR {
        GpioIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sdio detect wakeup enable"]
    #[inline(always)]
    pub fn sdio_en(&self) -> SdioEnR {
        SdioEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - sdmmc detect wakeup enable"]
    #[inline(always)]
    pub fn sdmmc_en(&self) -> SdmmcEnR {
        SdmmcEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - timer wakeup enable"]
    #[inline(always)]
    pub fn timer_en(&self) -> TimerEnR {
        TimerEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - usb device detect wakeup enable"]
    #[inline(always)]
    pub fn usbdev_en(&self) -> UsbdevEnR {
        UsbdevEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - software wakeup enable"]
    #[inline(always)]
    pub fn sft_en(&self) -> SftEnR {
        SftEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - m3 watch dog wakeup enable"]
    #[inline(always)]
    pub fn wdt_m0_en(&self) -> WdtM0EnR {
        WdtM0EnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - pmu time out wakeup enable"]
    #[inline(always)]
    pub fn timeout_en(&self) -> TimeoutEnR {
        TimeoutEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - pwm interrupt wakeup enable"]
    #[inline(always)]
    pub fn pwm_en(&self) -> PwmEnR {
        PwmEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - pcie interrupt wakeup enable"]
    #[inline(always)]
    pub fn pcie_en(&self) -> PcieEnR {
        PcieEnR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - cluster_l interrupt wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn int_cluster_l_en(&mut self) -> IntClusterLEnW<WakeupCfg4Spec> {
        IntClusterLEnW::new(self, 0)
    }
    #[doc = "Bit 1 - cluster_b interrupt wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn int_cluster_b_en(&mut self) -> IntClusterBEnW<WakeupCfg4Spec> {
        IntClusterBEnW::new(self, 1)
    }
    #[doc = "Bit 2 - gpio interrupt wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int_en(&mut self) -> GpioIntEnW<WakeupCfg4Spec> {
        GpioIntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - sdio detect wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_en(&mut self) -> SdioEnW<WakeupCfg4Spec> {
        SdioEnW::new(self, 3)
    }
    #[doc = "Bit 4 - sdmmc detect wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc_en(&mut self) -> SdmmcEnW<WakeupCfg4Spec> {
        SdmmcEnW::new(self, 4)
    }
    #[doc = "Bit 6 - timer wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer_en(&mut self) -> TimerEnW<WakeupCfg4Spec> {
        TimerEnW::new(self, 6)
    }
    #[doc = "Bit 7 - usb device detect wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbdev_en(&mut self) -> UsbdevEnW<WakeupCfg4Spec> {
        UsbdevEnW::new(self, 7)
    }
    #[doc = "Bit 8 - software wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn sft_en(&mut self) -> SftEnW<WakeupCfg4Spec> {
        SftEnW::new(self, 8)
    }
    #[doc = "Bit 9 - m3 watch dog wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_m0_en(&mut self) -> WdtM0EnW<WakeupCfg4Spec> {
        WdtM0EnW::new(self, 9)
    }
    #[doc = "Bit 10 - pmu time out wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_en(&mut self) -> TimeoutEnW<WakeupCfg4Spec> {
        TimeoutEnW::new(self, 10)
    }
    #[doc = "Bit 11 - pwm interrupt wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_en(&mut self) -> PwmEnW<WakeupCfg4Spec> {
        PwmEnW::new(self, 11)
    }
    #[doc = "Bit 13 - pcie interrupt wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcie_en(&mut self) -> PcieEnW<WakeupCfg4Spec> {
        PcieEnW::new(self, 13)
    }
}
#[doc = "pmu wakeup configure register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_cfg4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_cfg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WakeupCfg4Spec;
impl crate::RegisterSpec for WakeupCfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeup_cfg4::R`](R) reader structure"]
impl crate::Readable for WakeupCfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`wakeup_cfg4::W`](W) writer structure"]
impl crate::Writable for WakeupCfg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAKEUP_CFG4 to value 0"]
impl crate::Resettable for WakeupCfg4Spec {
    const RESET_VALUE: u32 = 0;
}
