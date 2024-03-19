#[doc = "Register `PCIE_CLIENT_VF_STATUS` reader"]
pub type R = crate::R<PcieClientVfStatusSpec>;
#[doc = "Virtual function bus master enable\n\nBit i of this bus reflects the setting of the Bus Master Enable bit of\n\nthe PCI Command Register of Virtual Function i. Client logic must\n\ncheck the state of this bit before initiating any memory read or\n\nwrite transactions from the VF.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VfBusMasterEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable One bit for each function"]
    B1 = 1,
}
impl From<VfBusMasterEn> for u8 {
    #[inline(always)]
    fn from(variant: VfBusMasterEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VfBusMasterEn {
    type Ux = u8;
}
#[doc = "Field `VF_BUS_MASTER_EN` reader - Virtual function bus master enable\n\nBit i of this bus reflects the setting of the Bus Master Enable bit of\n\nthe PCI Command Register of Virtual Function i. Client logic must\n\ncheck the state of this bit before initiating any memory read or\n\nwrite transactions from the VF."]
pub type VfBusMasterEnR = crate::FieldReader<VfBusMasterEn>;
impl VfBusMasterEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VfBusMasterEn> {
        match self.bits {
            0 => Some(VfBusMasterEn::B0),
            1 => Some(VfBusMasterEn::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VfBusMasterEn::B0
    }
    #[doc = "enable One bit for each function"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VfBusMasterEn::B1
    }
}
#[doc = "Virtual function enable\n\nThe core sets bit i of this bus when the host has configured the\n\ncorresponding Virtual Function i. Client logic must check the state\n\nof this bit before initiating any request from the VF.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VfEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable One bit for each function"]
    B1 = 1,
}
impl From<VfEn> for u8 {
    #[inline(always)]
    fn from(variant: VfEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VfEn {
    type Ux = u8;
}
#[doc = "Field `VF_EN` reader - Virtual function enable\n\nThe core sets bit i of this bus when the host has configured the\n\ncorresponding Virtual Function i. Client logic must check the state\n\nof this bit before initiating any request from the VF."]
pub type VfEnR = crate::FieldReader<VfEn>;
impl VfEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VfEn> {
        match self.bits {
            0 => Some(VfEn::B0),
            1 => Some(VfEn::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VfEn::B0
    }
    #[doc = "enable One bit for each function"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VfEn::B1
    }
}
impl R {
    #[doc = "Bits 0:7 - Virtual function bus master enable\n\nBit i of this bus reflects the setting of the Bus Master Enable bit of\n\nthe PCI Command Register of Virtual Function i. Client logic must\n\ncheck the state of this bit before initiating any memory read or\n\nwrite transactions from the VF."]
    #[inline(always)]
    pub fn vf_bus_master_en(&self) -> VfBusMasterEnR {
        VfBusMasterEnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Virtual function enable\n\nThe core sets bit i of this bus when the host has configured the\n\ncorresponding Virtual Function i. Client logic must check the state\n\nof this bit before initiating any request from the VF."]
    #[inline(always)]
    pub fn vf_en(&self) -> VfEnR {
        VfEnR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Virtual function status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_vf_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientVfStatusSpec;
impl crate::RegisterSpec for PcieClientVfStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_vf_status::R`](R) reader structure"]
impl crate::Readable for PcieClientVfStatusSpec {}
#[doc = "`reset()` method sets PCIE_CLIENT_VF_STATUS to value 0"]
impl crate::Resettable for PcieClientVfStatusSpec {
    const RESET_VALUE: u32 = 0;
}
