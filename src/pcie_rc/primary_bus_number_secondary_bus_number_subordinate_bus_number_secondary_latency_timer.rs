#[doc = "Register `PRIMARY_BUS_NUMBER_SECONDARY_BUS_NUMBER_SUBORDINATE_BUS_NUMBER_SECONDARY_LATENCY_TIMER` reader"]
pub type R =
    crate::R<PrimaryBusNumberSecondaryBusNumberSubordinateBusNumberSecondaryLatencyTimerSpec>;
#[doc = "Register `PRIMARY_BUS_NUMBER_SECONDARY_BUS_NUMBER_SUBORDINATE_BUS_NUMBER_SECONDARY_LATENCY_TIMER` writer"]
pub type W =
    crate::W<PrimaryBusNumberSecondaryBusNumberSubordinateBusNumberSecondaryLatencyTimerSpec>;
#[doc = "Field `PBN` reader - Primary Bus Number \\[PBN\\]
This field can be read and written from the local management bus, but its value is not used within the core."]
pub type PbnR = crate::FieldReader;
#[doc = "Field `PBN` writer - Primary Bus Number \\[PBN\\]
This field can be read and written from the local management bus, but its value is not used within the core."]
pub type PbnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SBN` reader - Secondary Bus Number \\[SBN\\]
This field can be read and written from the local management bus, but its value is not used within the core."]
pub type SbnR = crate::FieldReader;
#[doc = "Field `SBN` writer - Secondary Bus Number \\[SBN\\]
This field can be read and written from the local management bus, but its value is not used within the core."]
pub type SbnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SUBN` reader - Subordinate Bus Number \\[SUBN\\]
This field can be read and written from the local management bus, but its value is not used within the core."]
pub type SubnR = crate::FieldReader;
#[doc = "Field `SUBN` writer - Subordinate Bus Number \\[SUBN\\]
This field can be read and written from the local management bus, but its value is not used within the core."]
pub type SubnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SLTN` reader - Secondary Latency Timer \\[SLTN\\]
This field is not implemented."]
pub type SltnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Primary Bus Number \\[PBN\\]
This field can be read and written from the local management bus, but its value is not used within the core."]
    #[inline(always)]
    pub fn pbn(&self) -> PbnR {
        PbnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Secondary Bus Number \\[SBN\\]
This field can be read and written from the local management bus, but its value is not used within the core."]
    #[inline(always)]
    pub fn sbn(&self) -> SbnR {
        SbnR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Subordinate Bus Number \\[SUBN\\]
This field can be read and written from the local management bus, but its value is not used within the core."]
    #[inline(always)]
    pub fn subn(&self) -> SubnR {
        SubnR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Secondary Latency Timer \\[SLTN\\]
This field is not implemented."]
    #[inline(always)]
    pub fn sltn(&self) -> SltnR {
        SltnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Primary Bus Number \\[PBN\\]
This field can be read and written from the local management bus, but its value is not used within the core."]
    #[inline(always)]
    #[must_use]
    pub fn pbn(
        &mut self,
    ) -> PbnW<PrimaryBusNumberSecondaryBusNumberSubordinateBusNumberSecondaryLatencyTimerSpec> {
        PbnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Secondary Bus Number \\[SBN\\]
This field can be read and written from the local management bus, but its value is not used within the core."]
    #[inline(always)]
    #[must_use]
    pub fn sbn(
        &mut self,
    ) -> SbnW<PrimaryBusNumberSecondaryBusNumberSubordinateBusNumberSecondaryLatencyTimerSpec> {
        SbnW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Subordinate Bus Number \\[SUBN\\]
This field can be read and written from the local management bus, but its value is not used within the core."]
    #[inline(always)]
    #[must_use]
    pub fn subn(
        &mut self,
    ) -> SubnW<PrimaryBusNumberSecondaryBusNumberSubordinateBusNumberSecondaryLatencyTimerSpec>
    {
        SubnW::new(self, 16)
    }
}
#[doc = "Primary Bus Number, Secondary Bus Number, Subordinate Bus Number, Secondary Latency Timer This field is not implemented.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`primary_bus_number_secondary_bus_number_subordinate_bus_number_secondary_latency_timer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`primary_bus_number_secondary_bus_number_subordinate_bus_number_secondary_latency_timer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrimaryBusNumberSecondaryBusNumberSubordinateBusNumberSecondaryLatencyTimerSpec;
impl crate::RegisterSpec
    for PrimaryBusNumberSecondaryBusNumberSubordinateBusNumberSecondaryLatencyTimerSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`primary_bus_number_secondary_bus_number_subordinate_bus_number_secondary_latency_timer::R`](R) reader structure"]
impl crate::Readable
    for PrimaryBusNumberSecondaryBusNumberSubordinateBusNumberSecondaryLatencyTimerSpec
{
}
#[doc = "`write(|w| ..)` method takes [`primary_bus_number_secondary_bus_number_subordinate_bus_number_secondary_latency_timer::W`](W) writer structure"]
impl crate::Writable
    for PrimaryBusNumberSecondaryBusNumberSubordinateBusNumberSecondaryLatencyTimerSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIMARY_BUS_NUMBER_SECONDARY_BUS_NUMBER_SUBORDINATE_BUS_NUMBER_SECONDARY_LATENCY_TIMER to value 0"]
impl crate::Resettable
    for PrimaryBusNumberSecondaryBusNumberSubordinateBusNumberSecondaryLatencyTimerSpec
{
    const RESET_VALUE: u32 = 0;
}
