#[doc = "Register `STANDARD_INPUT_CAPABILITIES` reader"]
pub type R = crate::R<StandardInputCapabilitiesSpec>;
#[doc = "Field `Force_Off_VBUS_Source_or_Sink` reader - 0b: Not present in \n\nTCPC 1b: Present \n\nin TCPC"]
pub type ForceOffVbusSourceOrSinkR = crate::BitReader;
#[doc = "Field `Vbus_External_Over_Current_Fault` reader - 0b: Not present in \n\nTCPC 1b: Present \n\nin TCPC"]
pub type VbusExternalOverCurrentFaultR = crate::BitReader;
#[doc = "Field `Vbus_External_Over_Voltage_Fault` reader - 0b: Not present in \n\nTCPC 1b: Present \n\nin TCPC"]
pub type VbusExternalOverVoltageFaultR = crate::BitReader;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore \n\nread value. Always reads0."]
pub type NotUsedR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0b: Not present in \n\nTCPC 1b: Present \n\nin TCPC"]
    #[inline(always)]
    pub fn force_off_vbus_source_or_sink(&self) -> ForceOffVbusSourceOrSinkR {
        ForceOffVbusSourceOrSinkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0b: Not present in \n\nTCPC 1b: Present \n\nin TCPC"]
    #[inline(always)]
    pub fn vbus_external_over_current_fault(&self) -> VbusExternalOverCurrentFaultR {
        VbusExternalOverCurrentFaultR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0b: Not present in \n\nTCPC 1b: Present \n\nin TCPC"]
    #[inline(always)]
    pub fn vbus_external_over_voltage_fault(&self) -> VbusExternalOverVoltageFaultR {
        VbusExternalOverVoltageFaultR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore \n\nread value. Always reads0."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "Standard Input Capabilities Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`standard_input_capabilities::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StandardInputCapabilitiesSpec;
impl crate::RegisterSpec for StandardInputCapabilitiesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`standard_input_capabilities::R`](R) reader structure"]
impl crate::Readable for StandardInputCapabilitiesSpec {}
#[doc = "`reset()` method sets STANDARD_INPUT_CAPABILITIES to value 0x07"]
impl crate::Resettable for StandardInputCapabilitiesSpec {
    const RESET_VALUE: u32 = 0x07;
}
