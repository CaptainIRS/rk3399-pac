#[doc = "Register `PCIE_CLIENT_FLR_STATUS` reader"]
pub type R = crate::R<PcieClientFlrStatusSpec>;
#[doc = "Function level reset in progress\n\nThe core asserts bit i of this bus when the host initiates a reset of\n\nFunction i though its FLR bit in the configuration space. The core\n\ncontinues to maintain the output high until the client sets the\n\nFLR_DONE input for the corresponding Function to indicate the\n\ncompletion of the reset operation.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlrInProg {
    #[doc = "0: normal"]
    B0 = 0,
    #[doc = "1: function level reset in progress"]
    B1 = 1,
}
impl From<FlrInProg> for bool {
    #[inline(always)]
    fn from(variant: FlrInProg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLR_IN_PROG` reader - Function level reset in progress\n\nThe core asserts bit i of this bus when the host initiates a reset of\n\nFunction i though its FLR bit in the configuration space. The core\n\ncontinues to maintain the output high until the client sets the\n\nFLR_DONE input for the corresponding Function to indicate the\n\ncompletion of the reset operation."]
pub type FlrInProgR = crate::BitReader<FlrInProg>;
impl FlrInProgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FlrInProg {
        match self.bits {
            false => FlrInProg::B0,
            true => FlrInProg::B1,
        }
    }
    #[doc = "normal"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FlrInProg::B0
    }
    #[doc = "function level reset in progress"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FlrInProg::B1
    }
}
#[doc = "Virtual function level reset in progress\n\nThe core asserts bit i of this bus when the host initiates a reset of\n\nVirtual Function i though its FLR bit in the configuration space.\n\nThe core continues to maintain the output high until the client\n\nsets the FLR_DONE input for the corresponding VF to indicate the\n\ncompletion of the reset operation.\n\nOne bit for each function\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VfFlrInProg {
    #[doc = "0: normal"]
    B0 = 0,
    #[doc = "1: function level reset in progress"]
    B1 = 1,
}
impl From<VfFlrInProg> for u8 {
    #[inline(always)]
    fn from(variant: VfFlrInProg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VfFlrInProg {
    type Ux = u8;
}
#[doc = "Field `VF_FLR_IN_PROG` reader - Virtual function level reset in progress\n\nThe core asserts bit i of this bus when the host initiates a reset of\n\nVirtual Function i though its FLR bit in the configuration space.\n\nThe core continues to maintain the output high until the client\n\nsets the FLR_DONE input for the corresponding VF to indicate the\n\ncompletion of the reset operation.\n\nOne bit for each function"]
pub type VfFlrInProgR = crate::FieldReader<VfFlrInProg>;
impl VfFlrInProgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VfFlrInProg> {
        match self.bits {
            0 => Some(VfFlrInProg::B0),
            1 => Some(VfFlrInProg::B1),
            _ => None,
        }
    }
    #[doc = "normal"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VfFlrInProg::B0
    }
    #[doc = "function level reset in progress"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VfFlrInProg::B1
    }
}
impl R {
    #[doc = "Bit 0 - Function level reset in progress\n\nThe core asserts bit i of this bus when the host initiates a reset of\n\nFunction i though its FLR bit in the configuration space. The core\n\ncontinues to maintain the output high until the client sets the\n\nFLR_DONE input for the corresponding Function to indicate the\n\ncompletion of the reset operation."]
    #[inline(always)]
    pub fn flr_in_prog(&self) -> FlrInProgR {
        FlrInProgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Virtual function level reset in progress\n\nThe core asserts bit i of this bus when the host initiates a reset of\n\nVirtual Function i though its FLR bit in the configuration space.\n\nThe core continues to maintain the output high until the client\n\nsets the FLR_DONE input for the corresponding VF to indicate the\n\ncompletion of the reset operation.\n\nOne bit for each function"]
    #[inline(always)]
    pub fn vf_flr_in_prog(&self) -> VfFlrInProgR {
        VfFlrInProgR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Function level reset status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_flr_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientFlrStatusSpec;
impl crate::RegisterSpec for PcieClientFlrStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_flr_status::R`](R) reader structure"]
impl crate::Readable for PcieClientFlrStatusSpec {}
#[doc = "`reset()` method sets PCIE_CLIENT_FLR_STATUS to value 0"]
impl crate::Resettable for PcieClientFlrStatusSpec {
    const RESET_VALUE: u32 = 0;
}
