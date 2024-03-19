#[doc = "Register `PCIE_PF_REVISION_ID_AND_CLASS_CODE` reader"]
pub type R = crate::R<PciePfRevisionIdAndClassCodeSpec>;
#[doc = "Field `RID` reader - Revision ID \\[RID\\]\n\nAssigned by the manufacturer of the\n\ndevice to identify the revision\n\nnumber of the device. On power-up,\n\nthe core sets it to the value defined\n\nin the RTL file reg_defaults.h. This\n\nfield can be re- written\n\nindependently for each Function\n\nfrom the local management bus."]
pub type RidR = crate::FieldReader;
#[doc = "Field `PIB` reader - Programming Interface Byte \\[PIB\\]\n\nIdentifies the register set layout of\n\nthe device. On power-up, the core\n\nsets it to the value defined in the\n\nRTL file reg_defaults.h. This field\n\ncan be re- written independently for\n\neach Function from the local\n\nmanagement bus."]
pub type PibR = crate::FieldReader;
#[doc = "Field `SCC` reader - Sub-Class Code \\[SCC\\]\n\nIdentifies a sub-category within the\n\nselected function. On power-up, the\n\ncore sets it to the value defined in\n\nthe RTL file reg_defaults.h. This field\n\ncan be re-written independently for\n\neach Function from the local\n\nmanagement bus."]
pub type SccR = crate::FieldReader;
#[doc = "Field `CC` reader - Class Code \\[CC\\]\n\nIdentifies the function of the device.\n\nOn power- up, the core sets it to\n\nthe value defined in the RTL file\n\nreg_defaults.h. This field can be\n\nrewritten independently for each\n\nFunction from the local management\n\nbus"]
pub type CcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Revision ID \\[RID\\]\n\nAssigned by the manufacturer of the\n\ndevice to identify the revision\n\nnumber of the device. On power-up,\n\nthe core sets it to the value defined\n\nin the RTL file reg_defaults.h. This\n\nfield can be re- written\n\nindependently for each Function\n\nfrom the local management bus."]
    #[inline(always)]
    pub fn rid(&self) -> RidR {
        RidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Programming Interface Byte \\[PIB\\]\n\nIdentifies the register set layout of\n\nthe device. On power-up, the core\n\nsets it to the value defined in the\n\nRTL file reg_defaults.h. This field\n\ncan be re- written independently for\n\neach Function from the local\n\nmanagement bus."]
    #[inline(always)]
    pub fn pib(&self) -> PibR {
        PibR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sub-Class Code \\[SCC\\]\n\nIdentifies a sub-category within the\n\nselected function. On power-up, the\n\ncore sets it to the value defined in\n\nthe RTL file reg_defaults.h. This field\n\ncan be re-written independently for\n\neach Function from the local\n\nmanagement bus."]
    #[inline(always)]
    pub fn scc(&self) -> SccR {
        SccR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Class Code \\[CC\\]\n\nIdentifies the function of the device.\n\nOn power- up, the core sets it to\n\nthe value defined in the RTL file\n\nreg_defaults.h. This field can be\n\nrewritten independently for each\n\nFunction from the local management\n\nbus"]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Revision ID and Class Code Register\n\nIdentifies the function of the device.\n\nOn power- up, the core sets it to\n\nthe value defined in the RTL file\n\nreg_defaults.h. This field can be\n\nrewritten independently for each\n\nFunction from the local management\n\nbus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_revision_id_and_class_code::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfRevisionIdAndClassCodeSpec;
impl crate::RegisterSpec for PciePfRevisionIdAndClassCodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_revision_id_and_class_code::R`](R) reader structure"]
impl crate::Readable for PciePfRevisionIdAndClassCodeSpec {}
#[doc = "`reset()` method sets PCIE_PF_REVISION_ID_AND_CLASS_CODE to value 0"]
impl crate::Resettable for PciePfRevisionIdAndClassCodeSpec {
    const RESET_VALUE: u32 = 0;
}
