#[doc = "Register `POWER_STATUS_MASK` reader"]
pub type R = crate::R<PowerStatusMaskSpec>;
#[doc = "Register `POWER_STATUS_MASK` writer"]
pub type W = crate::W<PowerStatusMaskSpec>;
#[doc = "Field `Sinking_VBUS_Status_Interrupt_Mask` reader - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type SinkingVbusStatusInterruptMaskR = crate::BitReader;
#[doc = "Field `Sinking_VBUS_Status_Interrupt_Mask` writer - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type SinkingVbusStatusInterruptMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCONN_Present_Status_Interrupt_Mask` reader - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type VconnPresentStatusInterruptMaskR = crate::BitReader;
#[doc = "Field `VCONN_Present_Status_Interrupt_Mask` writer - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type VconnPresentStatusInterruptMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUS_Present_Status_Interrupt_Mask` reader - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type VbusPresentStatusInterruptMaskR = crate::BitReader;
#[doc = "Field `VBUS_Present_Status_Interrupt_Mask` writer - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type VbusPresentStatusInterruptMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUS_Present_Detection_Status_Interrupt_Mask` reader - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type VbusPresentDetectionStatusInterruptMaskR = crate::BitReader;
#[doc = "Field `VBUS_Present_Detection_Status_Interrupt_Mask` writer - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type VbusPresentDetectionStatusInterruptMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Sourcing_VBUS_Status_Interrupt_Mask` reader - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type SourcingVbusStatusInterruptMaskR = crate::BitReader;
#[doc = "Field `Sourcing_VBUS_Status_Interrupt_Mask` writer - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type SourcingVbusStatusInterruptMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Sourcing_High_Voltage_Status_Interrupt_Mask` reader - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type SourcingHighVoltageStatusInterruptMaskR = crate::BitReader;
#[doc = "Field `Sourcing_High_Voltage_Status_Interrupt_Mask` writer - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type SourcingHighVoltageStatusInterruptMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCPC_Initialization_Status_Mask` reader - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type TcpcInitializationStatusMaskR = crate::BitReader;
#[doc = "Field `TCPC_Initialization_Status_Mask` writer - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type TcpcInitializationStatusMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Debug_Accessory_Connected` reader - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type DebugAccessoryConnectedR = crate::BitReader;
#[doc = "Field `Debug_Accessory_Connected` writer - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
pub type DebugAccessoryConnectedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore \n\nread value."]
pub type NotUsedR = crate::FieldReader<u32>;
#[doc = "Field `not_used` writer - Not used. Always write as 0 and ignore \n\nread value."]
pub type NotUsedW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    pub fn sinking_vbus_status_interrupt_mask(&self) -> SinkingVbusStatusInterruptMaskR {
        SinkingVbusStatusInterruptMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    pub fn vconn_present_status_interrupt_mask(&self) -> VconnPresentStatusInterruptMaskR {
        VconnPresentStatusInterruptMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    pub fn vbus_present_status_interrupt_mask(&self) -> VbusPresentStatusInterruptMaskR {
        VbusPresentStatusInterruptMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    pub fn vbus_present_detection_status_interrupt_mask(
        &self,
    ) -> VbusPresentDetectionStatusInterruptMaskR {
        VbusPresentDetectionStatusInterruptMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    pub fn sourcing_vbus_status_interrupt_mask(&self) -> SourcingVbusStatusInterruptMaskR {
        SourcingVbusStatusInterruptMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    pub fn sourcing_high_voltage_status_interrupt_mask(
        &self,
    ) -> SourcingHighVoltageStatusInterruptMaskR {
        SourcingHighVoltageStatusInterruptMaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    pub fn tcpc_initialization_status_mask(&self) -> TcpcInitializationStatusMaskR {
        TcpcInitializationStatusMaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    pub fn debug_accessory_connected(&self) -> DebugAccessoryConnectedR {
        DebugAccessoryConnectedR::new(((self.bits >> 7) & 1) != 0)
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
    pub fn sinking_vbus_status_interrupt_mask(
        &mut self,
    ) -> SinkingVbusStatusInterruptMaskW<PowerStatusMaskSpec> {
        SinkingVbusStatusInterruptMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn vconn_present_status_interrupt_mask(
        &mut self,
    ) -> VconnPresentStatusInterruptMaskW<PowerStatusMaskSpec> {
        VconnPresentStatusInterruptMaskW::new(self, 1)
    }
    #[doc = "Bit 2 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_present_status_interrupt_mask(
        &mut self,
    ) -> VbusPresentStatusInterruptMaskW<PowerStatusMaskSpec> {
        VbusPresentStatusInterruptMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_present_detection_status_interrupt_mask(
        &mut self,
    ) -> VbusPresentDetectionStatusInterruptMaskW<PowerStatusMaskSpec> {
        VbusPresentDetectionStatusInterruptMaskW::new(self, 3)
    }
    #[doc = "Bit 4 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn sourcing_vbus_status_interrupt_mask(
        &mut self,
    ) -> SourcingVbusStatusInterruptMaskW<PowerStatusMaskSpec> {
        SourcingVbusStatusInterruptMaskW::new(self, 4)
    }
    #[doc = "Bit 5 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn sourcing_high_voltage_status_interrupt_mask(
        &mut self,
    ) -> SourcingHighVoltageStatusInterruptMaskW<PowerStatusMaskSpec> {
        SourcingHighVoltageStatusInterruptMaskW::new(self, 5)
    }
    #[doc = "Bit 6 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tcpc_initialization_status_mask(
        &mut self,
    ) -> TcpcInitializationStatusMaskW<PowerStatusMaskSpec> {
        TcpcInitializationStatusMaskW::new(self, 6)
    }
    #[doc = "Bit 7 - 0b: \n\nInterrupt masked, 1b: \n\nInterrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn debug_accessory_connected(&mut self) -> DebugAccessoryConnectedW<PowerStatusMaskSpec> {
        DebugAccessoryConnectedW::new(self, 7)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore \n\nread value."]
    #[inline(always)]
    #[must_use]
    pub fn not_used(&mut self) -> NotUsedW<PowerStatusMaskSpec> {
        NotUsedW::new(self, 8)
    }
}
#[doc = "Power Status Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_status_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_status_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerStatusMaskSpec;
impl crate::RegisterSpec for PowerStatusMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_status_mask::R`](R) reader structure"]
impl crate::Readable for PowerStatusMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`power_status_mask::W`](W) writer structure"]
impl crate::Writable for PowerStatusMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWER_STATUS_MASK to value 0xff"]
impl crate::Resettable for PowerStatusMaskSpec {
    const RESET_VALUE: u32 = 0xff;
}
