#[doc = "Register `PCIE_RC_SLOT_CAPABILITY` reader"]
pub type R = crate::R<PcieRcSlotCapabilitySpec>;
#[doc = "Register `PCIE_RC_SLOT_CAPABILITY` writer"]
pub type W = crate::W<PcieRcSlotCapabilitySpec>;
#[doc = "Field `ABPRSNT` reader - Attention Button Present \\[ABPRSNT\\]
When Set, this bit indicates that an Attention Button for this slot is electrically controlled by the chassis."]
pub type AbprsntR = crate::BitReader;
#[doc = "Field `ABPRSNT` writer - Attention Button Present \\[ABPRSNT\\]
When Set, this bit indicates that an Attention Button for this slot is electrically controlled by the chassis."]
pub type AbprsntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCP` reader - Power Controller Present \\[PCP\\]
When Set, this bit indicates that a software programmable Power Controller is implemented for this slot/adapter (depending on form factor)."]
pub type PcpR = crate::BitReader;
#[doc = "Field `PCP` writer - Power Controller Present \\[PCP\\]
When Set, this bit indicates that a software programmable Power Controller is implemented for this slot/adapter (depending on form factor)."]
pub type PcpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRLSP` reader - MRL Sensor Present \\[MRLSP\\]
When Set, this bit indicates that an MRL Sensor is implemented on the chassis for this slot."]
pub type MrlspR = crate::BitReader;
#[doc = "Field `MRLSP` writer - MRL Sensor Present \\[MRLSP\\]
When Set, this bit indicates that an MRL Sensor is implemented on the chassis for this slot."]
pub type MrlspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIP` reader - Attention Indicator Present \\[AIP\\]
When Set, this bit indicates that an Attention Indicator is electrically controlled by the chassis."]
pub type AipR = crate::BitReader;
#[doc = "Field `AIP` writer - Attention Indicator Present \\[AIP\\]
When Set, this bit indicates that an Attention Indicator is electrically controlled by the chassis."]
pub type AipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIP` reader - Power Indicator Present \\[PIP\\]
When Set, this bit indicates that a Power Indicator is electrically controlled by the chassis for this slot."]
pub type PipR = crate::BitReader;
#[doc = "Field `PIP` writer - Power Indicator Present \\[PIP\\]
When Set, this bit indicates that a Power Indicator is electrically controlled by the chassis for this slot."]
pub type PipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPS` reader - Hot-Plug Surprise \\[HPS\\]
When Set, this bit indicates that an adapter present in this slot might be removed from the system without any prior notification. This is a form factor specific capability. This bit is an indication to the operating system to allow for such removal without impacting continued software operation."]
pub type HpsR = crate::BitReader;
#[doc = "Field `HPS` writer - Hot-Plug Surprise \\[HPS\\]
When Set, this bit indicates that an adapter present in this slot might be removed from the system without any prior notification. This is a form factor specific capability. This bit is an indication to the operating system to allow for such removal without impacting continued software operation."]
pub type HpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPC` reader - Hot-Plug Capable \\[HPC\\]
When Set, this bit indicates that this slot is capable of supporting hot-plug operations."]
pub type HpcR = crate::BitReader;
#[doc = "Field `HPC` writer - Hot-Plug Capable \\[HPC\\]
When Set, this bit indicates that this slot is capable of supporting hot-plug operations."]
pub type HpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLV` reader - Slot Power Limit Value \\[SPLV\\]
In combination with the Slot Power Limit Scale value, specifies the upper limit on power supplied by the slot (see Section 6.9) or by other means to the adapter. Power limit (in Watts) is calculated by multiplying the value in this field by the value in the Slot Power Limit Scale field except when the Slot Power Limit Scale field equals 00b (1.0x) and Slot Power Limit Value exceeds EFh, the following alternative encodings are used: F0h = 250 W Slot Power Limit F1h = 275 W Slot Power Limit F2h = 300 W Slot Power Limit F3h to FFh= Reserved for Slot Power Limit values above 300 W This register must be implemented if the Slot Implemented bit is Set. Writes to this register also cause the Port to send the Set_Slot_Power_Limit Message. The default value prior to hardware/ firmware initialization is 0000 0000b."]
pub type SplvR = crate::FieldReader;
#[doc = "Field `SPLV` writer - Slot Power Limit Value \\[SPLV\\]
In combination with the Slot Power Limit Scale value, specifies the upper limit on power supplied by the slot (see Section 6.9) or by other means to the adapter. Power limit (in Watts) is calculated by multiplying the value in this field by the value in the Slot Power Limit Scale field except when the Slot Power Limit Scale field equals 00b (1.0x) and Slot Power Limit Value exceeds EFh, the following alternative encodings are used: F0h = 250 W Slot Power Limit F1h = 275 W Slot Power Limit F2h = 300 W Slot Power Limit F3h to FFh= Reserved for Slot Power Limit values above 300 W This register must be implemented if the Slot Implemented bit is Set. Writes to this register also cause the Port to send the Set_Slot_Power_Limit Message. The default value prior to hardware/ firmware initialization is 0000 0000b."]
pub type SplvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPLS` reader - Slot Power Limit Scale \\[SPLS\\]
Specifies the scale used for the Slot Power Limit Value. Range of Values: 00b = 1.0x 01b = 0.1x 10b = 0.01x 11b = 0.001x This register must be implemented if the Slot Implemented bit is Set. Writes to this register also cause the Port to send the Set_Slot_Power_Limit Message. The default value prior to hardware/firmware initialization is 00b."]
pub type SplsR = crate::FieldReader;
#[doc = "Field `SPLS` writer - Slot Power Limit Scale \\[SPLS\\]
Specifies the scale used for the Slot Power Limit Value. Range of Values: 00b = 1.0x 01b = 0.1x 10b = 0.01x 11b = 0.001x This register must be implemented if the Slot Implemented bit is Set. Writes to this register also cause the Port to send the Set_Slot_Power_Limit Message. The default value prior to hardware/firmware initialization is 00b."]
pub type SplsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EIP` reader - Electromechanical Interlock Present \\[EIP\\]
When Set, this bit indicates that an Electromechanical Interlock is implemented on the chassis for this slot."]
pub type EipR = crate::BitReader;
#[doc = "Field `EIP` writer - Electromechanical Interlock Present \\[EIP\\]
When Set, this bit indicates that an Electromechanical Interlock is implemented on the chassis for this slot."]
pub type EipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NCCS` reader - No Command Completed Support \\[NCCS\\]
When Set, this bit indicates that this slot does not generate software notification when an issued command is completed by the Hot-Plug Controller. This bit is only permitted to be Set if the hot-plug capable Port is able to accept writes to all fields of the Slot Control register without delay between successive writes."]
pub type NccsR = crate::BitReader;
#[doc = "Field `NCCS` writer - No Command Completed Support \\[NCCS\\]
When Set, this bit indicates that this slot does not generate software notification when an issued command is completed by the Hot-Plug Controller. This bit is only permitted to be Set if the hot-plug capable Port is able to accept writes to all fields of the Slot Control register without delay between successive writes."]
pub type NccsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSN` reader - Physical Slot Number \\[PSN\\]
This field indicates the physical slot number attached to this Port. This field must be hardware initialized to a value that assigns a slot number that is unique within the chassis, regardless of the form factor associated with the slot. This field must be initialized to zero for Ports connected to devices that are either integrated on the system board or integrated within the same silicon as the Switch device or Root Port."]
pub type PsnR = crate::FieldReader<u16>;
#[doc = "Field `PSN` writer - Physical Slot Number \\[PSN\\]
This field indicates the physical slot number attached to this Port. This field must be hardware initialized to a value that assigns a slot number that is unique within the chassis, regardless of the form factor associated with the slot. This field must be initialized to zero for Ports connected to devices that are either integrated on the system board or integrated within the same silicon as the Switch device or Root Port."]
pub type PsnW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bit 0 - Attention Button Present \\[ABPRSNT\\]
When Set, this bit indicates that an Attention Button for this slot is electrically controlled by the chassis."]
    #[inline(always)]
    pub fn abprsnt(&self) -> AbprsntR {
        AbprsntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Controller Present \\[PCP\\]
When Set, this bit indicates that a software programmable Power Controller is implemented for this slot/adapter (depending on form factor)."]
    #[inline(always)]
    pub fn pcp(&self) -> PcpR {
        PcpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MRL Sensor Present \\[MRLSP\\]
When Set, this bit indicates that an MRL Sensor is implemented on the chassis for this slot."]
    #[inline(always)]
    pub fn mrlsp(&self) -> MrlspR {
        MrlspR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Attention Indicator Present \\[AIP\\]
When Set, this bit indicates that an Attention Indicator is electrically controlled by the chassis."]
    #[inline(always)]
    pub fn aip(&self) -> AipR {
        AipR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power Indicator Present \\[PIP\\]
When Set, this bit indicates that a Power Indicator is electrically controlled by the chassis for this slot."]
    #[inline(always)]
    pub fn pip(&self) -> PipR {
        PipR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hot-Plug Surprise \\[HPS\\]
When Set, this bit indicates that an adapter present in this slot might be removed from the system without any prior notification. This is a form factor specific capability. This bit is an indication to the operating system to allow for such removal without impacting continued software operation."]
    #[inline(always)]
    pub fn hps(&self) -> HpsR {
        HpsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hot-Plug Capable \\[HPC\\]
When Set, this bit indicates that this slot is capable of supporting hot-plug operations."]
    #[inline(always)]
    pub fn hpc(&self) -> HpcR {
        HpcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:14 - Slot Power Limit Value \\[SPLV\\]
In combination with the Slot Power Limit Scale value, specifies the upper limit on power supplied by the slot (see Section 6.9) or by other means to the adapter. Power limit (in Watts) is calculated by multiplying the value in this field by the value in the Slot Power Limit Scale field except when the Slot Power Limit Scale field equals 00b (1.0x) and Slot Power Limit Value exceeds EFh, the following alternative encodings are used: F0h = 250 W Slot Power Limit F1h = 275 W Slot Power Limit F2h = 300 W Slot Power Limit F3h to FFh= Reserved for Slot Power Limit values above 300 W This register must be implemented if the Slot Implemented bit is Set. Writes to this register also cause the Port to send the Set_Slot_Power_Limit Message. The default value prior to hardware/ firmware initialization is 0000 0000b."]
    #[inline(always)]
    pub fn splv(&self) -> SplvR {
        SplvR::new(((self.bits >> 7) & 0xff) as u8)
    }
    #[doc = "Bits 15:16 - Slot Power Limit Scale \\[SPLS\\]
Specifies the scale used for the Slot Power Limit Value. Range of Values: 00b = 1.0x 01b = 0.1x 10b = 0.01x 11b = 0.001x This register must be implemented if the Slot Implemented bit is Set. Writes to this register also cause the Port to send the Set_Slot_Power_Limit Message. The default value prior to hardware/firmware initialization is 00b."]
    #[inline(always)]
    pub fn spls(&self) -> SplsR {
        SplsR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - Electromechanical Interlock Present \\[EIP\\]
When Set, this bit indicates that an Electromechanical Interlock is implemented on the chassis for this slot."]
    #[inline(always)]
    pub fn eip(&self) -> EipR {
        EipR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - No Command Completed Support \\[NCCS\\]
When Set, this bit indicates that this slot does not generate software notification when an issued command is completed by the Hot-Plug Controller. This bit is only permitted to be Set if the hot-plug capable Port is able to accept writes to all fields of the Slot Control register without delay between successive writes."]
    #[inline(always)]
    pub fn nccs(&self) -> NccsR {
        NccsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:31 - Physical Slot Number \\[PSN\\]
This field indicates the physical slot number attached to this Port. This field must be hardware initialized to a value that assigns a slot number that is unique within the chassis, regardless of the form factor associated with the slot. This field must be initialized to zero for Ports connected to devices that are either integrated on the system board or integrated within the same silicon as the Switch device or Root Port."]
    #[inline(always)]
    pub fn psn(&self) -> PsnR {
        PsnR::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Attention Button Present \\[ABPRSNT\\]
When Set, this bit indicates that an Attention Button for this slot is electrically controlled by the chassis."]
    #[inline(always)]
    #[must_use]
    pub fn abprsnt(&mut self) -> AbprsntW<PcieRcSlotCapabilitySpec> {
        AbprsntW::new(self, 0)
    }
    #[doc = "Bit 1 - Power Controller Present \\[PCP\\]
When Set, this bit indicates that a software programmable Power Controller is implemented for this slot/adapter (depending on form factor)."]
    #[inline(always)]
    #[must_use]
    pub fn pcp(&mut self) -> PcpW<PcieRcSlotCapabilitySpec> {
        PcpW::new(self, 1)
    }
    #[doc = "Bit 2 - MRL Sensor Present \\[MRLSP\\]
When Set, this bit indicates that an MRL Sensor is implemented on the chassis for this slot."]
    #[inline(always)]
    #[must_use]
    pub fn mrlsp(&mut self) -> MrlspW<PcieRcSlotCapabilitySpec> {
        MrlspW::new(self, 2)
    }
    #[doc = "Bit 3 - Attention Indicator Present \\[AIP\\]
When Set, this bit indicates that an Attention Indicator is electrically controlled by the chassis."]
    #[inline(always)]
    #[must_use]
    pub fn aip(&mut self) -> AipW<PcieRcSlotCapabilitySpec> {
        AipW::new(self, 3)
    }
    #[doc = "Bit 4 - Power Indicator Present \\[PIP\\]
When Set, this bit indicates that a Power Indicator is electrically controlled by the chassis for this slot."]
    #[inline(always)]
    #[must_use]
    pub fn pip(&mut self) -> PipW<PcieRcSlotCapabilitySpec> {
        PipW::new(self, 4)
    }
    #[doc = "Bit 5 - Hot-Plug Surprise \\[HPS\\]
When Set, this bit indicates that an adapter present in this slot might be removed from the system without any prior notification. This is a form factor specific capability. This bit is an indication to the operating system to allow for such removal without impacting continued software operation."]
    #[inline(always)]
    #[must_use]
    pub fn hps(&mut self) -> HpsW<PcieRcSlotCapabilitySpec> {
        HpsW::new(self, 5)
    }
    #[doc = "Bit 6 - Hot-Plug Capable \\[HPC\\]
When Set, this bit indicates that this slot is capable of supporting hot-plug operations."]
    #[inline(always)]
    #[must_use]
    pub fn hpc(&mut self) -> HpcW<PcieRcSlotCapabilitySpec> {
        HpcW::new(self, 6)
    }
    #[doc = "Bits 7:14 - Slot Power Limit Value \\[SPLV\\]
In combination with the Slot Power Limit Scale value, specifies the upper limit on power supplied by the slot (see Section 6.9) or by other means to the adapter. Power limit (in Watts) is calculated by multiplying the value in this field by the value in the Slot Power Limit Scale field except when the Slot Power Limit Scale field equals 00b (1.0x) and Slot Power Limit Value exceeds EFh, the following alternative encodings are used: F0h = 250 W Slot Power Limit F1h = 275 W Slot Power Limit F2h = 300 W Slot Power Limit F3h to FFh= Reserved for Slot Power Limit values above 300 W This register must be implemented if the Slot Implemented bit is Set. Writes to this register also cause the Port to send the Set_Slot_Power_Limit Message. The default value prior to hardware/ firmware initialization is 0000 0000b."]
    #[inline(always)]
    #[must_use]
    pub fn splv(&mut self) -> SplvW<PcieRcSlotCapabilitySpec> {
        SplvW::new(self, 7)
    }
    #[doc = "Bits 15:16 - Slot Power Limit Scale \\[SPLS\\]
Specifies the scale used for the Slot Power Limit Value. Range of Values: 00b = 1.0x 01b = 0.1x 10b = 0.01x 11b = 0.001x This register must be implemented if the Slot Implemented bit is Set. Writes to this register also cause the Port to send the Set_Slot_Power_Limit Message. The default value prior to hardware/firmware initialization is 00b."]
    #[inline(always)]
    #[must_use]
    pub fn spls(&mut self) -> SplsW<PcieRcSlotCapabilitySpec> {
        SplsW::new(self, 15)
    }
    #[doc = "Bit 17 - Electromechanical Interlock Present \\[EIP\\]
When Set, this bit indicates that an Electromechanical Interlock is implemented on the chassis for this slot."]
    #[inline(always)]
    #[must_use]
    pub fn eip(&mut self) -> EipW<PcieRcSlotCapabilitySpec> {
        EipW::new(self, 17)
    }
    #[doc = "Bit 18 - No Command Completed Support \\[NCCS\\]
When Set, this bit indicates that this slot does not generate software notification when an issued command is completed by the Hot-Plug Controller. This bit is only permitted to be Set if the hot-plug capable Port is able to accept writes to all fields of the Slot Control register without delay between successive writes."]
    #[inline(always)]
    #[must_use]
    pub fn nccs(&mut self) -> NccsW<PcieRcSlotCapabilitySpec> {
        NccsW::new(self, 18)
    }
    #[doc = "Bits 19:31 - Physical Slot Number \\[PSN\\]
This field indicates the physical slot number attached to this Port. This field must be hardware initialized to a value that assigns a slot number that is unique within the chassis, regardless of the form factor associated with the slot. This field must be initialized to zero for Ports connected to devices that are either integrated on the system board or integrated within the same silicon as the Switch device or Root Port."]
    #[inline(always)]
    #[must_use]
    pub fn psn(&mut self) -> PsnW<PcieRcSlotCapabilitySpec> {
        PsnW::new(self, 19)
    }
}
#[doc = "Slot Capability Register This field indicates the physical slot number attached to this Port. This field must be hardware initialized to a value that assigns a slot number that is unique within the chassis, regardless of the form factor associated with the slot. This field must be initialized to zero for Ports connected to devices that are either integrated on the system board or integrated within the same silicon as the Switch device or Root Port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_slot_capability::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_slot_capability::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcSlotCapabilitySpec;
impl crate::RegisterSpec for PcieRcSlotCapabilitySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_slot_capability::R`](R) reader structure"]
impl crate::Readable for PcieRcSlotCapabilitySpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_slot_capability::W`](W) writer structure"]
impl crate::Writable for PcieRcSlotCapabilitySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_RC_SLOT_CAPABILITY to value 0"]
impl crate::Resettable for PcieRcSlotCapabilitySpec {
    const RESET_VALUE: u32 = 0;
}
