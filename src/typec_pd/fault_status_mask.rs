#[doc = "Register `FAULT_STATUS_MASK` reader"]
pub type R = crate::R<FaultStatusMaskSpec>;
#[doc = "Register `FAULT_STATUS_MASK` writer"]
pub type W = crate::W<FaultStatusMaskSpec>;
#[doc = "Field `Interface_Error_Interrupt_Status_Mask` reader - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type InterfaceErrorInterruptStatusMaskR = crate::BitReader;
#[doc = "Field `Interface_Error_Interrupt_Status_Mask` writer - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type InterfaceErrorInterruptStatusMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Vconn_Over_Current_Fault_Interrupt_Status_Mask` reader - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type VconnOverCurrentFaultInterruptStatusMaskR = crate::BitReader;
#[doc = "Field `Vconn_Over_Current_Fault_Interrupt_Status_Mask` writer - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type VconnOverCurrentFaultInterruptStatusMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Vbus_Over_Voltage_Protection_Fault_Interrupt_Status_Mask` reader - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type VbusOverVoltageProtectionFaultInterruptStatusMaskR = crate::BitReader;
#[doc = "Field `Vbus_Over_Voltage_Protection_Fault_Interrupt_Status_Mask` writer - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type VbusOverVoltageProtectionFaultInterruptStatusMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Vbus_Over_Current_Protection_Fault_Interrupt_Status_Mask` reader - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type VbusOverCurrentProtectionFaultInterruptStatusMaskR = crate::BitReader;
#[doc = "Field `Vbus_Over_Current_Protection_Fault_Interrupt_Status_Mask` writer - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type VbusOverCurrentProtectionFaultInterruptStatusMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Force_Discharge_Failed_Mask` reader - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type ForceDischargeFailedMaskR = crate::BitReader;
#[doc = "Field `Force_Discharge_Failed_Mask` writer - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type ForceDischargeFailedMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Auto_Discharge_Failed_Mask` reader - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type AutoDischargeFailedMaskR = crate::BitReader;
#[doc = "Field `Auto_Discharge_Failed_Mask` writer - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type AutoDischargeFailedMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Force_Off_Vbus_Interrupt_Status_Mask` reader - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type ForceOffVbusInterruptStatusMaskR = crate::BitReader;
#[doc = "Field `Force_Off_Vbus_Interrupt_Status_Mask` writer - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type ForceOffVbusInterruptStatusMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore \n\nread value."]
pub type NotUsedR = crate::FieldReader<u32>;
#[doc = "Field `not_used` writer - Not used. Always write as 0 and ignore \n\nread value."]
pub type NotUsedW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    pub fn interface_error_interrupt_status_mask(&self) -> InterfaceErrorInterruptStatusMaskR {
        InterfaceErrorInterruptStatusMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    pub fn vconn_over_current_fault_interrupt_status_mask(
        &self,
    ) -> VconnOverCurrentFaultInterruptStatusMaskR {
        VconnOverCurrentFaultInterruptStatusMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    pub fn vbus_over_voltage_protection_fault_interrupt_status_mask(
        &self,
    ) -> VbusOverVoltageProtectionFaultInterruptStatusMaskR {
        VbusOverVoltageProtectionFaultInterruptStatusMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    pub fn vbus_over_current_protection_fault_interrupt_status_mask(
        &self,
    ) -> VbusOverCurrentProtectionFaultInterruptStatusMaskR {
        VbusOverCurrentProtectionFaultInterruptStatusMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    pub fn force_discharge_failed_mask(&self) -> ForceDischargeFailedMaskR {
        ForceDischargeFailedMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    pub fn auto_discharge_failed_mask(&self) -> AutoDischargeFailedMaskR {
        AutoDischargeFailedMaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    pub fn force_off_vbus_interrupt_status_mask(&self) -> ForceOffVbusInterruptStatusMaskR {
        ForceOffVbusInterruptStatusMaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore \n\nread value."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn interface_error_interrupt_status_mask(
        &mut self,
    ) -> InterfaceErrorInterruptStatusMaskW<FaultStatusMaskSpec> {
        InterfaceErrorInterruptStatusMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn vconn_over_current_fault_interrupt_status_mask(
        &mut self,
    ) -> VconnOverCurrentFaultInterruptStatusMaskW<FaultStatusMaskSpec> {
        VconnOverCurrentFaultInterruptStatusMaskW::new(self, 1)
    }
    #[doc = "Bit 2 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_over_voltage_protection_fault_interrupt_status_mask(
        &mut self,
    ) -> VbusOverVoltageProtectionFaultInterruptStatusMaskW<FaultStatusMaskSpec> {
        VbusOverVoltageProtectionFaultInterruptStatusMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_over_current_protection_fault_interrupt_status_mask(
        &mut self,
    ) -> VbusOverCurrentProtectionFaultInterruptStatusMaskW<FaultStatusMaskSpec> {
        VbusOverCurrentProtectionFaultInterruptStatusMaskW::new(self, 3)
    }
    #[doc = "Bit 4 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn force_discharge_failed_mask(
        &mut self,
    ) -> ForceDischargeFailedMaskW<FaultStatusMaskSpec> {
        ForceDischargeFailedMaskW::new(self, 4)
    }
    #[doc = "Bit 5 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn auto_discharge_failed_mask(&mut self) -> AutoDischargeFailedMaskW<FaultStatusMaskSpec> {
        AutoDischargeFailedMaskW::new(self, 5)
    }
    #[doc = "Bit 6 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn force_off_vbus_interrupt_status_mask(
        &mut self,
    ) -> ForceOffVbusInterruptStatusMaskW<FaultStatusMaskSpec> {
        ForceOffVbusInterruptStatusMaskW::new(self, 6)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore \n\nread value."]
    #[inline(always)]
    #[must_use]
    pub fn not_used(&mut self) -> NotUsedW<FaultStatusMaskSpec> {
        NotUsedW::new(self, 8)
    }
}
#[doc = "FAULT Status Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fault_status_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fault_status_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaultStatusMaskSpec;
impl crate::RegisterSpec for FaultStatusMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fault_status_mask::R`](R) reader structure"]
impl crate::Readable for FaultStatusMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`fault_status_mask::W`](W) writer structure"]
impl crate::Writable for FaultStatusMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FAULT_STATUS_MASK to value 0x7f"]
impl crate::Resettable for FaultStatusMaskSpec {
    const RESET_VALUE: u32 = 0x7f;
}
