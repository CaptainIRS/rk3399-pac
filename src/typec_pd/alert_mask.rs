#[doc = "Register `ALERT_MASK` reader"]
pub type R = crate::R<AlertMaskSpec>;
#[doc = "Register `ALERT_MASK` writer"]
pub type W = crate::W<AlertMaskSpec>;
#[doc = "Field `CC_Status_Interrupt_Mask` reader - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type CcStatusInterruptMaskR = crate::BitReader;
#[doc = "Field `CC_Status_Interrupt_Mask` writer - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type CcStatusInterruptMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Port_Power_Status_Interrupt_Mask` reader - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type PortPowerStatusInterruptMaskR = crate::BitReader;
#[doc = "Field `Port_Power_Status_Interrupt_Mask` writer - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type PortPowerStatusInterruptMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Receive_SOP_Message_Status_Interrupt_Mask` reader - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type ReceiveSopMessageStatusInterruptMaskR = crate::BitReader;
#[doc = "Field `Receive_SOP_Message_Status_Interrupt_Mask` writer - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type ReceiveSopMessageStatusInterruptMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Received_Hard_Reset_Message_Status_Interrupt_Mask` reader - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked \n\n(The Hard Reset should generally not \n\nbe masked)"]
pub type ReceivedHardResetMessageStatusInterruptMaskR = crate::BitReader;
#[doc = "Field `Received_Hard_Reset_Message_Status_Interrupt_Mask` writer - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked \n\n(The Hard Reset should generally not \n\nbe masked)"]
pub type ReceivedHardResetMessageStatusInterruptMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Transmit_SOP_Message_failed_Interrupt_Mask` reader - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type TransmitSopMessageFailedInterruptMaskR = crate::BitReader;
#[doc = "Field `Transmit_SOP_Message_failed_Interrupt_Mask` writer - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type TransmitSopMessageFailedInterruptMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Transmit_SOP_Message_discarded_Interrupt_Mask` reader - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type TransmitSopMessageDiscardedInterruptMaskR = crate::BitReader;
#[doc = "Field `Transmit_SOP_Message_discarded_Interrupt_Mask` writer - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type TransmitSopMessageDiscardedInterruptMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Transmit_SOP_Message_successful_Interrupt_Mask` reader - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type TransmitSopMessageSuccessfulInterruptMaskR = crate::BitReader;
#[doc = "Field `Transmit_SOP_Message_successful_Interrupt_Mask` writer - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type TransmitSopMessageSuccessfulInterruptMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUS_Voltage_Alarm_Hi` reader - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type VbusVoltageAlarmHiR = crate::BitReader;
#[doc = "Field `VBUS_Voltage_Alarm_Hi` writer - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type VbusVoltageAlarmHiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUS_Voltage_Alarm_Lo` reader - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type VbusVoltageAlarmLoR = crate::BitReader;
#[doc = "Field `VBUS_Voltage_Alarm_Lo` writer - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type VbusVoltageAlarmLoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Fault` reader - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type FaultR = crate::BitReader;
#[doc = "Field `Fault` writer - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type FaultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Rx_Buffer_Overflow` reader - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type RxBufferOverflowR = crate::BitReader;
#[doc = "Field `Rx_Buffer_Overflow` writer - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type RxBufferOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUS_Sink_Disconnect_Detected` reader - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type VbusSinkDisconnectDetectedR = crate::BitReader;
#[doc = "Field `VBUS_Sink_Disconnect_Detected` writer - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
pub type VbusSinkDisconnectDetectedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore \n\nread value."]
pub type NotUsedR = crate::FieldReader<u16>;
#[doc = "Field `not_used` writer - Not used. Always write as 0 and ignore \n\nread value."]
pub type NotUsedW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    pub fn cc_status_interrupt_mask(&self) -> CcStatusInterruptMaskR {
        CcStatusInterruptMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    pub fn port_power_status_interrupt_mask(&self) -> PortPowerStatusInterruptMaskR {
        PortPowerStatusInterruptMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    pub fn receive_sop_message_status_interrupt_mask(
        &self,
    ) -> ReceiveSopMessageStatusInterruptMaskR {
        ReceiveSopMessageStatusInterruptMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked \n\n(The Hard Reset should generally not \n\nbe masked)"]
    #[inline(always)]
    pub fn received_hard_reset_message_status_interrupt_mask(
        &self,
    ) -> ReceivedHardResetMessageStatusInterruptMaskR {
        ReceivedHardResetMessageStatusInterruptMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    pub fn transmit_sop_message_failed_interrupt_mask(
        &self,
    ) -> TransmitSopMessageFailedInterruptMaskR {
        TransmitSopMessageFailedInterruptMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    pub fn transmit_sop_message_discarded_interrupt_mask(
        &self,
    ) -> TransmitSopMessageDiscardedInterruptMaskR {
        TransmitSopMessageDiscardedInterruptMaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    pub fn transmit_sop_message_successful_interrupt_mask(
        &self,
    ) -> TransmitSopMessageSuccessfulInterruptMaskR {
        TransmitSopMessageSuccessfulInterruptMaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    pub fn vbus_voltage_alarm_hi(&self) -> VbusVoltageAlarmHiR {
        VbusVoltageAlarmHiR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    pub fn vbus_voltage_alarm_lo(&self) -> VbusVoltageAlarmLoR {
        VbusVoltageAlarmLoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    pub fn fault(&self) -> FaultR {
        FaultR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    pub fn rx_buffer_overflow(&self) -> RxBufferOverflowR {
        RxBufferOverflowR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    pub fn vbus_sink_disconnect_detected(&self) -> VbusSinkDisconnectDetectedR {
        VbusSinkDisconnectDetectedR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Not used. Always write as 0 and ignore \n\nread value."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn cc_status_interrupt_mask(&mut self) -> CcStatusInterruptMaskW<AlertMaskSpec> {
        CcStatusInterruptMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn port_power_status_interrupt_mask(
        &mut self,
    ) -> PortPowerStatusInterruptMaskW<AlertMaskSpec> {
        PortPowerStatusInterruptMaskW::new(self, 1)
    }
    #[doc = "Bit 2 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn receive_sop_message_status_interrupt_mask(
        &mut self,
    ) -> ReceiveSopMessageStatusInterruptMaskW<AlertMaskSpec> {
        ReceiveSopMessageStatusInterruptMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked \n\n(The Hard Reset should generally not \n\nbe masked)"]
    #[inline(always)]
    #[must_use]
    pub fn received_hard_reset_message_status_interrupt_mask(
        &mut self,
    ) -> ReceivedHardResetMessageStatusInterruptMaskW<AlertMaskSpec> {
        ReceivedHardResetMessageStatusInterruptMaskW::new(self, 3)
    }
    #[doc = "Bit 4 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_sop_message_failed_interrupt_mask(
        &mut self,
    ) -> TransmitSopMessageFailedInterruptMaskW<AlertMaskSpec> {
        TransmitSopMessageFailedInterruptMaskW::new(self, 4)
    }
    #[doc = "Bit 5 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_sop_message_discarded_interrupt_mask(
        &mut self,
    ) -> TransmitSopMessageDiscardedInterruptMaskW<AlertMaskSpec> {
        TransmitSopMessageDiscardedInterruptMaskW::new(self, 5)
    }
    #[doc = "Bit 6 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_sop_message_successful_interrupt_mask(
        &mut self,
    ) -> TransmitSopMessageSuccessfulInterruptMaskW<AlertMaskSpec> {
        TransmitSopMessageSuccessfulInterruptMaskW::new(self, 6)
    }
    #[doc = "Bit 7 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_voltage_alarm_hi(&mut self) -> VbusVoltageAlarmHiW<AlertMaskSpec> {
        VbusVoltageAlarmHiW::new(self, 7)
    }
    #[doc = "Bit 8 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_voltage_alarm_lo(&mut self) -> VbusVoltageAlarmLoW<AlertMaskSpec> {
        VbusVoltageAlarmLoW::new(self, 8)
    }
    #[doc = "Bit 9 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn fault(&mut self) -> FaultW<AlertMaskSpec> {
        FaultW::new(self, 9)
    }
    #[doc = "Bit 10 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn rx_buffer_overflow(&mut self) -> RxBufferOverflowW<AlertMaskSpec> {
        RxBufferOverflowW::new(self, 10)
    }
    #[doc = "Bit 11 - 0b: Interrupt masked, 1b: Interrupt \n\nunmasked"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_sink_disconnect_detected(&mut self) -> VbusSinkDisconnectDetectedW<AlertMaskSpec> {
        VbusSinkDisconnectDetectedW::new(self, 11)
    }
    #[doc = "Bits 16:31 - Not used. Always write as 0 and ignore \n\nread value."]
    #[inline(always)]
    #[must_use]
    pub fn not_used(&mut self) -> NotUsedW<AlertMaskSpec> {
        NotUsedW::new(self, 16)
    }
}
#[doc = "ALERT Status Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alert_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alert_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlertMaskSpec;
impl crate::RegisterSpec for AlertMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alert_mask::R`](R) reader structure"]
impl crate::Readable for AlertMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`alert_mask::W`](W) writer structure"]
impl crate::Writable for AlertMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALERT_MASK to value 0x0fff"]
impl crate::Resettable for AlertMaskSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
