#[doc = "Register `PCIE_CLIENT_VF_PWR_STATUS` reader"]
pub type R = crate::R<PcieClientVfPwrStatusSpec>;
#[doc = "Virtual function power status\n\nThese outputs provide the current power state of the Virtual\n\nFunctions. Bits \\[2:0\\]
capture the power state of Virtual Function\n\n0 , bits \\[5:3\\]
capture that of Virtual Function 1, and so on. The\n\npossible power states are:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum VfPwrSt {
    #[doc = "0: D0_uninitialized"]
    B000 = 0,
    #[doc = "1: D0_active"]
    B001 = 1,
    #[doc = "2: D1"]
    B010 = 2,
    #[doc = "4: D3_hot"]
    B100 = 4,
}
impl From<VfPwrSt> for u32 {
    #[inline(always)]
    fn from(variant: VfPwrSt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VfPwrSt {
    type Ux = u32;
}
#[doc = "Field `VF_PWR_ST` reader - Virtual function power status\n\nThese outputs provide the current power state of the Virtual\n\nFunctions. Bits \\[2:0\\]
capture the power state of Virtual Function\n\n0 , bits \\[5:3\\]
capture that of Virtual Function 1, and so on. The\n\npossible power states are:"]
pub type VfPwrStR = crate::FieldReader<VfPwrSt>;
impl VfPwrStR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VfPwrSt> {
        match self.bits {
            0 => Some(VfPwrSt::B000),
            1 => Some(VfPwrSt::B001),
            2 => Some(VfPwrSt::B010),
            4 => Some(VfPwrSt::B100),
            _ => None,
        }
    }
    #[doc = "D0_uninitialized"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == VfPwrSt::B000
    }
    #[doc = "D0_active"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == VfPwrSt::B001
    }
    #[doc = "D1"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == VfPwrSt::B010
    }
    #[doc = "D3_hot"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == VfPwrSt::B100
    }
}
impl R {
    #[doc = "Bits 0:23 - Virtual function power status\n\nThese outputs provide the current power state of the Virtual\n\nFunctions. Bits \\[2:0\\]
capture the power state of Virtual Function\n\n0 , bits \\[5:3\\]
capture that of Virtual Function 1, and so on. The\n\npossible power states are:"]
    #[inline(always)]
    pub fn vf_pwr_st(&self) -> VfPwrStR {
        VfPwrStR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Virtual function power status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_vf_pwr_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientVfPwrStatusSpec;
impl crate::RegisterSpec for PcieClientVfPwrStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_vf_pwr_status::R`](R) reader structure"]
impl crate::Readable for PcieClientVfPwrStatusSpec {}
#[doc = "`reset()` method sets PCIE_CLIENT_VF_PWR_STATUS to value 0"]
impl crate::Resettable for PcieClientVfPwrStatusSpec {
    const RESET_VALUE: u32 = 0;
}
