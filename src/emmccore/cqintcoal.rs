#[doc = "Register `CQINTCOAL` reader"]
pub type R = crate::R<CqintcoalSpec>;
#[doc = "Register `CQINTCOAL` writer"]
pub type W = crate::W<CqintcoalSpec>;
#[doc = "Field `ICTOVAL` reader - Interrupt Coalescing Timeout Value (ICTOVAL):\n\nSoftware uses this field to configure the maximum time allowed\n\nbetween the completion of a task on the bus and the generation\n\nof an interrupt.\n\nTimer Operation: The timer is reset by software during the\n\ninterrupt service routine.\n\nIt starts running when a data transfer task with INT=0 is\n\ncompleted, after the timer was reset. When the timer reaches the\n\nvalue configured in ICTOVAL field it generates an interrupt and\n\nstops.\n\nThe timer's unit is equal to 1024 clock periods of the clock whose\n\nfrequency is specified in the Internal Timer Clock Frequency field\n\nCQCAP register.\n\nThe minimum value is 01h (1024 clock periods) and the\n\nmaximum value is 7Fh (127*1024 clock periods).\n\nFor example, a CQCAP field value of 0 indicates a 19.2 MHz clock\n\nfrequency (period = 52.08 ns). If the setting in ICTOVAL is 10h,\n\nthe calculated polling period is 16*1024*52.08 ns= 853.33 us.\n\nNOTE:When ICTOVAL is 0, the timer is not running, and timer-\n\nbased interrupts are not generated.In order to write to this field,\n\nthe ICTOVALWEN bit must be set at the same write operation."]
pub type IctovalR = crate::FieldReader;
#[doc = "Field `ICTOVAL` writer - Interrupt Coalescing Timeout Value (ICTOVAL):\n\nSoftware uses this field to configure the maximum time allowed\n\nbetween the completion of a task on the bus and the generation\n\nof an interrupt.\n\nTimer Operation: The timer is reset by software during the\n\ninterrupt service routine.\n\nIt starts running when a data transfer task with INT=0 is\n\ncompleted, after the timer was reset. When the timer reaches the\n\nvalue configured in ICTOVAL field it generates an interrupt and\n\nstops.\n\nThe timer's unit is equal to 1024 clock periods of the clock whose\n\nfrequency is specified in the Internal Timer Clock Frequency field\n\nCQCAP register.\n\nThe minimum value is 01h (1024 clock periods) and the\n\nmaximum value is 7Fh (127*1024 clock periods).\n\nFor example, a CQCAP field value of 0 indicates a 19.2 MHz clock\n\nfrequency (period = 52.08 ns). If the setting in ICTOVAL is 10h,\n\nthe calculated polling period is 16*1024*52.08 ns= 853.33 us.\n\nNOTE:When ICTOVAL is 0, the timer is not running, and timer-\n\nbased interrupts are not generated.In order to write to this field,\n\nthe ICTOVALWEN bit must be set at the same write operation."]
pub type IctovalW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ICTOVALWEN` writer - Interrupt Coalescing Timeout Value Write Enable:\n\nWhen software writes 1, the value ICTOVAL is updated with the\n\ncontents written at the same cycle.\n\nWhen software writes 0, the value in ICTOVAL is not updated.\n\nNOTE: Write operations to ICTOVAL are only allowed when the\n\ntask queue is empty."]
pub type IctovalwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICCTH` reader - Interrupt Coalescing Counter Threshold (ICCTH):\n\nSoftware uses this field to configure the number of task\n\ncompletions (only tasks withINT=0 in the Task Descriptor) which\n\nare required in order to generate an interrupt.\n\nCounter Operation: As data transfer tasks with INT=0 complete,\n\nthey are counted byCQE. The counter is reset by software during\n\nthe interrupt service routine.\n\nThe counter stops counting when it reaches the value configured\n\nin ICCTH.\n\nThe maximum allowed value is 31\n\nNOTE : When ICCTH is 0, task completions are not counted, and\n\ncounting-based interrupts are not generated.\n\nIn order to write to this field, the ICCTHWEN bit must be set at\n\nthe same write operation."]
pub type IccthR = crate::FieldReader;
#[doc = "Field `ICCTH` writer - Interrupt Coalescing Counter Threshold (ICCTH):\n\nSoftware uses this field to configure the number of task\n\ncompletions (only tasks withINT=0 in the Task Descriptor) which\n\nare required in order to generate an interrupt.\n\nCounter Operation: As data transfer tasks with INT=0 complete,\n\nthey are counted byCQE. The counter is reset by software during\n\nthe interrupt service routine.\n\nThe counter stops counting when it reaches the value configured\n\nin ICCTH.\n\nThe maximum allowed value is 31\n\nNOTE : When ICCTH is 0, task completions are not counted, and\n\ncounting-based interrupts are not generated.\n\nIn order to write to this field, the ICCTHWEN bit must be set at\n\nthe same write operation."]
pub type IccthW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ICCTHWEN` writer - Interrupt Coalescing Counter Threshold Write Enable:\n\nWhen software writes 1, the value ICCTH is updated with the\n\ncontents written at the same cycle.\n\nWhen software writes 0, the value in ICCTH is not updated.\n\nNOTE: Write operations to ICCTH are only allowed when the task\n\nqueue is empty."]
pub type IccthwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` writer - Counter and Timer Reset(ICCTR):\n\nWhen host driver writes 1, the interrupt coalescing timer and\n\ncounter are reset"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt Coalescing Status Bit :\n\nThis bit indicates to software whether any tasks (with INT=0)\n\nhave completed and counted towards interrupt coalescing (i.e.,\n\nICSB is set ifand only if IC counter > 0).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icsb {
    #[doc = "1: At least one task completion has been counted (IC counter >0)"]
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
#[doc = "Field `ICSB` reader - Interrupt Coalescing Status Bit :\n\nThis bit indicates to software whether any tasks (with INT=0)\n\nhave completed and counted towards interrupt coalescing (i.e.,\n\nICSB is set ifand only if IC counter > 0)."]
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
    #[doc = "At least one task completion has been counted (IC counter >0)"]
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
#[doc = "Field `INTCOALENA` reader - Interrupt Coalescing Enable/Disable:\n\nWhen set to 0 by software, command responses are neither\n\ncounted nor timed. Interrupts are still triggered by completion of\n\ntasks with INT=1 in the Task Descriptor.\n\nWhen set to 1, the interrupt coalescing mechanism is enabled\n\nand coalesced interrupts are generated."]
pub type IntcoalenaR = crate::BitReader;
#[doc = "Field `INTCOALENA` writer - Interrupt Coalescing Enable/Disable:\n\nWhen set to 0 by software, command responses are neither\n\ncounted nor timed. Interrupts are still triggered by completion of\n\ntasks with INT=1 in the Task Descriptor.\n\nWhen set to 1, the interrupt coalescing mechanism is enabled\n\nand coalesced interrupts are generated."]
pub type IntcoalenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Interrupt Coalescing Timeout Value (ICTOVAL):\n\nSoftware uses this field to configure the maximum time allowed\n\nbetween the completion of a task on the bus and the generation\n\nof an interrupt.\n\nTimer Operation: The timer is reset by software during the\n\ninterrupt service routine.\n\nIt starts running when a data transfer task with INT=0 is\n\ncompleted, after the timer was reset. When the timer reaches the\n\nvalue configured in ICTOVAL field it generates an interrupt and\n\nstops.\n\nThe timer's unit is equal to 1024 clock periods of the clock whose\n\nfrequency is specified in the Internal Timer Clock Frequency field\n\nCQCAP register.\n\nThe minimum value is 01h (1024 clock periods) and the\n\nmaximum value is 7Fh (127*1024 clock periods).\n\nFor example, a CQCAP field value of 0 indicates a 19.2 MHz clock\n\nfrequency (period = 52.08 ns). If the setting in ICTOVAL is 10h,\n\nthe calculated polling period is 16*1024*52.08 ns= 853.33 us.\n\nNOTE:When ICTOVAL is 0, the timer is not running, and timer-\n\nbased interrupts are not generated.In order to write to this field,\n\nthe ICTOVALWEN bit must be set at the same write operation."]
    #[inline(always)]
    pub fn ictoval(&self) -> IctovalR {
        IctovalR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:12 - Interrupt Coalescing Counter Threshold (ICCTH):\n\nSoftware uses this field to configure the number of task\n\ncompletions (only tasks withINT=0 in the Task Descriptor) which\n\nare required in order to generate an interrupt.\n\nCounter Operation: As data transfer tasks with INT=0 complete,\n\nthey are counted byCQE. The counter is reset by software during\n\nthe interrupt service routine.\n\nThe counter stops counting when it reaches the value configured\n\nin ICCTH.\n\nThe maximum allowed value is 31\n\nNOTE : When ICCTH is 0, task completions are not counted, and\n\ncounting-based interrupts are not generated.\n\nIn order to write to this field, the ICCTHWEN bit must be set at\n\nthe same write operation."]
    #[inline(always)]
    pub fn iccth(&self) -> IccthR {
        IccthR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 20 - Interrupt Coalescing Status Bit :\n\nThis bit indicates to software whether any tasks (with INT=0)\n\nhave completed and counted towards interrupt coalescing (i.e.,\n\nICSB is set ifand only if IC counter > 0)."]
    #[inline(always)]
    pub fn icsb(&self) -> IcsbR {
        IcsbR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Coalescing Enable/Disable:\n\nWhen set to 0 by software, command responses are neither\n\ncounted nor timed. Interrupts are still triggered by completion of\n\ntasks with INT=1 in the Task Descriptor.\n\nWhen set to 1, the interrupt coalescing mechanism is enabled\n\nand coalesced interrupts are generated."]
    #[inline(always)]
    pub fn intcoalena(&self) -> IntcoalenaR {
        IntcoalenaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Interrupt Coalescing Timeout Value (ICTOVAL):\n\nSoftware uses this field to configure the maximum time allowed\n\nbetween the completion of a task on the bus and the generation\n\nof an interrupt.\n\nTimer Operation: The timer is reset by software during the\n\ninterrupt service routine.\n\nIt starts running when a data transfer task with INT=0 is\n\ncompleted, after the timer was reset. When the timer reaches the\n\nvalue configured in ICTOVAL field it generates an interrupt and\n\nstops.\n\nThe timer's unit is equal to 1024 clock periods of the clock whose\n\nfrequency is specified in the Internal Timer Clock Frequency field\n\nCQCAP register.\n\nThe minimum value is 01h (1024 clock periods) and the\n\nmaximum value is 7Fh (127*1024 clock periods).\n\nFor example, a CQCAP field value of 0 indicates a 19.2 MHz clock\n\nfrequency (period = 52.08 ns). If the setting in ICTOVAL is 10h,\n\nthe calculated polling period is 16*1024*52.08 ns= 853.33 us.\n\nNOTE:When ICTOVAL is 0, the timer is not running, and timer-\n\nbased interrupts are not generated.In order to write to this field,\n\nthe ICTOVALWEN bit must be set at the same write operation."]
    #[inline(always)]
    #[must_use]
    pub fn ictoval(&mut self) -> IctovalW<CqintcoalSpec> {
        IctovalW::new(self, 0)
    }
    #[doc = "Bit 7 - Interrupt Coalescing Timeout Value Write Enable:\n\nWhen software writes 1, the value ICTOVAL is updated with the\n\ncontents written at the same cycle.\n\nWhen software writes 0, the value in ICTOVAL is not updated.\n\nNOTE: Write operations to ICTOVAL are only allowed when the\n\ntask queue is empty."]
    #[inline(always)]
    #[must_use]
    pub fn ictovalwen(&mut self) -> IctovalwenW<CqintcoalSpec> {
        IctovalwenW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Interrupt Coalescing Counter Threshold (ICCTH):\n\nSoftware uses this field to configure the number of task\n\ncompletions (only tasks withINT=0 in the Task Descriptor) which\n\nare required in order to generate an interrupt.\n\nCounter Operation: As data transfer tasks with INT=0 complete,\n\nthey are counted byCQE. The counter is reset by software during\n\nthe interrupt service routine.\n\nThe counter stops counting when it reaches the value configured\n\nin ICCTH.\n\nThe maximum allowed value is 31\n\nNOTE : When ICCTH is 0, task completions are not counted, and\n\ncounting-based interrupts are not generated.\n\nIn order to write to this field, the ICCTHWEN bit must be set at\n\nthe same write operation."]
    #[inline(always)]
    #[must_use]
    pub fn iccth(&mut self) -> IccthW<CqintcoalSpec> {
        IccthW::new(self, 8)
    }
    #[doc = "Bit 15 - Interrupt Coalescing Counter Threshold Write Enable:\n\nWhen software writes 1, the value ICCTH is updated with the\n\ncontents written at the same cycle.\n\nWhen software writes 0, the value in ICCTH is not updated.\n\nNOTE: Write operations to ICCTH are only allowed when the task\n\nqueue is empty."]
    #[inline(always)]
    #[must_use]
    pub fn iccthwen(&mut self) -> IccthwenW<CqintcoalSpec> {
        IccthwenW::new(self, 15)
    }
    #[doc = "Bit 16 - Counter and Timer Reset(ICCTR):\n\nWhen host driver writes 1, the interrupt coalescing timer and\n\ncounter are reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<CqintcoalSpec> {
        ResetW::new(self, 16)
    }
    #[doc = "Bit 31 - Interrupt Coalescing Enable/Disable:\n\nWhen set to 0 by software, command responses are neither\n\ncounted nor timed. Interrupts are still triggered by completion of\n\ntasks with INT=1 in the Task Descriptor.\n\nWhen set to 1, the interrupt coalescing mechanism is enabled\n\nand coalesced interrupts are generated."]
    #[inline(always)]
    #[must_use]
    pub fn intcoalena(&mut self) -> IntcoalenaW<CqintcoalSpec> {
        IntcoalenaW::new(self, 31)
    }
}
#[doc = "Command queueing interrupt coalescing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqintcoal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqintcoal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqintcoalSpec;
impl crate::RegisterSpec for CqintcoalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqintcoal::R`](R) reader structure"]
impl crate::Readable for CqintcoalSpec {}
#[doc = "`write(|w| ..)` method takes [`cqintcoal::W`](W) writer structure"]
impl crate::Writable for CqintcoalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQINTCOAL to value 0"]
impl crate::Resettable for CqintcoalSpec {
    const RESET_VALUE: u32 = 0;
}
