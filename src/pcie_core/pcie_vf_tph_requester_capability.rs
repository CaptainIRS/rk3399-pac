#[doc = "Register `PCIE_VF_TPH_REQUESTER_CAPABILITY` reader"]
pub type R = crate::R<PcieVfTphRequesterCapabilitySpec>;
#[doc = "Field `NSTM` reader - No ST Mode Supported \\[NSTM\\]
When set to 1, indicates that this Function supports the 'No ST Mode' for the generation of TPH Steering Tags. In the No ST Mode, the device must use a Steering Tag value of 0 for all requests. This bit is hardwired to 1, as all TPH Requesters are required to support the No ST Mode of operation."]
pub type NstmR = crate::BitReader;
#[doc = "Field `IVMS` reader - Interrupt Vector Mode Supported \\[IVMS\\]
A setting of 1 indicates that the Function supports the Interrupt Vector Mode for TPH Steering Tag generation. In the Interrupt Vector Mode, Steering Tags are attached to MSI/MSI-X interrupt requests. The Steering Tag for each interrupt request is selected by the MSI/MSI-X interrupt vector number. This bit is set to 1 by default, but can be modified from the local management bus."]
pub type IvmsR = crate::BitReader;
#[doc = "Field `DSMS` reader - Device- Specific Mode Supported \\[DSMS\\]
A setting of 1 indicates that the Function supports the Device- Specific Mode for TPH Steering Tag generation. In this mode, the Steering Tags are supplied by the client for each request through the HAL master interface. The client typically chooses the Steering Tag values from the ST Table, but is not required to do so. This bit is set to 1 by default, but can be modified from the local management bus."]
pub type DsmsR = crate::BitReader;
#[doc = "Field `R0` reader - Reserved \\[R0\\]
Reserved"]
pub type R0R = crate::FieldReader;
#[doc = "Field `ERS` reader - Extended TPH Requester Supported \\[ERS\\]
When set to 1, indicates that the Function is capable of generating requests with a TPH TLP Prefix."]
pub type ErsR = crate::BitReader;
#[doc = "Field `STTL` reader - ST Table Location \\[STTL\\]
The setting of this field indicates if a Steering Tag Table is implemented for this Function, and its location if present. (00 = ST Table not present, 01 = ST Table in the TPH Requester Capability Structure, 10 = ST values stored in the MSI-X Table in client RAM, 11 = reserved.). This field can be modified from the local management bus."]
pub type SttlR = crate::FieldReader;
#[doc = "Field `R1` reader - Reserved \\[R1\\]
Reserved"]
pub type R1R = crate::FieldReader;
#[doc = "Field `STTS` reader - ST Table Size \\[STTS\\]
Specifies the number of entries in the Steering Tag Table (0 = 1 entry, 1 = 2 entries, and so on). Max limit is 64 entries when the ST Table is located in the TPH Requester Capability Structure, and 2048 entries when located in the MSI-X table. Each entry is 16 bits long. This field can be modified from the local management bus."]
pub type SttsR = crate::FieldReader<u16>;
#[doc = "Field `R2` reader - Reserved \\[R2\\]
Reserved"]
pub type R2R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - No ST Mode Supported \\[NSTM\\]
When set to 1, indicates that this Function supports the 'No ST Mode' for the generation of TPH Steering Tags. In the No ST Mode, the device must use a Steering Tag value of 0 for all requests. This bit is hardwired to 1, as all TPH Requesters are required to support the No ST Mode of operation."]
    #[inline(always)]
    pub fn nstm(&self) -> NstmR {
        NstmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Vector Mode Supported \\[IVMS\\]
A setting of 1 indicates that the Function supports the Interrupt Vector Mode for TPH Steering Tag generation. In the Interrupt Vector Mode, Steering Tags are attached to MSI/MSI-X interrupt requests. The Steering Tag for each interrupt request is selected by the MSI/MSI-X interrupt vector number. This bit is set to 1 by default, but can be modified from the local management bus."]
    #[inline(always)]
    pub fn ivms(&self) -> IvmsR {
        IvmsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Device- Specific Mode Supported \\[DSMS\\]
A setting of 1 indicates that the Function supports the Device- Specific Mode for TPH Steering Tag generation. In this mode, the Steering Tags are supplied by the client for each request through the HAL master interface. The client typically chooses the Steering Tag values from the ST Table, but is not required to do so. This bit is set to 1 by default, but can be modified from the local management bus."]
    #[inline(always)]
    pub fn dsms(&self) -> DsmsR {
        DsmsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Reserved \\[R0\\]
Reserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Extended TPH Requester Supported \\[ERS\\]
When set to 1, indicates that the Function is capable of generating requests with a TPH TLP Prefix."]
    #[inline(always)]
    pub fn ers(&self) -> ErsR {
        ErsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - ST Table Location \\[STTL\\]
The setting of this field indicates if a Steering Tag Table is implemented for this Function, and its location if present. (00 = ST Table not present, 01 = ST Table in the TPH Requester Capability Structure, 10 = ST values stored in the MSI-X Table in client RAM, 11 = reserved.). This field can be modified from the local management bus."]
    #[inline(always)]
    pub fn sttl(&self) -> SttlR {
        SttlR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:15 - Reserved \\[R1\\]
Reserved"]
    #[inline(always)]
    pub fn r1(&self) -> R1R {
        R1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - ST Table Size \\[STTS\\]
Specifies the number of entries in the Steering Tag Table (0 = 1 entry, 1 = 2 entries, and so on). Max limit is 64 entries when the ST Table is located in the TPH Requester Capability Structure, and 2048 entries when located in the MSI-X table. Each entry is 16 bits long. This field can be modified from the local management bus."]
    #[inline(always)]
    pub fn stts(&self) -> SttsR {
        SttsR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 27:31 - Reserved \\[R2\\]
Reserved"]
    #[inline(always)]
    pub fn r2(&self) -> R2R {
        R2R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[doc = "TPH Requester Capability Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_tph_requester_capability::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfTphRequesterCapabilitySpec;
impl crate::RegisterSpec for PcieVfTphRequesterCapabilitySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_tph_requester_capability::R`](R) reader structure"]
impl crate::Readable for PcieVfTphRequesterCapabilitySpec {}
#[doc = "`reset()` method sets PCIE_VF_TPH_REQUESTER_CAPABILITY to value 0x0007_0207"]
impl crate::Resettable for PcieVfTphRequesterCapabilitySpec {
    const RESET_VALUE: u32 = 0x0007_0207;
}
