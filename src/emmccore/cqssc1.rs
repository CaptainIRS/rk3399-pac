#[doc = "Register `CQSSC1` reader"]
pub type R = crate::R<Cqssc1Spec>;
#[doc = "Register `CQSSC1` writer"]
pub type W = crate::W<Cqssc1Spec>;
#[doc = "Field `SSCIT` reader - Send Status Command Idle Timer\n\nThis field indicates to CQE the polling period to use when using\n\nperiodic SEND_QUEUE_STATUS (CMD13) polling.\n\nPeriodic polling is used when tasks are pending in the device, but\n\nno data transfer is in progress. When a SEND_QUEUE_STATUS\n\nresponse indicating that no task is ready for execution, CQE\n\ncounts the configured time until it issues the next\n\nSEND_QUEUE_STATUS.\n\nTimer units are clock periods of the clock whose frequency is\n\nspecified in the Internal Timer Clock Frequency field CQCAP\n\negister.\n\nThe minimum value is 0001h (1 clock period) and the maximum\n\nvalue is FFFFh (65535 clock periods). Default interval is: 4096\n\nclock periods.\n\nFor example, a CQCAP field value of 0 indicates a 19.2 MHz clock\n\nfrequency (period = 52.08 ns). If the setting in CQSST is 1000h,\n\nthe calculated polling period is 4096*52.08 ns= 213.33 us."]
pub type SscitR = crate::FieldReader<u16>;
#[doc = "Field `SSCIT` writer - Send Status Command Idle Timer\n\nThis field indicates to CQE the polling period to use when using\n\nperiodic SEND_QUEUE_STATUS (CMD13) polling.\n\nPeriodic polling is used when tasks are pending in the device, but\n\nno data transfer is in progress. When a SEND_QUEUE_STATUS\n\nresponse indicating that no task is ready for execution, CQE\n\ncounts the configured time until it issues the next\n\nSEND_QUEUE_STATUS.\n\nTimer units are clock periods of the clock whose frequency is\n\nspecified in the Internal Timer Clock Frequency field CQCAP\n\negister.\n\nThe minimum value is 0001h (1 clock period) and the maximum\n\nvalue is FFFFh (65535 clock periods). Default interval is: 4096\n\nclock periods.\n\nFor example, a CQCAP field value of 0 indicates a 19.2 MHz clock\n\nfrequency (period = 52.08 ns). If the setting in CQSST is 1000h,\n\nthe calculated polling period is 4096*52.08 ns= 213.33 us."]
pub type SscitW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SSCBC` reader - Send Status Command Block Counter\n\nThis field indicates to CQE when to send SEND_QUEUE_STATUS\n\n(CMD13) command to inquire the status of the device's task\n\nqueue.\n\nA value of n means CQE shall send status command on the CMD\n\nline, during the transfer of data block BLOCK_CNT-n, on the data\n\nlines, where BLOCK_CNT is the number of blocks inthe current\n\ntransaction.\n\nA value of 0 means that SEND_QUEUE_STATUS (CMD13)\n\ncommand shall not be sent during the transaction. Instead it will\n\nbe sentonly when the data lines are idle.\n\nA value of 1 means that STATUS command is to be sent during\n\nthe last block of the transaction."]
pub type SscbcR = crate::FieldReader;
#[doc = "Field `SSCBC` writer - Send Status Command Block Counter\n\nThis field indicates to CQE when to send SEND_QUEUE_STATUS\n\n(CMD13) command to inquire the status of the device's task\n\nqueue.\n\nA value of n means CQE shall send status command on the CMD\n\nline, during the transfer of data block BLOCK_CNT-n, on the data\n\nlines, where BLOCK_CNT is the number of blocks inthe current\n\ntransaction.\n\nA value of 0 means that SEND_QUEUE_STATUS (CMD13)\n\ncommand shall not be sent during the transaction. Instead it will\n\nbe sentonly when the data lines are idle.\n\nA value of 1 means that STATUS command is to be sent during\n\nthe last block of the transaction."]
pub type SscbcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - Send Status Command Idle Timer\n\nThis field indicates to CQE the polling period to use when using\n\nperiodic SEND_QUEUE_STATUS (CMD13) polling.\n\nPeriodic polling is used when tasks are pending in the device, but\n\nno data transfer is in progress. When a SEND_QUEUE_STATUS\n\nresponse indicating that no task is ready for execution, CQE\n\ncounts the configured time until it issues the next\n\nSEND_QUEUE_STATUS.\n\nTimer units are clock periods of the clock whose frequency is\n\nspecified in the Internal Timer Clock Frequency field CQCAP\n\negister.\n\nThe minimum value is 0001h (1 clock period) and the maximum\n\nvalue is FFFFh (65535 clock periods). Default interval is: 4096\n\nclock periods.\n\nFor example, a CQCAP field value of 0 indicates a 19.2 MHz clock\n\nfrequency (period = 52.08 ns). If the setting in CQSST is 1000h,\n\nthe calculated polling period is 4096*52.08 ns= 213.33 us."]
    #[inline(always)]
    pub fn sscit(&self) -> SscitR {
        SscitR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Send Status Command Block Counter\n\nThis field indicates to CQE when to send SEND_QUEUE_STATUS\n\n(CMD13) command to inquire the status of the device's task\n\nqueue.\n\nA value of n means CQE shall send status command on the CMD\n\nline, during the transfer of data block BLOCK_CNT-n, on the data\n\nlines, where BLOCK_CNT is the number of blocks inthe current\n\ntransaction.\n\nA value of 0 means that SEND_QUEUE_STATUS (CMD13)\n\ncommand shall not be sent during the transaction. Instead it will\n\nbe sentonly when the data lines are idle.\n\nA value of 1 means that STATUS command is to be sent during\n\nthe last block of the transaction."]
    #[inline(always)]
    pub fn sscbc(&self) -> SscbcR {
        SscbcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Send Status Command Idle Timer\n\nThis field indicates to CQE the polling period to use when using\n\nperiodic SEND_QUEUE_STATUS (CMD13) polling.\n\nPeriodic polling is used when tasks are pending in the device, but\n\nno data transfer is in progress. When a SEND_QUEUE_STATUS\n\nresponse indicating that no task is ready for execution, CQE\n\ncounts the configured time until it issues the next\n\nSEND_QUEUE_STATUS.\n\nTimer units are clock periods of the clock whose frequency is\n\nspecified in the Internal Timer Clock Frequency field CQCAP\n\negister.\n\nThe minimum value is 0001h (1 clock period) and the maximum\n\nvalue is FFFFh (65535 clock periods). Default interval is: 4096\n\nclock periods.\n\nFor example, a CQCAP field value of 0 indicates a 19.2 MHz clock\n\nfrequency (period = 52.08 ns). If the setting in CQSST is 1000h,\n\nthe calculated polling period is 4096*52.08 ns= 213.33 us."]
    #[inline(always)]
    #[must_use]
    pub fn sscit(&mut self) -> SscitW<Cqssc1Spec> {
        SscitW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Send Status Command Block Counter\n\nThis field indicates to CQE when to send SEND_QUEUE_STATUS\n\n(CMD13) command to inquire the status of the device's task\n\nqueue.\n\nA value of n means CQE shall send status command on the CMD\n\nline, during the transfer of data block BLOCK_CNT-n, on the data\n\nlines, where BLOCK_CNT is the number of blocks inthe current\n\ntransaction.\n\nA value of 0 means that SEND_QUEUE_STATUS (CMD13)\n\ncommand shall not be sent during the transaction. Instead it will\n\nbe sentonly when the data lines are idle.\n\nA value of 1 means that STATUS command is to be sent during\n\nthe last block of the transaction."]
    #[inline(always)]
    #[must_use]
    pub fn sscbc(&mut self) -> SscbcW<Cqssc1Spec> {
        SscbcW::new(self, 16)
    }
}
#[doc = "Command queueing send status configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqssc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqssc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cqssc1Spec;
impl crate::RegisterSpec for Cqssc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqssc1::R`](R) reader structure"]
impl crate::Readable for Cqssc1Spec {}
#[doc = "`write(|w| ..)` method takes [`cqssc1::W`](W) writer structure"]
impl crate::Writable for Cqssc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQSSC1 to value 0x0001_1000"]
impl crate::Resettable for Cqssc1Spec {
    const RESET_VALUE: u32 = 0x0001_1000;
}
