#[doc = "Register `DESIGN_ID` reader"]
pub type R = crate::R<DesignIdSpec>;
#[doc = "Field `DESIGN_ID` reader - Design ID code fixed by HDMI that Identifies the\n\ninstantiated DWC_hdmi_tx controller. For example,\n\nDWC_hdmi_tx 2.11a, DESIGN_ID = 21"]
pub type DesignIdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Design ID code fixed by HDMI that Identifies the\n\ninstantiated DWC_hdmi_tx controller. For example,\n\nDWC_hdmi_tx 2.11a, DESIGN_ID = 21"]
    #[inline(always)]
    pub fn design_id(&self) -> DesignIdR {
        DesignIdR::new(self.bits)
    }
}
#[doc = "Design Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`design_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DesignIdSpec;
impl crate::RegisterSpec for DesignIdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`design_id::R`](R) reader structure"]
impl crate::Readable for DesignIdSpec {}
#[doc = "`reset()` method sets DESIGN_ID to value 0x21"]
impl crate::Resettable for DesignIdSpec {
    const RESET_VALUE: u8 = 0x21;
}
