#[doc = "Register `REVISION_ID_AND_CLASS_CODE` reader"]
pub type R = crate::R<RevisionIdAndClassCodeSpec>;
#[doc = "Field `RID` reader - Revision ID \\[RID\\]
Assigned by the manufacturer of the device to identify the revision RO Setting of this field Denali PCIe Core Register Specification, PMC-Sierra Version 3.4 202 number of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function."]
pub type RidR = crate::FieldReader;
#[doc = "Field `PIB` reader - Programming Interface Byte \\[PIB\\]
Identifies the register set layout of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function."]
pub type PibR = crate::FieldReader;
#[doc = "Field `SCC` reader - Sub-Class Code \\[SCC\\]
Identifies a sub-category within the selected function. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function."]
pub type SccR = crate::FieldReader;
#[doc = "Field `CC` reader - Class Code \\[CC\\]
Identifies the function of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function."]
pub type CcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Revision ID \\[RID\\]
Assigned by the manufacturer of the device to identify the revision RO Setting of this field Denali PCIe Core Register Specification, PMC-Sierra Version 3.4 202 number of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function."]
    #[inline(always)]
    pub fn rid(&self) -> RidR {
        RidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Programming Interface Byte \\[PIB\\]
Identifies the register set layout of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function."]
    #[inline(always)]
    pub fn pib(&self) -> PibR {
        PibR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sub-Class Code \\[SCC\\]
Identifies a sub-category within the selected function. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function."]
    #[inline(always)]
    pub fn scc(&self) -> SccR {
        SccR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Class Code \\[CC\\]
Identifies the function of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function."]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Revision ID and Class Code Register Identifies the function of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revision_id_and_class_code::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RevisionIdAndClassCodeSpec;
impl crate::RegisterSpec for RevisionIdAndClassCodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`revision_id_and_class_code::R`](R) reader structure"]
impl crate::Readable for RevisionIdAndClassCodeSpec {}
#[doc = "`reset()` method sets REVISION_ID_AND_CLASS_CODE to value 0"]
impl crate::Resettable for RevisionIdAndClassCodeSpec {
    const RESET_VALUE: u32 = 0;
}
