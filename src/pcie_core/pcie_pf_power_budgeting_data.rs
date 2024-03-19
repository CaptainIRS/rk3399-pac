#[doc = "Register `PCIE_PF_POWER_BUDGETING_DATA` reader"]
pub type R = crate::R<PciePfPowerBudgetingDataSpec>;
#[doc = "Field `BP` reader - Base Power \\[BP\\]\n\nSpecifies base power(in watts) of the\n\nselected power state"]
pub type BpR = crate::FieldReader;
#[doc = "Field `DS` reader - Data Scale \\[DS\\]\n\nScale factor applicable to the Base\n\nPower field."]
pub type DsR = crate::FieldReader;
#[doc = "Field `PSS` reader - PM Sub- State \\[PSS\\]\n\nSpecifies the power management\n\nsub-state of the selected power state"]
pub type PssR = crate::FieldReader;
#[doc = "Field `PS` reader - PM State \\[PS\\]\n\nSpecifies the power management\n\nstate of the Function, for which this\n\npower management data applies."]
pub type PsR = crate::FieldReader;
#[doc = "Field `TYPE` reader - Type \\[TYPE\\]\n\nSpecifies the operation condition for\n\nwhich the data applies."]
pub type TypeR = crate::FieldReader;
#[doc = "Field `PR` reader - Power Rail \\[PR\\]\n\nSpecifies the power rail\n\ncorresponding to the power\n\nmanagement data in this register."]
pub type PrR = crate::FieldReader;
#[doc = "Field `R1` reader - Reserved \\[R1\\]\n\nReserved"]
pub type R1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Base Power \\[BP\\]\n\nSpecifies base power(in watts) of the\n\nselected power state"]
    #[inline(always)]
    pub fn bp(&self) -> BpR {
        BpR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Data Scale \\[DS\\]\n\nScale factor applicable to the Base\n\nPower field."]
    #[inline(always)]
    pub fn ds(&self) -> DsR {
        DsR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - PM Sub- State \\[PSS\\]\n\nSpecifies the power management\n\nsub-state of the selected power state"]
    #[inline(always)]
    pub fn pss(&self) -> PssR {
        PssR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:14 - PM State \\[PS\\]\n\nSpecifies the power management\n\nstate of the Function, for which this\n\npower management data applies."]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:17 - Type \\[TYPE\\]\n\nSpecifies the operation condition for\n\nwhich the data applies."]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Power Rail \\[PR\\]\n\nSpecifies the power rail\n\ncorresponding to the power\n\nmanagement data in this register."]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:31 - Reserved \\[R1\\]\n\nReserved"]
    #[inline(always)]
    pub fn r1(&self) -> R1R {
        R1R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
#[doc = "Power Budgeting Data Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_power_budgeting_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfPowerBudgetingDataSpec;
impl crate::RegisterSpec for PciePfPowerBudgetingDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_power_budgeting_data::R`](R) reader structure"]
impl crate::Readable for PciePfPowerBudgetingDataSpec {}
#[doc = "`reset()` method sets PCIE_PF_POWER_BUDGETING_DATA to value 0x000b_80f0"]
impl crate::Resettable for PciePfPowerBudgetingDataSpec {
    const RESET_VALUE: u32 = 0x000b_80f0;
}
