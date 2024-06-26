#[doc = "Register `CQCFG` reader"]
pub type R = crate::R<CqcfgSpec>;
#[doc = "Register `CQCFG` writer"]
pub type W = crate::W<CqcfgSpec>;
#[doc = "Field `CQENA` reader - Command Queueing Enable\n\nSoftware shall write 1 this bit when in order to enable command\n\nqueueing mode (i.e. enable CQE).\n\nWhen this bit is 0, CQE is disabled and software controls the\n\neMMC bus using the legacy eMMC host controller.\n\nBefore software writes 1 to this bit, software shall verify that the\n\neMMC host controller is in idle state and there are no commands\n\nor data transfers ongoing.\n\nWhen software wants to exit command queueing mode, it shall\n\nclear all previous tasks if such exist before setting this bit to 0."]
pub type CqenaR = crate::BitReader;
#[doc = "Field `CQENA` writer - Command Queueing Enable\n\nSoftware shall write 1 this bit when in order to enable command\n\nqueueing mode (i.e. enable CQE).\n\nWhen this bit is 0, CQE is disabled and software controls the\n\neMMC bus using the legacy eMMC host controller.\n\nBefore software writes 1 to this bit, software shall verify that the\n\neMMC host controller is in idle state and there are no commands\n\nor data transfers ongoing.\n\nWhen software wants to exit command queueing mode, it shall\n\nclear all previous tasks if such exist before setting this bit to 0."]
pub type CqenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "This bit indicates whether the task descriptor size is 128 bits or\n\n64 bits as detailed in Data Structures section. This bit can only be\n\nconfigured when Command Queueing Enable bit is 0 (command\n\nqueueing is disabled)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Taskdescriptorsize {
    #[doc = "1: Task descriptor size is 128 bits"]
    B1 = 1,
    #[doc = "0: Task descriptor size is 64 bits"]
    B0 = 0,
}
impl From<Taskdescriptorsize> for bool {
    #[inline(always)]
    fn from(variant: Taskdescriptorsize) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKDESCRIPTORSIZE` reader - This bit indicates whether the task descriptor size is 128 bits or\n\n64 bits as detailed in Data Structures section. This bit can only be\n\nconfigured when Command Queueing Enable bit is 0 (command\n\nqueueing is disabled)"]
pub type TaskdescriptorsizeR = crate::BitReader<Taskdescriptorsize>;
impl TaskdescriptorsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Taskdescriptorsize {
        match self.bits {
            true => Taskdescriptorsize::B1,
            false => Taskdescriptorsize::B0,
        }
    }
    #[doc = "Task descriptor size is 128 bits"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Taskdescriptorsize::B1
    }
    #[doc = "Task descriptor size is 64 bits"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Taskdescriptorsize::B0
    }
}
#[doc = "Field `TASKDESCRIPTORSIZE` writer - This bit indicates whether the task descriptor size is 128 bits or\n\n64 bits as detailed in Data Structures section. This bit can only be\n\nconfigured when Command Queueing Enable bit is 0 (command\n\nqueueing is disabled)"]
pub type TaskdescriptorsizeW<'a, REG> = crate::BitWriter<'a, REG, Taskdescriptorsize>;
impl<'a, REG> TaskdescriptorsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task descriptor size is 128 bits"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Taskdescriptorsize::B1)
    }
    #[doc = "Task descriptor size is 64 bits"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Taskdescriptorsize::B0)
    }
}
#[doc = "Direct Command (DCMD) Enable\n\nThis bit indicates to the hardware whether the Task Descriptor in\n\nslot #31 of the TDL is a Data Transfer Task Descriptor, or a\n\nDirect Command Task Descriptor.\n\nCQE uses this bit when a task is issued in slot #31, to determine\n\nhow to decode the Task Descriptor.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcmdena {
    #[doc = "1: Task descriptor in slot #31 is a DCMD Task Descriptor"]
    B1 = 1,
    #[doc = "0: Task descriptor in slot #31 is a Data Transfer Task Descriptor"]
    B0 = 0,
}
impl From<Dcmdena> for bool {
    #[inline(always)]
    fn from(variant: Dcmdena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMDENA` reader - Direct Command (DCMD) Enable\n\nThis bit indicates to the hardware whether the Task Descriptor in\n\nslot #31 of the TDL is a Data Transfer Task Descriptor, or a\n\nDirect Command Task Descriptor.\n\nCQE uses this bit when a task is issued in slot #31, to determine\n\nhow to decode the Task Descriptor."]
pub type DcmdenaR = crate::BitReader<Dcmdena>;
impl DcmdenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcmdena {
        match self.bits {
            true => Dcmdena::B1,
            false => Dcmdena::B0,
        }
    }
    #[doc = "Task descriptor in slot #31 is a DCMD Task Descriptor"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dcmdena::B1
    }
    #[doc = "Task descriptor in slot #31 is a Data Transfer Task Descriptor"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dcmdena::B0
    }
}
#[doc = "Field `DCMDENA` writer - Direct Command (DCMD) Enable\n\nThis bit indicates to the hardware whether the Task Descriptor in\n\nslot #31 of the TDL is a Data Transfer Task Descriptor, or a\n\nDirect Command Task Descriptor.\n\nCQE uses this bit when a task is issued in slot #31, to determine\n\nhow to decode the Task Descriptor."]
pub type DcmdenaW<'a, REG> = crate::BitWriter<'a, REG, Dcmdena>;
impl<'a, REG> DcmdenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task descriptor in slot #31 is a DCMD Task Descriptor"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcmdena::B1)
    }
    #[doc = "Task descriptor in slot #31 is a Data Transfer Task Descriptor"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcmdena::B0)
    }
}
impl R {
    #[doc = "Bit 0 - Command Queueing Enable\n\nSoftware shall write 1 this bit when in order to enable command\n\nqueueing mode (i.e. enable CQE).\n\nWhen this bit is 0, CQE is disabled and software controls the\n\neMMC bus using the legacy eMMC host controller.\n\nBefore software writes 1 to this bit, software shall verify that the\n\neMMC host controller is in idle state and there are no commands\n\nor data transfers ongoing.\n\nWhen software wants to exit command queueing mode, it shall\n\nclear all previous tasks if such exist before setting this bit to 0."]
    #[inline(always)]
    pub fn cqena(&self) -> CqenaR {
        CqenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit indicates whether the task descriptor size is 128 bits or\n\n64 bits as detailed in Data Structures section. This bit can only be\n\nconfigured when Command Queueing Enable bit is 0 (command\n\nqueueing is disabled)"]
    #[inline(always)]
    pub fn taskdescriptorsize(&self) -> TaskdescriptorsizeR {
        TaskdescriptorsizeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Direct Command (DCMD) Enable\n\nThis bit indicates to the hardware whether the Task Descriptor in\n\nslot #31 of the TDL is a Data Transfer Task Descriptor, or a\n\nDirect Command Task Descriptor.\n\nCQE uses this bit when a task is issued in slot #31, to determine\n\nhow to decode the Task Descriptor."]
    #[inline(always)]
    pub fn dcmdena(&self) -> DcmdenaR {
        DcmdenaR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Queueing Enable\n\nSoftware shall write 1 this bit when in order to enable command\n\nqueueing mode (i.e. enable CQE).\n\nWhen this bit is 0, CQE is disabled and software controls the\n\neMMC bus using the legacy eMMC host controller.\n\nBefore software writes 1 to this bit, software shall verify that the\n\neMMC host controller is in idle state and there are no commands\n\nor data transfers ongoing.\n\nWhen software wants to exit command queueing mode, it shall\n\nclear all previous tasks if such exist before setting this bit to 0."]
    #[inline(always)]
    #[must_use]
    pub fn cqena(&mut self) -> CqenaW<CqcfgSpec> {
        CqenaW::new(self, 0)
    }
    #[doc = "Bit 8 - This bit indicates whether the task descriptor size is 128 bits or\n\n64 bits as detailed in Data Structures section. This bit can only be\n\nconfigured when Command Queueing Enable bit is 0 (command\n\nqueueing is disabled)"]
    #[inline(always)]
    #[must_use]
    pub fn taskdescriptorsize(&mut self) -> TaskdescriptorsizeW<CqcfgSpec> {
        TaskdescriptorsizeW::new(self, 8)
    }
    #[doc = "Bit 12 - Direct Command (DCMD) Enable\n\nThis bit indicates to the hardware whether the Task Descriptor in\n\nslot #31 of the TDL is a Data Transfer Task Descriptor, or a\n\nDirect Command Task Descriptor.\n\nCQE uses this bit when a task is issued in slot #31, to determine\n\nhow to decode the Task Descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn dcmdena(&mut self) -> DcmdenaW<CqcfgSpec> {
        DcmdenaW::new(self, 12)
    }
}
#[doc = "Command queueing configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqcfgSpec;
impl crate::RegisterSpec for CqcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqcfg::R`](R) reader structure"]
impl crate::Readable for CqcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cqcfg::W`](W) writer structure"]
impl crate::Writable for CqcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQCFG to value 0"]
impl crate::Resettable for CqcfgSpec {
    const RESET_VALUE: u32 = 0;
}
