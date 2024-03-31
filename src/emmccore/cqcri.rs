#[doc = "Register `CQCRI` reader"]
pub type R = crate::R<CqcriSpec>;
#[doc = "Field `LCRI` reader - Last Command Response Index\n\nThis field stores the index of the last received command\n\nresponse. CQE shall update the value every time a command\n\nresponse is received."]
pub type LcriR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Last Command Response Index\n\nThis field stores the index of the last received command\n\nresponse. CQE shall update the value every time a command\n\nresponse is received."]
    #[inline(always)]
    pub fn lcri(&self) -> LcriR {
        LcriR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Command queueing command response index register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqcri::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqcriSpec;
impl crate::RegisterSpec for CqcriSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqcri::R`](R) reader structure"]
impl crate::Readable for CqcriSpec {}
#[doc = "`reset()` method sets CQCRI to value 0"]
impl crate::Resettable for CqcriSpec {
    const RESET_VALUE: u32 = 0;
}
