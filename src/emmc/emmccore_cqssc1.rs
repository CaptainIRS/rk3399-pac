#[doc = "Register `EMMCCORE_CQSSC1` reader"]
pub type R = crate::R<EmmccoreCqssc1Spec>;
#[doc = "Register `EMMCCORE_CQSSC1` writer"]
pub type W = crate::W<EmmccoreCqssc1Spec>;
#[doc = "Field `SSCIT` reader - Send Status Command Idle Timer This field indicates to CQE the polling period to use when using periodic SEND_QUEUE_STATUS (CMD13) polling. Periodic polling is used when tasks are pending in the device, but no data transfer is in progress. When a SEND_QUEUE_STATUS response indicating that no task is ready for execution, CQE counts the configured time until it issues the next SEND_QUEUE_STATUS. Timer units are clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP egister. The minimum value is 0001h (1 clock period) and the maximum value is FFFFh (65535 clock periods). Default interval is: 4096 clock periods. For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency (period = 52.08 ns). If the setting in CQSST is 1000h, the calculated polling period is 4096*52.08 ns= 213.33 us."]
pub type SscitR = crate::FieldReader<u16>;
#[doc = "Field `SSCIT` writer - Send Status Command Idle Timer This field indicates to CQE the polling period to use when using periodic SEND_QUEUE_STATUS (CMD13) polling. Periodic polling is used when tasks are pending in the device, but no data transfer is in progress. When a SEND_QUEUE_STATUS response indicating that no task is ready for execution, CQE counts the configured time until it issues the next SEND_QUEUE_STATUS. Timer units are clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP egister. The minimum value is 0001h (1 clock period) and the maximum value is FFFFh (65535 clock periods). Default interval is: 4096 clock periods. For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency (period = 52.08 ns). If the setting in CQSST is 1000h, the calculated polling period is 4096*52.08 ns= 213.33 us."]
pub type SscitW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SSCBC` reader - Send Status Command Block Counter This field indicates to CQE when to send SEND_QUEUE_STATUS (CMD13) command to inquire the status of the device's task queue. A value of n means CQE shall send status command on the CMD line, during the transfer of data block BLOCK_CNT-n, on the data lines, where BLOCK_CNT is the number of blocks inthe current transaction. A value of 0 means that SEND_QUEUE_STATUS (CMD13) command shall not be sent during the transaction. Instead it will be sentonly when the data lines are idle. A value of 1 means that STATUS command is to be sent during the last block of the transaction."]
pub type SscbcR = crate::FieldReader;
#[doc = "Field `SSCBC` writer - Send Status Command Block Counter This field indicates to CQE when to send SEND_QUEUE_STATUS (CMD13) command to inquire the status of the device's task queue. A value of n means CQE shall send status command on the CMD line, during the transfer of data block BLOCK_CNT-n, on the data lines, where BLOCK_CNT is the number of blocks inthe current transaction. A value of 0 means that SEND_QUEUE_STATUS (CMD13) command shall not be sent during the transaction. Instead it will be sentonly when the data lines are idle. A value of 1 means that STATUS command is to be sent during the last block of the transaction."]
pub type SscbcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - Send Status Command Idle Timer This field indicates to CQE the polling period to use when using periodic SEND_QUEUE_STATUS (CMD13) polling. Periodic polling is used when tasks are pending in the device, but no data transfer is in progress. When a SEND_QUEUE_STATUS response indicating that no task is ready for execution, CQE counts the configured time until it issues the next SEND_QUEUE_STATUS. Timer units are clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP egister. The minimum value is 0001h (1 clock period) and the maximum value is FFFFh (65535 clock periods). Default interval is: 4096 clock periods. For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency (period = 52.08 ns). If the setting in CQSST is 1000h, the calculated polling period is 4096*52.08 ns= 213.33 us."]
    #[inline(always)]
    pub fn sscit(&self) -> SscitR {
        SscitR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Send Status Command Block Counter This field indicates to CQE when to send SEND_QUEUE_STATUS (CMD13) command to inquire the status of the device's task queue. A value of n means CQE shall send status command on the CMD line, during the transfer of data block BLOCK_CNT-n, on the data lines, where BLOCK_CNT is the number of blocks inthe current transaction. A value of 0 means that SEND_QUEUE_STATUS (CMD13) command shall not be sent during the transaction. Instead it will be sentonly when the data lines are idle. A value of 1 means that STATUS command is to be sent during the last block of the transaction."]
    #[inline(always)]
    pub fn sscbc(&self) -> SscbcR {
        SscbcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Send Status Command Idle Timer This field indicates to CQE the polling period to use when using periodic SEND_QUEUE_STATUS (CMD13) polling. Periodic polling is used when tasks are pending in the device, but no data transfer is in progress. When a SEND_QUEUE_STATUS response indicating that no task is ready for execution, CQE counts the configured time until it issues the next SEND_QUEUE_STATUS. Timer units are clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP egister. The minimum value is 0001h (1 clock period) and the maximum value is FFFFh (65535 clock periods). Default interval is: 4096 clock periods. For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency (period = 52.08 ns). If the setting in CQSST is 1000h, the calculated polling period is 4096*52.08 ns= 213.33 us."]
    #[inline(always)]
    #[must_use]
    pub fn sscit(&mut self) -> SscitW<EmmccoreCqssc1Spec> {
        SscitW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Send Status Command Block Counter This field indicates to CQE when to send SEND_QUEUE_STATUS (CMD13) command to inquire the status of the device's task queue. A value of n means CQE shall send status command on the CMD line, during the transfer of data block BLOCK_CNT-n, on the data lines, where BLOCK_CNT is the number of blocks inthe current transaction. A value of 0 means that SEND_QUEUE_STATUS (CMD13) command shall not be sent during the transaction. Instead it will be sentonly when the data lines are idle. A value of 1 means that STATUS command is to be sent during the last block of the transaction."]
    #[inline(always)]
    #[must_use]
    pub fn sscbc(&mut self) -> SscbcW<EmmccoreCqssc1Spec> {
        SscbcW::new(self, 16)
    }
}
#[doc = "Command queueing send status configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqssc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqssc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreCqssc1Spec;
impl crate::RegisterSpec for EmmccoreCqssc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_cqssc1::R`](R) reader structure"]
impl crate::Readable for EmmccoreCqssc1Spec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_cqssc1::W`](W) writer structure"]
impl crate::Writable for EmmccoreCqssc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_CQSSC1 to value 0x0001_1000"]
impl crate::Resettable for EmmccoreCqssc1Spec {
    const RESET_VALUE: u32 = 0x0001_1000;
}
