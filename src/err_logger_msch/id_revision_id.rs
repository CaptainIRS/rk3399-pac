#[doc = "Register `Id_RevisionId` reader"]
pub type R = crate::R<IdRevisionIdSpec>;
#[doc = "Field `REVISIONID` reader - Constant."]
pub type RevisionidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Constant."]
    #[inline(always)]
    pub fn revisionid(&self) -> RevisionidR {
        RevisionidR::new(self.bits)
    }
}
#[doc = "It is the same for each error logger.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_revision_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdRevisionIdSpec;
impl crate::RegisterSpec for IdRevisionIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_revision_id::R`](R) reader structure"]
impl crate::Readable for IdRevisionIdSpec {}
#[doc = "`reset()` method sets Id_RevisionId to value 0x0001_aa00"]
impl crate::Resettable for IdRevisionIdSpec {
    const RESET_VALUE: u32 = 0x0001_aa00;
}
