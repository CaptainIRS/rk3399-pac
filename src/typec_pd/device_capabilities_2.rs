#[doc = "Register `DEVICE_CAPABILITIES_2` reader"]
pub type R = crate::R<DeviceCapabilities2Spec>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VconnOvercurrentFaultCapable {
    #[doc = "0: TCPC is not capable of detecting a Vconnfault"]
    B0 = 0,
    #[doc = "1: TCPC is capable of detecting a Vconn fault Support for FAULT_STATUS. VCONNOverCurrentFault and FAULT_CONTROL.VCONNOverCurrentFaul t implemented"]
    B1 = 1,
}
impl From<VconnOvercurrentFaultCapable> for bool {
    #[inline(always)]
    fn from(variant: VconnOvercurrentFaultCapable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Vconn_Overcurrent_Fault_Capable` reader - "]
pub type VconnOvercurrentFaultCapableR = crate::BitReader<VconnOvercurrentFaultCapable>;
impl VconnOvercurrentFaultCapableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VconnOvercurrentFaultCapable {
        match self.bits {
            false => VconnOvercurrentFaultCapable::B0,
            true => VconnOvercurrentFaultCapable::B1,
        }
    }
    #[doc = "TCPC is not capable of detecting a Vconnfault"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VconnOvercurrentFaultCapable::B0
    }
    #[doc = "TCPC is capable of detecting a Vconn fault Support for FAULT_STATUS. VCONNOverCurrentFault and FAULT_CONTROL.VCONNOverCurrentFaul t implemented"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VconnOvercurrentFaultCapable::B1
    }
}
#[doc = "\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VconnPowerSupported {
    #[doc = "0: 1.0W"]
    B000 = 0,
    #[doc = "1: 1.5W"]
    B001 = 1,
    #[doc = "2: 2.0W"]
    B010 = 2,
    #[doc = "3: 3W"]
    B011 = 3,
    #[doc = "4: 4W"]
    B100 = 4,
    #[doc = "5: 5W"]
    B101 = 5,
    #[doc = "6: 6W"]
    B110 = 6,
    #[doc = "7: External"]
    B111 = 7,
}
impl From<VconnPowerSupported> for u8 {
    #[inline(always)]
    fn from(variant: VconnPowerSupported) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VconnPowerSupported {
    type Ux = u8;
}
#[doc = "Field `VCONN_Power_Supported` reader - "]
pub type VconnPowerSupportedR = crate::FieldReader<VconnPowerSupported>;
impl VconnPowerSupportedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VconnPowerSupported {
        match self.bits {
            0 => VconnPowerSupported::B000,
            1 => VconnPowerSupported::B001,
            2 => VconnPowerSupported::B010,
            3 => VconnPowerSupported::B011,
            4 => VconnPowerSupported::B100,
            5 => VconnPowerSupported::B101,
            6 => VconnPowerSupported::B110,
            7 => VconnPowerSupported::B111,
            _ => unreachable!(),
        }
    }
    #[doc = "1.0W"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == VconnPowerSupported::B000
    }
    #[doc = "1.5W"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == VconnPowerSupported::B001
    }
    #[doc = "2.0W"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == VconnPowerSupported::B010
    }
    #[doc = "3W"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == VconnPowerSupported::B011
    }
    #[doc = "4W"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == VconnPowerSupported::B100
    }
    #[doc = "5W"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == VconnPowerSupported::B101
    }
    #[doc = "6W"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == VconnPowerSupported::B110
    }
    #[doc = "External"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == VconnPowerSupported::B111
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VbusVoltageAlarmLsb {
    #[doc = "0: TCPC has 25mV LSB for its voltage alarm and uses all 10 bits in VBUS_VOLTAGE_ALARM_HI_CFG and VBUS_VOLTAGE_ALARM_LO_CFG."]
    B00 = 0,
    #[doc = "1: TCPC has 50mV LSB for its voltage alarm and uses only 9bits. VBUS_VOLTAGE_ALARM_HI_CFG\\[0\\]and VBUS_VOLTAGE_ALARM_LO_CFG\\[0\\]
are ignored by TCPC."]
    B01 = 1,
    #[doc = "2: TCPC has 100mV LSB for its voltage alarm and uses only 8bits. VBUS_VOLTAGE_ALARM_HI_CFG\\[1:0\\]
and VBUS_VOLTAGE_ALARM_LO_CFG\\[1:0\\]
are ignored by TCPC."]
    B10 = 2,
    #[doc = "3: reserved Support for VBUS_VOLTAGE_ALARM_LO_CFG and VBUS_VOLTAGE_ALARM_HI implemented"]
    B11 = 3,
}
impl From<VbusVoltageAlarmLsb> for u8 {
    #[inline(always)]
    fn from(variant: VbusVoltageAlarmLsb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VbusVoltageAlarmLsb {
    type Ux = u8;
}
#[doc = "Field `VBUS_Voltage_Alarm_LSB` reader - "]
pub type VbusVoltageAlarmLsbR = crate::FieldReader<VbusVoltageAlarmLsb>;
impl VbusVoltageAlarmLsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbusVoltageAlarmLsb {
        match self.bits {
            0 => VbusVoltageAlarmLsb::B00,
            1 => VbusVoltageAlarmLsb::B01,
            2 => VbusVoltageAlarmLsb::B10,
            3 => VbusVoltageAlarmLsb::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "TCPC has 25mV LSB for its voltage alarm and uses all 10 bits in VBUS_VOLTAGE_ALARM_HI_CFG and VBUS_VOLTAGE_ALARM_LO_CFG."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == VbusVoltageAlarmLsb::B00
    }
    #[doc = "TCPC has 50mV LSB for its voltage alarm and uses only 9bits. VBUS_VOLTAGE_ALARM_HI_CFG\\[0\\]and VBUS_VOLTAGE_ALARM_LO_CFG\\[0\\]
are ignored by TCPC."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == VbusVoltageAlarmLsb::B01
    }
    #[doc = "TCPC has 100mV LSB for its voltage alarm and uses only 8bits. VBUS_VOLTAGE_ALARM_HI_CFG\\[1:0\\]
and VBUS_VOLTAGE_ALARM_LO_CFG\\[1:0\\]
are ignored by TCPC."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == VbusVoltageAlarmLsb::B10
    }
    #[doc = "reserved Support for VBUS_VOLTAGE_ALARM_LO_CFG and VBUS_VOLTAGE_ALARM_HI implemented"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == VbusVoltageAlarmLsb::B11
    }
}
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopDischargeThreshold {
    #[doc = "0: VBUS_STOP_DISCHARGE_THRESHOLD not implemented (default)"]
    B0 = 0,
    #[doc = "1: VBUS_STOP_DISCHARGE_THRESHOLD implemented"]
    B1 = 1,
}
impl From<StopDischargeThreshold> for bool {
    #[inline(always)]
    fn from(variant: StopDischargeThreshold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Stop_Discharge_Threshold` reader - "]
pub type StopDischargeThresholdR = crate::BitReader<StopDischargeThreshold>;
impl StopDischargeThresholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StopDischargeThreshold {
        match self.bits {
            false => StopDischargeThreshold::B0,
            true => StopDischargeThreshold::B1,
        }
    }
    #[doc = "VBUS_STOP_DISCHARGE_THRESHOLD not implemented (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == StopDischargeThreshold::B0
    }
    #[doc = "VBUS_STOP_DISCHARGE_THRESHOLD implemented"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == StopDischargeThreshold::B1
    }
}
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SinkDisconnectDetection {
    #[doc = "0: VBUS_SINK_ DISCONNECT_THRESHOLD not implemented (default: Use POWER_STATUS.VbusPresent=0b to indicate a Sink disconnect)"]
    B0 = 0,
    #[doc = "1: VBUS_SINK_DISCONNECT_THRESHOLD implemented"]
    B1 = 1,
}
impl From<SinkDisconnectDetection> for bool {
    #[inline(always)]
    fn from(variant: SinkDisconnectDetection) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Sink_Disconnect_Detection` reader - "]
pub type SinkDisconnectDetectionR = crate::BitReader<SinkDisconnectDetection>;
impl SinkDisconnectDetectionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SinkDisconnectDetection {
        match self.bits {
            false => SinkDisconnectDetection::B0,
            true => SinkDisconnectDetection::B1,
        }
    }
    #[doc = "VBUS_SINK_ DISCONNECT_THRESHOLD not implemented (default: Use POWER_STATUS.VbusPresent=0b to indicate a Sink disconnect)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SinkDisconnectDetection::B0
    }
    #[doc = "VBUS_SINK_DISCONNECT_THRESHOLD implemented"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SinkDisconnectDetection::B1
    }
}
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore \n\nreadv alue. Always reads0."]
pub type NotUsedR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vconn_overcurrent_fault_capable(&self) -> VconnOvercurrentFaultCapableR {
        VconnOvercurrentFaultCapableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn vconn_power_supported(&self) -> VconnPowerSupportedR {
        VconnPowerSupportedR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn vbus_voltage_alarm_lsb(&self) -> VbusVoltageAlarmLsbR {
        VbusVoltageAlarmLsbR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn stop_discharge_threshold(&self) -> StopDischargeThresholdR {
        StopDischargeThresholdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sink_disconnect_detection(&self) -> SinkDisconnectDetectionR {
        SinkDisconnectDetectionR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Not used. Always write as 0 and ignore \n\nreadv alue. Always reads0."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Device Capabilities Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`device_capabilities_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeviceCapabilities2Spec;
impl crate::RegisterSpec for DeviceCapabilities2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`device_capabilities_2::R`](R) reader structure"]
impl crate::Readable for DeviceCapabilities2Spec {}
#[doc = "`reset()` method sets DEVICE_CAPABILITIES_2 to value 0xcf"]
impl crate::Resettable for DeviceCapabilities2Spec {
    const RESET_VALUE: u32 = 0xcf;
}
