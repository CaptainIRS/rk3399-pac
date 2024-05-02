#[doc = "Register `DEVICE_CAPABILITIES_1` reader"]
pub type R = crate::R<DeviceCapabilities1Spec>;
#[doc = "Field `Source_VBUS` reader - 0b: TCPC \n\nis not capable of controlling the \n\nsource path to VBUS. 1b: TCPC is capable of \n\ncontrolling the source path to VBUS \n\nSupport \n\nfor \n\nPOWER_STATUS.SourcingVbus, \n\nCOMMAND.SourceVbusDefaultVoltage, \n\nCOMMAND.DisableSourceVbus, \n\nCOMMAND.EnableVbusDetect \n\nand \n\nCOMMAND.DisableVbusDetect \n\nimplemented"]
pub type SourceVbusR = crate::BitReader;
#[doc = "Field `Source_High_Voltage_VBUS` reader - 0b: TCPC \n\nis not capable of controlling the \n\nsource high voltage path to VBUS. 1b: TCPC is \n\ncapable of controlling the source high voltage \n\npath \n\nto \n\nVBUS \n\nSupport \n\nfor \n\nPOWER_STATUS.SourcingHighVoltage \n\nand \n\nCOMMAND.SourceVbusHighVoltage \n\nimplemented"]
pub type SourceHighVoltageVbusR = crate::BitReader;
#[doc = "Field `Sink_VBUS` reader - 0b: TCPC is not capable controlling the sink \n\npath to the system load. 1b: TCPC is capable \n\nof controlling the sink path to the system load"]
pub type SinkVbusR = crate::BitReader;
#[doc = "Field `Source_VCONN` reader - 0b: TCPC is not capable of switching \n\nVCONN. 1b: TCPC is capable of switching \n\nVCONN \n\nSupport for POWER_CONTROL.EnableV CONN \n\nand \n\nPOWER_STATUS.VCONNPresent \n\nimplemented"]
pub type SourceVconnR = crate::BitReader;
#[doc = "Field `SOPp_DBG_SOPb_DBG_Support` reader - 0b: \n\nAll \n\nSOP* \n\nexcept \n\nSOP’_DBG/SOP’’_DBG. 1b: All SOP* \n\nmessages are supported Configured in \n\nRECEIVE_DETECT and TRANSMIT"]
pub type SoppDbgSopbDbgSupportR = crate::BitReader;
#[doc = "\n\nValue on reset: 6"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RolesSupported {
    #[doc = "0: USB Type-C Port Manager can configure the Port as Source only or Sink only (not DRP). 001b: Source only. 010b: Sink only. 011b: Sink with accessory support."]
    B000 = 0,
    #[doc = "4: DRP only. 101b: Source, Sink, DRP, Adapter/Cable all supported. 110b: Source, Sink, DRP. 111b: Not valid"]
    B100 = 4,
}
impl From<RolesSupported> for u8 {
    #[inline(always)]
    fn from(variant: RolesSupported) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RolesSupported {
    type Ux = u8;
}
#[doc = "Field `Roles_Supported` reader - "]
pub type RolesSupportedR = crate::FieldReader<RolesSupported>;
impl RolesSupportedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RolesSupported> {
        match self.bits {
            0 => Some(RolesSupported::B000),
            4 => Some(RolesSupported::B100),
            _ => None,
        }
    }
    #[doc = "USB Type-C Port Manager can configure the Port as Source only or Sink only (not DRP). 001b: Source only. 010b: Sink only. 011b: Sink with accessory support."]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == RolesSupported::B000
    }
    #[doc = "DRP only. 101b: Source, Sink, DRP, Adapter/Cable all supported. 110b: Source, Sink, DRP. 111b: Not valid"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == RolesSupported::B100
    }
}
#[doc = "\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SourceResistorSupported {
    #[doc = "0: Rp default only."]
    B00 = 0,
    #[doc = "1: Rp 1.5A and default."]
    B01 = 1,
    #[doc = "2: Rp 3.0A, 1.5A, and default.11b:Reserved. Rp values which may be configured by the TCPM via the ROLE_CONTROL register"]
    B10 = 2,
}
impl From<SourceResistorSupported> for u8 {
    #[inline(always)]
    fn from(variant: SourceResistorSupported) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SourceResistorSupported {
    type Ux = u8;
}
#[doc = "Field `Source_Resistor_Supported` reader - "]
pub type SourceResistorSupportedR = crate::FieldReader<SourceResistorSupported>;
impl SourceResistorSupportedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SourceResistorSupported> {
        match self.bits {
            0 => Some(SourceResistorSupported::B00),
            1 => Some(SourceResistorSupported::B01),
            2 => Some(SourceResistorSupported::B10),
            _ => None,
        }
    }
    #[doc = "Rp default only."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SourceResistorSupported::B00
    }
    #[doc = "Rp 1.5A and default."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SourceResistorSupported::B01
    }
    #[doc = "Rp 3.0A, 1.5A, and default.11b:Reserved. Rp values which may be configured by the TCPM via the ROLE_CONTROL register"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SourceResistorSupported::B10
    }
}
#[doc = "Field `VBUS_Measurement_and_Alarm_Capable` reader - 0b: No \n\nVBUS \n\nvoltage measurement \n\nnor \n\nVBUSAlarms. 1b: VBUS voltage measurement \n\nand VBUS Alarms Support for VBUS_VOLTAGE, \n\nVBUS_VOLTAGE_ALARM_HI_CFG, \n\nVBUS_VOLTAGE_ALARM_LO_CFG implemented"]
pub type VbusMeasurementAndAlarmCapableR = crate::BitReader;
#[doc = "Field `ForceDischarge` reader - 0b: No Force Discharge \n\nimplemented \n\nin \n\nTCPC. 1b: Force Discharge is implemented in \n\nthe \n\nTCPC \n\nSupport \n\nfor \n\nPOWER_CONTROL.ForceDischarge, \n\nFAULT_STATUS.VbusDischargeFail, \n\nFAULT_STATUS.VBUSDischargeFaultDetectionTi\n\nmer, \n\nand \n\nVBUS_STOP_DISCHARGE_THRESHOLD \n\nimplemented"]
pub type ForceDischargeR = crate::BitReader;
#[doc = "Field `BleedDischarge` reader - 0b: No Bleed Discharge implemented in \n\nTCPC. 1b: Bleed Discharge is implemented \n\nin \n\nthe \n\nTCPC \n\nSupport \n\nfor \n\nPOWER_CONTROL.EnableBleedDischarge \n\nimplemented"]
pub type BleedDischargeR = crate::BitReader;
#[doc = "Field `VBUS_OVP_Reporting` reader - 0b: VBUS OVP is not reported by the \n\nTCPC. 1b: VBUS OVP is reported by the \n\nTCPC \n\nSupport \n\nfor \n\nboth \n\nFAULT_STATUS.InternalorExternalOVP \n\nand \n\nFAULT_CONTROL.InternalorExternalOVP \n\nimplemented"]
pub type VbusOvpReportingR = crate::BitReader;
#[doc = "Field `VBUS_OCP_Reporting` reader - 0b: VBUS OCP is not reported by the \n\nTCPC. 1b: VBUS OCP is reported by the \n\nTCPC \n\nSupport \n\nfor \n\nboth \n\nFAULT_STATUS.InternalorExternalOCP \n\nand \n\nFAULT_CONTROL.InternalorExternalOC\n\nP implemented"]
pub type VbusOcpReportingR = crate::BitReader;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore read \n\nvalue. Always reads0."]
pub type NotUsedR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - 0b: TCPC \n\nis not capable of controlling the \n\nsource path to VBUS. 1b: TCPC is capable of \n\ncontrolling the source path to VBUS \n\nSupport \n\nfor \n\nPOWER_STATUS.SourcingVbus, \n\nCOMMAND.SourceVbusDefaultVoltage, \n\nCOMMAND.DisableSourceVbus, \n\nCOMMAND.EnableVbusDetect \n\nand \n\nCOMMAND.DisableVbusDetect \n\nimplemented"]
    #[inline(always)]
    pub fn source_vbus(&self) -> SourceVbusR {
        SourceVbusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0b: TCPC \n\nis not capable of controlling the \n\nsource high voltage path to VBUS. 1b: TCPC is \n\ncapable of controlling the source high voltage \n\npath \n\nto \n\nVBUS \n\nSupport \n\nfor \n\nPOWER_STATUS.SourcingHighVoltage \n\nand \n\nCOMMAND.SourceVbusHighVoltage \n\nimplemented"]
    #[inline(always)]
    pub fn source_high_voltage_vbus(&self) -> SourceHighVoltageVbusR {
        SourceHighVoltageVbusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0b: TCPC is not capable controlling the sink \n\npath to the system load. 1b: TCPC is capable \n\nof controlling the sink path to the system load"]
    #[inline(always)]
    pub fn sink_vbus(&self) -> SinkVbusR {
        SinkVbusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0b: TCPC is not capable of switching \n\nVCONN. 1b: TCPC is capable of switching \n\nVCONN \n\nSupport for POWER_CONTROL.EnableV CONN \n\nand \n\nPOWER_STATUS.VCONNPresent \n\nimplemented"]
    #[inline(always)]
    pub fn source_vconn(&self) -> SourceVconnR {
        SourceVconnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0b: \n\nAll \n\nSOP* \n\nexcept \n\nSOP’_DBG/SOP’’_DBG. 1b: All SOP* \n\nmessages are supported Configured in \n\nRECEIVE_DETECT and TRANSMIT"]
    #[inline(always)]
    pub fn sopp_dbg_sopb_dbg_support(&self) -> SoppDbgSopbDbgSupportR {
        SoppDbgSopbDbgSupportR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn roles_supported(&self) -> RolesSupportedR {
        RolesSupportedR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn source_resistor_supported(&self) -> SourceResistorSupportedR {
        SourceResistorSupportedR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - 0b: No \n\nVBUS \n\nvoltage measurement \n\nnor \n\nVBUSAlarms. 1b: VBUS voltage measurement \n\nand VBUS Alarms Support for VBUS_VOLTAGE, \n\nVBUS_VOLTAGE_ALARM_HI_CFG, \n\nVBUS_VOLTAGE_ALARM_LO_CFG implemented"]
    #[inline(always)]
    pub fn vbus_measurement_and_alarm_capable(&self) -> VbusMeasurementAndAlarmCapableR {
        VbusMeasurementAndAlarmCapableR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 0b: No Force Discharge \n\nimplemented \n\nin \n\nTCPC. 1b: Force Discharge is implemented in \n\nthe \n\nTCPC \n\nSupport \n\nfor \n\nPOWER_CONTROL.ForceDischarge, \n\nFAULT_STATUS.VbusDischargeFail, \n\nFAULT_STATUS.VBUSDischargeFaultDetectionTi\n\nmer, \n\nand \n\nVBUS_STOP_DISCHARGE_THRESHOLD \n\nimplemented"]
    #[inline(always)]
    pub fn force_discharge(&self) -> ForceDischargeR {
        ForceDischargeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 0b: No Bleed Discharge implemented in \n\nTCPC. 1b: Bleed Discharge is implemented \n\nin \n\nthe \n\nTCPC \n\nSupport \n\nfor \n\nPOWER_CONTROL.EnableBleedDischarge \n\nimplemented"]
    #[inline(always)]
    pub fn bleed_discharge(&self) -> BleedDischargeR {
        BleedDischargeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 0b: VBUS OVP is not reported by the \n\nTCPC. 1b: VBUS OVP is reported by the \n\nTCPC \n\nSupport \n\nfor \n\nboth \n\nFAULT_STATUS.InternalorExternalOVP \n\nand \n\nFAULT_CONTROL.InternalorExternalOVP \n\nimplemented"]
    #[inline(always)]
    pub fn vbus_ovp_reporting(&self) -> VbusOvpReportingR {
        VbusOvpReportingR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 0b: VBUS OCP is not reported by the \n\nTCPC. 1b: VBUS OCP is reported by the \n\nTCPC \n\nSupport \n\nfor \n\nboth \n\nFAULT_STATUS.InternalorExternalOCP \n\nand \n\nFAULT_CONTROL.InternalorExternalOC\n\nP implemented"]
    #[inline(always)]
    pub fn vbus_ocp_reporting(&self) -> VbusOcpReportingR {
        VbusOcpReportingR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Not used. Always write as 0 and ignore read \n\nvalue. Always reads0."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Device Capabilities Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`device_capabilities_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeviceCapabilities1Spec;
impl crate::RegisterSpec for DeviceCapabilities1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`device_capabilities_1::R`](R) reader structure"]
impl crate::Readable for DeviceCapabilities1Spec {}
#[doc = "`reset()` method sets DEVICE_CAPABILITIES_1 to value 0x7edf"]
impl crate::Resettable for DeviceCapabilities1Spec {
    const RESET_VALUE: u32 = 0x7edf;
}
