#[doc = "Register `PMU_WAKEUP_STATUS` reader"]
pub type R = crate::R<PmuWakeupStatusSpec>;
#[doc = "Register `PMU_WAKEUP_STATUS` writer"]
pub type W = crate::W<PmuWakeupStatusSpec>;
#[doc = "cluster_l interrupt wakeup status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupIntClusterLStatus {
    #[doc = "0: not wakeup by interrupt cluster_l"]
    B0 = 0,
    #[doc = "1: wakeup by interrupt cluster_l"]
    B1 = 1,
}
impl From<WakeupIntClusterLStatus> for bool {
    #[inline(always)]
    fn from(variant: WakeupIntClusterLStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_INT_CLUSTER_L_STATUS` reader - cluster_l interrupt wakeup status"]
pub type WakeupIntClusterLStatusR = crate::BitReader<WakeupIntClusterLStatus>;
impl WakeupIntClusterLStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupIntClusterLStatus {
        match self.bits {
            false => WakeupIntClusterLStatus::B0,
            true => WakeupIntClusterLStatus::B1,
        }
    }
    #[doc = "not wakeup by interrupt cluster_l"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupIntClusterLStatus::B0
    }
    #[doc = "wakeup by interrupt cluster_l"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupIntClusterLStatus::B1
    }
}
#[doc = "Field `WAKEUP_INT_CLUSTER_L_STATUS` writer - cluster_l interrupt wakeup status"]
pub type WakeupIntClusterLStatusW<'a, REG> = crate::BitWriter<'a, REG, WakeupIntClusterLStatus>;
impl<'a, REG> WakeupIntClusterLStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not wakeup by interrupt cluster_l"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupIntClusterLStatus::B0)
    }
    #[doc = "wakeup by interrupt cluster_l"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupIntClusterLStatus::B1)
    }
}
#[doc = "cluster_b interrupt wakeup status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupIntClusterBStatus {
    #[doc = "0: not wakeup by interrupt cluster_b"]
    B0 = 0,
    #[doc = "1: wakeup by interrupt cluster_b"]
    B1 = 1,
}
impl From<WakeupIntClusterBStatus> for bool {
    #[inline(always)]
    fn from(variant: WakeupIntClusterBStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_INT_CLUSTER_B_STATUS` reader - cluster_b interrupt wakeup status"]
pub type WakeupIntClusterBStatusR = crate::BitReader<WakeupIntClusterBStatus>;
impl WakeupIntClusterBStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupIntClusterBStatus {
        match self.bits {
            false => WakeupIntClusterBStatus::B0,
            true => WakeupIntClusterBStatus::B1,
        }
    }
    #[doc = "not wakeup by interrupt cluster_b"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupIntClusterBStatus::B0
    }
    #[doc = "wakeup by interrupt cluster_b"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupIntClusterBStatus::B1
    }
}
#[doc = "Field `WAKEUP_INT_CLUSTER_B_STATUS` writer - cluster_b interrupt wakeup status"]
pub type WakeupIntClusterBStatusW<'a, REG> = crate::BitWriter<'a, REG, WakeupIntClusterBStatus>;
impl<'a, REG> WakeupIntClusterBStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not wakeup by interrupt cluster_b"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupIntClusterBStatus::B0)
    }
    #[doc = "wakeup by interrupt cluster_b"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupIntClusterBStatus::B1)
    }
}
#[doc = "gpio interrupt wakeup status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupGpioIntStatus {
    #[doc = "0: not wakeup by gpio int"]
    B0 = 0,
    #[doc = "1: wakeup by gpio int"]
    B1 = 1,
}
impl From<WakeupGpioIntStatus> for bool {
    #[inline(always)]
    fn from(variant: WakeupGpioIntStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_GPIO_INT_STATUS` reader - gpio interrupt wakeup status"]
pub type WakeupGpioIntStatusR = crate::BitReader<WakeupGpioIntStatus>;
impl WakeupGpioIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupGpioIntStatus {
        match self.bits {
            false => WakeupGpioIntStatus::B0,
            true => WakeupGpioIntStatus::B1,
        }
    }
    #[doc = "not wakeup by gpio int"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupGpioIntStatus::B0
    }
    #[doc = "wakeup by gpio int"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupGpioIntStatus::B1
    }
}
#[doc = "Field `WAKEUP_GPIO_INT_STATUS` writer - gpio interrupt wakeup status"]
pub type WakeupGpioIntStatusW<'a, REG> = crate::BitWriter<'a, REG, WakeupGpioIntStatus>;
impl<'a, REG> WakeupGpioIntStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not wakeup by gpio int"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupGpioIntStatus::B0)
    }
    #[doc = "wakeup by gpio int"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupGpioIntStatus::B1)
    }
}
#[doc = "sdio wakeup status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupSdioStatus {
    #[doc = "0: not wakeup by sdio detect"]
    B0 = 0,
    #[doc = "1: wakeup by sdio detect"]
    B1 = 1,
}
impl From<WakeupSdioStatus> for bool {
    #[inline(always)]
    fn from(variant: WakeupSdioStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_SDIO_STATUS` reader - sdio wakeup status"]
pub type WakeupSdioStatusR = crate::BitReader<WakeupSdioStatus>;
impl WakeupSdioStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupSdioStatus {
        match self.bits {
            false => WakeupSdioStatus::B0,
            true => WakeupSdioStatus::B1,
        }
    }
    #[doc = "not wakeup by sdio detect"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupSdioStatus::B0
    }
    #[doc = "wakeup by sdio detect"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupSdioStatus::B1
    }
}
#[doc = "Field `WAKEUP_SDIO_STATUS` writer - sdio wakeup status"]
pub type WakeupSdioStatusW<'a, REG> = crate::BitWriter<'a, REG, WakeupSdioStatus>;
impl<'a, REG> WakeupSdioStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not wakeup by sdio detect"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupSdioStatus::B0)
    }
    #[doc = "wakeup by sdio detect"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupSdioStatus::B1)
    }
}
#[doc = "sdmmc wakeup status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupSdmmcStatus {
    #[doc = "0: not wakeup by sdmmc detect"]
    B0 = 0,
    #[doc = "1: wakeup by sdmmc detect"]
    B1 = 1,
}
impl From<WakeupSdmmcStatus> for bool {
    #[inline(always)]
    fn from(variant: WakeupSdmmcStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_SDMMC_STATUS` reader - sdmmc wakeup status"]
pub type WakeupSdmmcStatusR = crate::BitReader<WakeupSdmmcStatus>;
impl WakeupSdmmcStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupSdmmcStatus {
        match self.bits {
            false => WakeupSdmmcStatus::B0,
            true => WakeupSdmmcStatus::B1,
        }
    }
    #[doc = "not wakeup by sdmmc detect"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupSdmmcStatus::B0
    }
    #[doc = "wakeup by sdmmc detect"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupSdmmcStatus::B1
    }
}
#[doc = "Field `WAKEUP_SDMMC_STATUS` writer - sdmmc wakeup status"]
pub type WakeupSdmmcStatusW<'a, REG> = crate::BitWriter<'a, REG, WakeupSdmmcStatus>;
impl<'a, REG> WakeupSdmmcStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not wakeup by sdmmc detect"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupSdmmcStatus::B0)
    }
    #[doc = "wakeup by sdmmc detect"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupSdmmcStatus::B1)
    }
}
#[doc = "timer wakeup status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupTimerStatus {
    #[doc = "0: not wakeup by timer"]
    B0 = 0,
    #[doc = "1: wakeup by timer"]
    B1 = 1,
}
impl From<WakeupTimerStatus> for bool {
    #[inline(always)]
    fn from(variant: WakeupTimerStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_TIMER_STATUS` reader - timer wakeup status"]
pub type WakeupTimerStatusR = crate::BitReader<WakeupTimerStatus>;
impl WakeupTimerStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupTimerStatus {
        match self.bits {
            false => WakeupTimerStatus::B0,
            true => WakeupTimerStatus::B1,
        }
    }
    #[doc = "not wakeup by timer"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupTimerStatus::B0
    }
    #[doc = "wakeup by timer"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupTimerStatus::B1
    }
}
#[doc = "Field `WAKEUP_TIMER_STATUS` writer - timer wakeup status"]
pub type WakeupTimerStatusW<'a, REG> = crate::BitWriter<'a, REG, WakeupTimerStatus>;
impl<'a, REG> WakeupTimerStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not wakeup by timer"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupTimerStatus::B0)
    }
    #[doc = "wakeup by timer"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupTimerStatus::B1)
    }
}
#[doc = "usbdev detect wakeup status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupUsbdevStatus {
    #[doc = "0: not wakeup by usbdev detect"]
    B0 = 0,
    #[doc = "1: wakeup by usbdev detect"]
    B1 = 1,
}
impl From<WakeupUsbdevStatus> for bool {
    #[inline(always)]
    fn from(variant: WakeupUsbdevStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_USBDEV_STATUS` reader - usbdev detect wakeup status"]
pub type WakeupUsbdevStatusR = crate::BitReader<WakeupUsbdevStatus>;
impl WakeupUsbdevStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupUsbdevStatus {
        match self.bits {
            false => WakeupUsbdevStatus::B0,
            true => WakeupUsbdevStatus::B1,
        }
    }
    #[doc = "not wakeup by usbdev detect"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupUsbdevStatus::B0
    }
    #[doc = "wakeup by usbdev detect"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupUsbdevStatus::B1
    }
}
#[doc = "Field `WAKEUP_USBDEV_STATUS` writer - usbdev detect wakeup status"]
pub type WakeupUsbdevStatusW<'a, REG> = crate::BitWriter<'a, REG, WakeupUsbdevStatus>;
impl<'a, REG> WakeupUsbdevStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not wakeup by usbdev detect"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupUsbdevStatus::B0)
    }
    #[doc = "wakeup by usbdev detect"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupUsbdevStatus::B1)
    }
}
#[doc = "m0 software control wakeup status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupSftM0Status {
    #[doc = "0: not wakeup by software"]
    B0 = 0,
    #[doc = "1: wakeup by software"]
    B1 = 1,
}
impl From<WakeupSftM0Status> for bool {
    #[inline(always)]
    fn from(variant: WakeupSftM0Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_SFT_M0_STATUS` reader - m0 software control wakeup status"]
pub type WakeupSftM0StatusR = crate::BitReader<WakeupSftM0Status>;
impl WakeupSftM0StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupSftM0Status {
        match self.bits {
            false => WakeupSftM0Status::B0,
            true => WakeupSftM0Status::B1,
        }
    }
    #[doc = "not wakeup by software"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupSftM0Status::B0
    }
    #[doc = "wakeup by software"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupSftM0Status::B1
    }
}
#[doc = "Field `WAKEUP_SFT_M0_STATUS` writer - m0 software control wakeup status"]
pub type WakeupSftM0StatusW<'a, REG> = crate::BitWriter<'a, REG, WakeupSftM0Status>;
impl<'a, REG> WakeupSftM0StatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not wakeup by software"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupSftM0Status::B0)
    }
    #[doc = "wakeup by software"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupSftM0Status::B1)
    }
}
#[doc = "m0 wdt interrupt wakeup status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupWdtM0Status {
    #[doc = "0: not wakeup by m0 wdt interrupt"]
    B0 = 0,
    #[doc = "1: wakeup by m0 wdt interrupt"]
    B1 = 1,
}
impl From<WakeupWdtM0Status> for bool {
    #[inline(always)]
    fn from(variant: WakeupWdtM0Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_WDT_M0_STATUS` reader - m0 wdt interrupt wakeup status"]
pub type WakeupWdtM0StatusR = crate::BitReader<WakeupWdtM0Status>;
impl WakeupWdtM0StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupWdtM0Status {
        match self.bits {
            false => WakeupWdtM0Status::B0,
            true => WakeupWdtM0Status::B1,
        }
    }
    #[doc = "not wakeup by m0 wdt interrupt"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupWdtM0Status::B0
    }
    #[doc = "wakeup by m0 wdt interrupt"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupWdtM0Status::B1
    }
}
#[doc = "Field `WAKEUP_WDT_M0_STATUS` writer - m0 wdt interrupt wakeup status"]
pub type WakeupWdtM0StatusW<'a, REG> = crate::BitWriter<'a, REG, WakeupWdtM0Status>;
impl<'a, REG> WakeupWdtM0StatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not wakeup by m0 wdt interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupWdtM0Status::B0)
    }
    #[doc = "wakeup by m0 wdt interrupt"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupWdtM0Status::B1)
    }
}
#[doc = "timeout wakeup status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupTimeoutStatus {
    #[doc = "0: not wakeup by timeout"]
    B0 = 0,
    #[doc = "1: wakeup by timeout"]
    B1 = 1,
}
impl From<WakeupTimeoutStatus> for bool {
    #[inline(always)]
    fn from(variant: WakeupTimeoutStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_TIMEOUT_STATUS` reader - timeout wakeup status"]
pub type WakeupTimeoutStatusR = crate::BitReader<WakeupTimeoutStatus>;
impl WakeupTimeoutStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupTimeoutStatus {
        match self.bits {
            false => WakeupTimeoutStatus::B0,
            true => WakeupTimeoutStatus::B1,
        }
    }
    #[doc = "not wakeup by timeout"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupTimeoutStatus::B0
    }
    #[doc = "wakeup by timeout"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupTimeoutStatus::B1
    }
}
#[doc = "Field `WAKEUP_TIMEOUT_STATUS` writer - timeout wakeup status"]
pub type WakeupTimeoutStatusW<'a, REG> = crate::BitWriter<'a, REG, WakeupTimeoutStatus>;
impl<'a, REG> WakeupTimeoutStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not wakeup by timeout"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupTimeoutStatus::B0)
    }
    #[doc = "wakeup by timeout"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupTimeoutStatus::B1)
    }
}
#[doc = "pwm wakeup status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupPwmStatus {
    #[doc = "0: not wakeup by pwm"]
    B0 = 0,
    #[doc = "1: wakeup by pwm"]
    B1 = 1,
}
impl From<WakeupPwmStatus> for bool {
    #[inline(always)]
    fn from(variant: WakeupPwmStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_PWM_STATUS` reader - pwm wakeup status"]
pub type WakeupPwmStatusR = crate::BitReader<WakeupPwmStatus>;
impl WakeupPwmStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupPwmStatus {
        match self.bits {
            false => WakeupPwmStatus::B0,
            true => WakeupPwmStatus::B1,
        }
    }
    #[doc = "not wakeup by pwm"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupPwmStatus::B0
    }
    #[doc = "wakeup by pwm"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupPwmStatus::B1
    }
}
#[doc = "Field `WAKEUP_PWM_STATUS` writer - pwm wakeup status"]
pub type WakeupPwmStatusW<'a, REG> = crate::BitWriter<'a, REG, WakeupPwmStatus>;
impl<'a, REG> WakeupPwmStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not wakeup by pwm"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupPwmStatus::B0)
    }
    #[doc = "wakeup by pwm"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupPwmStatus::B1)
    }
}
#[doc = "pcie wakeup status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupPcieStatus {
    #[doc = "0: not wakeup by pcie"]
    B0 = 0,
    #[doc = "1: wakeup by pcie"]
    B1 = 1,
}
impl From<WakeupPcieStatus> for bool {
    #[inline(always)]
    fn from(variant: WakeupPcieStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_PCIE_STATUS` reader - pcie wakeup status"]
pub type WakeupPcieStatusR = crate::BitReader<WakeupPcieStatus>;
impl WakeupPcieStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupPcieStatus {
        match self.bits {
            false => WakeupPcieStatus::B0,
            true => WakeupPcieStatus::B1,
        }
    }
    #[doc = "not wakeup by pcie"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupPcieStatus::B0
    }
    #[doc = "wakeup by pcie"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupPcieStatus::B1
    }
}
#[doc = "Field `WAKEUP_PCIE_STATUS` writer - pcie wakeup status"]
pub type WakeupPcieStatusW<'a, REG> = crate::BitWriter<'a, REG, WakeupPcieStatus>;
impl<'a, REG> WakeupPcieStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not wakeup by pcie"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupPcieStatus::B0)
    }
    #[doc = "wakeup by pcie"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupPcieStatus::B1)
    }
}
impl R {
    #[doc = "Bit 0 - cluster_l interrupt wakeup status"]
    #[inline(always)]
    pub fn wakeup_int_cluster_l_status(&self) -> WakeupIntClusterLStatusR {
        WakeupIntClusterLStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - cluster_b interrupt wakeup status"]
    #[inline(always)]
    pub fn wakeup_int_cluster_b_status(&self) -> WakeupIntClusterBStatusR {
        WakeupIntClusterBStatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - gpio interrupt wakeup status"]
    #[inline(always)]
    pub fn wakeup_gpio_int_status(&self) -> WakeupGpioIntStatusR {
        WakeupGpioIntStatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sdio wakeup status"]
    #[inline(always)]
    pub fn wakeup_sdio_status(&self) -> WakeupSdioStatusR {
        WakeupSdioStatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - sdmmc wakeup status"]
    #[inline(always)]
    pub fn wakeup_sdmmc_status(&self) -> WakeupSdmmcStatusR {
        WakeupSdmmcStatusR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - timer wakeup status"]
    #[inline(always)]
    pub fn wakeup_timer_status(&self) -> WakeupTimerStatusR {
        WakeupTimerStatusR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - usbdev detect wakeup status"]
    #[inline(always)]
    pub fn wakeup_usbdev_status(&self) -> WakeupUsbdevStatusR {
        WakeupUsbdevStatusR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - m0 software control wakeup status"]
    #[inline(always)]
    pub fn wakeup_sft_m0_status(&self) -> WakeupSftM0StatusR {
        WakeupSftM0StatusR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - m0 wdt interrupt wakeup status"]
    #[inline(always)]
    pub fn wakeup_wdt_m0_status(&self) -> WakeupWdtM0StatusR {
        WakeupWdtM0StatusR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - timeout wakeup status"]
    #[inline(always)]
    pub fn wakeup_timeout_status(&self) -> WakeupTimeoutStatusR {
        WakeupTimeoutStatusR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - pwm wakeup status"]
    #[inline(always)]
    pub fn wakeup_pwm_status(&self) -> WakeupPwmStatusR {
        WakeupPwmStatusR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - pcie wakeup status"]
    #[inline(always)]
    pub fn wakeup_pcie_status(&self) -> WakeupPcieStatusR {
        WakeupPcieStatusR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - cluster_l interrupt wakeup status"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_int_cluster_l_status(&mut self) -> WakeupIntClusterLStatusW<PmuWakeupStatusSpec> {
        WakeupIntClusterLStatusW::new(self, 0)
    }
    #[doc = "Bit 1 - cluster_b interrupt wakeup status"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_int_cluster_b_status(&mut self) -> WakeupIntClusterBStatusW<PmuWakeupStatusSpec> {
        WakeupIntClusterBStatusW::new(self, 1)
    }
    #[doc = "Bit 2 - gpio interrupt wakeup status"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_gpio_int_status(&mut self) -> WakeupGpioIntStatusW<PmuWakeupStatusSpec> {
        WakeupGpioIntStatusW::new(self, 2)
    }
    #[doc = "Bit 3 - sdio wakeup status"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_sdio_status(&mut self) -> WakeupSdioStatusW<PmuWakeupStatusSpec> {
        WakeupSdioStatusW::new(self, 3)
    }
    #[doc = "Bit 4 - sdmmc wakeup status"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_sdmmc_status(&mut self) -> WakeupSdmmcStatusW<PmuWakeupStatusSpec> {
        WakeupSdmmcStatusW::new(self, 4)
    }
    #[doc = "Bit 6 - timer wakeup status"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_timer_status(&mut self) -> WakeupTimerStatusW<PmuWakeupStatusSpec> {
        WakeupTimerStatusW::new(self, 6)
    }
    #[doc = "Bit 7 - usbdev detect wakeup status"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_usbdev_status(&mut self) -> WakeupUsbdevStatusW<PmuWakeupStatusSpec> {
        WakeupUsbdevStatusW::new(self, 7)
    }
    #[doc = "Bit 8 - m0 software control wakeup status"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_sft_m0_status(&mut self) -> WakeupSftM0StatusW<PmuWakeupStatusSpec> {
        WakeupSftM0StatusW::new(self, 8)
    }
    #[doc = "Bit 9 - m0 wdt interrupt wakeup status"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_wdt_m0_status(&mut self) -> WakeupWdtM0StatusW<PmuWakeupStatusSpec> {
        WakeupWdtM0StatusW::new(self, 9)
    }
    #[doc = "Bit 10 - timeout wakeup status"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_timeout_status(&mut self) -> WakeupTimeoutStatusW<PmuWakeupStatusSpec> {
        WakeupTimeoutStatusW::new(self, 10)
    }
    #[doc = "Bit 11 - pwm wakeup status"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_pwm_status(&mut self) -> WakeupPwmStatusW<PmuWakeupStatusSpec> {
        WakeupPwmStatusW::new(self, 11)
    }
    #[doc = "Bit 13 - pcie wakeup status"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_pcie_status(&mut self) -> WakeupPcieStatusW<PmuWakeupStatusSpec> {
        WakeupPcieStatusW::new(self, 13)
    }
}
#[doc = "pmu interrupt wakeup status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_wakeup_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_wakeup_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuWakeupStatusSpec;
impl crate::RegisterSpec for PmuWakeupStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_wakeup_status::R`](R) reader structure"]
impl crate::Readable for PmuWakeupStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_wakeup_status::W`](W) writer structure"]
impl crate::Writable for PmuWakeupStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_WAKEUP_STATUS to value 0"]
impl crate::Resettable for PmuWakeupStatusSpec {
    const RESET_VALUE: u32 = 0;
}
