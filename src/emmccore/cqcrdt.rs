#[doc = "Register `CQCRDT` reader"]
pub type R = crate::R<CqcrdtSpec>;
#[doc = "Field `DCLR` reader - Direct Command Last Response\n\nThis register contains the response of the command generated by\n\nthe last direct-command (DCMD) task which was sent.\n\nCQE shall update this register when it receives the response for a\n\nDCMD task.\n\nThis register is considered valid only after bit 31 of CQTDBR\n\nregister is cleared by CQE."]
pub type DclrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Direct Command Last Response\n\nThis register contains the response of the command generated by\n\nthe last direct-command (DCMD) task which was sent.\n\nCQE shall update this register when it receives the response for a\n\nDCMD task.\n\nThis register is considered valid only after bit 31 of CQTDBR\n\nregister is cleared by CQE."]
    #[inline(always)]
    pub fn dclr(&self) -> DclrR {
        DclrR::new(self.bits)
    }
}
#[doc = "Command queueing command response for direct-command task register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqcrdt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqcrdtSpec;
impl crate::RegisterSpec for CqcrdtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqcrdt::R`](R) reader structure"]
impl crate::Readable for CqcrdtSpec {}
#[doc = "`reset()` method sets CQCRDT to value 0"]
impl crate::Resettable for CqcrdtSpec {
    const RESET_VALUE: u32 = 0;
}