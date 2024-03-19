#[doc = "Register `PCIE_PF_TPH_REQUESTER_CAPABILITY` reader"]
pub type R = crate::R<PciePfTphRequesterCapabilitySpec>;
#[doc = "Field `NSM` reader - No ST Mode Supported \\[NSM\\]\n\nWhen set to 1, indicates that this\n\nFunction supports the 'No ST Mode'\n\nfor the generation of TPH Steering\n\nTags. In the No ST Mode, the device\n\nmust use a Steering Tag value of 0\n\nfor all requests. This bit is hardwired\n\nto 1, as all TPH Requesters are\n\nrequired to support the No ST Mode\n\nof operation."]
pub type NsmR = crate::BitReader;
#[doc = "Field `IVMS` reader - Interrupt Vector Mode Supported \\[IVMS\\]\n\nA setting of 1 indicates that the\n\nFunction supports the Interrupt\n\nVector Mode for TPH Steering Tag\n\ngeneration. In the Interrupt Vector\n\nMode, Steering Tags are attached to\n\nMSI/MSI-X interrupt requests. The\n\nSteering Tag for each interrupt\n\nrequest is selected by the MSI/MSI-X\n\ninterrupt vector number. This bit is\n\nset to 1 by default, but can be\n\nmodified from the local management\n\nbus."]
pub type IvmsR = crate::BitReader;
#[doc = "Field `DSMS` reader - Device- Specific Mode Supported \\[DSMS\\]\n\nA setting of 1 indicates that the\n\nFunction supports the Device-\n\nSpecific Mode for TPH Steering Tag\n\ngeneration. In this mode, the\n\nSteering Tags are supplied by the\n\nclient for each request through the\n\nHAL master interface. The client\n\ntypically chooses the Steering Tag\n\nvalues from the ST Table, but is not\n\nrequired to do so. This bit is set to 1\n\nby default, but can be modified from\n\nthe local management bus."]
pub type DsmsR = crate::BitReader;
#[doc = "Field `R0` reader - Reserved \\[R0\\]\n\nReserved"]
pub type R0R = crate::FieldReader;
#[doc = "Field `ERS` reader - Extended TPH Requester Supported \\[ERS\\]\n\nWhen set to 1, indicates that the\n\nFunction is capable of generating\n\nrequests with a TPH TLP Prefix."]
pub type ErsR = crate::BitReader;
#[doc = "Field `STL` reader - ST Table Location \\[STL\\]\n\nThe setting of this field indicates if a\n\nSteering Tag Table is implemented\n\nfor this Function, and its location if\n\npresent. (00 = ST Table not present,\n\n01 = ST Table in the TPH Requester\n\nCapability Structure, 10 = ST values\n\nstored in the MSI-X Table in client\n\nRAM, 11 = reserved.). This field can\n\nbe modified from the local\n\nmanagement bus."]
pub type StlR = crate::FieldReader;
#[doc = "Field `R1` reader - Reserved \\[R1\\]\n\nReserved"]
pub type R1R = crate::FieldReader;
#[doc = "Field `STS` reader - ST Table Size \\[STS\\]\n\nSpecifies the number of entries in\n\nthe Steering Tag Table (0 = 1 entry,\n\n1 = 2 entries, and so on). Max limit\n\nis 64 entries when the ST Table is\n\nlocated in the TPH Requester\n\nCapability Structure, and 2048\n\nentries when located in the MSI-X\n\ntable. Each entry is 16 bits long. This\n\nfield can be modified from the local\n\nmanagement bus."]
pub type StsR = crate::FieldReader<u16>;
#[doc = "Field `R2` reader - Reserved \\[R2\\]\n\nReserved"]
pub type R2R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - No ST Mode Supported \\[NSM\\]\n\nWhen set to 1, indicates that this\n\nFunction supports the 'No ST Mode'\n\nfor the generation of TPH Steering\n\nTags. In the No ST Mode, the device\n\nmust use a Steering Tag value of 0\n\nfor all requests. This bit is hardwired\n\nto 1, as all TPH Requesters are\n\nrequired to support the No ST Mode\n\nof operation."]
    #[inline(always)]
    pub fn nsm(&self) -> NsmR {
        NsmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Vector Mode Supported \\[IVMS\\]\n\nA setting of 1 indicates that the\n\nFunction supports the Interrupt\n\nVector Mode for TPH Steering Tag\n\ngeneration. In the Interrupt Vector\n\nMode, Steering Tags are attached to\n\nMSI/MSI-X interrupt requests. The\n\nSteering Tag for each interrupt\n\nrequest is selected by the MSI/MSI-X\n\ninterrupt vector number. This bit is\n\nset to 1 by default, but can be\n\nmodified from the local management\n\nbus."]
    #[inline(always)]
    pub fn ivms(&self) -> IvmsR {
        IvmsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Device- Specific Mode Supported \\[DSMS\\]\n\nA setting of 1 indicates that the\n\nFunction supports the Device-\n\nSpecific Mode for TPH Steering Tag\n\ngeneration. In this mode, the\n\nSteering Tags are supplied by the\n\nclient for each request through the\n\nHAL master interface. The client\n\ntypically chooses the Steering Tag\n\nvalues from the ST Table, but is not\n\nrequired to do so. This bit is set to 1\n\nby default, but can be modified from\n\nthe local management bus."]
    #[inline(always)]
    pub fn dsms(&self) -> DsmsR {
        DsmsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Reserved \\[R0\\]\n\nReserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Extended TPH Requester Supported \\[ERS\\]\n\nWhen set to 1, indicates that the\n\nFunction is capable of generating\n\nrequests with a TPH TLP Prefix."]
    #[inline(always)]
    pub fn ers(&self) -> ErsR {
        ErsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - ST Table Location \\[STL\\]\n\nThe setting of this field indicates if a\n\nSteering Tag Table is implemented\n\nfor this Function, and its location if\n\npresent. (00 = ST Table not present,\n\n01 = ST Table in the TPH Requester\n\nCapability Structure, 10 = ST values\n\nstored in the MSI-X Table in client\n\nRAM, 11 = reserved.). This field can\n\nbe modified from the local\n\nmanagement bus."]
    #[inline(always)]
    pub fn stl(&self) -> StlR {
        StlR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:15 - Reserved \\[R1\\]\n\nReserved"]
    #[inline(always)]
    pub fn r1(&self) -> R1R {
        R1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - ST Table Size \\[STS\\]\n\nSpecifies the number of entries in\n\nthe Steering Tag Table (0 = 1 entry,\n\n1 = 2 entries, and so on). Max limit\n\nis 64 entries when the ST Table is\n\nlocated in the TPH Requester\n\nCapability Structure, and 2048\n\nentries when located in the MSI-X\n\ntable. Each entry is 16 bits long. This\n\nfield can be modified from the local\n\nmanagement bus."]
    #[inline(always)]
    pub fn sts(&self) -> StsR {
        StsR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 27:31 - Reserved \\[R2\\]\n\nReserved"]
    #[inline(always)]
    pub fn r2(&self) -> R2R {
        R2R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[doc = "TPH Requester Capability Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_tph_requester_capability::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfTphRequesterCapabilitySpec;
impl crate::RegisterSpec for PciePfTphRequesterCapabilitySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_tph_requester_capability::R`](R) reader structure"]
impl crate::Readable for PciePfTphRequesterCapabilitySpec {}
#[doc = "`reset()` method sets PCIE_PF_TPH_REQUESTER_CAPABILITY to value 0x0007_0207"]
impl crate::Resettable for PciePfTphRequesterCapabilitySpec {
    const RESET_VALUE: u32 = 0x0007_0207;
}
