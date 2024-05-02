#[doc = "Register `POWER_STATUS` reader"]
pub type R = crate::R<PowerStatusSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SinkVbus {
    #[doc = "0: Sink is Disconnected (Default and if not supported)"]
    B0 = 0,
    #[doc = "1: TCPC is sinking VBUS to the system load Required"]
    B1 = 1,
}
impl From<SinkVbus> for bool {
    #[inline(always)]
    fn from(variant: SinkVbus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Sink_VBUS` reader - "]
pub type SinkVbusR = crate::BitReader<SinkVbus>;
impl SinkVbusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SinkVbus {
        match self.bits {
            false => SinkVbus::B0,
            true => SinkVbus::B1,
        }
    }
    #[doc = "Sink is Disconnected (Default and if not supported)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SinkVbus::B0
    }
    #[doc = "TCPC is sinking VBUS to the system load Required"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SinkVbus::B1
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VconnPresent {
    #[doc = "0: VCONN is not present"]
    B0 = 0,
    #[doc = "1: This bit is asserted when VCONN is enabled for CC1 or CC2. Measurement of board-level VCONN supply is not supported. If POWER_CONTROL.EnableVCONN is disabled VCONN Present will be set to0b. Required"]
    B1 = 1,
}
impl From<VconnPresent> for bool {
    #[inline(always)]
    fn from(variant: VconnPresent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCONN_Present` reader - "]
pub type VconnPresentR = crate::BitReader<VconnPresent>;
impl VconnPresentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VconnPresent {
        match self.bits {
            false => VconnPresent::B0,
            true => VconnPresent::B1,
        }
    }
    #[doc = "VCONN is not present"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VconnPresent::B0
    }
    #[doc = "This bit is asserted when VCONN is enabled for CC1 or CC2. Measurement of board-level VCONN supply is not supported. If POWER_CONTROL.EnableVCONN is disabled VCONN Present will be set to0b. Required"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VconnPresent::B1
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VbusPresent {
    #[doc = "0: VBUS Disconnected"]
    B0 = 0,
    #[doc = "1: VBUSConnected The TCPC shall report VBUS present when TCPC detects VBUS rises above 4V. The TCPC shall report VBUS is not present when TCPC detects VBUS falls below 3.5V. The TCPC may report VBUS is not present if VBUS is between 3.5V and4V. Required"]
    B1 = 1,
}
impl From<VbusPresent> for bool {
    #[inline(always)]
    fn from(variant: VbusPresent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUS_Present` reader - "]
pub type VbusPresentR = crate::BitReader<VbusPresent>;
impl VbusPresentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbusPresent {
        match self.bits {
            false => VbusPresent::B0,
            true => VbusPresent::B1,
        }
    }
    #[doc = "VBUS Disconnected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VbusPresent::B0
    }
    #[doc = "VBUSConnected The TCPC shall report VBUS present when TCPC detects VBUS rises above 4V. The TCPC shall report VBUS is not present when TCPC detects VBUS falls below 3.5V. The TCPC may report VBUS is not present if VBUS is between 3.5V and4V. Required"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VbusPresent::B1
    }
}
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VbusPresentDetectionEnabled {
    #[doc = "0: VBUS Present Detection Disabled"]
    B0 = 0,
    #[doc = "1: VBUS Present Detection Enabled (default) Indicates if the TCPC is monitoring for VBUS Present or if the circuit has been powered off. Required"]
    B1 = 1,
}
impl From<VbusPresentDetectionEnabled> for bool {
    #[inline(always)]
    fn from(variant: VbusPresentDetectionEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUS_Present_Detection_Enabled` reader - "]
pub type VbusPresentDetectionEnabledR = crate::BitReader<VbusPresentDetectionEnabled>;
impl VbusPresentDetectionEnabledR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbusPresentDetectionEnabled {
        match self.bits {
            false => VbusPresentDetectionEnabled::B0,
            true => VbusPresentDetectionEnabled::B1,
        }
    }
    #[doc = "VBUS Present Detection Disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VbusPresentDetectionEnabled::B0
    }
    #[doc = "VBUS Present Detection Enabled (default) Indicates if the TCPC is monitoring for VBUS Present or if the circuit has been powered off. Required"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VbusPresentDetectionEnabled::B1
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourcingVbus {
    #[doc = "0: Sourcing Vbus is disabled"]
    B0 = 0,
    #[doc = "1: Sourcing Vbus is enabled This does not control the path, just provides a monitor of the status. Required"]
    B1 = 1,
}
impl From<SourcingVbus> for bool {
    #[inline(always)]
    fn from(variant: SourcingVbus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Sourcing_VBUS` reader - "]
pub type SourcingVbusR = crate::BitReader<SourcingVbus>;
impl SourcingVbusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SourcingVbus {
        match self.bits {
            false => SourcingVbus::B0,
            true => SourcingVbus::B1,
        }
    }
    #[doc = "Sourcing Vbus is disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SourcingVbus::B0
    }
    #[doc = "Sourcing Vbus is enabled This does not control the path, just provides a monitor of the status. Required"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SourcingVbus::B1
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourcingHighVoltage {
    #[doc = "0: vSafe5V"]
    B0 = 0,
    #[doc = "1: HighVoltage This does not control the path, just provides a monitor of the status. Assert as long as supplying voltage greater than vSafe5V. Required if voltage higher than vSafe5V can be sourced"]
    B1 = 1,
}
impl From<SourcingHighVoltage> for bool {
    #[inline(always)]
    fn from(variant: SourcingHighVoltage) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Sourcing_High_Voltage` reader - "]
pub type SourcingHighVoltageR = crate::BitReader<SourcingHighVoltage>;
impl SourcingHighVoltageR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SourcingHighVoltage {
        match self.bits {
            false => SourcingHighVoltage::B0,
            true => SourcingHighVoltage::B1,
        }
    }
    #[doc = "vSafe5V"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SourcingHighVoltage::B0
    }
    #[doc = "HighVoltage This does not control the path, just provides a monitor of the status. Assert as long as supplying voltage greater than vSafe5V. Required if voltage higher than vSafe5V can be sourced"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SourcingHighVoltage::B1
    }
}
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TcpcInitializationStatus {
    #[doc = "0: The TCPC has completed initialization and all registers are valid"]
    B0 = 0,
    #[doc = "1: The TCPC is still performing internal initialization and the only registers that are guaranteed to return the correct values are 00h..0Fh. Required"]
    B1 = 1,
}
impl From<TcpcInitializationStatus> for bool {
    #[inline(always)]
    fn from(variant: TcpcInitializationStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCPC_Initialization_Status` reader - "]
pub type TcpcInitializationStatusR = crate::BitReader<TcpcInitializationStatus>;
impl TcpcInitializationStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TcpcInitializationStatus {
        match self.bits {
            false => TcpcInitializationStatus::B0,
            true => TcpcInitializationStatus::B1,
        }
    }
    #[doc = "The TCPC has completed initialization and all registers are valid"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TcpcInitializationStatus::B0
    }
    #[doc = "The TCPC is still performing internal initialization and the only registers that are guaranteed to return the correct values are 00h..0Fh. Required"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TcpcInitializationStatus::B1
    }
}
#[doc = "Field `Debug_Accessory_Connected` reader - 0b: \n\nNo \n\nDebug \n\nAccessory \n\nconnected \n\n(default) 1b: Debug Accessoryconnected \n\nReflects \n\nthe \n\nstate \n\nof \n\ntheDebugAccessoryConnected# \n\noutput \n\nif \n\nsupported Required \n\n(Register \n\nis \n\nrequired but \n\noutput is not required)"]
pub type DebugAccessoryConnectedR = crate::BitReader;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore read \n\nvalue. Always reads0."]
pub type NotUsedR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sink_vbus(&self) -> SinkVbusR {
        SinkVbusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vconn_present(&self) -> VconnPresentR {
        VconnPresentR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vbus_present(&self) -> VbusPresentR {
        VbusPresentR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vbus_present_detection_enabled(&self) -> VbusPresentDetectionEnabledR {
        VbusPresentDetectionEnabledR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sourcing_vbus(&self) -> SourcingVbusR {
        SourcingVbusR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sourcing_high_voltage(&self) -> SourcingHighVoltageR {
        SourcingHighVoltageR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tcpc_initialization_status(&self) -> TcpcInitializationStatusR {
        TcpcInitializationStatusR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0b: \n\nNo \n\nDebug \n\nAccessory \n\nconnected \n\n(default) 1b: Debug Accessoryconnected \n\nReflects \n\nthe \n\nstate \n\nof \n\ntheDebugAccessoryConnected# \n\noutput \n\nif \n\nsupported Required \n\n(Register \n\nis \n\nrequired but \n\noutput is not required)"]
    #[inline(always)]
    pub fn debug_accessory_connected(&self) -> DebugAccessoryConnectedR {
        DebugAccessoryConnectedR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore read \n\nvalue. Always reads0."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "Power Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerStatusSpec;
impl crate::RegisterSpec for PowerStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_status::R`](R) reader structure"]
impl crate::Readable for PowerStatusSpec {}
#[doc = "`reset()` method sets POWER_STATUS to value 0x48"]
impl crate::Resettable for PowerStatusSpec {
    const RESET_VALUE: u32 = 0x48;
}
