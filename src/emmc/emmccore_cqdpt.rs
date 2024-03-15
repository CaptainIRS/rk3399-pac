#[doc = "Register `EMMCCORE_CQDPT` reader"]
pub type R = crate::R<EmmccoreCqdptSpec>;
#[doc = "Field `DPT` reader - Device Pending Tasks Bit n of this register is set if and only if QUEUED_TASK_PARAMS (CMD44) and QUEUED_TASK_ADDRESS (CMD45) were sent for this specific task and if this task hasn’t been executed yet. CQE shall set this bit after receiving a successful response for CMD45. CQE shall clear this bit after the task has completed execution. Software needs to read this register in the task-discard procedure, when the controlleris halted, to determine if the task is queued in the device. If the task is queued, the driver sends a CMDQ_TASK_MGMT (CMD48) to the device ordering it to discard the task. Then software clears the task in the CQE. Only then the software orders CQE to resume its operation using CQCTL register."]
pub type DptR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Device Pending Tasks Bit n of this register is set if and only if QUEUED_TASK_PARAMS (CMD44) and QUEUED_TASK_ADDRESS (CMD45) were sent for this specific task and if this task hasn’t been executed yet. CQE shall set this bit after receiving a successful response for CMD45. CQE shall clear this bit after the task has completed execution. Software needs to read this register in the task-discard procedure, when the controlleris halted, to determine if the task is queued in the device. If the task is queued, the driver sends a CMDQ_TASK_MGMT (CMD48) to the device ordering it to discard the task. Then software clears the task in the CQE. Only then the software orders CQE to resume its operation using CQCTL register."]
    #[inline(always)]
    pub fn dpt(&self) -> DptR {
        DptR::new(self.bits)
    }
}
#[doc = "Command queueing device pending tasks register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqdpt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreCqdptSpec;
impl crate::RegisterSpec for EmmccoreCqdptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_cqdpt::R`](R) reader structure"]
impl crate::Readable for EmmccoreCqdptSpec {}
#[doc = "`reset()` method sets EMMCCORE_CQDPT to value 0"]
impl crate::Resettable for EmmccoreCqdptSpec {
    const RESET_VALUE: u32 = 0;
}
