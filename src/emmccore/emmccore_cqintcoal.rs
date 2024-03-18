#[doc = "Register `EMMCCORE_CQINTCOAL` reader"]
pub type R = crate::R<EmmccoreCqintcoalSpec>;
#[doc = "Register `EMMCCORE_CQINTCOAL` writer"]
pub type W = crate::W<EmmccoreCqintcoalSpec>;
#[doc = "Field `ICTOVAL` reader - Interrupt Coalescing Timeout Value (ICTOVAL): Software uses this field to configure the maximum time allowed between the completion of a task on the bus and the generation of an interrupt. Timer Operation: The timer is reset by software during the interrupt service routine. It starts running when a data transfer task with INT=0 is completed, after the timer was reset. When the timer reaches the value configured in ICTOVAL field it generates an interrupt and stops. The timer's unit is equal to 1024 clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register. The minimum value is 01h (1024 clock periods) and the maximum value is 7Fh (127*1024 clock periods). For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency (period = 52.08 ns). If the setting in ICTOVAL is 10h, the calculated polling period is 16*1024*52.08 ns= 853.33 us. NOTE:When ICTOVAL is 0, the timer is not running, and timer- based interrupts are not generated.In order to write to this field, the ICTOVALWEN bit must be set at the same write operation."]
pub type IctovalR = crate::FieldReader;
#[doc = "Field `ICTOVAL` writer - Interrupt Coalescing Timeout Value (ICTOVAL): Software uses this field to configure the maximum time allowed between the completion of a task on the bus and the generation of an interrupt. Timer Operation: The timer is reset by software during the interrupt service routine. It starts running when a data transfer task with INT=0 is completed, after the timer was reset. When the timer reaches the value configured in ICTOVAL field it generates an interrupt and stops. The timer's unit is equal to 1024 clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register. The minimum value is 01h (1024 clock periods) and the maximum value is 7Fh (127*1024 clock periods). For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency (period = 52.08 ns). If the setting in ICTOVAL is 10h, the calculated polling period is 16*1024*52.08 ns= 853.33 us. NOTE:When ICTOVAL is 0, the timer is not running, and timer- based interrupts are not generated.In order to write to this field, the ICTOVALWEN bit must be set at the same write operation."]
pub type IctovalW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ICTOVALWEN` writer - Interrupt Coalescing Timeout Value Write Enable: When software writes 1, the value ICTOVAL is updated with the contents written at the same cycle. When software writes 0, the value in ICTOVAL is not updated. NOTE: Write operations to ICTOVAL are only allowed when the task queue is empty."]
pub type IctovalwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICCTH` reader - Interrupt Coalescing Counter Threshold (ICCTH): Software uses this field to configure the number of task completions (only tasks withINT=0 in the Task Descriptor) which are required in order to generate an interrupt. Counter Operation: As data transfer tasks with INT=0 complete, they are counted byCQE. The counter is reset by software during the interrupt service routine. The counter stops counting when it reaches the value configured in ICCTH. The maximum allowed value is 31 NOTE : When ICCTH is 0, task completions are not counted, and counting-based interrupts are not generated. In order to write to this field, the ICCTHWEN bit must be set at the same write operation."]
pub type IccthR = crate::FieldReader;
#[doc = "Field `ICCTH` writer - Interrupt Coalescing Counter Threshold (ICCTH): Software uses this field to configure the number of task completions (only tasks withINT=0 in the Task Descriptor) which are required in order to generate an interrupt. Counter Operation: As data transfer tasks with INT=0 complete, they are counted byCQE. The counter is reset by software during the interrupt service routine. The counter stops counting when it reaches the value configured in ICCTH. The maximum allowed value is 31 NOTE : When ICCTH is 0, task completions are not counted, and counting-based interrupts are not generated. In order to write to this field, the ICCTHWEN bit must be set at the same write operation."]
pub type IccthW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ICCTHWEN` writer - Interrupt Coalescing Counter Threshold Write Enable: When software writes 1, the value ICCTH is updated with the contents written at the same cycle. When software writes 0, the value in ICCTH is not updated. NOTE: Write operations to ICCTH are only allowed when the task queue is empty."]
pub type IccthwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` writer - Counter and Timer Reset(ICCTR): When host driver writes 1, the interrupt coalescing timer and counter are reset"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt Coalescing Status Bit : This bit indicates to software whether any tasks (with INT=0) have completed and counted towards interrupt coalescing (i.e., ICSB is set ifand only if IC counter > 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icsb {
    #[doc = "1: No task completions have occurred since last counter reset (IC counter =0)"]
    B1 = 1,
    #[doc = "0: No task completions have occurred since last counter reset (IC counter =0)"]
    B0 = 0,
}
impl From<Icsb> for bool {
    #[inline(always)]
    fn from(variant: Icsb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICSB` reader - Interrupt Coalescing Status Bit : This bit indicates to software whether any tasks (with INT=0) have completed and counted towards interrupt coalescing (i.e., ICSB is set ifand only if IC counter > 0)."]
pub type IcsbR = crate::BitReader<Icsb>;
impl IcsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icsb {
        match self.bits {
            true => Icsb::B1,
            false => Icsb::B0,
        }
    }
    #[doc = "No task completions have occurred since last counter reset (IC counter =0)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Icsb::B1
    }
    #[doc = "No task completions have occurred since last counter reset (IC counter =0)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Icsb::B0
    }
}
#[doc = "Field `INTCOALENA` reader - Interrupt Coalescing Enable/Disable: When set to 0 by software, command responses are neither counted nor timed. Interrupts are still triggered by completion of tasks with INT=1 in the Task Descriptor. When set to 1, the interrupt coalescing mechanism is enabled and coalesced interrupts are generated."]
pub type IntcoalenaR = crate::BitReader;
#[doc = "Field `INTCOALENA` writer - Interrupt Coalescing Enable/Disable: When set to 0 by software, command responses are neither counted nor timed. Interrupts are still triggered by completion of tasks with INT=1 in the Task Descriptor. When set to 1, the interrupt coalescing mechanism is enabled and coalesced interrupts are generated."]
pub type IntcoalenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Interrupt Coalescing Timeout Value (ICTOVAL): Software uses this field to configure the maximum time allowed between the completion of a task on the bus and the generation of an interrupt. Timer Operation: The timer is reset by software during the interrupt service routine. It starts running when a data transfer task with INT=0 is completed, after the timer was reset. When the timer reaches the value configured in ICTOVAL field it generates an interrupt and stops. The timer's unit is equal to 1024 clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register. The minimum value is 01h (1024 clock periods) and the maximum value is 7Fh (127*1024 clock periods). For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency (period = 52.08 ns). If the setting in ICTOVAL is 10h, the calculated polling period is 16*1024*52.08 ns= 853.33 us. NOTE:When ICTOVAL is 0, the timer is not running, and timer- based interrupts are not generated.In order to write to this field, the ICTOVALWEN bit must be set at the same write operation."]
    #[inline(always)]
    pub fn ictoval(&self) -> IctovalR {
        IctovalR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:12 - Interrupt Coalescing Counter Threshold (ICCTH): Software uses this field to configure the number of task completions (only tasks withINT=0 in the Task Descriptor) which are required in order to generate an interrupt. Counter Operation: As data transfer tasks with INT=0 complete, they are counted byCQE. The counter is reset by software during the interrupt service routine. The counter stops counting when it reaches the value configured in ICCTH. The maximum allowed value is 31 NOTE : When ICCTH is 0, task completions are not counted, and counting-based interrupts are not generated. In order to write to this field, the ICCTHWEN bit must be set at the same write operation."]
    #[inline(always)]
    pub fn iccth(&self) -> IccthR {
        IccthR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 20 - Interrupt Coalescing Status Bit : This bit indicates to software whether any tasks (with INT=0) have completed and counted towards interrupt coalescing (i.e., ICSB is set ifand only if IC counter > 0)."]
    #[inline(always)]
    pub fn icsb(&self) -> IcsbR {
        IcsbR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Coalescing Enable/Disable: When set to 0 by software, command responses are neither counted nor timed. Interrupts are still triggered by completion of tasks with INT=1 in the Task Descriptor. When set to 1, the interrupt coalescing mechanism is enabled and coalesced interrupts are generated."]
    #[inline(always)]
    pub fn intcoalena(&self) -> IntcoalenaR {
        IntcoalenaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Interrupt Coalescing Timeout Value (ICTOVAL): Software uses this field to configure the maximum time allowed between the completion of a task on the bus and the generation of an interrupt. Timer Operation: The timer is reset by software during the interrupt service routine. It starts running when a data transfer task with INT=0 is completed, after the timer was reset. When the timer reaches the value configured in ICTOVAL field it generates an interrupt and stops. The timer's unit is equal to 1024 clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register. The minimum value is 01h (1024 clock periods) and the maximum value is 7Fh (127*1024 clock periods). For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency (period = 52.08 ns). If the setting in ICTOVAL is 10h, the calculated polling period is 16*1024*52.08 ns= 853.33 us. NOTE:When ICTOVAL is 0, the timer is not running, and timer- based interrupts are not generated.In order to write to this field, the ICTOVALWEN bit must be set at the same write operation."]
    #[inline(always)]
    #[must_use]
    pub fn ictoval(&mut self) -> IctovalW<EmmccoreCqintcoalSpec> {
        IctovalW::new(self, 0)
    }
    #[doc = "Bit 7 - Interrupt Coalescing Timeout Value Write Enable: When software writes 1, the value ICTOVAL is updated with the contents written at the same cycle. When software writes 0, the value in ICTOVAL is not updated. NOTE: Write operations to ICTOVAL are only allowed when the task queue is empty."]
    #[inline(always)]
    #[must_use]
    pub fn ictovalwen(&mut self) -> IctovalwenW<EmmccoreCqintcoalSpec> {
        IctovalwenW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Interrupt Coalescing Counter Threshold (ICCTH): Software uses this field to configure the number of task completions (only tasks withINT=0 in the Task Descriptor) which are required in order to generate an interrupt. Counter Operation: As data transfer tasks with INT=0 complete, they are counted byCQE. The counter is reset by software during the interrupt service routine. The counter stops counting when it reaches the value configured in ICCTH. The maximum allowed value is 31 NOTE : When ICCTH is 0, task completions are not counted, and counting-based interrupts are not generated. In order to write to this field, the ICCTHWEN bit must be set at the same write operation."]
    #[inline(always)]
    #[must_use]
    pub fn iccth(&mut self) -> IccthW<EmmccoreCqintcoalSpec> {
        IccthW::new(self, 8)
    }
    #[doc = "Bit 15 - Interrupt Coalescing Counter Threshold Write Enable: When software writes 1, the value ICCTH is updated with the contents written at the same cycle. When software writes 0, the value in ICCTH is not updated. NOTE: Write operations to ICCTH are only allowed when the task queue is empty."]
    #[inline(always)]
    #[must_use]
    pub fn iccthwen(&mut self) -> IccthwenW<EmmccoreCqintcoalSpec> {
        IccthwenW::new(self, 15)
    }
    #[doc = "Bit 16 - Counter and Timer Reset(ICCTR): When host driver writes 1, the interrupt coalescing timer and counter are reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<EmmccoreCqintcoalSpec> {
        ResetW::new(self, 16)
    }
    #[doc = "Bit 31 - Interrupt Coalescing Enable/Disable: When set to 0 by software, command responses are neither counted nor timed. Interrupts are still triggered by completion of tasks with INT=1 in the Task Descriptor. When set to 1, the interrupt coalescing mechanism is enabled and coalesced interrupts are generated."]
    #[inline(always)]
    #[must_use]
    pub fn intcoalena(&mut self) -> IntcoalenaW<EmmccoreCqintcoalSpec> {
        IntcoalenaW::new(self, 31)
    }
}
#[doc = "Command queueing interrupt coalescing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqintcoal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqintcoal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreCqintcoalSpec;
impl crate::RegisterSpec for EmmccoreCqintcoalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_cqintcoal::R`](R) reader structure"]
impl crate::Readable for EmmccoreCqintcoalSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_cqintcoal::W`](W) writer structure"]
impl crate::Writable for EmmccoreCqintcoalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_CQINTCOAL to value 0"]
impl crate::Resettable for EmmccoreCqintcoalSpec {
    const RESET_VALUE: u32 = 0;
}
