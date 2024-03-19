#[doc = "Register `PCIE_RC_SLOT_CAPABILITY` reader"]
pub type R = crate::R<PcieRcSlotCapabilitySpec>;
#[doc = "Register `PCIE_RC_SLOT_CAPABILITY` writer"]
pub type W = crate::W<PcieRcSlotCapabilitySpec>;
#[doc = "Field `ABPRSNT` reader - Attention Button Present \\[ABPRSNT\\]\n\nWhen Set, this bit indicates that\n\nan Attention Button for this slot is\n\nelectrically controlled by the\n\nchassis."]
pub type AbprsntR = crate::BitReader;
#[doc = "Field `ABPRSNT` writer - Attention Button Present \\[ABPRSNT\\]\n\nWhen Set, this bit indicates that\n\nan Attention Button for this slot is\n\nelectrically controlled by the\n\nchassis."]
pub type AbprsntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCP` reader - Power Controller Present \\[PCP\\]\n\nWhen Set, this bit indicates that a\n\nsoftware programmable Power\n\nController is implemented for this\n\nslot/adapter (depending on form\n\nfactor)."]
pub type PcpR = crate::BitReader;
#[doc = "Field `PCP` writer - Power Controller Present \\[PCP\\]\n\nWhen Set, this bit indicates that a\n\nsoftware programmable Power\n\nController is implemented for this\n\nslot/adapter (depending on form\n\nfactor)."]
pub type PcpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRLSP` reader - MRL Sensor Present \\[MRLSP\\]\n\nWhen Set, this bit indicates that\n\nan MRL Sensor is implemented on\n\nthe chassis for this slot."]
pub type MrlspR = crate::BitReader;
#[doc = "Field `MRLSP` writer - MRL Sensor Present \\[MRLSP\\]\n\nWhen Set, this bit indicates that\n\nan MRL Sensor is implemented on\n\nthe chassis for this slot."]
pub type MrlspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIP` reader - Attention Indicator Present \\[AIP\\]\n\nWhen Set, this bit indicates that\n\nan Attention Indicator is\n\nelectrically controlled by the\n\nchassis."]
pub type AipR = crate::BitReader;
#[doc = "Field `AIP` writer - Attention Indicator Present \\[AIP\\]\n\nWhen Set, this bit indicates that\n\nan Attention Indicator is\n\nelectrically controlled by the\n\nchassis."]
pub type AipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIP` reader - Power Indicator Present \\[PIP\\]\n\nWhen Set, this bit indicates that a\n\nPower Indicator is electrically\n\ncontrolled by the chassis for this\n\nslot."]
pub type PipR = crate::BitReader;
#[doc = "Field `PIP` writer - Power Indicator Present \\[PIP\\]\n\nWhen Set, this bit indicates that a\n\nPower Indicator is electrically\n\ncontrolled by the chassis for this\n\nslot."]
pub type PipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPS` reader - Hot-Plug Surprise \\[HPS\\]\n\nWhen Set, this bit indicates that\n\nan adapter present in this slot\n\nmight be removed from the\n\nsystem without any prior\n\nnotification. This is a form factor\n\nspecific capability. This bit is an\n\nindication to the operating system\n\nto allow for such removal without\n\nimpacting continued software\n\noperation."]
pub type HpsR = crate::BitReader;
#[doc = "Field `HPS` writer - Hot-Plug Surprise \\[HPS\\]\n\nWhen Set, this bit indicates that\n\nan adapter present in this slot\n\nmight be removed from the\n\nsystem without any prior\n\nnotification. This is a form factor\n\nspecific capability. This bit is an\n\nindication to the operating system\n\nto allow for such removal without\n\nimpacting continued software\n\noperation."]
pub type HpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPC` reader - Hot-Plug Capable \\[HPC\\]\n\nWhen Set, this bit indicates that\n\nthis slot is capable of supporting\n\nhot-plug operations."]
pub type HpcR = crate::BitReader;
#[doc = "Field `HPC` writer - Hot-Plug Capable \\[HPC\\]\n\nWhen Set, this bit indicates that\n\nthis slot is capable of supporting\n\nhot-plug operations."]
pub type HpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLV` reader - Slot Power Limit Value \\[SPLV\\]\n\nIn combination with the Slot\n\nPower Limit Scale value, specifies\n\nthe upper limit on power supplied\n\nby the slot (see Section 6.9) or by\n\nother means to the adapter.\n\nPower limit (in Watts) is\n\ncalculated by multiplying the\n\nvalue in this field by the value in\n\nthe Slot Power Limit Scale field\n\nexcept when the Slot Power Limit\n\nScale field equals 00b (1.0x) and\n\nSlot Power Limit Value exceeds\n\nEFh, the following alternative\n\nencodings are used: F0h = 250 W\n\nSlot Power Limit F1h = 275 W\n\nSlot Power Limit F2h = 300 W\n\nSlot Power Limit F3h to FFh=\n\nReserved for Slot Power Limit\n\nvalues above 300 W This register\n\nmust be implemented if the Slot\n\nImplemented bit is Set. Writes to\n\nthis register also cause the Port to\n\nsend the Set_Slot_Power_Limit\n\nMessage. The default value prior\n\nto hardware/ firmware\n\ninitialization is 0000 0000b."]
pub type SplvR = crate::FieldReader;
#[doc = "Field `SPLV` writer - Slot Power Limit Value \\[SPLV\\]\n\nIn combination with the Slot\n\nPower Limit Scale value, specifies\n\nthe upper limit on power supplied\n\nby the slot (see Section 6.9) or by\n\nother means to the adapter.\n\nPower limit (in Watts) is\n\ncalculated by multiplying the\n\nvalue in this field by the value in\n\nthe Slot Power Limit Scale field\n\nexcept when the Slot Power Limit\n\nScale field equals 00b (1.0x) and\n\nSlot Power Limit Value exceeds\n\nEFh, the following alternative\n\nencodings are used: F0h = 250 W\n\nSlot Power Limit F1h = 275 W\n\nSlot Power Limit F2h = 300 W\n\nSlot Power Limit F3h to FFh=\n\nReserved for Slot Power Limit\n\nvalues above 300 W This register\n\nmust be implemented if the Slot\n\nImplemented bit is Set. Writes to\n\nthis register also cause the Port to\n\nsend the Set_Slot_Power_Limit\n\nMessage. The default value prior\n\nto hardware/ firmware\n\ninitialization is 0000 0000b."]
pub type SplvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPLS` reader - Slot Power Limit Scale \\[SPLS\\]\n\nSpecifies the scale used for the\n\nSlot Power Limit Value. Range of\n\nValues: 00b = 1.0x 01b = 0.1x\n\n10b = 0.01x 11b = 0.001x This\n\nregister must be implemented if\n\nthe Slot Implemented bit is Set.\n\nWrites to this register also cause\n\nthe Port to send the\n\nSet_Slot_Power_Limit Message.\n\nThe default value prior to\n\nhardware/firmware initialization is\n\n00b."]
pub type SplsR = crate::FieldReader;
#[doc = "Field `SPLS` writer - Slot Power Limit Scale \\[SPLS\\]\n\nSpecifies the scale used for the\n\nSlot Power Limit Value. Range of\n\nValues: 00b = 1.0x 01b = 0.1x\n\n10b = 0.01x 11b = 0.001x This\n\nregister must be implemented if\n\nthe Slot Implemented bit is Set.\n\nWrites to this register also cause\n\nthe Port to send the\n\nSet_Slot_Power_Limit Message.\n\nThe default value prior to\n\nhardware/firmware initialization is\n\n00b."]
pub type SplsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EIP` reader - Electromechanical Interlock Present \\[EIP\\]\n\nWhen Set, this bit indicates that\n\nan Electromechanical Interlock is\n\nimplemented on the chassis for\n\nthis slot."]
pub type EipR = crate::BitReader;
#[doc = "Field `EIP` writer - Electromechanical Interlock Present \\[EIP\\]\n\nWhen Set, this bit indicates that\n\nan Electromechanical Interlock is\n\nimplemented on the chassis for\n\nthis slot."]
pub type EipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NCCS` reader - No Command Completed Support \\[NCCS\\]\n\nWhen Set, this bit indicates that\n\nthis slot does not generate\n\nsoftware notification when an\n\nissued command is completed by\n\nthe Hot-Plug Controller. This bit is\n\nonly permitted to be Set if the\n\nhot-plug capable Port is able to\n\naccept writes to all fields of the\n\nSlot Control register without delay\n\nbetween successive writes."]
pub type NccsR = crate::BitReader;
#[doc = "Field `NCCS` writer - No Command Completed Support \\[NCCS\\]\n\nWhen Set, this bit indicates that\n\nthis slot does not generate\n\nsoftware notification when an\n\nissued command is completed by\n\nthe Hot-Plug Controller. This bit is\n\nonly permitted to be Set if the\n\nhot-plug capable Port is able to\n\naccept writes to all fields of the\n\nSlot Control register without delay\n\nbetween successive writes."]
pub type NccsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSN` reader - Physical Slot Number \\[PSN\\]\n\nThis field indicates the physical\n\nslot number attached to this Port.\n\nThis field must be hardware\n\ninitialized to a value that assigns\n\na slot number that is unique\n\nwithin the chassis, regardless of\n\nthe form factor associated with\n\nthe slot. This field must be\n\ninitialized to zero for Ports\n\nconnected to devices that are\n\neither integrated on the system\n\nboard or integrated within the\n\nsame silicon as the Switch device\n\nor Root Port."]
pub type PsnR = crate::FieldReader<u16>;
#[doc = "Field `PSN` writer - Physical Slot Number \\[PSN\\]\n\nThis field indicates the physical\n\nslot number attached to this Port.\n\nThis field must be hardware\n\ninitialized to a value that assigns\n\na slot number that is unique\n\nwithin the chassis, regardless of\n\nthe form factor associated with\n\nthe slot. This field must be\n\ninitialized to zero for Ports\n\nconnected to devices that are\n\neither integrated on the system\n\nboard or integrated within the\n\nsame silicon as the Switch device\n\nor Root Port."]
pub type PsnW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bit 0 - Attention Button Present \\[ABPRSNT\\]\n\nWhen Set, this bit indicates that\n\nan Attention Button for this slot is\n\nelectrically controlled by the\n\nchassis."]
    #[inline(always)]
    pub fn abprsnt(&self) -> AbprsntR {
        AbprsntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Controller Present \\[PCP\\]\n\nWhen Set, this bit indicates that a\n\nsoftware programmable Power\n\nController is implemented for this\n\nslot/adapter (depending on form\n\nfactor)."]
    #[inline(always)]
    pub fn pcp(&self) -> PcpR {
        PcpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MRL Sensor Present \\[MRLSP\\]\n\nWhen Set, this bit indicates that\n\nan MRL Sensor is implemented on\n\nthe chassis for this slot."]
    #[inline(always)]
    pub fn mrlsp(&self) -> MrlspR {
        MrlspR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Attention Indicator Present \\[AIP\\]\n\nWhen Set, this bit indicates that\n\nan Attention Indicator is\n\nelectrically controlled by the\n\nchassis."]
    #[inline(always)]
    pub fn aip(&self) -> AipR {
        AipR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power Indicator Present \\[PIP\\]\n\nWhen Set, this bit indicates that a\n\nPower Indicator is electrically\n\ncontrolled by the chassis for this\n\nslot."]
    #[inline(always)]
    pub fn pip(&self) -> PipR {
        PipR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hot-Plug Surprise \\[HPS\\]\n\nWhen Set, this bit indicates that\n\nan adapter present in this slot\n\nmight be removed from the\n\nsystem without any prior\n\nnotification. This is a form factor\n\nspecific capability. This bit is an\n\nindication to the operating system\n\nto allow for such removal without\n\nimpacting continued software\n\noperation."]
    #[inline(always)]
    pub fn hps(&self) -> HpsR {
        HpsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hot-Plug Capable \\[HPC\\]\n\nWhen Set, this bit indicates that\n\nthis slot is capable of supporting\n\nhot-plug operations."]
    #[inline(always)]
    pub fn hpc(&self) -> HpcR {
        HpcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:14 - Slot Power Limit Value \\[SPLV\\]\n\nIn combination with the Slot\n\nPower Limit Scale value, specifies\n\nthe upper limit on power supplied\n\nby the slot (see Section 6.9) or by\n\nother means to the adapter.\n\nPower limit (in Watts) is\n\ncalculated by multiplying the\n\nvalue in this field by the value in\n\nthe Slot Power Limit Scale field\n\nexcept when the Slot Power Limit\n\nScale field equals 00b (1.0x) and\n\nSlot Power Limit Value exceeds\n\nEFh, the following alternative\n\nencodings are used: F0h = 250 W\n\nSlot Power Limit F1h = 275 W\n\nSlot Power Limit F2h = 300 W\n\nSlot Power Limit F3h to FFh=\n\nReserved for Slot Power Limit\n\nvalues above 300 W This register\n\nmust be implemented if the Slot\n\nImplemented bit is Set. Writes to\n\nthis register also cause the Port to\n\nsend the Set_Slot_Power_Limit\n\nMessage. The default value prior\n\nto hardware/ firmware\n\ninitialization is 0000 0000b."]
    #[inline(always)]
    pub fn splv(&self) -> SplvR {
        SplvR::new(((self.bits >> 7) & 0xff) as u8)
    }
    #[doc = "Bits 15:16 - Slot Power Limit Scale \\[SPLS\\]\n\nSpecifies the scale used for the\n\nSlot Power Limit Value. Range of\n\nValues: 00b = 1.0x 01b = 0.1x\n\n10b = 0.01x 11b = 0.001x This\n\nregister must be implemented if\n\nthe Slot Implemented bit is Set.\n\nWrites to this register also cause\n\nthe Port to send the\n\nSet_Slot_Power_Limit Message.\n\nThe default value prior to\n\nhardware/firmware initialization is\n\n00b."]
    #[inline(always)]
    pub fn spls(&self) -> SplsR {
        SplsR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - Electromechanical Interlock Present \\[EIP\\]\n\nWhen Set, this bit indicates that\n\nan Electromechanical Interlock is\n\nimplemented on the chassis for\n\nthis slot."]
    #[inline(always)]
    pub fn eip(&self) -> EipR {
        EipR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - No Command Completed Support \\[NCCS\\]\n\nWhen Set, this bit indicates that\n\nthis slot does not generate\n\nsoftware notification when an\n\nissued command is completed by\n\nthe Hot-Plug Controller. This bit is\n\nonly permitted to be Set if the\n\nhot-plug capable Port is able to\n\naccept writes to all fields of the\n\nSlot Control register without delay\n\nbetween successive writes."]
    #[inline(always)]
    pub fn nccs(&self) -> NccsR {
        NccsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:31 - Physical Slot Number \\[PSN\\]\n\nThis field indicates the physical\n\nslot number attached to this Port.\n\nThis field must be hardware\n\ninitialized to a value that assigns\n\na slot number that is unique\n\nwithin the chassis, regardless of\n\nthe form factor associated with\n\nthe slot. This field must be\n\ninitialized to zero for Ports\n\nconnected to devices that are\n\neither integrated on the system\n\nboard or integrated within the\n\nsame silicon as the Switch device\n\nor Root Port."]
    #[inline(always)]
    pub fn psn(&self) -> PsnR {
        PsnR::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Attention Button Present \\[ABPRSNT\\]\n\nWhen Set, this bit indicates that\n\nan Attention Button for this slot is\n\nelectrically controlled by the\n\nchassis."]
    #[inline(always)]
    #[must_use]
    pub fn abprsnt(&mut self) -> AbprsntW<PcieRcSlotCapabilitySpec> {
        AbprsntW::new(self, 0)
    }
    #[doc = "Bit 1 - Power Controller Present \\[PCP\\]\n\nWhen Set, this bit indicates that a\n\nsoftware programmable Power\n\nController is implemented for this\n\nslot/adapter (depending on form\n\nfactor)."]
    #[inline(always)]
    #[must_use]
    pub fn pcp(&mut self) -> PcpW<PcieRcSlotCapabilitySpec> {
        PcpW::new(self, 1)
    }
    #[doc = "Bit 2 - MRL Sensor Present \\[MRLSP\\]\n\nWhen Set, this bit indicates that\n\nan MRL Sensor is implemented on\n\nthe chassis for this slot."]
    #[inline(always)]
    #[must_use]
    pub fn mrlsp(&mut self) -> MrlspW<PcieRcSlotCapabilitySpec> {
        MrlspW::new(self, 2)
    }
    #[doc = "Bit 3 - Attention Indicator Present \\[AIP\\]\n\nWhen Set, this bit indicates that\n\nan Attention Indicator is\n\nelectrically controlled by the\n\nchassis."]
    #[inline(always)]
    #[must_use]
    pub fn aip(&mut self) -> AipW<PcieRcSlotCapabilitySpec> {
        AipW::new(self, 3)
    }
    #[doc = "Bit 4 - Power Indicator Present \\[PIP\\]\n\nWhen Set, this bit indicates that a\n\nPower Indicator is electrically\n\ncontrolled by the chassis for this\n\nslot."]
    #[inline(always)]
    #[must_use]
    pub fn pip(&mut self) -> PipW<PcieRcSlotCapabilitySpec> {
        PipW::new(self, 4)
    }
    #[doc = "Bit 5 - Hot-Plug Surprise \\[HPS\\]\n\nWhen Set, this bit indicates that\n\nan adapter present in this slot\n\nmight be removed from the\n\nsystem without any prior\n\nnotification. This is a form factor\n\nspecific capability. This bit is an\n\nindication to the operating system\n\nto allow for such removal without\n\nimpacting continued software\n\noperation."]
    #[inline(always)]
    #[must_use]
    pub fn hps(&mut self) -> HpsW<PcieRcSlotCapabilitySpec> {
        HpsW::new(self, 5)
    }
    #[doc = "Bit 6 - Hot-Plug Capable \\[HPC\\]\n\nWhen Set, this bit indicates that\n\nthis slot is capable of supporting\n\nhot-plug operations."]
    #[inline(always)]
    #[must_use]
    pub fn hpc(&mut self) -> HpcW<PcieRcSlotCapabilitySpec> {
        HpcW::new(self, 6)
    }
    #[doc = "Bits 7:14 - Slot Power Limit Value \\[SPLV\\]\n\nIn combination with the Slot\n\nPower Limit Scale value, specifies\n\nthe upper limit on power supplied\n\nby the slot (see Section 6.9) or by\n\nother means to the adapter.\n\nPower limit (in Watts) is\n\ncalculated by multiplying the\n\nvalue in this field by the value in\n\nthe Slot Power Limit Scale field\n\nexcept when the Slot Power Limit\n\nScale field equals 00b (1.0x) and\n\nSlot Power Limit Value exceeds\n\nEFh, the following alternative\n\nencodings are used: F0h = 250 W\n\nSlot Power Limit F1h = 275 W\n\nSlot Power Limit F2h = 300 W\n\nSlot Power Limit F3h to FFh=\n\nReserved for Slot Power Limit\n\nvalues above 300 W This register\n\nmust be implemented if the Slot\n\nImplemented bit is Set. Writes to\n\nthis register also cause the Port to\n\nsend the Set_Slot_Power_Limit\n\nMessage. The default value prior\n\nto hardware/ firmware\n\ninitialization is 0000 0000b."]
    #[inline(always)]
    #[must_use]
    pub fn splv(&mut self) -> SplvW<PcieRcSlotCapabilitySpec> {
        SplvW::new(self, 7)
    }
    #[doc = "Bits 15:16 - Slot Power Limit Scale \\[SPLS\\]\n\nSpecifies the scale used for the\n\nSlot Power Limit Value. Range of\n\nValues: 00b = 1.0x 01b = 0.1x\n\n10b = 0.01x 11b = 0.001x This\n\nregister must be implemented if\n\nthe Slot Implemented bit is Set.\n\nWrites to this register also cause\n\nthe Port to send the\n\nSet_Slot_Power_Limit Message.\n\nThe default value prior to\n\nhardware/firmware initialization is\n\n00b."]
    #[inline(always)]
    #[must_use]
    pub fn spls(&mut self) -> SplsW<PcieRcSlotCapabilitySpec> {
        SplsW::new(self, 15)
    }
    #[doc = "Bit 17 - Electromechanical Interlock Present \\[EIP\\]\n\nWhen Set, this bit indicates that\n\nan Electromechanical Interlock is\n\nimplemented on the chassis for\n\nthis slot."]
    #[inline(always)]
    #[must_use]
    pub fn eip(&mut self) -> EipW<PcieRcSlotCapabilitySpec> {
        EipW::new(self, 17)
    }
    #[doc = "Bit 18 - No Command Completed Support \\[NCCS\\]\n\nWhen Set, this bit indicates that\n\nthis slot does not generate\n\nsoftware notification when an\n\nissued command is completed by\n\nthe Hot-Plug Controller. This bit is\n\nonly permitted to be Set if the\n\nhot-plug capable Port is able to\n\naccept writes to all fields of the\n\nSlot Control register without delay\n\nbetween successive writes."]
    #[inline(always)]
    #[must_use]
    pub fn nccs(&mut self) -> NccsW<PcieRcSlotCapabilitySpec> {
        NccsW::new(self, 18)
    }
    #[doc = "Bits 19:31 - Physical Slot Number \\[PSN\\]\n\nThis field indicates the physical\n\nslot number attached to this Port.\n\nThis field must be hardware\n\ninitialized to a value that assigns\n\na slot number that is unique\n\nwithin the chassis, regardless of\n\nthe form factor associated with\n\nthe slot. This field must be\n\ninitialized to zero for Ports\n\nconnected to devices that are\n\neither integrated on the system\n\nboard or integrated within the\n\nsame silicon as the Switch device\n\nor Root Port."]
    #[inline(always)]
    #[must_use]
    pub fn psn(&mut self) -> PsnW<PcieRcSlotCapabilitySpec> {
        PsnW::new(self, 19)
    }
}
#[doc = "Slot Capability Register\n\nThis field indicates the physical\n\nslot number attached to this Port.\n\nThis field must be hardware\n\ninitialized to a value that assigns\n\na slot number that is unique\n\nwithin the chassis, regardless of\n\nthe form factor associated with\n\nthe slot. This field must be\n\ninitialized to zero for Ports\n\nconnected to devices that are\n\neither integrated on the system\n\nboard or integrated within the\n\nsame silicon as the Switch device\n\nor Root Port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_slot_capability::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_slot_capability::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
