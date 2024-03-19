#[doc = "Register `PCIE_RC_SLOT_CONTROL_AND_STATUS` reader"]
pub type R = crate::R<PcieRcSlotControlAndStatusSpec>;
#[doc = "Register `PCIE_RC_SLOT_CONTROL_AND_STATUS` writer"]
pub type W = crate::W<PcieRcSlotControlAndStatusSpec>;
#[doc = "Field `ABPE` reader - Attention Button Pressed Enable \\[ABPE\\]
When Set to 1b, this bit enables software notification on an attention button pressed event. If the Attention Button Present bit in the Slot Capabilities register is 0b, this bit is permitted to be read- only with a value of 0b. Default value of this bit is 0b."]
pub type AbpeR = crate::BitReader;
#[doc = "Field `ABPE` writer - Attention Button Pressed Enable \\[ABPE\\]
When Set to 1b, this bit enables software notification on an attention button pressed event. If the Attention Button Present bit in the Slot Capabilities register is 0b, this bit is permitted to be read- only with a value of 0b. Default value of this bit is 0b."]
pub type AbpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFDE` reader - Power Fault Detected Enable \\[PFDE\\]
When Set, this bit enables software notification on a power fault event If a Power Controller that supports power fault detection is not implemented, this bit is permitted to be read-only with a value of 0b. Default value of this bit is 0b."]
pub type PfdeR = crate::BitReader;
#[doc = "Field `PFDE` writer - Power Fault Detected Enable \\[PFDE\\]
When Set, this bit enables software notification on a power fault event If a Power Controller that supports power fault detection is not implemented, this bit is permitted to be read-only with a value of 0b. Default value of this bit is 0b."]
pub type PfdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSCE` reader - MRL Sensor Changed Enable \\[MSCE\\]
When Set, this bit enables software notification on a MRL sensor changed event If the MRL Sensor Present bit in the Slot Capabilities register is Clear, this bit is permitted to be read-only with a value of 0b. Default value of this bit is 0b."]
pub type MsceR = crate::BitReader;
#[doc = "Field `MSCE` writer - MRL Sensor Changed Enable \\[MSCE\\]
When Set, this bit enables software notification on a MRL sensor changed event If the MRL Sensor Present bit in the Slot Capabilities register is Clear, this bit is permitted to be read-only with a value of 0b. Default value of this bit is 0b."]
pub type MsceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDCE` reader - Presence Detect Changed Enable \\[PDCE\\]
When Set, this bit enables software notification on a presence detect changed event. If the Hot- Plug Capable bit in the Slot Capabilities register is 0b, this bit is permitted to be read-only with a value of 0b. Default value of this bit is 0b."]
pub type PdceR = crate::BitReader;
#[doc = "Field `PDCE` writer - Presence Detect Changed Enable \\[PDCE\\]
When Set, this bit enables software notification on a presence detect changed event. If the Hot- Plug Capable bit in the Slot Capabilities register is 0b, this bit is permitted to be read-only with a value of 0b. Default value of this bit is 0b."]
pub type PdceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCIE` reader - Command Completed Interrupt Enable \\[CCIE\\]
If Command Completed notification is supported (if the No Command Completed Support bit in the Slot Capabilities register is 0b), when Set, this bit enables software notification when a hot- plug command is completed by the Hot-Plug Controller. If Command Completed notification is not supported, this bit must be hardwired to 0b. Default value of this bit is 0b."]
pub type CcieR = crate::BitReader;
#[doc = "Field `CCIE` writer - Command Completed Interrupt Enable \\[CCIE\\]
If Command Completed notification is supported (if the No Command Completed Support bit in the Slot Capabilities register is 0b), when Set, this bit enables software notification when a hot- plug command is completed by the Hot-Plug Controller. If Command Completed notification is not supported, this bit must be hardwired to 0b. Default value of this bit is 0b."]
pub type CcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPIE` reader - Hot-Plug Interrupt Enable \\[HPIE\\]
When Set, this bit enables generation of an interrupt on enabled hot-plug events. If the Hot Plug Capable bit in the Slot Capabilities register is Clear, this bit is permitted to be read-only with a value of 0b. Default value of this bit is 0b."]
pub type HpieR = crate::BitReader;
#[doc = "Field `HPIE` writer - Hot-Plug Interrupt Enable \\[HPIE\\]
When Set, this bit enables generation of an interrupt on enabled hot-plug events. If the Hot Plug Capable bit in the Slot Capabilities register is Clear, this bit is permitted to be read-only with a value of 0b. Default value of this bit is 0b."]
pub type HpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIC` reader - Attention Indicator Control \\[AIC\\]
If an Attention Indicator is implemented, writes to this field set the Attention Indicator to the written state. Reads of this field must reflect the value from the latest write, Defined encodings are: 00b Reserved 01b On 10b Blink 11b Off"]
pub type AicR = crate::FieldReader;
#[doc = "Field `AIC` writer - Attention Indicator Control \\[AIC\\]
If an Attention Indicator is implemented, writes to this field set the Attention Indicator to the written state. Reads of this field must reflect the value from the latest write, Defined encodings are: 00b Reserved 01b On 10b Blink 11b Off"]
pub type AicW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIC` reader - Power Indicator Control \\[PIC\\]
If a Power Indicator is implemented, writes to this field set the Power Indicator to the written state. Reads of this field must reflect the value from the latest write, Defined encodings are: 00b Reserved 01b On 10b Blink 11b Off"]
pub type PicR = crate::FieldReader;
#[doc = "Field `PIC` writer - Power Indicator Control \\[PIC\\]
If a Power Indicator is implemented, writes to this field set the Power Indicator to the written state. Reads of this field must reflect the value from the latest write, Defined encodings are: 00b Reserved 01b On 10b Blink 11b Off"]
pub type PicW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCC` reader - Power Controller Control \\[PCC\\]
If a Power Controller is implemented, this bit when written sets the power state of the slot per the defined encodings. Reads of this bit must reflect the value from the latest write, even if the corresponding hot-plug command is not complete, unless software issues a write, if required to, without waiting for the previous command to complete in which case the read value is undefined. The defined encodings are: 0b Power On 1b Power Off"]
pub type PccR = crate::BitReader;
#[doc = "Field `PCC` writer - Power Controller Control \\[PCC\\]
If a Power Controller is implemented, this bit when written sets the power state of the slot per the defined encodings. Reads of this bit must reflect the value from the latest write, even if the corresponding hot-plug command is not complete, unless software issues a write, if required to, without waiting for the previous command to complete in which case the read value is undefined. The defined encodings are: 0b Power On 1b Power Off"]
pub type PccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMIC` reader - Electromechanical Interlock Control \\[EMIC\\]
If an Electromechanical Interlock is implemented, a write of 1b to this bit causes the state of the interlock to toggle. A write of 0b to this bit has no effect. A read of this bit always returns a 0b."]
pub type EmicR = crate::BitReader;
#[doc = "Field `EMIC` writer - Electromechanical Interlock Control \\[EMIC\\]
If an Electromechanical Interlock is implemented, a write of 1b to this bit causes the state of the interlock to toggle. A write of 0b to this bit has no effect. A read of this bit always returns a 0b."]
pub type EmicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLLSCE` reader - Data Link Layer State Changed Enable \\[DLLSCE\\]
If the Data Link Layer Link Active Reporting capability is 1b, this bit enables software notification when Data Link Layer Link Active bit is changed. If the Data Link Layer Link Active Reporting Capable bit is 0b, this bit is permitted to be read- only with a value of 0b. Default value of this bit is 0b."]
pub type DllsceR = crate::BitReader;
#[doc = "Field `DLLSCE` writer - Data Link Layer State Changed Enable \\[DLLSCE\\]
If the Data Link Layer Link Active Reporting capability is 1b, this bit enables software notification when Data Link Layer Link Active bit is changed. If the Data Link Layer Link Active Reporting Capable bit is 0b, this bit is permitted to be read- only with a value of 0b. Default value of this bit is 0b."]
pub type DllsceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSCS1` reader - Reserved \\[RSCS1\\]
Reserved"]
pub type Rscs1R = crate::FieldReader;
#[doc = "Field `ABPRSD` reader - Attention Button Pressed \\[ABPRSD\\]
If an Attention Button is implemented, this bit is Set when the attention button is pressed. If an Attention Button is not supported, this bit must not be Set."]
pub type AbprsdR = crate::BitReader;
#[doc = "Field `ABPRSD` writer - Attention Button Pressed \\[ABPRSD\\]
If an Attention Button is implemented, this bit is Set when the attention button is pressed. If an Attention Button is not supported, this bit must not be Set."]
pub type AbprsdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PFD` reader - Power Fault Detected \\[PFD\\]
If a Power Controller that supports power fault detection is implemented, this bit is Set when the Power Controller detects a power fault at this slot. Note that, depending on hardware capability, it is possible that a power fault can be detected at any time, independent of the Power Controller Control setting or the occupancy of the slot. If power fault detection is not supported, this bit must not be Set."]
pub type PfdR = crate::BitReader;
#[doc = "Field `PFD` writer - Power Fault Detected \\[PFD\\]
If a Power Controller that supports power fault detection is implemented, this bit is Set when the Power Controller detects a power fault at this slot. Note that, depending on hardware capability, it is possible that a power fault can be detected at any time, independent of the Power Controller Control setting or the occupancy of the slot. If power fault detection is not supported, this bit must not be Set."]
pub type PfdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MRLSC` reader - MRL Sensor Changed \\[MRLSC\\]
If an MRL sensor is implemented, this bit is Set when a MRL Sensor state change is detected. If an MRL sensor is not implemented, this bit must not be Set."]
pub type MrlscR = crate::BitReader;
#[doc = "Field `MRLSC` writer - MRL Sensor Changed \\[MRLSC\\]
If an MRL sensor is implemented, this bit is Set when a MRL Sensor state change is detected. If an MRL sensor is not implemented, this bit must not be Set."]
pub type MrlscW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PDC` reader - Presence Detect Changed \\[PDC\\]
This bit is set when the value reported in the Presence Detect State bit is changed."]
pub type PdcR = crate::BitReader;
#[doc = "Field `PDC` writer - Presence Detect Changed \\[PDC\\]
This bit is set when the value reported in the Presence Detect State bit is changed."]
pub type PdcW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CMDCMPL` reader - Command Completed \\[CMDCMPL\\]
If Command Completed notification is supported (if the No Command Completed Support bit in the Slot Capabilities register is 0b), this bit is Set when a hot-plug command has completed and the Hot-Plug Controller is ready to accept a subsequent command. The Command Completed status bit is Set as an indication to host software that the Hot- Plug Controller has processed the previous command and is ready to receive the next command; it provides no guarantee that the action corresponding to the command is complete. If Command Completed notification is not supported, this bit must be hardwired to 0b."]
pub type CmdcmplR = crate::BitReader;
#[doc = "Field `CMDCMPL` writer - Command Completed \\[CMDCMPL\\]
If Command Completed notification is supported (if the No Command Completed Support bit in the Slot Capabilities register is 0b), this bit is Set when a hot-plug command has completed and the Hot-Plug Controller is ready to accept a subsequent command. The Command Completed status bit is Set as an indication to host software that the Hot- Plug Controller has processed the previous command and is ready to receive the next command; it provides no guarantee that the action corresponding to the command is complete. If Command Completed notification is not supported, this bit must be hardwired to 0b."]
pub type CmdcmplW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MRLSS` reader - MRL Sensor State \\[MRLSS\\]
This bit reports the status of the MRL sensor if implemented. Defined encodings are: 0b MRL Closed 1b MRL Open"]
pub type MrlssR = crate::BitReader;
#[doc = "Field `PDS` reader - Presence Detect State \\[PDS\\]
This bit indicates the presence of an adapter in the slot, reflected by the logical “OR” of the Physical Layer in-band presence detect mechanism and, if present, any out-of-band presence detect mechanism defined for the slot’s corresponding form factor. Note that the in-band presence detect mechanism requires that power be applied to an adapter for its presence to be detected. Consequently, form factors that require a power controller for hot- plug must implement a physical pin presence detect mechanism. Defined encodings are: 0b Slot Empty 1b Card Present in slot."]
pub type PdsR = crate::BitReader;
#[doc = "Field `EMIS` reader - Electromechanical Interlock Status \\[EMIS\\]
If an Electromechanical Interlock is implemented, this bit indicates the status of the Electromechanical Interlock. Defined encodings are: 0b Electromechanical Interlock Disengaged 1b Electromechanical Interlock Engaged"]
pub type EmisR = crate::BitReader;
#[doc = "Field `DLLSC` reader - Data Link Layer State Changed \\[DLLSC\\]
This bit is Set when the value reported in the Data Link Layer Link Active bit of the Link Status register is changed. In response to a Data Link Layer State Changed event, software must read the Data Link Layer Link Active bit of the Link Status register to determine if the Link is active before initiating configuration cycles to the hot plugged device."]
pub type DllscR = crate::BitReader;
#[doc = "Field `DLLSC` writer - Data Link Layer State Changed \\[DLLSC\\]
This bit is Set when the value reported in the Data Link Layer Link Active bit of the Link Status register is changed. In response to a Data Link Layer State Changed event, software must read the Data Link Layer Link Active bit of the Link Status register to determine if the Link is active before initiating configuration cycles to the hot plugged device."]
pub type DllscW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RSCS2` reader - Reserved \\[RSCS2\\]
(no description)"]
pub type Rscs2R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Attention Button Pressed Enable \\[ABPE\\]
When Set to 1b, this bit enables software notification on an attention button pressed event. If the Attention Button Present bit in the Slot Capabilities register is 0b, this bit is permitted to be read- only with a value of 0b. Default value of this bit is 0b."]
    #[inline(always)]
    pub fn abpe(&self) -> AbpeR {
        AbpeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Fault Detected Enable \\[PFDE\\]
When Set, this bit enables software notification on a power fault event If a Power Controller that supports power fault detection is not implemented, this bit is permitted to be read-only with a value of 0b. Default value of this bit is 0b."]
    #[inline(always)]
    pub fn pfde(&self) -> PfdeR {
        PfdeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MRL Sensor Changed Enable \\[MSCE\\]
When Set, this bit enables software notification on a MRL sensor changed event If the MRL Sensor Present bit in the Slot Capabilities register is Clear, this bit is permitted to be read-only with a value of 0b. Default value of this bit is 0b."]
    #[inline(always)]
    pub fn msce(&self) -> MsceR {
        MsceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Presence Detect Changed Enable \\[PDCE\\]
When Set, this bit enables software notification on a presence detect changed event. If the Hot- Plug Capable bit in the Slot Capabilities register is 0b, this bit is permitted to be read-only with a value of 0b. Default value of this bit is 0b."]
    #[inline(always)]
    pub fn pdce(&self) -> PdceR {
        PdceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Command Completed Interrupt Enable \\[CCIE\\]
If Command Completed notification is supported (if the No Command Completed Support bit in the Slot Capabilities register is 0b), when Set, this bit enables software notification when a hot- plug command is completed by the Hot-Plug Controller. If Command Completed notification is not supported, this bit must be hardwired to 0b. Default value of this bit is 0b."]
    #[inline(always)]
    pub fn ccie(&self) -> CcieR {
        CcieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hot-Plug Interrupt Enable \\[HPIE\\]
When Set, this bit enables generation of an interrupt on enabled hot-plug events. If the Hot Plug Capable bit in the Slot Capabilities register is Clear, this bit is permitted to be read-only with a value of 0b. Default value of this bit is 0b."]
    #[inline(always)]
    pub fn hpie(&self) -> HpieR {
        HpieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Attention Indicator Control \\[AIC\\]
If an Attention Indicator is implemented, writes to this field set the Attention Indicator to the written state. Reads of this field must reflect the value from the latest write, Defined encodings are: 00b Reserved 01b On 10b Blink 11b Off"]
    #[inline(always)]
    pub fn aic(&self) -> AicR {
        AicR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Power Indicator Control \\[PIC\\]
If a Power Indicator is implemented, writes to this field set the Power Indicator to the written state. Reads of this field must reflect the value from the latest write, Defined encodings are: 00b Reserved 01b On 10b Blink 11b Off"]
    #[inline(always)]
    pub fn pic(&self) -> PicR {
        PicR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Power Controller Control \\[PCC\\]
If a Power Controller is implemented, this bit when written sets the power state of the slot per the defined encodings. Reads of this bit must reflect the value from the latest write, even if the corresponding hot-plug command is not complete, unless software issues a write, if required to, without waiting for the previous command to complete in which case the read value is undefined. The defined encodings are: 0b Power On 1b Power Off"]
    #[inline(always)]
    pub fn pcc(&self) -> PccR {
        PccR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Electromechanical Interlock Control \\[EMIC\\]
If an Electromechanical Interlock is implemented, a write of 1b to this bit causes the state of the interlock to toggle. A write of 0b to this bit has no effect. A read of this bit always returns a 0b."]
    #[inline(always)]
    pub fn emic(&self) -> EmicR {
        EmicR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data Link Layer State Changed Enable \\[DLLSCE\\]
If the Data Link Layer Link Active Reporting capability is 1b, this bit enables software notification when Data Link Layer Link Active bit is changed. If the Data Link Layer Link Active Reporting Capable bit is 0b, this bit is permitted to be read- only with a value of 0b. Default value of this bit is 0b."]
    #[inline(always)]
    pub fn dllsce(&self) -> DllsceR {
        DllsceR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Reserved \\[RSCS1\\]
Reserved"]
    #[inline(always)]
    pub fn rscs1(&self) -> Rscs1R {
        Rscs1R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - Attention Button Pressed \\[ABPRSD\\]
If an Attention Button is implemented, this bit is Set when the attention button is pressed. If an Attention Button is not supported, this bit must not be Set."]
    #[inline(always)]
    pub fn abprsd(&self) -> AbprsdR {
        AbprsdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Power Fault Detected \\[PFD\\]
If a Power Controller that supports power fault detection is implemented, this bit is Set when the Power Controller detects a power fault at this slot. Note that, depending on hardware capability, it is possible that a power fault can be detected at any time, independent of the Power Controller Control setting or the occupancy of the slot. If power fault detection is not supported, this bit must not be Set."]
    #[inline(always)]
    pub fn pfd(&self) -> PfdR {
        PfdR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MRL Sensor Changed \\[MRLSC\\]
If an MRL sensor is implemented, this bit is Set when a MRL Sensor state change is detected. If an MRL sensor is not implemented, this bit must not be Set."]
    #[inline(always)]
    pub fn mrlsc(&self) -> MrlscR {
        MrlscR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Presence Detect Changed \\[PDC\\]
This bit is set when the value reported in the Presence Detect State bit is changed."]
    #[inline(always)]
    pub fn pdc(&self) -> PdcR {
        PdcR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Command Completed \\[CMDCMPL\\]
If Command Completed notification is supported (if the No Command Completed Support bit in the Slot Capabilities register is 0b), this bit is Set when a hot-plug command has completed and the Hot-Plug Controller is ready to accept a subsequent command. The Command Completed status bit is Set as an indication to host software that the Hot- Plug Controller has processed the previous command and is ready to receive the next command; it provides no guarantee that the action corresponding to the command is complete. If Command Completed notification is not supported, this bit must be hardwired to 0b."]
    #[inline(always)]
    pub fn cmdcmpl(&self) -> CmdcmplR {
        CmdcmplR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MRL Sensor State \\[MRLSS\\]
This bit reports the status of the MRL sensor if implemented. Defined encodings are: 0b MRL Closed 1b MRL Open"]
    #[inline(always)]
    pub fn mrlss(&self) -> MrlssR {
        MrlssR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Presence Detect State \\[PDS\\]
This bit indicates the presence of an adapter in the slot, reflected by the logical “OR” of the Physical Layer in-band presence detect mechanism and, if present, any out-of-band presence detect mechanism defined for the slot’s corresponding form factor. Note that the in-band presence detect mechanism requires that power be applied to an adapter for its presence to be detected. Consequently, form factors that require a power controller for hot- plug must implement a physical pin presence detect mechanism. Defined encodings are: 0b Slot Empty 1b Card Present in slot."]
    #[inline(always)]
    pub fn pds(&self) -> PdsR {
        PdsR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Electromechanical Interlock Status \\[EMIS\\]
If an Electromechanical Interlock is implemented, this bit indicates the status of the Electromechanical Interlock. Defined encodings are: 0b Electromechanical Interlock Disengaged 1b Electromechanical Interlock Engaged"]
    #[inline(always)]
    pub fn emis(&self) -> EmisR {
        EmisR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Data Link Layer State Changed \\[DLLSC\\]
This bit is Set when the value reported in the Data Link Layer Link Active bit of the Link Status register is changed. In response to a Data Link Layer State Changed event, software must read the Data Link Layer Link Active bit of the Link Status register to determine if the Link is active before initiating configuration cycles to the hot plugged device."]
    #[inline(always)]
    pub fn dllsc(&self) -> DllscR {
        DllscR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - Reserved \\[RSCS2\\]
(no description)"]
    #[inline(always)]
    pub fn rscs2(&self) -> Rscs2R {
        Rscs2R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Attention Button Pressed Enable \\[ABPE\\]
When Set to 1b, this bit enables software notification on an attention button pressed event. If the Attention Button Present bit in the Slot Capabilities register is 0b, this bit is permitted to be read- only with a value of 0b. Default value of this bit is 0b."]
    #[inline(always)]
    #[must_use]
    pub fn abpe(&mut self) -> AbpeW<PcieRcSlotControlAndStatusSpec> {
        AbpeW::new(self, 0)
    }
    #[doc = "Bit 1 - Power Fault Detected Enable \\[PFDE\\]
When Set, this bit enables software notification on a power fault event If a Power Controller that supports power fault detection is not implemented, this bit is permitted to be read-only with a value of 0b. Default value of this bit is 0b."]
    #[inline(always)]
    #[must_use]
    pub fn pfde(&mut self) -> PfdeW<PcieRcSlotControlAndStatusSpec> {
        PfdeW::new(self, 1)
    }
    #[doc = "Bit 2 - MRL Sensor Changed Enable \\[MSCE\\]
When Set, this bit enables software notification on a MRL sensor changed event If the MRL Sensor Present bit in the Slot Capabilities register is Clear, this bit is permitted to be read-only with a value of 0b. Default value of this bit is 0b."]
    #[inline(always)]
    #[must_use]
    pub fn msce(&mut self) -> MsceW<PcieRcSlotControlAndStatusSpec> {
        MsceW::new(self, 2)
    }
    #[doc = "Bit 3 - Presence Detect Changed Enable \\[PDCE\\]
When Set, this bit enables software notification on a presence detect changed event. If the Hot- Plug Capable bit in the Slot Capabilities register is 0b, this bit is permitted to be read-only with a value of 0b. Default value of this bit is 0b."]
    #[inline(always)]
    #[must_use]
    pub fn pdce(&mut self) -> PdceW<PcieRcSlotControlAndStatusSpec> {
        PdceW::new(self, 3)
    }
    #[doc = "Bit 4 - Command Completed Interrupt Enable \\[CCIE\\]
If Command Completed notification is supported (if the No Command Completed Support bit in the Slot Capabilities register is 0b), when Set, this bit enables software notification when a hot- plug command is completed by the Hot-Plug Controller. If Command Completed notification is not supported, this bit must be hardwired to 0b. Default value of this bit is 0b."]
    #[inline(always)]
    #[must_use]
    pub fn ccie(&mut self) -> CcieW<PcieRcSlotControlAndStatusSpec> {
        CcieW::new(self, 4)
    }
    #[doc = "Bit 5 - Hot-Plug Interrupt Enable \\[HPIE\\]
When Set, this bit enables generation of an interrupt on enabled hot-plug events. If the Hot Plug Capable bit in the Slot Capabilities register is Clear, this bit is permitted to be read-only with a value of 0b. Default value of this bit is 0b."]
    #[inline(always)]
    #[must_use]
    pub fn hpie(&mut self) -> HpieW<PcieRcSlotControlAndStatusSpec> {
        HpieW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Attention Indicator Control \\[AIC\\]
If an Attention Indicator is implemented, writes to this field set the Attention Indicator to the written state. Reads of this field must reflect the value from the latest write, Defined encodings are: 00b Reserved 01b On 10b Blink 11b Off"]
    #[inline(always)]
    #[must_use]
    pub fn aic(&mut self) -> AicW<PcieRcSlotControlAndStatusSpec> {
        AicW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Power Indicator Control \\[PIC\\]
If a Power Indicator is implemented, writes to this field set the Power Indicator to the written state. Reads of this field must reflect the value from the latest write, Defined encodings are: 00b Reserved 01b On 10b Blink 11b Off"]
    #[inline(always)]
    #[must_use]
    pub fn pic(&mut self) -> PicW<PcieRcSlotControlAndStatusSpec> {
        PicW::new(self, 8)
    }
    #[doc = "Bit 10 - Power Controller Control \\[PCC\\]
If a Power Controller is implemented, this bit when written sets the power state of the slot per the defined encodings. Reads of this bit must reflect the value from the latest write, even if the corresponding hot-plug command is not complete, unless software issues a write, if required to, without waiting for the previous command to complete in which case the read value is undefined. The defined encodings are: 0b Power On 1b Power Off"]
    #[inline(always)]
    #[must_use]
    pub fn pcc(&mut self) -> PccW<PcieRcSlotControlAndStatusSpec> {
        PccW::new(self, 10)
    }
    #[doc = "Bit 11 - Electromechanical Interlock Control \\[EMIC\\]
If an Electromechanical Interlock is implemented, a write of 1b to this bit causes the state of the interlock to toggle. A write of 0b to this bit has no effect. A read of this bit always returns a 0b."]
    #[inline(always)]
    #[must_use]
    pub fn emic(&mut self) -> EmicW<PcieRcSlotControlAndStatusSpec> {
        EmicW::new(self, 11)
    }
    #[doc = "Bit 12 - Data Link Layer State Changed Enable \\[DLLSCE\\]
If the Data Link Layer Link Active Reporting capability is 1b, this bit enables software notification when Data Link Layer Link Active bit is changed. If the Data Link Layer Link Active Reporting Capable bit is 0b, this bit is permitted to be read- only with a value of 0b. Default value of this bit is 0b."]
    #[inline(always)]
    #[must_use]
    pub fn dllsce(&mut self) -> DllsceW<PcieRcSlotControlAndStatusSpec> {
        DllsceW::new(self, 12)
    }
    #[doc = "Bit 16 - Attention Button Pressed \\[ABPRSD\\]
If an Attention Button is implemented, this bit is Set when the attention button is pressed. If an Attention Button is not supported, this bit must not be Set."]
    #[inline(always)]
    #[must_use]
    pub fn abprsd(&mut self) -> AbprsdW<PcieRcSlotControlAndStatusSpec> {
        AbprsdW::new(self, 16)
    }
    #[doc = "Bit 17 - Power Fault Detected \\[PFD\\]
If a Power Controller that supports power fault detection is implemented, this bit is Set when the Power Controller detects a power fault at this slot. Note that, depending on hardware capability, it is possible that a power fault can be detected at any time, independent of the Power Controller Control setting or the occupancy of the slot. If power fault detection is not supported, this bit must not be Set."]
    #[inline(always)]
    #[must_use]
    pub fn pfd(&mut self) -> PfdW<PcieRcSlotControlAndStatusSpec> {
        PfdW::new(self, 17)
    }
    #[doc = "Bit 18 - MRL Sensor Changed \\[MRLSC\\]
If an MRL sensor is implemented, this bit is Set when a MRL Sensor state change is detected. If an MRL sensor is not implemented, this bit must not be Set."]
    #[inline(always)]
    #[must_use]
    pub fn mrlsc(&mut self) -> MrlscW<PcieRcSlotControlAndStatusSpec> {
        MrlscW::new(self, 18)
    }
    #[doc = "Bit 19 - Presence Detect Changed \\[PDC\\]
This bit is set when the value reported in the Presence Detect State bit is changed."]
    #[inline(always)]
    #[must_use]
    pub fn pdc(&mut self) -> PdcW<PcieRcSlotControlAndStatusSpec> {
        PdcW::new(self, 19)
    }
    #[doc = "Bit 20 - Command Completed \\[CMDCMPL\\]
If Command Completed notification is supported (if the No Command Completed Support bit in the Slot Capabilities register is 0b), this bit is Set when a hot-plug command has completed and the Hot-Plug Controller is ready to accept a subsequent command. The Command Completed status bit is Set as an indication to host software that the Hot- Plug Controller has processed the previous command and is ready to receive the next command; it provides no guarantee that the action corresponding to the command is complete. If Command Completed notification is not supported, this bit must be hardwired to 0b."]
    #[inline(always)]
    #[must_use]
    pub fn cmdcmpl(&mut self) -> CmdcmplW<PcieRcSlotControlAndStatusSpec> {
        CmdcmplW::new(self, 20)
    }
    #[doc = "Bit 24 - Data Link Layer State Changed \\[DLLSC\\]
This bit is Set when the value reported in the Data Link Layer Link Active bit of the Link Status register is changed. In response to a Data Link Layer State Changed event, software must read the Data Link Layer Link Active bit of the Link Status register to determine if the Link is active before initiating configuration cycles to the hot plugged device."]
    #[inline(always)]
    #[must_use]
    pub fn dllsc(&mut self) -> DllscW<PcieRcSlotControlAndStatusSpec> {
        DllscW::new(self, 24)
    }
}
#[doc = "Slot Control and Status Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_slot_control_and_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_slot_control_and_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcSlotControlAndStatusSpec;
impl crate::RegisterSpec for PcieRcSlotControlAndStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_slot_control_and_status::R`](R) reader structure"]
impl crate::Readable for PcieRcSlotControlAndStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_slot_control_and_status::W`](W) writer structure"]
impl crate::Writable for PcieRcSlotControlAndStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x011f_0000;
}
#[doc = "`reset()` method sets PCIE_RC_SLOT_CONTROL_AND_STATUS to value 0x07c0"]
impl crate::Resettable for PcieRcSlotControlAndStatusSpec {
    const RESET_VALUE: u32 = 0x07c0;
}
