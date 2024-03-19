#[doc = "Register `CCI500_SYS_CTRL` reader"]
pub type R = crate::R<Cci500SysCtrlSpec>;
#[doc = "Register `CCI500_SYS_CTRL` writer"]
pub type W = crate::W<Cci500SysCtrlSpec>;
#[doc = "snoop_disable control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnoopDisable {
    #[doc = "0: Snoop masters according to the Snoop Control Registers"]
    B0 = 0,
    #[doc = "1: Disable all snoops, but not DVM messages"]
    B1 = 1,
}
impl From<SnoopDisable> for bool {
    #[inline(always)]
    fn from(variant: SnoopDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNOOP_DISABLE` reader - snoop_disable control"]
pub type SnoopDisableR = crate::BitReader<SnoopDisable>;
impl SnoopDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SnoopDisable {
        match self.bits {
            false => SnoopDisable::B0,
            true => SnoopDisable::B1,
        }
    }
    #[doc = "Snoop masters according to the Snoop Control Registers"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SnoopDisable::B0
    }
    #[doc = "Disable all snoops, but not DVM messages"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SnoopDisable::B1
    }
}
#[doc = "Field `SNOOP_DISABLE` writer - snoop_disable control"]
pub type SnoopDisableW<'a, REG> = crate::BitWriter<'a, REG, SnoopDisable>;
impl<'a, REG> SnoopDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Snoop masters according to the Snoop Control Registers"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SnoopDisable::B0)
    }
    #[doc = "Disable all snoops, but not DVM messages"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SnoopDisable::B1)
    }
}
#[doc = "DVM message disable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DvmDisable {
    #[doc = "0: Send speculative fetches according to the Speculation Control Register"]
    B0 = 0,
    #[doc = "1: Disable speculative fetches from all master interfaces"]
    B1 = 1,
}
impl From<DvmDisable> for bool {
    #[inline(always)]
    fn from(variant: DvmDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DVM_DISABLE` reader - DVM message disable"]
pub type DvmDisableR = crate::BitReader<DvmDisable>;
impl DvmDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DvmDisable {
        match self.bits {
            false => DvmDisable::B0,
            true => DvmDisable::B1,
        }
    }
    #[doc = "Send speculative fetches according to the Speculation Control Register"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DvmDisable::B0
    }
    #[doc = "Disable speculative fetches from all master interfaces"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DvmDisable::B1
    }
}
#[doc = "Field `DVM_DISABLE` writer - DVM message disable"]
pub type DvmDisableW<'a, REG> = crate::BitWriter<'a, REG, DvmDisable>;
impl<'a, REG> DvmDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Send speculative fetches according to the Speculation Control Register"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DvmDisable::B0)
    }
    #[doc = "Disable speculative fetches from all master interfaces"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DvmDisable::B1)
    }
}
#[doc = "Disable the snoop filter\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnoopFilterDisable {
    #[doc = "0: Snoop filter operation is defined by the power state input, PSTATE."]
    B0 = 0,
    #[doc = "1: Disable snoop filter operation"]
    B1 = 1,
}
impl From<SnoopFilterDisable> for bool {
    #[inline(always)]
    fn from(variant: SnoopFilterDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNOOP_FILTER_DISABLE` reader - Disable the snoop filter"]
pub type SnoopFilterDisableR = crate::BitReader<SnoopFilterDisable>;
impl SnoopFilterDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SnoopFilterDisable {
        match self.bits {
            false => SnoopFilterDisable::B0,
            true => SnoopFilterDisable::B1,
        }
    }
    #[doc = "Snoop filter operation is defined by the power state input, PSTATE."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SnoopFilterDisable::B0
    }
    #[doc = "Disable snoop filter operation"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SnoopFilterDisable::B1
    }
}
#[doc = "Field `SNOOP_FILTER_DISABLE` writer - Disable the snoop filter"]
pub type SnoopFilterDisableW<'a, REG> = crate::BitWriter<'a, REG, SnoopFilterDisable>;
impl<'a, REG> SnoopFilterDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Snoop filter operation is defined by the power state input, PSTATE."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SnoopFilterDisable::B0)
    }
    #[doc = "Disable snoop filter operation"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SnoopFilterDisable::B1)
    }
}
#[doc = "Disable regional clock gating\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisableClockGating {
    #[doc = "0: Regional clock gating operates in the CCI-500"]
    B0 = 0,
    #[doc = "1: Disables regional clock gating in the CCI-500"]
    B1 = 1,
}
impl From<DisableClockGating> for bool {
    #[inline(always)]
    fn from(variant: DisableClockGating) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLE_CLOCK_GATING` reader - Disable regional clock gating"]
pub type DisableClockGatingR = crate::BitReader<DisableClockGating>;
impl DisableClockGatingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DisableClockGating {
        match self.bits {
            false => DisableClockGating::B0,
            true => DisableClockGating::B1,
        }
    }
    #[doc = "Regional clock gating operates in the CCI-500"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DisableClockGating::B0
    }
    #[doc = "Disables regional clock gating in the CCI-500"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DisableClockGating::B1
    }
}
#[doc = "Field `DISABLE_CLOCK_GATING` writer - Disable regional clock gating"]
pub type DisableClockGatingW<'a, REG> = crate::BitWriter<'a, REG, DisableClockGating>;
impl<'a, REG> DisableClockGatingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Regional clock gating operates in the CCI-500"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DisableClockGating::B0)
    }
    #[doc = "Disables regional clock gating in the CCI-500"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DisableClockGating::B1)
    }
}
impl R {
    #[doc = "Bit 0 - snoop_disable control"]
    #[inline(always)]
    pub fn snoop_disable(&self) -> SnoopDisableR {
        SnoopDisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVM message disable"]
    #[inline(always)]
    pub fn dvm_disable(&self) -> DvmDisableR {
        DvmDisableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable the snoop filter"]
    #[inline(always)]
    pub fn snoop_filter_disable(&self) -> SnoopFilterDisableR {
        SnoopFilterDisableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable regional clock gating"]
    #[inline(always)]
    pub fn disable_clock_gating(&self) -> DisableClockGatingR {
        DisableClockGatingR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - snoop_disable control"]
    #[inline(always)]
    #[must_use]
    pub fn snoop_disable(&mut self) -> SnoopDisableW<Cci500SysCtrlSpec> {
        SnoopDisableW::new(self, 0)
    }
    #[doc = "Bit 1 - DVM message disable"]
    #[inline(always)]
    #[must_use]
    pub fn dvm_disable(&mut self) -> DvmDisableW<Cci500SysCtrlSpec> {
        DvmDisableW::new(self, 1)
    }
    #[doc = "Bit 2 - Disable the snoop filter"]
    #[inline(always)]
    #[must_use]
    pub fn snoop_filter_disable(&mut self) -> SnoopFilterDisableW<Cci500SysCtrlSpec> {
        SnoopFilterDisableW::new(self, 2)
    }
    #[doc = "Bit 3 - Disable regional clock gating"]
    #[inline(always)]
    #[must_use]
    pub fn disable_clock_gating(&mut self) -> DisableClockGatingW<Cci500SysCtrlSpec> {
        DisableClockGatingW::new(self, 3)
    }
}
#[doc = "Control Override Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_sys_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_sys_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cci500SysCtrlSpec;
impl crate::RegisterSpec for Cci500SysCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci500_sys_ctrl::R`](R) reader structure"]
impl crate::Readable for Cci500SysCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cci500_sys_ctrl::W`](W) writer structure"]
impl crate::Writable for Cci500SysCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCI500_SYS_CTRL to value 0"]
impl crate::Resettable for Cci500SysCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
