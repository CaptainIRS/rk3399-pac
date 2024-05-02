#[doc = "Register `FAULT_CONTROL` reader"]
pub type R = crate::R<FaultControlSpec>;
#[doc = "Register `FAULT_CONTROL` writer"]
pub type W = crate::W<FaultControlSpec>;
#[doc = "Field `VCONN_Over_Current_Fault` reader - 0b: \n\nFault \n\ndetection \n\ncircuit \n\nenabled 1b: \n\nFault detection \n\ncircuit disabled Required if \n\nDEVICE_CAPABILITIES_2.VCONNOvercurrent\n\nFaultCapable =1b"]
pub type VconnOverCurrentFaultR = crate::BitReader;
#[doc = "Field `VCONN_Over_Current_Fault` writer - 0b: \n\nFault \n\ndetection \n\ncircuit \n\nenabled 1b: \n\nFault detection \n\ncircuit disabled Required if \n\nDEVICE_CAPABILITIES_2.VCONNOvercurrent\n\nFaultCapable =1b"]
pub type VconnOverCurrentFaultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Internal_or_External_OVP_VBUS_Over_Voltage_Protection_Fault` reader - 0b: Internal and External OVP circuit \n\nenabled 1b: Internal and External OVP \n\ncircuit disabled Required if \n\nDEVICE_CAPABILIITES_1.VBUSOVPReporting \n\n=1b"]
pub type InternalOrExternalOvpVbusOverVoltageProtectionFaultR = crate::BitReader;
#[doc = "Field `Internal_or_External_OVP_VBUS_Over_Voltage_Protection_Fault` writer - 0b: Internal and External OVP circuit \n\nenabled 1b: Internal and External OVP \n\ncircuit disabled Required if \n\nDEVICE_CAPABILIITES_1.VBUSOVPReporting \n\n=1b"]
pub type InternalOrExternalOvpVbusOverVoltageProtectionFaultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Internal_or_External_OCP_VBUS_Over_Current_Protection_Fault` reader - 0b: Internal and External OCP circuit \n\nenabled 1b: Internal and External OCP \n\ncircuit disabled Required if \n\nDEVICE_CAPABILIITES_1.VBUSOCPReporting \n\n=1b"]
pub type InternalOrExternalOcpVbusOverCurrentProtectionFaultR = crate::BitReader;
#[doc = "Field `Internal_or_External_OCP_VBUS_Over_Current_Protection_Fault` writer - 0b: Internal and External OCP circuit \n\nenabled 1b: Internal and External OCP \n\ncircuit disabled Required if \n\nDEVICE_CAPABILIITES_1.VBUSOCPReporting \n\n=1b"]
pub type InternalOrExternalOcpVbusOverCurrentProtectionFaultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUS_Discharge_Fault_Detection_Timer` reader - 0b: VBUS Discharge Fault Detection Timer \n\nenabled 1b: VBUS Discharge Fault Detection \n\nTimer disabled This enables the timers for \n\nboth \n\nFAULT_STATUTS.AutoDischargeFailed \n\nand FAULT_STATUS.ForceDischargeFailed \n\nRequired"]
pub type VbusDischargeFaultDetectionTimerR = crate::BitReader;
#[doc = "Field `VBUS_Discharge_Fault_Detection_Timer` writer - 0b: VBUS Discharge Fault Detection Timer \n\nenabled 1b: VBUS Discharge Fault Detection \n\nTimer disabled This enables the timers for \n\nboth \n\nFAULT_STATUTS.AutoDischargeFailed \n\nand FAULT_STATUS.ForceDischargeFailed \n\nRequired"]
pub type VbusDischargeFaultDetectionTimerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForceOffVbusSourceOrSink {
    #[doc = "0: Allow STANDARD INPUT SIGNAL Force Off Vbus control(default)"]
    B0 = 0,
    #[doc = "1: Block STANDARD INPUT SIGNAL Force Off Vbus control This enables or disables the STANDARD INPUT SIGNAL Force Off Vbus (4.5.1) functionality for debug purposes. Required if STANDARD_INPUT_CAPABILITES.ForceOff VBUS"]
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
    #[doc = "Allow STANDARD INPUT SIGNAL Force Off Vbus control(default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ForceOffVbusSourceOrSink::B0
    }
    #[doc = "Block STANDARD INPUT SIGNAL Force Off Vbus control This enables or disables the STANDARD INPUT SIGNAL Force Off Vbus (4.5.1) functionality for debug purposes. Required if STANDARD_INPUT_CAPABILITES.ForceOff VBUS"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ForceOffVbusSourceOrSink::B1
    }
}
#[doc = "Field `Force_Off_VBUS_Source_or_Sink` writer - "]
pub type ForceOffVbusSourceOrSinkW<'a, REG> = crate::BitWriter<'a, REG, ForceOffVbusSourceOrSink>;
impl<'a, REG> ForceOffVbusSourceOrSinkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Allow STANDARD INPUT SIGNAL Force Off Vbus control(default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ForceOffVbusSourceOrSink::B0)
    }
    #[doc = "Block STANDARD INPUT SIGNAL Force Off Vbus control This enables or disables the STANDARD INPUT SIGNAL Force Off Vbus (4.5.1) functionality for debug purposes. Required if STANDARD_INPUT_CAPABILITES.ForceOff VBUS"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ForceOffVbusSourceOrSink::B1)
    }
}
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore read \n\nvalue."]
pub type NotUsedR = crate::FieldReader<u32>;
#[doc = "Field `not_used` writer - Not used. Always write as 0 and ignore read \n\nvalue."]
pub type NotUsedW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0b: \n\nFault \n\ndetection \n\ncircuit \n\nenabled 1b: \n\nFault detection \n\ncircuit disabled Required if \n\nDEVICE_CAPABILITIES_2.VCONNOvercurrent\n\nFaultCapable =1b"]
    #[inline(always)]
    pub fn vconn_over_current_fault(&self) -> VconnOverCurrentFaultR {
        VconnOverCurrentFaultR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0b: Internal and External OVP circuit \n\nenabled 1b: Internal and External OVP \n\ncircuit disabled Required if \n\nDEVICE_CAPABILIITES_1.VBUSOVPReporting \n\n=1b"]
    #[inline(always)]
    pub fn internal_or_external_ovp_vbus_over_voltage_protection_fault(
        &self,
    ) -> InternalOrExternalOvpVbusOverVoltageProtectionFaultR {
        InternalOrExternalOvpVbusOverVoltageProtectionFaultR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0b: Internal and External OCP circuit \n\nenabled 1b: Internal and External OCP \n\ncircuit disabled Required if \n\nDEVICE_CAPABILIITES_1.VBUSOCPReporting \n\n=1b"]
    #[inline(always)]
    pub fn internal_or_external_ocp_vbus_over_current_protection_fault(
        &self,
    ) -> InternalOrExternalOcpVbusOverCurrentProtectionFaultR {
        InternalOrExternalOcpVbusOverCurrentProtectionFaultR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0b: VBUS Discharge Fault Detection Timer \n\nenabled 1b: VBUS Discharge Fault Detection \n\nTimer disabled This enables the timers for \n\nboth \n\nFAULT_STATUTS.AutoDischargeFailed \n\nand FAULT_STATUS.ForceDischargeFailed \n\nRequired"]
    #[inline(always)]
    pub fn vbus_discharge_fault_detection_timer(&self) -> VbusDischargeFaultDetectionTimerR {
        VbusDischargeFaultDetectionTimerR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn force_off_vbus_source_or_sink(&self) -> ForceOffVbusSourceOrSinkR {
        ForceOffVbusSourceOrSinkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore read \n\nvalue."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0b: \n\nFault \n\ndetection \n\ncircuit \n\nenabled 1b: \n\nFault detection \n\ncircuit disabled Required if \n\nDEVICE_CAPABILITIES_2.VCONNOvercurrent\n\nFaultCapable =1b"]
    #[inline(always)]
    #[must_use]
    pub fn vconn_over_current_fault(&mut self) -> VconnOverCurrentFaultW<FaultControlSpec> {
        VconnOverCurrentFaultW::new(self, 0)
    }
    #[doc = "Bit 1 - 0b: Internal and External OVP circuit \n\nenabled 1b: Internal and External OVP \n\ncircuit disabled Required if \n\nDEVICE_CAPABILIITES_1.VBUSOVPReporting \n\n=1b"]
    #[inline(always)]
    #[must_use]
    pub fn internal_or_external_ovp_vbus_over_voltage_protection_fault(
        &mut self,
    ) -> InternalOrExternalOvpVbusOverVoltageProtectionFaultW<FaultControlSpec> {
        InternalOrExternalOvpVbusOverVoltageProtectionFaultW::new(self, 1)
    }
    #[doc = "Bit 2 - 0b: Internal and External OCP circuit \n\nenabled 1b: Internal and External OCP \n\ncircuit disabled Required if \n\nDEVICE_CAPABILIITES_1.VBUSOCPReporting \n\n=1b"]
    #[inline(always)]
    #[must_use]
    pub fn internal_or_external_ocp_vbus_over_current_protection_fault(
        &mut self,
    ) -> InternalOrExternalOcpVbusOverCurrentProtectionFaultW<FaultControlSpec> {
        InternalOrExternalOcpVbusOverCurrentProtectionFaultW::new(self, 2)
    }
    #[doc = "Bit 3 - 0b: VBUS Discharge Fault Detection Timer \n\nenabled 1b: VBUS Discharge Fault Detection \n\nTimer disabled This enables the timers for \n\nboth \n\nFAULT_STATUTS.AutoDischargeFailed \n\nand FAULT_STATUS.ForceDischargeFailed \n\nRequired"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_discharge_fault_detection_timer(
        &mut self,
    ) -> VbusDischargeFaultDetectionTimerW<FaultControlSpec> {
        VbusDischargeFaultDetectionTimerW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn force_off_vbus_source_or_sink(&mut self) -> ForceOffVbusSourceOrSinkW<FaultControlSpec> {
        ForceOffVbusSourceOrSinkW::new(self, 4)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore read \n\nvalue."]
    #[inline(always)]
    #[must_use]
    pub fn not_used(&mut self) -> NotUsedW<FaultControlSpec> {
        NotUsedW::new(self, 8)
    }
}
#[doc = "Fault Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fault_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fault_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaultControlSpec;
impl crate::RegisterSpec for FaultControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fault_control::R`](R) reader structure"]
impl crate::Readable for FaultControlSpec {}
#[doc = "`write(|w| ..)` method takes [`fault_control::W`](W) writer structure"]
impl crate::Writable for FaultControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FAULT_CONTROL to value 0"]
impl crate::Resettable for FaultControlSpec {
    const RESET_VALUE: u32 = 0;
}
