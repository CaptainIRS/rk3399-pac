#[doc = "Register `FAULT_STATUS` reader"]
pub type R = crate::R<FaultStatusSpec>;
#[doc = "Register `FAULT_STATUS` writer"]
pub type W = crate::W<FaultStatusSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterfaceError {
    #[doc = "0: NoError"]
    B0 = 0,
    #[doc = "1: Error hasoccurred. A TRANSMIT has been sent with an empty TRANSMIT_BUFFER. May be asserted if a non-zero value has been written to a reserved bit in aregister. Required"]
    B1 = 1,
}
impl From<InterfaceError> for bool {
    #[inline(always)]
    fn from(variant: InterfaceError) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Interface_Error` reader - "]
pub type InterfaceErrorR = crate::BitReader<InterfaceError>;
impl InterfaceErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InterfaceError {
        match self.bits {
            false => InterfaceError::B0,
            true => InterfaceError::B1,
        }
    }
    #[doc = "NoError"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == InterfaceError::B0
    }
    #[doc = "Error hasoccurred. A TRANSMIT has been sent with an empty TRANSMIT_BUFFER. May be asserted if a non-zero value has been written to a reserved bit in aregister. Required"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == InterfaceError::B1
    }
}
#[doc = "Field `Interface_Error` writer - "]
pub type InterfaceErrorW<'a, REG> = crate::BitWriter1C<'a, REG, InterfaceError>;
impl<'a, REG> InterfaceErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NoError"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(InterfaceError::B0)
    }
    #[doc = "Error hasoccurred. A TRANSMIT has been sent with an empty TRANSMIT_BUFFER. May be asserted if a non-zero value has been written to a reserved bit in aregister. Required"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(InterfaceError::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VconnOverCurrentFault {
    #[doc = "0: No Faultdetected"]
    B0 = 0,
    #[doc = "1: Over current VCONN fault latched Requiredif DEVICE_CAPABILITIES_2.VCONNOvercurrentF ault Capable =1b"]
    B1 = 1,
}
impl From<VconnOverCurrentFault> for bool {
    #[inline(always)]
    fn from(variant: VconnOverCurrentFault) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCONN_Over_Current_Fault` reader - "]
pub type VconnOverCurrentFaultR = crate::BitReader<VconnOverCurrentFault>;
impl VconnOverCurrentFaultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VconnOverCurrentFault {
        match self.bits {
            false => VconnOverCurrentFault::B0,
            true => VconnOverCurrentFault::B1,
        }
    }
    #[doc = "No Faultdetected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VconnOverCurrentFault::B0
    }
    #[doc = "Over current VCONN fault latched Requiredif DEVICE_CAPABILITIES_2.VCONNOvercurrentF ault Capable =1b"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VconnOverCurrentFault::B1
    }
}
#[doc = "Field `VCONN_Over_Current_Fault` writer - "]
pub type VconnOverCurrentFaultW<'a, REG> = crate::BitWriter1C<'a, REG, VconnOverCurrentFault>;
impl<'a, REG> VconnOverCurrentFaultW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Faultdetected"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VconnOverCurrentFault::B0)
    }
    #[doc = "Over current VCONN fault latched Requiredif DEVICE_CAPABILITIES_2.VCONNOvercurrentF ault Capable =1b"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VconnOverCurrentFault::B1)
    }
}
#[doc = "Field `Internal_or_External_OVP_VBUS_Over_Voltage_Protection_Fault` reader - 0b: Not in an over-voltage protection \n\nstate 1b: Over-voltage faultlatched. \n\nRequired \n\nif \n\nDEVICE_CAPABILIITES_1.VBUSOVPReportin\n\ng= \n\n1b"]
pub type InternalOrExternalOvpVbusOverVoltageProtectionFaultR = crate::BitReader;
#[doc = "Field `Internal_or_External_OVP_VBUS_Over_Voltage_Protection_Fault` writer - 0b: Not in an over-voltage protection \n\nstate 1b: Over-voltage faultlatched. \n\nRequired \n\nif \n\nDEVICE_CAPABILIITES_1.VBUSOVPReportin\n\ng= \n\n1b"]
pub type InternalOrExternalOvpVbusOverVoltageProtectionFaultW<'a, REG> =
    crate::BitWriter1C<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InternalOrExternalOcpVbusOverCurrentProtectionFault {
    #[doc = "0: Not in an over-current protectionstate"]
    B0 = 0,
    #[doc = "1: Over-current faultlatched Required if DEVICE_CAPABILIITES_1.VBUSOCPReportin g= 1b"]
    B1 = 1,
}
impl From<InternalOrExternalOcpVbusOverCurrentProtectionFault> for bool {
    #[inline(always)]
    fn from(variant: InternalOrExternalOcpVbusOverCurrentProtectionFault) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Internal_or_External_OCP_VBUS_Over_Current_Protection_Fault` reader - "]
pub type InternalOrExternalOcpVbusOverCurrentProtectionFaultR =
    crate::BitReader<InternalOrExternalOcpVbusOverCurrentProtectionFault>;
impl InternalOrExternalOcpVbusOverCurrentProtectionFaultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InternalOrExternalOcpVbusOverCurrentProtectionFault {
        match self.bits {
            false => InternalOrExternalOcpVbusOverCurrentProtectionFault::B0,
            true => InternalOrExternalOcpVbusOverCurrentProtectionFault::B1,
        }
    }
    #[doc = "Not in an over-current protectionstate"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == InternalOrExternalOcpVbusOverCurrentProtectionFault::B0
    }
    #[doc = "Over-current faultlatched Required if DEVICE_CAPABILIITES_1.VBUSOCPReportin g= 1b"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == InternalOrExternalOcpVbusOverCurrentProtectionFault::B1
    }
}
#[doc = "Field `Internal_or_External_OCP_VBUS_Over_Current_Protection_Fault` writer - "]
pub type InternalOrExternalOcpVbusOverCurrentProtectionFaultW<'a, REG> =
    crate::BitWriter1C<'a, REG, InternalOrExternalOcpVbusOverCurrentProtectionFault>;
impl<'a, REG> InternalOrExternalOcpVbusOverCurrentProtectionFaultW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not in an over-current protectionstate"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(InternalOrExternalOcpVbusOverCurrentProtectionFault::B0)
    }
    #[doc = "Over-current faultlatched Required if DEVICE_CAPABILIITES_1.VBUSOCPReportin g= 1b"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(InternalOrExternalOcpVbusOverCurrentProtectionFault::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForceDischargeFailed {
    #[doc = "0: No dischargefailure"]
    B0 = 0,
    #[doc = "1: Discharge commanded by the TCPMfailed If POWER_CONTROL.ForceDischarge is set, the TCPC shall report a discharge fails if VBUS is not below vSafe0V withintSafe0V. Required if DEVICE_CAPABILITIES_1.ForceDischarge= 1b"]
    B1 = 1,
}
impl From<ForceDischargeFailed> for bool {
    #[inline(always)]
    fn from(variant: ForceDischargeFailed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Force_Discharge_Failed` reader - "]
pub type ForceDischargeFailedR = crate::BitReader<ForceDischargeFailed>;
impl ForceDischargeFailedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ForceDischargeFailed {
        match self.bits {
            false => ForceDischargeFailed::B0,
            true => ForceDischargeFailed::B1,
        }
    }
    #[doc = "No dischargefailure"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ForceDischargeFailed::B0
    }
    #[doc = "Discharge commanded by the TCPMfailed If POWER_CONTROL.ForceDischarge is set, the TCPC shall report a discharge fails if VBUS is not below vSafe0V withintSafe0V. Required if DEVICE_CAPABILITIES_1.ForceDischarge= 1b"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ForceDischargeFailed::B1
    }
}
#[doc = "Field `Force_Discharge_Failed` writer - "]
pub type ForceDischargeFailedW<'a, REG> = crate::BitWriter1C<'a, REG, ForceDischargeFailed>;
impl<'a, REG> ForceDischargeFailedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No dischargefailure"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ForceDischargeFailed::B0)
    }
    #[doc = "Discharge commanded by the TCPMfailed If POWER_CONTROL.ForceDischarge is set, the TCPC shall report a discharge fails if VBUS is not below vSafe0V withintSafe0V. Required if DEVICE_CAPABILITIES_1.ForceDischarge= 1b"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ForceDischargeFailed::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutoDischargeFailed {
    #[doc = "0: No dischargefailure"]
    B0 = 0,
    #[doc = "1: Discharge commanded by the TCPMfailed If POWER_CONTROL.AutoDischargeDisconnect is set, the TCPC shall report discharge fails if VBUS is not below vSafe0V withintSafe0V. Required"]
    B1 = 1,
}
impl From<AutoDischargeFailed> for bool {
    #[inline(always)]
    fn from(variant: AutoDischargeFailed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Auto_Discharge_Failed` reader - "]
pub type AutoDischargeFailedR = crate::BitReader<AutoDischargeFailed>;
impl AutoDischargeFailedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AutoDischargeFailed {
        match self.bits {
            false => AutoDischargeFailed::B0,
            true => AutoDischargeFailed::B1,
        }
    }
    #[doc = "No dischargefailure"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AutoDischargeFailed::B0
    }
    #[doc = "Discharge commanded by the TCPMfailed If POWER_CONTROL.AutoDischargeDisconnect is set, the TCPC shall report discharge fails if VBUS is not below vSafe0V withintSafe0V. Required"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AutoDischargeFailed::B1
    }
}
#[doc = "Field `Auto_Discharge_Failed` writer - "]
pub type AutoDischargeFailedW<'a, REG> = crate::BitWriter1C<'a, REG, AutoDischargeFailed>;
impl<'a, REG> AutoDischargeFailedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No dischargefailure"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AutoDischargeFailed::B0)
    }
    #[doc = "Discharge commanded by the TCPMfailed If POWER_CONTROL.AutoDischargeDisconnect is set, the TCPC shall report discharge fails if VBUS is not below vSafe0V withintSafe0V. Required"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AutoDischargeFailed::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForceOffVbusSourceOrSink {
    #[doc = "0: No Fault Detected, no action (default and not supported)"]
    B0 = 0,
    #[doc = "1: VBUS Source/Sink has been forced off due to external fault The TCPC has disconnected VBUS due to STANDARD_INPUT.ForceOffVbus. Required if STANDARD_INPUT_CAPABILITIES_1.ForceOffV bus =1b"]
    B1 = 1,
}
impl From<ForceOffVbusSourceOrSink> for bool {
    #[inline(always)]
    fn from(variant: ForceOffVbusSourceOrSink) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Force_Off_VBUS_Source_or_Sink` reader - "]
pub type ForceOffVbusSourceOrSinkR = crate::BitReader<ForceOffVbusSourceOrSink>;
impl ForceOffVbusSourceOrSinkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ForceOffVbusSourceOrSink {
        match self.bits {
            false => ForceOffVbusSourceOrSink::B0,
            true => ForceOffVbusSourceOrSink::B1,
        }
    }
    #[doc = "No Fault Detected, no action (default and not supported)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ForceOffVbusSourceOrSink::B0
    }
    #[doc = "VBUS Source/Sink has been forced off due to external fault The TCPC has disconnected VBUS due to STANDARD_INPUT.ForceOffVbus. Required if STANDARD_INPUT_CAPABILITIES_1.ForceOffV bus =1b"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ForceOffVbusSourceOrSink::B1
    }
}
#[doc = "Field `Force_Off_VBUS_Source_or_Sink` writer - "]
pub type ForceOffVbusSourceOrSinkW<'a, REG> = crate::BitWriter1C<'a, REG, ForceOffVbusSourceOrSink>;
impl<'a, REG> ForceOffVbusSourceOrSinkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Fault Detected, no action (default and not supported)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ForceOffVbusSourceOrSink::B0)
    }
    #[doc = "VBUS Source/Sink has been forced off due to external fault The TCPC has disconnected VBUS due to STANDARD_INPUT.ForceOffVbus. Required if STANDARD_INPUT_CAPABILITIES_1.ForceOffV bus =1b"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ForceOffVbusSourceOrSink::B1)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn interface_error(&self) -> InterfaceErrorR {
        InterfaceErrorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vconn_over_current_fault(&self) -> VconnOverCurrentFaultR {
        VconnOverCurrentFaultR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0b: Not in an over-voltage protection \n\nstate 1b: Over-voltage faultlatched. \n\nRequired \n\nif \n\nDEVICE_CAPABILIITES_1.VBUSOVPReportin\n\ng= \n\n1b"]
    #[inline(always)]
    pub fn internal_or_external_ovp_vbus_over_voltage_protection_fault(
        &self,
    ) -> InternalOrExternalOvpVbusOverVoltageProtectionFaultR {
        InternalOrExternalOvpVbusOverVoltageProtectionFaultR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn internal_or_external_ocp_vbus_over_current_protection_fault(
        &self,
    ) -> InternalOrExternalOcpVbusOverCurrentProtectionFaultR {
        InternalOrExternalOcpVbusOverCurrentProtectionFaultR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn force_discharge_failed(&self) -> ForceDischargeFailedR {
        ForceDischargeFailedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn auto_discharge_failed(&self) -> AutoDischargeFailedR {
        AutoDischargeFailedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn force_off_vbus_source_or_sink(&self) -> ForceOffVbusSourceOrSinkR {
        ForceOffVbusSourceOrSinkR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn interface_error(&mut self) -> InterfaceErrorW<FaultStatusSpec> {
        InterfaceErrorW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn vconn_over_current_fault(&mut self) -> VconnOverCurrentFaultW<FaultStatusSpec> {
        VconnOverCurrentFaultW::new(self, 1)
    }
    #[doc = "Bit 2 - 0b: Not in an over-voltage protection \n\nstate 1b: Over-voltage faultlatched. \n\nRequired \n\nif \n\nDEVICE_CAPABILIITES_1.VBUSOVPReportin\n\ng= \n\n1b"]
    #[inline(always)]
    #[must_use]
    pub fn internal_or_external_ovp_vbus_over_voltage_protection_fault(
        &mut self,
    ) -> InternalOrExternalOvpVbusOverVoltageProtectionFaultW<FaultStatusSpec> {
        InternalOrExternalOvpVbusOverVoltageProtectionFaultW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn internal_or_external_ocp_vbus_over_current_protection_fault(
        &mut self,
    ) -> InternalOrExternalOcpVbusOverCurrentProtectionFaultW<FaultStatusSpec> {
        InternalOrExternalOcpVbusOverCurrentProtectionFaultW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn force_discharge_failed(&mut self) -> ForceDischargeFailedW<FaultStatusSpec> {
        ForceDischargeFailedW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn auto_discharge_failed(&mut self) -> AutoDischargeFailedW<FaultStatusSpec> {
        AutoDischargeFailedW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn force_off_vbus_source_or_sink(&mut self) -> ForceOffVbusSourceOrSinkW<FaultStatusSpec> {
        ForceOffVbusSourceOrSinkW::new(self, 6)
    }
}
#[doc = "Fault Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fault_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fault_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaultStatusSpec;
impl crate::RegisterSpec for FaultStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fault_status::R`](R) reader structure"]
impl crate::Readable for FaultStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`fault_status::W`](W) writer structure"]
impl crate::Writable for FaultStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x7f;
}
#[doc = "`reset()` method sets FAULT_STATUS to value 0"]
impl crate::Resettable for FaultStatusSpec {
    const RESET_VALUE: u32 = 0;
}
