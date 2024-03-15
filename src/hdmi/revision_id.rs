#[doc = "Register `REVISION_ID` reader"]
pub type R = crate::R<RevisionIdSpec>;
#[doc = "Field `REVISION_ID` reader - Revision ID code fixed by HDMI that Identifies the instantiated DWC_hdmi_tx controller."]
pub type RevisionIdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Revision ID code fixed by HDMI that Identifies the instantiated DWC_hdmi_tx controller."]
    #[inline(always)]
    pub fn revision_id(&self) -> RevisionIdR {
        RevisionIdR::new(self.bits)
    }
}
#[doc = "Revision ID code fixed by HDMI that Identifies the instantiated DWC_hdmi_tx controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revision_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RevisionIdSpec;
impl crate::RegisterSpec for RevisionIdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`revision_id::R`](R) reader structure"]
impl crate::Readable for RevisionIdSpec {}
#[doc = "`reset()` method sets REVISION_ID to value 0x1a"]
impl crate::Resettable for RevisionIdSpec {
    const RESET_VALUE: u8 = 0x1a;
}
