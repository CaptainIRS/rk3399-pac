#[doc = "Register `PROBE_Id_RevisionId` reader"]
pub type R = crate::R<ProbeIdRevisionIdSpec>;
#[doc = "Field `REVISIONID` reader - Constant."]
pub type RevisionidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Constant."]
    #[inline(always)]
    pub fn revisionid(&self) -> RevisionidR {
        RevisionidR::new(self.bits)
    }
}
#[doc = "Revision ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_id_revision_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProbeIdRevisionIdSpec;
impl crate::RegisterSpec for ProbeIdRevisionIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`probe_id_revision_id::R`](R) reader structure"]
impl crate::Readable for ProbeIdRevisionIdSpec {}
#[doc = "`reset()` method sets PROBE_Id_RevisionId to value 0x0001_aa00"]
impl crate::Resettable for ProbeIdRevisionIdSpec {
    const RESET_VALUE: u32 = 0x0001_aa00;
}
