#[doc = "Register `STANDARD_OUTPUT_CAPABILITIES` reader"]
pub type R = crate::R<StandardOutputCapabilitiesSpec>;
#[doc = "Field `Connector_Orientation` reader - 0b: Not present in \n\nTCPC 1b: Present \n\ninTCPC"]
pub type ConnectorOrientationR = crate::BitReader;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionPresent {
    #[doc = "0: No Connection"]
    B0 = 0,
    #[doc = "1: Connection Controlled by theTCPM."]
    B1 = 1,
}
impl From<ConnectionPresent> for bool {
    #[inline(always)]
    fn from(variant: ConnectionPresent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Connection_Present` reader - "]
pub type ConnectionPresentR = crate::BitReader<ConnectionPresent>;
impl ConnectionPresentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ConnectionPresent {
        match self.bits {
            false => ConnectionPresent::B0,
            true => ConnectionPresent::B1,
        }
    }
    #[doc = "No Connection"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ConnectionPresent::B0
    }
    #[doc = "Connection Controlled by theTCPM."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ConnectionPresent::B1
    }
}
#[doc = "Field `MUX_Configuration_Control` reader - 0b: Not present in \n\nTCPC 1b: Present \n\ninTCPC"]
pub type MuxConfigurationControlR = crate::BitReader;
#[doc = "Field `Active_Cable_Indicator` reader - 0b: Not present in \n\nTCPC 1b: Present in \n\nTCPC"]
pub type ActiveCableIndicatorR = crate::BitReader;
#[doc = "Field `Audio_Adapter_Accessory_Indicator` reader - 0b: Not present in \n\nTCPC 1b: Present in \n\nTCPC"]
pub type AudioAdapterAccessoryIndicatorR = crate::BitReader;
#[doc = "Field `VBUS_Present_Monitor` reader - 0b: Not present in \n\nTCPC 1b: Present in \n\nTCPC"]
pub type VbusPresentMonitorR = crate::BitReader;
#[doc = "Field `Debug_Accessory_Indicator` reader - 0b: Not present in \n\nTCPC 1b: Present in \n\nTCPC"]
pub type DebugAccessoryIndicatorR = crate::BitReader;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore \n\nread value. Always reads0."]
pub type NotUsedR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0b: Not present in \n\nTCPC 1b: Present \n\ninTCPC"]
    #[inline(always)]
    pub fn connector_orientation(&self) -> ConnectorOrientationR {
        ConnectorOrientationR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn connection_present(&self) -> ConnectionPresentR {
        ConnectionPresentR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0b: Not present in \n\nTCPC 1b: Present \n\ninTCPC"]
    #[inline(always)]
    pub fn mux_configuration_control(&self) -> MuxConfigurationControlR {
        MuxConfigurationControlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0b: Not present in \n\nTCPC 1b: Present in \n\nTCPC"]
    #[inline(always)]
    pub fn active_cable_indicator(&self) -> ActiveCableIndicatorR {
        ActiveCableIndicatorR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0b: Not present in \n\nTCPC 1b: Present in \n\nTCPC"]
    #[inline(always)]
    pub fn audio_adapter_accessory_indicator(&self) -> AudioAdapterAccessoryIndicatorR {
        AudioAdapterAccessoryIndicatorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0b: Not present in \n\nTCPC 1b: Present in \n\nTCPC"]
    #[inline(always)]
    pub fn vbus_present_monitor(&self) -> VbusPresentMonitorR {
        VbusPresentMonitorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0b: Not present in \n\nTCPC 1b: Present in \n\nTCPC"]
    #[inline(always)]
    pub fn debug_accessory_indicator(&self) -> DebugAccessoryIndicatorR {
        DebugAccessoryIndicatorR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore \n\nread value. Always reads0."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "Standard Output Capabilities Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`standard_output_capabilities::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StandardOutputCapabilitiesSpec;
impl crate::RegisterSpec for StandardOutputCapabilitiesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`standard_output_capabilities::R`](R) reader structure"]
impl crate::Readable for StandardOutputCapabilitiesSpec {}
#[doc = "`reset()` method sets STANDARD_OUTPUT_CAPABILITIES to value 0x7f"]
impl crate::Resettable for StandardOutputCapabilitiesSpec {
    const RESET_VALUE: u32 = 0x7f;
}
