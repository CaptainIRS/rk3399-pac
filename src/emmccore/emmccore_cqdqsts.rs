#[doc = "Register `EMMCCORE_CQDQSTS` reader"]
pub type R = crate::R<EmmccoreCqdqstsSpec>;
#[doc = "Field `DQS` reader - Device Queue Status\n\nEvery time the Host controller receives a queue status register\n\n(QSR) from the device, it updates this register with the response\n\nof status command, i.e. the device's queue status."]
pub type DqsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Device Queue Status\n\nEvery time the Host controller receives a queue status register\n\n(QSR) from the device, it updates this register with the response\n\nof status command, i.e. the device's queue status."]
    #[inline(always)]
    pub fn dqs(&self) -> DqsR {
        DqsR::new(self.bits)
    }
}
#[doc = "Command queueing device queue status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqdqsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreCqdqstsSpec;
impl crate::RegisterSpec for EmmccoreCqdqstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_cqdqsts::R`](R) reader structure"]
impl crate::Readable for EmmccoreCqdqstsSpec {}
#[doc = "`reset()` method sets EMMCCORE_CQDQSTS to value 0"]
impl crate::Resettable for EmmccoreCqdqstsSpec {
    const RESET_VALUE: u32 = 0;
}
