#[doc = "Register `PCIE_CLIENT_POWER_STATUS` reader"]
pub type R = crate::R<PcieClientPowerStatusSpec>;
#[doc = "Field `LINK_PWR_ST` reader - Link power state\n\nCurrent power state of the PCIe link:\n\n4'b0001 = L0\n\n4'b0010 = L0s\n\n4'b0100 = L1\n\n4'b1000 = L2"]
pub type LinkPwrStR = crate::FieldReader;
#[doc = "Function power state\n\nThese outputs provide the current power state of the Physical\n\nFunctions. Bits \\[2:0\\]
capture the power state of Function 0 The\n\npossible power states are:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FcPwrSt {
    #[doc = "0: D0_uninitialized"]
    B000 = 0,
    #[doc = "1: D0_active"]
    B001 = 1,
    #[doc = "2: D1"]
    B010 = 2,
    #[doc = "4: D3_hot"]
    B100 = 4,
}
impl From<FcPwrSt> for u8 {
    #[inline(always)]
    fn from(variant: FcPwrSt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FcPwrSt {
    type Ux = u8;
}
#[doc = "Field `FC_PWR_ST` reader - Function power state\n\nThese outputs provide the current power state of the Physical\n\nFunctions. Bits \\[2:0\\]
capture the power state of Function 0 The\n\npossible power states are:"]
pub type FcPwrStR = crate::FieldReader<FcPwrSt>;
impl FcPwrStR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FcPwrSt> {
        match self.bits {
            0 => Some(FcPwrSt::B000),
            1 => Some(FcPwrSt::B001),
            2 => Some(FcPwrSt::B010),
            4 => Some(FcPwrSt::B100),
            _ => None,
        }
    }
    #[doc = "D0_uninitialized"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == FcPwrSt::B000
    }
    #[doc = "D0_active"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == FcPwrSt::B001
    }
    #[doc = "D1"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == FcPwrSt::B010
    }
    #[doc = "D3_hot"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == FcPwrSt::B100
    }
}
#[doc = "Field `L1_PM_SUBST` reader - L1 power management substate\n\nThis output provides the current state of the L1 PM substates\n\nstate machine. This output is in the PM_CLK clock domain. Its\n\nencodings are:\n\n3'b000 = LTSSM not in L1 state\n\n3'b001 = L1.0 substate\n\n3'b010 = L1.1 substate\n\n3'b011 = Reserved\n\n3'b100 = L1.2.Entry substate\n\n3'b101 = L1.2.Idle substate\n\n3'b110 = L1.2.Exit substate\n\n3'b111 = Reserved"]
pub type L1PmSubstR = crate::FieldReader;
#[doc = "Field `PWR_STCG_FC_NUM` reader - Power state change function number\n\nFunction number of the function for which a power state change\n\noccurred."]
pub type PwrStcgFcNumR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Link power state\n\nCurrent power state of the PCIe link:\n\n4'b0001 = L0\n\n4'b0010 = L0s\n\n4'b0100 = L1\n\n4'b1000 = L2"]
    #[inline(always)]
    pub fn link_pwr_st(&self) -> LinkPwrStR {
        LinkPwrStR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Function power state\n\nThese outputs provide the current power state of the Physical\n\nFunctions. Bits \\[2:0\\]
capture the power state of Function 0 The\n\npossible power states are:"]
    #[inline(always)]
    pub fn fc_pwr_st(&self) -> FcPwrStR {
        FcPwrStR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - L1 power management substate\n\nThis output provides the current state of the L1 PM substates\n\nstate machine. This output is in the PM_CLK clock domain. Its\n\nencodings are:\n\n3'b000 = LTSSM not in L1 state\n\n3'b001 = L1.0 substate\n\n3'b010 = L1.1 substate\n\n3'b011 = Reserved\n\n3'b100 = L1.2.Entry substate\n\n3'b101 = L1.2.Idle substate\n\n3'b110 = L1.2.Exit substate\n\n3'b111 = Reserved"]
    #[inline(always)]
    pub fn l1_pm_subst(&self) -> L1PmSubstR {
        L1PmSubstR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Power state change function number\n\nFunction number of the function for which a power state change\n\noccurred."]
    #[inline(always)]
    pub fn pwr_stcg_fc_num(&self) -> PwrStcgFcNumR {
        PwrStcgFcNumR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "PCIe power management status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_power_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientPowerStatusSpec;
impl crate::RegisterSpec for PcieClientPowerStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_power_status::R`](R) reader structure"]
impl crate::Readable for PcieClientPowerStatusSpec {}
#[doc = "`reset()` method sets PCIE_CLIENT_POWER_STATUS to value 0"]
impl crate::Resettable for PcieClientPowerStatusSpec {
    const RESET_VALUE: u32 = 0;
}
