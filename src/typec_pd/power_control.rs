#[doc = "Register `POWER_CONTROL` reader"]
pub type R = crate::R<PowerControlSpec>;
#[doc = "Register `POWER_CONTROL` writer"]
pub type W = crate::W<PowerControlSpec>;
#[doc = "Field `Force_Discharge` reader - 0b: \n\nDisable \n\nforced \n\ndischarge \n\n(default) 1b: Enable forced discharge \n\nof VBUS. Required if \n\nDEVICE_CAPABILITIES_1.ForceDischarge =1b"]
pub type ForceDischargeR = crate::BitReader;
#[doc = "Field `Force_Discharge` writer - 0b: \n\nDisable \n\nforced \n\ndischarge \n\n(default) 1b: Enable forced discharge \n\nof VBUS. Required if \n\nDEVICE_CAPABILITIES_1.ForceDischarge =1b"]
pub type ForceDischargeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnableBleedDischarge {
    #[doc = "0: Disable bleed discharge(default)"]
    B0 = 0,
    #[doc = "1: Enable bleed discharge of VBUS Bleed Discharge is a low current discharge to provide a minimum load current if needed 10K Ohms or 2mArecommended Required if DEVICE_CAPABILITIES_1.BleedDischarge =1b"]
    B1 = 1,
}
impl From<EnableBleedDischarge> for bool {
    #[inline(always)]
    fn from(variant: EnableBleedDischarge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Enable_Bleed_Discharge` reader - "]
pub type EnableBleedDischargeR = crate::BitReader<EnableBleedDischarge>;
impl EnableBleedDischargeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnableBleedDischarge {
        match self.bits {
            false => EnableBleedDischarge::B0,
            true => EnableBleedDischarge::B1,
        }
    }
    #[doc = "Disable bleed discharge(default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EnableBleedDischarge::B0
    }
    #[doc = "Enable bleed discharge of VBUS Bleed Discharge is a low current discharge to provide a minimum load current if needed 10K Ohms or 2mArecommended Required if DEVICE_CAPABILITIES_1.BleedDischarge =1b"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EnableBleedDischarge::B1
    }
}
#[doc = "Field `Enable_Bleed_Discharge` writer - "]
pub type EnableBleedDischargeW<'a, REG> = crate::BitWriter<'a, REG, EnableBleedDischarge>;
impl<'a, REG> EnableBleedDischargeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable bleed discharge(default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EnableBleedDischarge::B0)
    }
    #[doc = "Enable bleed discharge of VBUS Bleed Discharge is a low current discharge to provide a minimum load current if needed 10K Ohms or 2mArecommended Required if DEVICE_CAPABILITIES_1.BleedDischarge =1b"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EnableBleedDischarge::B1)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutoDischargeDisconnect {
    #[doc = "0: The TCPC shall not automatically discharge VBUS based on VBUS voltage."]
    B0 = 0,
    #[doc = "1: The TCPC shall automatically discharge (default) Refer to 4.4.5.4.1 and 4.4.5.4.2. Strength of discharge set by tAutoDischarge in Table4-18 Setting this bit in a Source TCPC triggers thefollowing actions upon disconnection detection: 1. Disable sourcing power over VBUS 2. VBUS discharge Sourcing power over VBUS shall be disabled before or at same time as starting VBUS discharge. Setting this bit in a Sink TCPC triggers the following action upon disconnection detection: 1. VBUS discharge The TCPC shall automatically disable discharge once the voltage on VBUS is below vSafe0V (max) or VBUS_STOP_DISCHARGE_THRESHOLD. Disconnect detection is defined in Figure 4- 12. VBUS_STOP_DISCHARGE_THRESHOLD,if enabled, takes priority over vSafe0V. Required"]
    B1 = 1,
}
impl From<AutoDischargeDisconnect> for bool {
    #[inline(always)]
    fn from(variant: AutoDischargeDisconnect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AutoDischargeDisconnect` reader - "]
pub type AutoDischargeDisconnectR = crate::BitReader<AutoDischargeDisconnect>;
impl AutoDischargeDisconnectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AutoDischargeDisconnect {
        match self.bits {
            false => AutoDischargeDisconnect::B0,
            true => AutoDischargeDisconnect::B1,
        }
    }
    #[doc = "The TCPC shall not automatically discharge VBUS based on VBUS voltage."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AutoDischargeDisconnect::B0
    }
    #[doc = "The TCPC shall automatically discharge (default) Refer to 4.4.5.4.1 and 4.4.5.4.2. Strength of discharge set by tAutoDischarge in Table4-18 Setting this bit in a Source TCPC triggers thefollowing actions upon disconnection detection: 1. Disable sourcing power over VBUS 2. VBUS discharge Sourcing power over VBUS shall be disabled before or at same time as starting VBUS discharge. Setting this bit in a Sink TCPC triggers the following action upon disconnection detection: 1. VBUS discharge The TCPC shall automatically disable discharge once the voltage on VBUS is below vSafe0V (max) or VBUS_STOP_DISCHARGE_THRESHOLD. Disconnect detection is defined in Figure 4- 12. VBUS_STOP_DISCHARGE_THRESHOLD,if enabled, takes priority over vSafe0V. Required"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AutoDischargeDisconnect::B1
    }
}
#[doc = "Field `AutoDischargeDisconnect` writer - "]
pub type AutoDischargeDisconnectW<'a, REG> = crate::BitWriter<'a, REG, AutoDischargeDisconnect>;
impl<'a, REG> AutoDischargeDisconnectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The TCPC shall not automatically discharge VBUS based on VBUS voltage."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AutoDischargeDisconnect::B0)
    }
    #[doc = "The TCPC shall automatically discharge (default) Refer to 4.4.5.4.1 and 4.4.5.4.2. Strength of discharge set by tAutoDischarge in Table4-18 Setting this bit in a Source TCPC triggers thefollowing actions upon disconnection detection: 1. Disable sourcing power over VBUS 2. VBUS discharge Sourcing power over VBUS shall be disabled before or at same time as starting VBUS discharge. Setting this bit in a Sink TCPC triggers the following action upon disconnection detection: 1. VBUS discharge The TCPC shall automatically disable discharge once the voltage on VBUS is below vSafe0V (max) or VBUS_STOP_DISCHARGE_THRESHOLD. Disconnect detection is defined in Figure 4- 12. VBUS_STOP_DISCHARGE_THRESHOLD,if enabled, takes priority over vSafe0V. Required"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AutoDischargeDisconnect::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisableVoltageAlarms {
    #[doc = "0: Voltage Alarms Power status reporting is enabled (default)"]
    B0 = 0,
    #[doc = "1: Voltage Alarms Power status reporting is disabled Controls VBUS_VOLTAGE_ALARM_HI_CFG and VBUS_VOLTAGE_ALARM_LO_CFG. Required DEVICE_CAPABILITIES_1.VBUSMeasurementandA larmCapable =1b"]
    B1 = 1,
}
impl From<DisableVoltageAlarms> for bool {
    #[inline(always)]
    fn from(variant: DisableVoltageAlarms) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Disable_Voltage_Alarms` reader - "]
pub type DisableVoltageAlarmsR = crate::BitReader<DisableVoltageAlarms>;
impl DisableVoltageAlarmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DisableVoltageAlarms {
        match self.bits {
            false => DisableVoltageAlarms::B0,
            true => DisableVoltageAlarms::B1,
        }
    }
    #[doc = "Voltage Alarms Power status reporting is enabled (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DisableVoltageAlarms::B0
    }
    #[doc = "Voltage Alarms Power status reporting is disabled Controls VBUS_VOLTAGE_ALARM_HI_CFG and VBUS_VOLTAGE_ALARM_LO_CFG. Required DEVICE_CAPABILITIES_1.VBUSMeasurementandA larmCapable =1b"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DisableVoltageAlarms::B1
    }
}
#[doc = "Field `Disable_Voltage_Alarms` writer - "]
pub type DisableVoltageAlarmsW<'a, REG> = crate::BitWriter<'a, REG, DisableVoltageAlarms>;
impl<'a, REG> DisableVoltageAlarmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage Alarms Power status reporting is enabled (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DisableVoltageAlarms::B0)
    }
    #[doc = "Voltage Alarms Power status reporting is disabled Controls VBUS_VOLTAGE_ALARM_HI_CFG and VBUS_VOLTAGE_ALARM_LO_CFG. Required DEVICE_CAPABILITIES_1.VBUSMeasurementandA larmCapable =1b"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DisableVoltageAlarms::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VbusVoltageMonitor {
    #[doc = "0: VBUS_VOLTAGE Monitoring is enabled (default)"]
    B0 = 0,
    #[doc = "1: VBUS_VOLTAGE Monitoring is disabled Controls only VBUS_VOLTAGE Monitoring. VBUS_VOLTAGE shall report all zeros if disabled. Required DEVICE_CAPABILITIES_1.VBUSMeasurementandA larmCapable =1b"]
    B1 = 1,
}
impl From<VbusVoltageMonitor> for bool {
    #[inline(always)]
    fn from(variant: VbusVoltageMonitor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUS_VOLTAGE_Monitor` reader - "]
pub type VbusVoltageMonitorR = crate::BitReader<VbusVoltageMonitor>;
impl VbusVoltageMonitorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbusVoltageMonitor {
        match self.bits {
            false => VbusVoltageMonitor::B0,
            true => VbusVoltageMonitor::B1,
        }
    }
    #[doc = "VBUS_VOLTAGE Monitoring is enabled (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VbusVoltageMonitor::B0
    }
    #[doc = "VBUS_VOLTAGE Monitoring is disabled Controls only VBUS_VOLTAGE Monitoring. VBUS_VOLTAGE shall report all zeros if disabled. Required DEVICE_CAPABILITIES_1.VBUSMeasurementandA larmCapable =1b"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VbusVoltageMonitor::B1
    }
}
#[doc = "Field `VBUS_VOLTAGE_Monitor` writer - "]
pub type VbusVoltageMonitorW<'a, REG> = crate::BitWriter<'a, REG, VbusVoltageMonitor>;
impl<'a, REG> VbusVoltageMonitorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBUS_VOLTAGE Monitoring is enabled (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VbusVoltageMonitor::B0)
    }
    #[doc = "VBUS_VOLTAGE Monitoring is disabled Controls only VBUS_VOLTAGE Monitoring. VBUS_VOLTAGE shall report all zeros if disabled. Required DEVICE_CAPABILITIES_1.VBUSMeasurementandA larmCapable =1b"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VbusVoltageMonitor::B1)
    }
}
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore read \n\nvalue."]
pub type NotUsedR = crate::FieldReader<u32>;
#[doc = "Field `not_used` writer - Not used. Always write as 0 and ignore read \n\nvalue."]
pub type NotUsedW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 2 - 0b: \n\nDisable \n\nforced \n\ndischarge \n\n(default) 1b: Enable forced discharge \n\nof VBUS. Required if \n\nDEVICE_CAPABILITIES_1.ForceDischarge =1b"]
    #[inline(always)]
    pub fn force_discharge(&self) -> ForceDischargeR {
        ForceDischargeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn enable_bleed_discharge(&self) -> EnableBleedDischargeR {
        EnableBleedDischargeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn auto_discharge_disconnect(&self) -> AutoDischargeDisconnectR {
        AutoDischargeDisconnectR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn disable_voltage_alarms(&self) -> DisableVoltageAlarmsR {
        DisableVoltageAlarmsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn vbus_voltage_monitor(&self) -> VbusVoltageMonitorR {
        VbusVoltageMonitorR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore read \n\nvalue."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 2 - 0b: \n\nDisable \n\nforced \n\ndischarge \n\n(default) 1b: Enable forced discharge \n\nof VBUS. Required if \n\nDEVICE_CAPABILITIES_1.ForceDischarge =1b"]
    #[inline(always)]
    #[must_use]
    pub fn force_discharge(&mut self) -> ForceDischargeW<PowerControlSpec> {
        ForceDischargeW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn enable_bleed_discharge(&mut self) -> EnableBleedDischargeW<PowerControlSpec> {
        EnableBleedDischargeW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn auto_discharge_disconnect(&mut self) -> AutoDischargeDisconnectW<PowerControlSpec> {
        AutoDischargeDisconnectW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn disable_voltage_alarms(&mut self) -> DisableVoltageAlarmsW<PowerControlSpec> {
        DisableVoltageAlarmsW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_voltage_monitor(&mut self) -> VbusVoltageMonitorW<PowerControlSpec> {
        VbusVoltageMonitorW::new(self, 6)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore read \n\nvalue."]
    #[inline(always)]
    #[must_use]
    pub fn not_used(&mut self) -> NotUsedW<PowerControlSpec> {
        NotUsedW::new(self, 8)
    }
}
#[doc = "Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerControlSpec;
impl crate::RegisterSpec for PowerControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_control::R`](R) reader structure"]
impl crate::Readable for PowerControlSpec {}
#[doc = "`write(|w| ..)` method takes [`power_control::W`](W) writer structure"]
impl crate::Writable for PowerControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWER_CONTROL to value 0x10"]
impl crate::Resettable for PowerControlSpec {
    const RESET_VALUE: u32 = 0x10;
}
