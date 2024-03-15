#[doc = "Register `EMMCCORE_CQCRI` reader"]
pub type R = crate::R<EmmccoreCqcriSpec>;
#[doc = "Field `LCRI` reader - Last Command Response Index This field stores the index of the last received command response. CQE shall update the value every time a command response is received."]
pub type LcriR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Last Command Response Index This field stores the index of the last received command response. CQE shall update the value every time a command response is received."]
    #[inline(always)]
    pub fn lcri(&self) -> LcriR {
        LcriR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Command queueing command response index register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqcri::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreCqcriSpec;
impl crate::RegisterSpec for EmmccoreCqcriSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_cqcri::R`](R) reader structure"]
impl crate::Readable for EmmccoreCqcriSpec {}
#[doc = "`reset()` method sets EMMCCORE_CQCRI to value 0"]
impl crate::Resettable for EmmccoreCqcriSpec {
    const RESET_VALUE: u32 = 0;
}
