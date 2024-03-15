#[doc = "Register `EMMCCORE_CQCTRL` reader"]
pub type R = crate::R<EmmccoreCqctrlSpec>;
#[doc = "Register `EMMCCORE_CQCTRL` writer"]
pub type W = crate::W<EmmccoreCqctrlSpec>;
#[doc = "Field `HALT` reader - Host software shall write 1 to the bit when it wants to acquire software control over the eMMC bus and disable CQE from issuing commands on the bus. For example, issuing a Discard Task command (CMDQ_TASK_MGMT). When software writes 1, CQE shall complete the ongoing task if such a task is in progress. Once the task is completed and CQE is in idle state, CQE shall not issue new commands and shall indicate so to software by setting this bit to 1. Software may poll on this bit until it is set to 1, and may only then send commands on the eMMC bus. In order to exit halt state (i.e. resume CQE activity), software shall clear this bit (write 0). Writing 0 when the value is already 0 shall have no effect."]
pub type HaltR = crate::BitReader;
#[doc = "Field `HALT` writer - Host software shall write 1 to the bit when it wants to acquire software control over the eMMC bus and disable CQE from issuing commands on the bus. For example, issuing a Discard Task command (CMDQ_TASK_MGMT). When software writes 1, CQE shall complete the ongoing task if such a task is in progress. Once the task is completed and CQE is in idle state, CQE shall not issue new commands and shall indicate so to software by setting this bit to 1. Software may poll on this bit until it is set to 1, and may only then send commands on the eMMC bus. In order to exit halt state (i.e. resume CQE activity), software shall clear this bit (write 0). Writing 0 when the value is already 0 shall have no effect."]
pub type HaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARALLTASKS` reader - Software shall write 1 this bit when it wants to clear all the tasks sent to the device. This bit can only be written when CQE is in halt state (i.e. Halt bit is 1). When software writes 1, the value of the register is updated to 1, and CQE shall reset CQTDBR register and all other context information for all unfinished tasks. Then CQE will clear this bit. Software should poll on this bit until it is set to back 0 and may then resume normal operation, by clearing the Halt bit. CQE does not communicate to the device that the tasks were cleared. It is software's responsibility to order the device to discard the tasks in its queue using CMDQ_TASK_MGMT command. Writing 0 to this register shall have no effect."]
pub type ClearalltasksR = crate::BitReader;
#[doc = "Field `CLEARALLTASKS` writer - Software shall write 1 this bit when it wants to clear all the tasks sent to the device. This bit can only be written when CQE is in halt state (i.e. Halt bit is 1). When software writes 1, the value of the register is updated to 1, and CQE shall reset CQTDBR register and all other context information for all unfinished tasks. Then CQE will clear this bit. Software should poll on this bit until it is set to back 0 and may then resume normal operation, by clearing the Halt bit. CQE does not communicate to the device that the tasks were cleared. It is software's responsibility to order the device to discard the tasks in its queue using CMDQ_TASK_MGMT command. Writing 0 to this register shall have no effect."]
pub type ClearalltasksW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Host software shall write 1 to the bit when it wants to acquire software control over the eMMC bus and disable CQE from issuing commands on the bus. For example, issuing a Discard Task command (CMDQ_TASK_MGMT). When software writes 1, CQE shall complete the ongoing task if such a task is in progress. Once the task is completed and CQE is in idle state, CQE shall not issue new commands and shall indicate so to software by setting this bit to 1. Software may poll on this bit until it is set to 1, and may only then send commands on the eMMC bus. In order to exit halt state (i.e. resume CQE activity), software shall clear this bit (write 0). Writing 0 when the value is already 0 shall have no effect."]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Software shall write 1 this bit when it wants to clear all the tasks sent to the device. This bit can only be written when CQE is in halt state (i.e. Halt bit is 1). When software writes 1, the value of the register is updated to 1, and CQE shall reset CQTDBR register and all other context information for all unfinished tasks. Then CQE will clear this bit. Software should poll on this bit until it is set to back 0 and may then resume normal operation, by clearing the Halt bit. CQE does not communicate to the device that the tasks were cleared. It is software's responsibility to order the device to discard the tasks in its queue using CMDQ_TASK_MGMT command. Writing 0 to this register shall have no effect."]
    #[inline(always)]
    pub fn clearalltasks(&self) -> ClearalltasksR {
        ClearalltasksR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Host software shall write 1 to the bit when it wants to acquire software control over the eMMC bus and disable CQE from issuing commands on the bus. For example, issuing a Discard Task command (CMDQ_TASK_MGMT). When software writes 1, CQE shall complete the ongoing task if such a task is in progress. Once the task is completed and CQE is in idle state, CQE shall not issue new commands and shall indicate so to software by setting this bit to 1. Software may poll on this bit until it is set to 1, and may only then send commands on the eMMC bus. In order to exit halt state (i.e. resume CQE activity), software shall clear this bit (write 0). Writing 0 when the value is already 0 shall have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HaltW<EmmccoreCqctrlSpec> {
        HaltW::new(self, 0)
    }
    #[doc = "Bit 8 - Software shall write 1 this bit when it wants to clear all the tasks sent to the device. This bit can only be written when CQE is in halt state (i.e. Halt bit is 1). When software writes 1, the value of the register is updated to 1, and CQE shall reset CQTDBR register and all other context information for all unfinished tasks. Then CQE will clear this bit. Software should poll on this bit until it is set to back 0 and may then resume normal operation, by clearing the Halt bit. CQE does not communicate to the device that the tasks were cleared. It is software's responsibility to order the device to discard the tasks in its queue using CMDQ_TASK_MGMT command. Writing 0 to this register shall have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn clearalltasks(&mut self) -> ClearalltasksW<EmmccoreCqctrlSpec> {
        ClearalltasksW::new(self, 8)
    }
}
#[doc = "Command queueing control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreCqctrlSpec;
impl crate::RegisterSpec for EmmccoreCqctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_cqctrl::R`](R) reader structure"]
impl crate::Readable for EmmccoreCqctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_cqctrl::W`](W) writer structure"]
impl crate::Writable for EmmccoreCqctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_CQCTRL to value 0"]
impl crate::Resettable for EmmccoreCqctrlSpec {
    const RESET_VALUE: u32 = 0;
}
