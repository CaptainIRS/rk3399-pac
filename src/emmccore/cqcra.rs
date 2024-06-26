#[doc = "Register `CQCRA` reader"]
pub type R = crate::R<CqcraSpec>;
#[doc = "Field `LCRA` reader - Last Command Response Argument\n\nThis field stores the argument of the last received command. CQE\n\nshall update the value every time a command response is\n\nreceived."]
pub type LcraR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Last Command Response Argument\n\nThis field stores the argument of the last received command. CQE\n\nshall update the value every time a command response is\n\nreceived."]
    #[inline(always)]
    pub fn lcra(&self) -> LcraR {
        LcraR::new(self.bits)
    }
}
#[doc = "Command queueing command response argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqcra::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqcraSpec;
impl crate::RegisterSpec for CqcraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqcra::R`](R) reader structure"]
impl crate::Readable for CqcraSpec {}
#[doc = "`reset()` method sets CQCRA to value 0"]
impl crate::Resettable for CqcraSpec {
    const RESET_VALUE: u32 = 0;
}
