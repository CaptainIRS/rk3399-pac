#[doc = "Register `EMMCCORE_CQCRA` reader"]
pub type R = crate::R<EmmccoreCqcraSpec>;
#[doc = "Field `LCRA` reader - Last Command Response Argument This field stores the argument of the last received command. CQE shall update the value every time a command response is received."]
pub type LcraR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Last Command Response Argument This field stores the argument of the last received command. CQE shall update the value every time a command response is received."]
    #[inline(always)]
    pub fn lcra(&self) -> LcraR {
        LcraR::new(self.bits)
    }
}
#[doc = "Command queueing command response argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqcra::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreCqcraSpec;
impl crate::RegisterSpec for EmmccoreCqcraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_cqcra::R`](R) reader structure"]
impl crate::Readable for EmmccoreCqcraSpec {}
#[doc = "`reset()` method sets EMMCCORE_CQCRA to value 0"]
impl crate::Resettable for EmmccoreCqcraSpec {
    const RESET_VALUE: u32 = 0;
}
