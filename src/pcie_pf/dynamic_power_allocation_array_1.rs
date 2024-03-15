#[doc = "Register `DYNAMIC_POWER_ALLOCATION_ARRAY_1` reader"]
pub type R = crate::R<DynamicPowerAllocationArray1Spec>;
#[doc = "Field `SPA0_1` reader - Substate Power Allocation 4 \\[SPA0_1\\]
This field contains the power allocation for the DPA substate #4 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts."]
pub type Spa0_1R = crate::FieldReader;
#[doc = "Field `SPA1_1` reader - Substate Power Allocation 5 \\[SPA1_1\\]
This field contains the power allocation for the DPA substate #5 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts."]
pub type Spa1_1R = crate::FieldReader;
#[doc = "Field `SPA2_1` reader - Substate Power Allocation 6 \\[SPA2_1\\]
This field contains the power allocation for the DPA substate #6 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts."]
pub type Spa2_1R = crate::FieldReader;
#[doc = "Field `SPA3_1` reader - Substate Power Allocation 7 \\[SPA3_1\\]
This field contains the power allocation for the DPA substate #7 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts."]
pub type Spa3_1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Substate Power Allocation 4 \\[SPA0_1\\]
This field contains the power allocation for the DPA substate #4 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts."]
    #[inline(always)]
    pub fn spa0_1(&self) -> Spa0_1R {
        Spa0_1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Substate Power Allocation 5 \\[SPA1_1\\]
This field contains the power allocation for the DPA substate #5 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts."]
    #[inline(always)]
    pub fn spa1_1(&self) -> Spa1_1R {
        Spa1_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Substate Power Allocation 6 \\[SPA2_1\\]
This field contains the power allocation for the DPA substate #6 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts."]
    #[inline(always)]
    pub fn spa2_1(&self) -> Spa2_1R {
        Spa2_1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Substate Power Allocation 7 \\[SPA3_1\\]
This field contains the power allocation for the DPA substate #7 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts."]
    #[inline(always)]
    pub fn spa3_1(&self) -> Spa3_1R {
        Spa3_1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Dynamic Power Allocation Array Register 1 This field contains the power allocation for the DPA substate #7 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dynamic_power_allocation_array_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DynamicPowerAllocationArray1Spec;
impl crate::RegisterSpec for DynamicPowerAllocationArray1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dynamic_power_allocation_array_1::R`](R) reader structure"]
impl crate::Readable for DynamicPowerAllocationArray1Spec {}
#[doc = "`reset()` method sets DYNAMIC_POWER_ALLOCATION_ARRAY_1 to value 0x0706_0504"]
impl crate::Resettable for DynamicPowerAllocationArray1Spec {
    const RESET_VALUE: u32 = 0x0706_0504;
}
