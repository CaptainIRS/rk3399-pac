#[doc = "Register `PCIE_PF_DYNAMIC_POWER_ALLOCATION_ARRAY_0` reader"]
pub type R = crate::R<PciePfDynamicPowerAllocationArray0Spec>;
#[doc = "Field `SPA0_0` reader - Substate Power Allocation 0 \\[SPA0_0\\]\n\nThis field contains the power\n\nallocation for the DPA substate #0\n\ncovered by this register. This value,\n\nwhen multiplied by the Power\n\nAllocation Scale programmed in the\n\nDPA Capability Register, provides the\n\npower associated with the\n\ncorresponding substate in Watts."]
pub type Spa0_0R = crate::FieldReader;
#[doc = "Field `SPA1_0` reader - Substate Power Allocation 1 \\[SPA1_0\\]\n\nThis field contains the power\n\nallocation for the DPA substate #1\n\ncovered by this register. This value,\n\nwhen multiplied by the Power\n\nAllocation Scale programmed in the\n\nDPA Capability Register, provides the\n\npower associated with the\n\ncorresponding substate in Watts."]
pub type Spa1_0R = crate::FieldReader;
#[doc = "Field `SPA2_0` reader - Substate Power Allocation 2 \\[SPA2_0\\]\n\nThis field contains the power\n\nallocation for the DPA substate #2\n\ncovered by this register. This value,\n\nwhen multiplied by the Power\n\nAllocation Scale programmed in the\n\nDPA Capability Register, provides the\n\npower associated with the\n\ncorresponding substate in Watts."]
pub type Spa2_0R = crate::FieldReader;
#[doc = "Field `SPA3_0` reader - Substate Power Allocation 3 \\[SPA3_0\\]\n\nThis field contains the power\n\nallocation for the DPA substate #3\n\ncovered by this register. This value,\n\nwhen multiplied by the Power\n\nAllocation Scale programmed in the\n\nDPA Capability Register, provides the\n\npower associated with the\n\ncorresponding substate in Watts."]
pub type Spa3_0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Substate Power Allocation 0 \\[SPA0_0\\]\n\nThis field contains the power\n\nallocation for the DPA substate #0\n\ncovered by this register. This value,\n\nwhen multiplied by the Power\n\nAllocation Scale programmed in the\n\nDPA Capability Register, provides the\n\npower associated with the\n\ncorresponding substate in Watts."]
    #[inline(always)]
    pub fn spa0_0(&self) -> Spa0_0R {
        Spa0_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Substate Power Allocation 1 \\[SPA1_0\\]\n\nThis field contains the power\n\nallocation for the DPA substate #1\n\ncovered by this register. This value,\n\nwhen multiplied by the Power\n\nAllocation Scale programmed in the\n\nDPA Capability Register, provides the\n\npower associated with the\n\ncorresponding substate in Watts."]
    #[inline(always)]
    pub fn spa1_0(&self) -> Spa1_0R {
        Spa1_0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Substate Power Allocation 2 \\[SPA2_0\\]\n\nThis field contains the power\n\nallocation for the DPA substate #2\n\ncovered by this register. This value,\n\nwhen multiplied by the Power\n\nAllocation Scale programmed in the\n\nDPA Capability Register, provides the\n\npower associated with the\n\ncorresponding substate in Watts."]
    #[inline(always)]
    pub fn spa2_0(&self) -> Spa2_0R {
        Spa2_0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Substate Power Allocation 3 \\[SPA3_0\\]\n\nThis field contains the power\n\nallocation for the DPA substate #3\n\ncovered by this register. This value,\n\nwhen multiplied by the Power\n\nAllocation Scale programmed in the\n\nDPA Capability Register, provides the\n\npower associated with the\n\ncorresponding substate in Watts."]
    #[inline(always)]
    pub fn spa3_0(&self) -> Spa3_0R {
        Spa3_0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Dynamic Power Allocation Array Register 0\n\nThis field contains the power\n\nallocation for the DPA substate #3\n\ncovered by this register. This value,\n\nwhen multiplied by the Power\n\nAllocation Scale programmed in the\n\nDPA Capability Register, provides the\n\npower associated with the\n\ncorresponding substate in Watts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_dynamic_power_allocation_array_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfDynamicPowerAllocationArray0Spec;
impl crate::RegisterSpec for PciePfDynamicPowerAllocationArray0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_dynamic_power_allocation_array_0::R`](R) reader structure"]
impl crate::Readable for PciePfDynamicPowerAllocationArray0Spec {}
#[doc = "`reset()` method sets PCIE_PF_DYNAMIC_POWER_ALLOCATION_ARRAY_0 to value 0x0302_0100"]
impl crate::Resettable for PciePfDynamicPowerAllocationArray0Spec {
    const RESET_VALUE: u32 = 0x0302_0100;
}
