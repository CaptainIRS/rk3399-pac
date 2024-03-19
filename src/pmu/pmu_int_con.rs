#[doc = "Register `PMU_INT_CON` reader"]
pub type R = crate::R<PmuIntConSpec>;
#[doc = "Register `PMU_INT_CON` writer"]
pub type W = crate::W<PmuIntConSpec>;
#[doc = "global interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmuIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PmuIntEn> for bool {
    #[inline(always)]
    fn from(variant: PmuIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMU_INT_EN` reader - global interrupt enable"]
pub type PmuIntEnR = crate::BitReader<PmuIntEn>;
impl PmuIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PmuIntEn {
        match self.bits {
            false => PmuIntEn::B0,
            true => PmuIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PmuIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PmuIntEn::B1
    }
}
#[doc = "Field `PMU_INT_EN` writer - global interrupt enable"]
pub type PmuIntEnW<'a, REG> = crate::BitWriter<'a, REG, PmuIntEn>;
impl<'a, REG> PmuIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PmuIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PmuIntEn::B1)
    }
}
#[doc = "power mode wakeup interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrmodeWakeupIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PwrmodeWakeupIntEn> for bool {
    #[inline(always)]
    fn from(variant: PwrmodeWakeupIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRMODE_WAKEUP_INT_EN` reader - power mode wakeup interrupt enable"]
pub type PwrmodeWakeupIntEnR = crate::BitReader<PwrmodeWakeupIntEn>;
impl PwrmodeWakeupIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrmodeWakeupIntEn {
        match self.bits {
            false => PwrmodeWakeupIntEn::B0,
            true => PwrmodeWakeupIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwrmodeWakeupIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwrmodeWakeupIntEn::B1
    }
}
#[doc = "Field `PWRMODE_WAKEUP_INT_EN` writer - power mode wakeup interrupt enable"]
pub type PwrmodeWakeupIntEnW<'a, REG> = crate::BitWriter<'a, REG, PwrmodeWakeupIntEn>;
impl<'a, REG> PwrmodeWakeupIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PwrmodeWakeupIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PwrmodeWakeupIntEn::B1)
    }
}
#[doc = "gpio0 negedge wakeup interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupGpio0NegIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<WakeupGpio0NegIntEn> for bool {
    #[inline(always)]
    fn from(variant: WakeupGpio0NegIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_GPIO0_NEG_INT_EN` reader - gpio0 negedge wakeup interrupt enable"]
pub type WakeupGpio0NegIntEnR = crate::BitReader<WakeupGpio0NegIntEn>;
impl WakeupGpio0NegIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupGpio0NegIntEn {
        match self.bits {
            false => WakeupGpio0NegIntEn::B0,
            true => WakeupGpio0NegIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupGpio0NegIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupGpio0NegIntEn::B1
    }
}
#[doc = "Field `WAKEUP_GPIO0_NEG_INT_EN` writer - gpio0 negedge wakeup interrupt enable"]
pub type WakeupGpio0NegIntEnW<'a, REG> = crate::BitWriter<'a, REG, WakeupGpio0NegIntEn>;
impl<'a, REG> WakeupGpio0NegIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupGpio0NegIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupGpio0NegIntEn::B1)
    }
}
#[doc = "gpio posedge wakeup interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupGpio0PosIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<WakeupGpio0PosIntEn> for bool {
    #[inline(always)]
    fn from(variant: WakeupGpio0PosIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_GPIO0_POS_INT_EN` reader - gpio posedge wakeup interrupt enable"]
pub type WakeupGpio0PosIntEnR = crate::BitReader<WakeupGpio0PosIntEn>;
impl WakeupGpio0PosIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupGpio0PosIntEn {
        match self.bits {
            false => WakeupGpio0PosIntEn::B0,
            true => WakeupGpio0PosIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupGpio0PosIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupGpio0PosIntEn::B1
    }
}
#[doc = "Field `WAKEUP_GPIO0_POS_INT_EN` writer - gpio posedge wakeup interrupt enable"]
pub type WakeupGpio0PosIntEnW<'a, REG> = crate::BitWriter<'a, REG, WakeupGpio0PosIntEn>;
impl<'a, REG> WakeupGpio0PosIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupGpio0PosIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupGpio0PosIntEn::B1)
    }
}
#[doc = "gpio1 negedge wakeup interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupGpio1NegIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<WakeupGpio1NegIntEn> for bool {
    #[inline(always)]
    fn from(variant: WakeupGpio1NegIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_GPIO1_NEG_INT_EN` reader - gpio1 negedge wakeup interrupt enable"]
pub type WakeupGpio1NegIntEnR = crate::BitReader<WakeupGpio1NegIntEn>;
impl WakeupGpio1NegIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupGpio1NegIntEn {
        match self.bits {
            false => WakeupGpio1NegIntEn::B0,
            true => WakeupGpio1NegIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupGpio1NegIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupGpio1NegIntEn::B1
    }
}
#[doc = "Field `WAKEUP_GPIO1_NEG_INT_EN` writer - gpio1 negedge wakeup interrupt enable"]
pub type WakeupGpio1NegIntEnW<'a, REG> = crate::BitWriter<'a, REG, WakeupGpio1NegIntEn>;
impl<'a, REG> WakeupGpio1NegIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupGpio1NegIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupGpio1NegIntEn::B1)
    }
}
#[doc = "gpio1 posedge wakeup interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupGpio1PosIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<WakeupGpio1PosIntEn> for bool {
    #[inline(always)]
    fn from(variant: WakeupGpio1PosIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_GPIO1_POS_INT_EN` reader - gpio1 posedge wakeup interrupt enable"]
pub type WakeupGpio1PosIntEnR = crate::BitReader<WakeupGpio1PosIntEn>;
impl WakeupGpio1PosIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupGpio1PosIntEn {
        match self.bits {
            false => WakeupGpio1PosIntEn::B0,
            true => WakeupGpio1PosIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WakeupGpio1PosIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WakeupGpio1PosIntEn::B1
    }
}
#[doc = "Field `WAKEUP_GPIO1_POS_INT_EN` writer - gpio1 posedge wakeup interrupt enable"]
pub type WakeupGpio1PosIntEnW<'a, REG> = crate::BitWriter<'a, REG, WakeupGpio1PosIntEn>;
impl<'a, REG> WakeupGpio1PosIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupGpio1PosIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupGpio1PosIntEn::B1)
    }
}
impl R {
    #[doc = "Bit 0 - global interrupt enable"]
    #[inline(always)]
    pub fn pmu_int_en(&self) -> PmuIntEnR {
        PmuIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - power mode wakeup interrupt enable"]
    #[inline(always)]
    pub fn pwrmode_wakeup_int_en(&self) -> PwrmodeWakeupIntEnR {
        PwrmodeWakeupIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - gpio0 negedge wakeup interrupt enable"]
    #[inline(always)]
    pub fn wakeup_gpio0_neg_int_en(&self) -> WakeupGpio0NegIntEnR {
        WakeupGpio0NegIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - gpio posedge wakeup interrupt enable"]
    #[inline(always)]
    pub fn wakeup_gpio0_pos_int_en(&self) -> WakeupGpio0PosIntEnR {
        WakeupGpio0PosIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - gpio1 negedge wakeup interrupt enable"]
    #[inline(always)]
    pub fn wakeup_gpio1_neg_int_en(&self) -> WakeupGpio1NegIntEnR {
        WakeupGpio1NegIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - gpio1 posedge wakeup interrupt enable"]
    #[inline(always)]
    pub fn wakeup_gpio1_pos_int_en(&self) -> WakeupGpio1PosIntEnR {
        WakeupGpio1PosIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - global interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_int_en(&mut self) -> PmuIntEnW<PmuIntConSpec> {
        PmuIntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - power mode wakeup interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwrmode_wakeup_int_en(&mut self) -> PwrmodeWakeupIntEnW<PmuIntConSpec> {
        PwrmodeWakeupIntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - gpio0 negedge wakeup interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_gpio0_neg_int_en(&mut self) -> WakeupGpio0NegIntEnW<PmuIntConSpec> {
        WakeupGpio0NegIntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - gpio posedge wakeup interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_gpio0_pos_int_en(&mut self) -> WakeupGpio0PosIntEnW<PmuIntConSpec> {
        WakeupGpio0PosIntEnW::new(self, 3)
    }
    #[doc = "Bit 4 - gpio1 negedge wakeup interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_gpio1_neg_int_en(&mut self) -> WakeupGpio1NegIntEnW<PmuIntConSpec> {
        WakeupGpio1NegIntEnW::new(self, 4)
    }
    #[doc = "Bit 5 - gpio1 posedge wakeup interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_gpio1_pos_int_en(&mut self) -> WakeupGpio1PosIntEnW<PmuIntConSpec> {
        WakeupGpio1PosIntEnW::new(self, 5)
    }
}
#[doc = "pmu interrupt configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_int_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_int_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuIntConSpec;
impl crate::RegisterSpec for PmuIntConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_int_con::R`](R) reader structure"]
impl crate::Readable for PmuIntConSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_int_con::W`](W) writer structure"]
impl crate::Writable for PmuIntConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_INT_CON to value 0"]
impl crate::Resettable for PmuIntConSpec {
    const RESET_VALUE: u32 = 0;
}
