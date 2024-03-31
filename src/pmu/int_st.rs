#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<IntStSpec>;
#[doc = "Register `INT_ST` writer"]
pub type W = crate::W<IntStSpec>;
#[doc = "power mode wakeup status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrmodeWakeupStatus {
    #[doc = "0: not wakeup from power mode"]
    B0 = 0,
    #[doc = "1: wakeup from power mode"]
    B1 = 1,
}
impl From<PwrmodeWakeupStatus> for bool {
    #[inline(always)]
    fn from(variant: PwrmodeWakeupStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRMODE_WAKEUP_STATUS` reader - power mode wakeup status"]
pub type PwrmodeWakeupStatusR = crate::BitReader<PwrmodeWakeupStatus>;
impl PwrmodeWakeupStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrmodeWakeupStatus {
        match self.bits {
            false => PwrmodeWakeupStatus::B0,
            true => PwrmodeWakeupStatus::B1,
        }
    }
    #[doc = "not wakeup from power mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwrmodeWakeupStatus::B0
    }
    #[doc = "wakeup from power mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwrmodeWakeupStatus::B1
    }
}
#[doc = "Field `PWRMODE_WAKEUP_STATUS` writer - power mode wakeup status"]
pub type PwrmodeWakeupStatusW<'a, REG> = crate::BitWriter<'a, REG, PwrmodeWakeupStatus>;
impl<'a, REG> PwrmodeWakeupStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not wakeup from power mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PwrmodeWakeupStatus::B0)
    }
    #[doc = "wakeup from power mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PwrmodeWakeupStatus::B1)
    }
}
#[doc = "gpio0 negedge pulse wakeup status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupGpio0NegStatus {
    #[doc = "0: not wakeup by gpio negedge pulse"]
    B0 = 0,
    #[doc = "1: wakeup by gpio negedge pulse"]
    B1 = 1,
}
impl From<WakeupGpio0NegStatus> for bool {
    #[inline(always)]
    fn from(variant: WakeupGpio0NegStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_GPIO0_NEG_STATUS` reader - gpio0 negedge pulse wakeup status"]
pub type WakeupGpio0NegStatusR = crate::BitReader<WakeupGpio0NegStatus>;
impl WakeupGpio0NegStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupGpio0NegStatus {
        match self.bits {
            false => WakeupGpio0NegStatus::B0,
            true => WakeupGpio0NegStatus::B1,
        }
    }
    #[doc = "not wakeup by gpio negedge pulse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupGpio0NegStatus::B0
    }
    #[doc = "wakeup by gpio negedge pulse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupGpio0NegStatus::B1
    }
}
#[doc = "Field `WAKEUP_GPIO0_NEG_STATUS` writer - gpio0 negedge pulse wakeup status"]
pub type WakeupGpio0NegStatusW<'a, REG> = crate::BitWriter<'a, REG, WakeupGpio0NegStatus>;
impl<'a, REG> WakeupGpio0NegStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not wakeup by gpio negedge pulse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupGpio0NegStatus::B0)
    }
    #[doc = "wakeup by gpio negedge pulse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupGpio0NegStatus::B1)
    }
}
#[doc = "gpio0 posedge pulse wakeup status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupGpio0PosStatus {
    #[doc = "0: not wakeup by gpio0 posedge pulse"]
    B0 = 0,
    #[doc = "1: wakeup by gpio0 posedge pulse"]
    B1 = 1,
}
impl From<WakeupGpio0PosStatus> for bool {
    #[inline(always)]
    fn from(variant: WakeupGpio0PosStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_GPIO0_POS_STATUS` reader - gpio0 posedge pulse wakeup status"]
pub type WakeupGpio0PosStatusR = crate::BitReader<WakeupGpio0PosStatus>;
impl WakeupGpio0PosStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupGpio0PosStatus {
        match self.bits {
            false => WakeupGpio0PosStatus::B0,
            true => WakeupGpio0PosStatus::B1,
        }
    }
    #[doc = "not wakeup by gpio0 posedge pulse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupGpio0PosStatus::B0
    }
    #[doc = "wakeup by gpio0 posedge pulse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupGpio0PosStatus::B1
    }
}
#[doc = "Field `WAKEUP_GPIO0_POS_STATUS` writer - gpio0 posedge pulse wakeup status"]
pub type WakeupGpio0PosStatusW<'a, REG> = crate::BitWriter<'a, REG, WakeupGpio0PosStatus>;
impl<'a, REG> WakeupGpio0PosStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not wakeup by gpio0 posedge pulse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupGpio0PosStatus::B0)
    }
    #[doc = "wakeup by gpio0 posedge pulse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupGpio0PosStatus::B1)
    }
}
#[doc = "gpio1 posedge pulse wakeup status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupGpio1PosStatus {
    #[doc = "0: not wakeup by gpio1 posedge pulse"]
    B0 = 0,
    #[doc = "1: wakeup by gpio1 posedge pulse"]
    B1 = 1,
}
impl From<WakeupGpio1PosStatus> for bool {
    #[inline(always)]
    fn from(variant: WakeupGpio1PosStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_GPIO1_POS_STATUS` reader - gpio1 posedge pulse wakeup status"]
pub type WakeupGpio1PosStatusR = crate::BitReader<WakeupGpio1PosStatus>;
impl WakeupGpio1PosStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupGpio1PosStatus {
        match self.bits {
            false => WakeupGpio1PosStatus::B0,
            true => WakeupGpio1PosStatus::B1,
        }
    }
    #[doc = "not wakeup by gpio1 posedge pulse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupGpio1PosStatus::B0
    }
    #[doc = "wakeup by gpio1 posedge pulse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupGpio1PosStatus::B1
    }
}
#[doc = "Field `WAKEUP_GPIO1_POS_STATUS` writer - gpio1 posedge pulse wakeup status"]
pub type WakeupGpio1PosStatusW<'a, REG> = crate::BitWriter<'a, REG, WakeupGpio1PosStatus>;
impl<'a, REG> WakeupGpio1PosStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not wakeup by gpio1 posedge pulse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupGpio1PosStatus::B0)
    }
    #[doc = "wakeup by gpio1 posedge pulse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupGpio1PosStatus::B1)
    }
}
#[doc = "gpio1 negedge pulse wakeup status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupGpio1NegStatus {
    #[doc = "0: not wakeup by gpio1 negedge pulse"]
    B0 = 0,
    #[doc = "1: wakeup by gpio1 negedge pulse"]
    B1 = 1,
}
impl From<WakeupGpio1NegStatus> for bool {
    #[inline(always)]
    fn from(variant: WakeupGpio1NegStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_GPIO1_NEG_STATUS` reader - gpio1 negedge pulse wakeup status"]
pub type WakeupGpio1NegStatusR = crate::BitReader<WakeupGpio1NegStatus>;
impl WakeupGpio1NegStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupGpio1NegStatus {
        match self.bits {
            false => WakeupGpio1NegStatus::B0,
            true => WakeupGpio1NegStatus::B1,
        }
    }
    #[doc = "not wakeup by gpio1 negedge pulse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupGpio1NegStatus::B0
    }
    #[doc = "wakeup by gpio1 negedge pulse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupGpio1NegStatus::B1
    }
}
#[doc = "Field `WAKEUP_GPIO1_NEG_STATUS` writer - gpio1 negedge pulse wakeup status"]
pub type WakeupGpio1NegStatusW<'a, REG> = crate::BitWriter<'a, REG, WakeupGpio1NegStatus>;
impl<'a, REG> WakeupGpio1NegStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not wakeup by gpio1 negedge pulse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupGpio1NegStatus::B0)
    }
    #[doc = "wakeup by gpio1 negedge pulse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupGpio1NegStatus::B1)
    }
}
impl R {
    #[doc = "Bit 1 - power mode wakeup status"]
    #[inline(always)]
    pub fn pwrmode_wakeup_status(&self) -> PwrmodeWakeupStatusR {
        PwrmodeWakeupStatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - gpio0 negedge pulse wakeup status"]
    #[inline(always)]
    pub fn wakeup_gpio0_neg_status(&self) -> WakeupGpio0NegStatusR {
        WakeupGpio0NegStatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - gpio0 posedge pulse wakeup status"]
    #[inline(always)]
    pub fn wakeup_gpio0_pos_status(&self) -> WakeupGpio0PosStatusR {
        WakeupGpio0PosStatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - gpio1 posedge pulse wakeup status"]
    #[inline(always)]
    pub fn wakeup_gpio1_pos_status(&self) -> WakeupGpio1PosStatusR {
        WakeupGpio1PosStatusR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - gpio1 negedge pulse wakeup status"]
    #[inline(always)]
    pub fn wakeup_gpio1_neg_status(&self) -> WakeupGpio1NegStatusR {
        WakeupGpio1NegStatusR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - power mode wakeup status"]
    #[inline(always)]
    #[must_use]
    pub fn pwrmode_wakeup_status(&mut self) -> PwrmodeWakeupStatusW<IntStSpec> {
        PwrmodeWakeupStatusW::new(self, 1)
    }
    #[doc = "Bit 2 - gpio0 negedge pulse wakeup status"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_gpio0_neg_status(&mut self) -> WakeupGpio0NegStatusW<IntStSpec> {
        WakeupGpio0NegStatusW::new(self, 2)
    }
    #[doc = "Bit 3 - gpio0 posedge pulse wakeup status"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_gpio0_pos_status(&mut self) -> WakeupGpio0PosStatusW<IntStSpec> {
        WakeupGpio0PosStatusW::new(self, 3)
    }
    #[doc = "Bit 4 - gpio1 posedge pulse wakeup status"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_gpio1_pos_status(&mut self) -> WakeupGpio1PosStatusW<IntStSpec> {
        WakeupGpio1PosStatusW::new(self, 4)
    }
    #[doc = "Bit 5 - gpio1 negedge pulse wakeup status"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_gpio1_neg_status(&mut self) -> WakeupGpio1NegStatusW<IntStSpec> {
        WakeupGpio1NegStatusW::new(self, 5)
    }
}
#[doc = "pmu interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStSpec;
impl crate::RegisterSpec for IntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for IntStSpec {}
#[doc = "`write(|w| ..)` method takes [`int_st::W`](W) writer structure"]
impl crate::Writable for IntStSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for IntStSpec {
    const RESET_VALUE: u32 = 0;
}
