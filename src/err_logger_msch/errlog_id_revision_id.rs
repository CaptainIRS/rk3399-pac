#[doc = "Register `ERRLOG_Id_RevisionId` reader"]
pub type R = crate::R<ErrlogIdRevisionIdSpec>;
#[doc = "Field `REVISIONID` reader - Constant."]
pub type RevisionidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Constant."]
    #[inline(always)]
    pub fn revisionid(&self) -> RevisionidR {
        RevisionidR::new(self.bits)
    }
}
#[doc = "It is the same for each error logger.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_id_revision_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrlogIdRevisionIdSpec;
impl crate::RegisterSpec for ErrlogIdRevisionIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errlog_id_revision_id::R`](R) reader structure"]
impl crate::Readable for ErrlogIdRevisionIdSpec {}
#[doc = "`reset()` method sets ERRLOG_Id_RevisionId to value 0x0001_aa00"]
impl crate::Resettable for ErrlogIdRevisionIdSpec {
    const RESET_VALUE: u32 = 0x0001_aa00;
}
