#[doc = "Register `EMMCCORE_CQTDB` reader"]
pub type R = crate::R<EmmccoreCqtdbSpec>;
#[doc = "Register `EMMCCORE_CQTDB` writer"]
pub type W = crate::W<EmmccoreCqtdbSpec>;
#[doc = "Field `TASKDOORBELL` reader - Command Queueing Task Doorbell\n\nSoftware shall configure TDLBA and TDLBAU, and enable CQE in\n\nCQCFG before using this register.\n\nWriting 1 to bit n of this register triggers CQE to start processing\n\nthe task encoded in slot n of the TDL.\n\nCQE always processes tasks in-order according to the order\n\nsubmitted to the list by CQTDBR write transactions.\n\nCQE processes Data Transfer tasks by reading the Task\n\nDescriptor and sending QUEUED_TASK_PARAMS (CMD44) and\n\nQUEUED_TASK_ADDRESS (CMD45) commands to the device.\n\nCQE processes DCMD tasks (in slot #31, when enabled) by\n\nreading the Task Descriptor, and generating the command\n\nencoded by its index and argument.\n\nThe corresponding bit is cleared to 0 by CQE in one of the\n\nfollowing events:\n\na. When a task execution is completed (with success or error)\n\nb. The task is cleared using CQTCLR register\n\nc. All tasks are cleared using CQCTL register\n\nd. CQE is disabled using CQCFG register\n\nSoftware may initiate multiple tasks at the same time (batch\n\nsubmission) by writing 1 to multiple bits of this register in the\n\nsame transaction.\n\nIn the case of batch submission:\n\nCQE shall process the tasks in order of the task index, starting\n\nwith the lowest index.\n\nIf one or more tasks in the batch are marked with QBR, the\n\nordering of execution will be based on said processing order.\n\nWriting 0 by software shall have no impact on the hardware, and\n\nwill not change the value of the register bit."]
pub type TaskdoorbellR = crate::FieldReader<u32>;
#[doc = "Field `TASKDOORBELL` writer - Command Queueing Task Doorbell\n\nSoftware shall configure TDLBA and TDLBAU, and enable CQE in\n\nCQCFG before using this register.\n\nWriting 1 to bit n of this register triggers CQE to start processing\n\nthe task encoded in slot n of the TDL.\n\nCQE always processes tasks in-order according to the order\n\nsubmitted to the list by CQTDBR write transactions.\n\nCQE processes Data Transfer tasks by reading the Task\n\nDescriptor and sending QUEUED_TASK_PARAMS (CMD44) and\n\nQUEUED_TASK_ADDRESS (CMD45) commands to the device.\n\nCQE processes DCMD tasks (in slot #31, when enabled) by\n\nreading the Task Descriptor, and generating the command\n\nencoded by its index and argument.\n\nThe corresponding bit is cleared to 0 by CQE in one of the\n\nfollowing events:\n\na. When a task execution is completed (with success or error)\n\nb. The task is cleared using CQTCLR register\n\nc. All tasks are cleared using CQCTL register\n\nd. CQE is disabled using CQCFG register\n\nSoftware may initiate multiple tasks at the same time (batch\n\nsubmission) by writing 1 to multiple bits of this register in the\n\nsame transaction.\n\nIn the case of batch submission:\n\nCQE shall process the tasks in order of the task index, starting\n\nwith the lowest index.\n\nIf one or more tasks in the batch are marked with QBR, the\n\nordering of execution will be based on said processing order.\n\nWriting 0 by software shall have no impact on the hardware, and\n\nwill not change the value of the register bit."]
pub type TaskdoorbellW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Queueing Task Doorbell\n\nSoftware shall configure TDLBA and TDLBAU, and enable CQE in\n\nCQCFG before using this register.\n\nWriting 1 to bit n of this register triggers CQE to start processing\n\nthe task encoded in slot n of the TDL.\n\nCQE always processes tasks in-order according to the order\n\nsubmitted to the list by CQTDBR write transactions.\n\nCQE processes Data Transfer tasks by reading the Task\n\nDescriptor and sending QUEUED_TASK_PARAMS (CMD44) and\n\nQUEUED_TASK_ADDRESS (CMD45) commands to the device.\n\nCQE processes DCMD tasks (in slot #31, when enabled) by\n\nreading the Task Descriptor, and generating the command\n\nencoded by its index and argument.\n\nThe corresponding bit is cleared to 0 by CQE in one of the\n\nfollowing events:\n\na. When a task execution is completed (with success or error)\n\nb. The task is cleared using CQTCLR register\n\nc. All tasks are cleared using CQCTL register\n\nd. CQE is disabled using CQCFG register\n\nSoftware may initiate multiple tasks at the same time (batch\n\nsubmission) by writing 1 to multiple bits of this register in the\n\nsame transaction.\n\nIn the case of batch submission:\n\nCQE shall process the tasks in order of the task index, starting\n\nwith the lowest index.\n\nIf one or more tasks in the batch are marked with QBR, the\n\nordering of execution will be based on said processing order.\n\nWriting 0 by software shall have no impact on the hardware, and\n\nwill not change the value of the register bit."]
    #[inline(always)]
    pub fn taskdoorbell(&self) -> TaskdoorbellR {
        TaskdoorbellR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Queueing Task Doorbell\n\nSoftware shall configure TDLBA and TDLBAU, and enable CQE in\n\nCQCFG before using this register.\n\nWriting 1 to bit n of this register triggers CQE to start processing\n\nthe task encoded in slot n of the TDL.\n\nCQE always processes tasks in-order according to the order\n\nsubmitted to the list by CQTDBR write transactions.\n\nCQE processes Data Transfer tasks by reading the Task\n\nDescriptor and sending QUEUED_TASK_PARAMS (CMD44) and\n\nQUEUED_TASK_ADDRESS (CMD45) commands to the device.\n\nCQE processes DCMD tasks (in slot #31, when enabled) by\n\nreading the Task Descriptor, and generating the command\n\nencoded by its index and argument.\n\nThe corresponding bit is cleared to 0 by CQE in one of the\n\nfollowing events:\n\na. When a task execution is completed (with success or error)\n\nb. The task is cleared using CQTCLR register\n\nc. All tasks are cleared using CQCTL register\n\nd. CQE is disabled using CQCFG register\n\nSoftware may initiate multiple tasks at the same time (batch\n\nsubmission) by writing 1 to multiple bits of this register in the\n\nsame transaction.\n\nIn the case of batch submission:\n\nCQE shall process the tasks in order of the task index, starting\n\nwith the lowest index.\n\nIf one or more tasks in the batch are marked with QBR, the\n\nordering of execution will be based on said processing order.\n\nWriting 0 by software shall have no impact on the hardware, and\n\nwill not change the value of the register bit."]
    #[inline(always)]
    #[must_use]
    pub fn taskdoorbell(&mut self) -> TaskdoorbellW<EmmccoreCqtdbSpec> {
        TaskdoorbellW::new(self, 0)
    }
}
#[doc = "Command queueing task doorbell register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqtdb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqtdb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreCqtdbSpec;
impl crate::RegisterSpec for EmmccoreCqtdbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_cqtdb::R`](R) reader structure"]
impl crate::Readable for EmmccoreCqtdbSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_cqtdb::W`](W) writer structure"]
impl crate::Writable for EmmccoreCqtdbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_CQTDB to value 0"]
impl crate::Resettable for EmmccoreCqtdbSpec {
    const RESET_VALUE: u32 = 0;
}
