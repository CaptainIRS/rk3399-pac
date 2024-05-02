#[doc = "Register `ALERT` reader"]
pub type R = crate::R<AlertSpec>;
#[doc = "Register `ALERT` writer"]
pub type W = crate::W<AlertSpec>;
#[doc = "Field `CC_Status` reader - 0b: Cleared, 1b: CC status changed"]
pub type CcStatusR = crate::BitReader;
#[doc = "Field `CC_Status` writer - 0b: Cleared, 1b: CC status changed"]
pub type CcStatusW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `Port_Power_Status` reader - 0b: Cleared, 1b: Port status changed"]
pub type PortPowerStatusR = crate::BitReader;
#[doc = "Field `Port_Power_Status` writer - 0b: Cleared, 1b: Port status changed"]
pub type PortPowerStatusW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReceiveSopMessageStatus {
    #[doc = "0: Cleared,"]
    B0 = 0,
    #[doc = "1: RECEIVE_BUFFER register changed. RECEIVE_BYTE_COUNT being set to 0 does not set this bit."]
    B1 = 1,
}
impl From<ReceiveSopMessageStatus> for bool {
    #[inline(always)]
    fn from(variant: ReceiveSopMessageStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Receive_SOP_Message_Status` reader - "]
pub type ReceiveSopMessageStatusR = crate::BitReader<ReceiveSopMessageStatus>;
impl ReceiveSopMessageStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReceiveSopMessageStatus {
        match self.bits {
            false => ReceiveSopMessageStatus::B0,
            true => ReceiveSopMessageStatus::B1,
        }
    }
    #[doc = "Cleared,"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ReceiveSopMessageStatus::B0
    }
    #[doc = "RECEIVE_BUFFER register changed. RECEIVE_BYTE_COUNT being set to 0 does not set this bit."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ReceiveSopMessageStatus::B1
    }
}
#[doc = "Field `Receive_SOP_Message_Status` writer - "]
pub type ReceiveSopMessageStatusW<'a, REG> = crate::BitWriter1C<'a, REG, ReceiveSopMessageStatus>;
impl<'a, REG> ReceiveSopMessageStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared,"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ReceiveSopMessageStatus::B0)
    }
    #[doc = "RECEIVE_BUFFER register changed. RECEIVE_BYTE_COUNT being set to 0 does not set this bit."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ReceiveSopMessageStatus::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReceivedHardReset {
    #[doc = "0: Cleared,"]
    B0 = 0,
    #[doc = "1: Received Hard Reset message Note: The TCPM is not expected to mask the ’’Received Hard Reset’’ alert bit."]
    B1 = 1,
}
impl From<ReceivedHardReset> for bool {
    #[inline(always)]
    fn from(variant: ReceivedHardReset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Received_Hard_Reset` reader - "]
pub type ReceivedHardResetR = crate::BitReader<ReceivedHardReset>;
impl ReceivedHardResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReceivedHardReset {
        match self.bits {
            false => ReceivedHardReset::B0,
            true => ReceivedHardReset::B1,
        }
    }
    #[doc = "Cleared,"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ReceivedHardReset::B0
    }
    #[doc = "Received Hard Reset message Note: The TCPM is not expected to mask the ’’Received Hard Reset’’ alert bit."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ReceivedHardReset::B1
    }
}
#[doc = "Field `Received_Hard_Reset` writer - "]
pub type ReceivedHardResetW<'a, REG> = crate::BitWriter1C<'a, REG, ReceivedHardReset>;
impl<'a, REG> ReceivedHardResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared,"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ReceivedHardReset::B0)
    }
    #[doc = "Received Hard Reset message Note: The TCPM is not expected to mask the ’’Received Hard Reset’’ alert bit."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ReceivedHardReset::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransmitSopMessageFailed {
    #[doc = "0: Cleared,"]
    B0 = 0,
    #[doc = "1: SOP* message transmission not successful, no GoodCRC response received on SOP* message transmission. Transmit SOP* message buffer registers are empty."]
    B1 = 1,
}
impl From<TransmitSopMessageFailed> for bool {
    #[inline(always)]
    fn from(variant: TransmitSopMessageFailed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Transmit_SOP_Message_failed` reader - "]
pub type TransmitSopMessageFailedR = crate::BitReader<TransmitSopMessageFailed>;
impl TransmitSopMessageFailedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TransmitSopMessageFailed {
        match self.bits {
            false => TransmitSopMessageFailed::B0,
            true => TransmitSopMessageFailed::B1,
        }
    }
    #[doc = "Cleared,"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TransmitSopMessageFailed::B0
    }
    #[doc = "SOP* message transmission not successful, no GoodCRC response received on SOP* message transmission. Transmit SOP* message buffer registers are empty."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TransmitSopMessageFailed::B1
    }
}
#[doc = "Field `Transmit_SOP_Message_failed` writer - "]
pub type TransmitSopMessageFailedW<'a, REG> = crate::BitWriter1C<'a, REG, TransmitSopMessageFailed>;
impl<'a, REG> TransmitSopMessageFailedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared,"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TransmitSopMessageFailed::B0)
    }
    #[doc = "SOP* message transmission not successful, no GoodCRC response received on SOP* message transmission. Transmit SOP* message buffer registers are empty."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TransmitSopMessageFailed::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransmitSopMessageDiscarded {
    #[doc = "0: Cleared,"]
    B0 = 0,
    #[doc = "1: Reset or SOP* message transmission not sent due to incoming receive message. Transmit SOP* message buffer registers are empty."]
    B1 = 1,
}
impl From<TransmitSopMessageDiscarded> for bool {
    #[inline(always)]
    fn from(variant: TransmitSopMessageDiscarded) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Transmit_SOP_Message_discarded` reader - "]
pub type TransmitSopMessageDiscardedR = crate::BitReader<TransmitSopMessageDiscarded>;
impl TransmitSopMessageDiscardedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TransmitSopMessageDiscarded {
        match self.bits {
            false => TransmitSopMessageDiscarded::B0,
            true => TransmitSopMessageDiscarded::B1,
        }
    }
    #[doc = "Cleared,"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TransmitSopMessageDiscarded::B0
    }
    #[doc = "Reset or SOP* message transmission not sent due to incoming receive message. Transmit SOP* message buffer registers are empty."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TransmitSopMessageDiscarded::B1
    }
}
#[doc = "Field `Transmit_SOP_Message_discarded` writer - "]
pub type TransmitSopMessageDiscardedW<'a, REG> =
    crate::BitWriter1C<'a, REG, TransmitSopMessageDiscarded>;
impl<'a, REG> TransmitSopMessageDiscardedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared,"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TransmitSopMessageDiscarded::B0)
    }
    #[doc = "Reset or SOP* message transmission not sent due to incoming receive message. Transmit SOP* message buffer registers are empty."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TransmitSopMessageDiscarded::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransmitSopMessageSuccessful {
    #[doc = "0: Cleared,"]
    B0 = 0,
    #[doc = "1: Reset or SOP* message transmission successful. GoodCRC response received on SOP* message transmission. Transmit SOP* message buffer registers are empty."]
    B1 = 1,
}
impl From<TransmitSopMessageSuccessful> for bool {
    #[inline(always)]
    fn from(variant: TransmitSopMessageSuccessful) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Transmit_SOP_Message_successful` reader - "]
pub type TransmitSopMessageSuccessfulR = crate::BitReader<TransmitSopMessageSuccessful>;
impl TransmitSopMessageSuccessfulR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TransmitSopMessageSuccessful {
        match self.bits {
            false => TransmitSopMessageSuccessful::B0,
            true => TransmitSopMessageSuccessful::B1,
        }
    }
    #[doc = "Cleared,"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TransmitSopMessageSuccessful::B0
    }
    #[doc = "Reset or SOP* message transmission successful. GoodCRC response received on SOP* message transmission. Transmit SOP* message buffer registers are empty."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TransmitSopMessageSuccessful::B1
    }
}
#[doc = "Field `Transmit_SOP_Message_successful` writer - "]
pub type TransmitSopMessageSuccessfulW<'a, REG> =
    crate::BitWriter1C<'a, REG, TransmitSopMessageSuccessful>;
impl<'a, REG> TransmitSopMessageSuccessfulW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared,"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TransmitSopMessageSuccessful::B0)
    }
    #[doc = "Reset or SOP* message transmission successful. GoodCRC response received on SOP* message transmission. Transmit SOP* message buffer registers are empty."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TransmitSopMessageSuccessful::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VbusVoltageAlarmHi {
    #[doc = "0: Cleared"]
    B0 = 0,
    #[doc = "1: A high-voltage alarm has occurred"]
    B1 = 1,
}
impl From<VbusVoltageAlarmHi> for bool {
    #[inline(always)]
    fn from(variant: VbusVoltageAlarmHi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUS_Voltage_Alarm_Hi` reader - "]
pub type VbusVoltageAlarmHiR = crate::BitReader<VbusVoltageAlarmHi>;
impl VbusVoltageAlarmHiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbusVoltageAlarmHi {
        match self.bits {
            false => VbusVoltageAlarmHi::B0,
            true => VbusVoltageAlarmHi::B1,
        }
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VbusVoltageAlarmHi::B0
    }
    #[doc = "A high-voltage alarm has occurred"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VbusVoltageAlarmHi::B1
    }
}
#[doc = "Field `VBUS_Voltage_Alarm_Hi` writer - "]
pub type VbusVoltageAlarmHiW<'a, REG> = crate::BitWriter1C<'a, REG, VbusVoltageAlarmHi>;
impl<'a, REG> VbusVoltageAlarmHiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VbusVoltageAlarmHi::B0)
    }
    #[doc = "A high-voltage alarm has occurred"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VbusVoltageAlarmHi::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VbusVoltageAlarmLo {
    #[doc = "0: Cleared"]
    B0 = 0,
    #[doc = "1: A low-voltage alarm has occurred"]
    B1 = 1,
}
impl From<VbusVoltageAlarmLo> for bool {
    #[inline(always)]
    fn from(variant: VbusVoltageAlarmLo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUS_Voltage_Alarm_Lo` reader - "]
pub type VbusVoltageAlarmLoR = crate::BitReader<VbusVoltageAlarmLo>;
impl VbusVoltageAlarmLoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbusVoltageAlarmLo {
        match self.bits {
            false => VbusVoltageAlarmLo::B0,
            true => VbusVoltageAlarmLo::B1,
        }
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VbusVoltageAlarmLo::B0
    }
    #[doc = "A low-voltage alarm has occurred"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VbusVoltageAlarmLo::B1
    }
}
#[doc = "Field `VBUS_Voltage_Alarm_Lo` writer - "]
pub type VbusVoltageAlarmLoW<'a, REG> = crate::BitWriter1C<'a, REG, VbusVoltageAlarmLo>;
impl<'a, REG> VbusVoltageAlarmLoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VbusVoltageAlarmLo::B0)
    }
    #[doc = "A low-voltage alarm has occurred"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VbusVoltageAlarmLo::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    #[doc = "0: No Fault"]
    B0 = 0,
    #[doc = "1: An Fault has occurred. Read the FAULT_STATUS register"]
    B1 = 1,
}
impl From<Fault> for bool {
    #[inline(always)]
    fn from(variant: Fault) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Fault` reader - "]
pub type FaultR = crate::BitReader<Fault>;
impl FaultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fault {
        match self.bits {
            false => Fault::B0,
            true => Fault::B1,
        }
    }
    #[doc = "No Fault"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fault::B0
    }
    #[doc = "An Fault has occurred. Read the FAULT_STATUS register"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fault::B1
    }
}
#[doc = "Field `Fault` writer - "]
pub type FaultW<'a, REG> = crate::BitWriter1C<'a, REG, Fault>;
impl<'a, REG> FaultW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Fault"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fault::B0)
    }
    #[doc = "An Fault has occurred. Read the FAULT_STATUS register"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fault::B1)
    }
}
#[doc = "Field `Rx_Buffer_Overflow` reader - 0b: \n\nTCPC \n\nRx \n\nbuffer \n\nis \n\nfunctioning properly 1b: TCPC \n\nRx buffer has overflowed \n\nWriting 1 to this register acknowledges \n\nthe overflow. The overflow is cleared by \n\nwriting to ALERT.RecevieSOP* Message \n\nStatus"]
pub type RxBufferOverflowR = crate::BitReader;
#[doc = "Field `Rx_Buffer_Overflow` writer - 0b: \n\nTCPC \n\nRx \n\nbuffer \n\nis \n\nfunctioning properly 1b: TCPC \n\nRx buffer has overflowed \n\nWriting 1 to this register acknowledges \n\nthe overflow. The overflow is cleared by \n\nwriting to ALERT.RecevieSOP* Message \n\nStatus"]
pub type RxBufferOverflowW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VbusSinkDisconnectDetected {
    #[doc = "0: Cleared"]
    B0 = 0,
    #[doc = "1: A Vbus Sink Disconnect Threshold crossing has been detected"]
    B1 = 1,
}
impl From<VbusSinkDisconnectDetected> for bool {
    #[inline(always)]
    fn from(variant: VbusSinkDisconnectDetected) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUS_Sink_Disconnect_Detected` reader - "]
pub type VbusSinkDisconnectDetectedR = crate::BitReader<VbusSinkDisconnectDetected>;
impl VbusSinkDisconnectDetectedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbusSinkDisconnectDetected {
        match self.bits {
            false => VbusSinkDisconnectDetected::B0,
            true => VbusSinkDisconnectDetected::B1,
        }
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VbusSinkDisconnectDetected::B0
    }
    #[doc = "A Vbus Sink Disconnect Threshold crossing has been detected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VbusSinkDisconnectDetected::B1
    }
}
#[doc = "Field `VBUS_Sink_Disconnect_Detected` writer - "]
pub type VbusSinkDisconnectDetectedW<'a, REG> =
    crate::BitWriter1C<'a, REG, VbusSinkDisconnectDetected>;
impl<'a, REG> VbusSinkDisconnectDetectedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VbusSinkDisconnectDetected::B0)
    }
    #[doc = "A Vbus Sink Disconnect Threshold crossing has been detected"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VbusSinkDisconnectDetected::B1)
    }
}
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore \n\nread value."]
pub type NotUsedR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - 0b: Cleared, 1b: CC status changed"]
    #[inline(always)]
    pub fn cc_status(&self) -> CcStatusR {
        CcStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0b: Cleared, 1b: Port status changed"]
    #[inline(always)]
    pub fn port_power_status(&self) -> PortPowerStatusR {
        PortPowerStatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn receive_sop_message_status(&self) -> ReceiveSopMessageStatusR {
        ReceiveSopMessageStatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn received_hard_reset(&self) -> ReceivedHardResetR {
        ReceivedHardResetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn transmit_sop_message_failed(&self) -> TransmitSopMessageFailedR {
        TransmitSopMessageFailedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn transmit_sop_message_discarded(&self) -> TransmitSopMessageDiscardedR {
        TransmitSopMessageDiscardedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn transmit_sop_message_successful(&self) -> TransmitSopMessageSuccessfulR {
        TransmitSopMessageSuccessfulR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn vbus_voltage_alarm_hi(&self) -> VbusVoltageAlarmHiR {
        VbusVoltageAlarmHiR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn vbus_voltage_alarm_lo(&self) -> VbusVoltageAlarmLoR {
        VbusVoltageAlarmLoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fault(&self) -> FaultR {
        FaultR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 0b: \n\nTCPC \n\nRx \n\nbuffer \n\nis \n\nfunctioning properly 1b: TCPC \n\nRx buffer has overflowed \n\nWriting 1 to this register acknowledges \n\nthe overflow. The overflow is cleared by \n\nwriting to ALERT.RecevieSOP* Message \n\nStatus"]
    #[inline(always)]
    pub fn rx_buffer_overflow(&self) -> RxBufferOverflowR {
        RxBufferOverflowR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
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
    #[doc = "Bit 0 - 0b: Cleared, 1b: CC status changed"]
    #[inline(always)]
    #[must_use]
    pub fn cc_status(&mut self) -> CcStatusW<AlertSpec> {
        CcStatusW::new(self, 0)
    }
    #[doc = "Bit 1 - 0b: Cleared, 1b: Port status changed"]
    #[inline(always)]
    #[must_use]
    pub fn port_power_status(&mut self) -> PortPowerStatusW<AlertSpec> {
        PortPowerStatusW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn receive_sop_message_status(&mut self) -> ReceiveSopMessageStatusW<AlertSpec> {
        ReceiveSopMessageStatusW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn received_hard_reset(&mut self) -> ReceivedHardResetW<AlertSpec> {
        ReceivedHardResetW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_sop_message_failed(&mut self) -> TransmitSopMessageFailedW<AlertSpec> {
        TransmitSopMessageFailedW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_sop_message_discarded(&mut self) -> TransmitSopMessageDiscardedW<AlertSpec> {
        TransmitSopMessageDiscardedW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_sop_message_successful(&mut self) -> TransmitSopMessageSuccessfulW<AlertSpec> {
        TransmitSopMessageSuccessfulW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_voltage_alarm_hi(&mut self) -> VbusVoltageAlarmHiW<AlertSpec> {
        VbusVoltageAlarmHiW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_voltage_alarm_lo(&mut self) -> VbusVoltageAlarmLoW<AlertSpec> {
        VbusVoltageAlarmLoW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn fault(&mut self) -> FaultW<AlertSpec> {
        FaultW::new(self, 9)
    }
    #[doc = "Bit 10 - 0b: \n\nTCPC \n\nRx \n\nbuffer \n\nis \n\nfunctioning properly 1b: TCPC \n\nRx buffer has overflowed \n\nWriting 1 to this register acknowledges \n\nthe overflow. The overflow is cleared by \n\nwriting to ALERT.RecevieSOP* Message \n\nStatus"]
    #[inline(always)]
    #[must_use]
    pub fn rx_buffer_overflow(&mut self) -> RxBufferOverflowW<AlertSpec> {
        RxBufferOverflowW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_sink_disconnect_detected(&mut self) -> VbusSinkDisconnectDetectedW<AlertSpec> {
        VbusSinkDisconnectDetectedW::new(self, 11)
    }
}
#[doc = "ALERT Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alert::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alert::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlertSpec;
impl crate::RegisterSpec for AlertSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alert::R`](R) reader structure"]
impl crate::Readable for AlertSpec {}
#[doc = "`write(|w| ..)` method takes [`alert::W`](W) writer structure"]
impl crate::Writable for AlertSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0fff;
}
#[doc = "`reset()` method sets ALERT to value 0"]
impl crate::Resettable for AlertSpec {
    const RESET_VALUE: u32 = 0;
}
