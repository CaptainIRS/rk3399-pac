#[doc = "Register `EMMCCORE_CQTDLBAU` reader"]
pub type R = crate::R<EmmccoreCqtdlbauSpec>;
#[doc = "Register `EMMCCORE_CQTDLBAU` writer"]
pub type W = crate::W<EmmccoreCqtdlbauSpec>;
#[doc = "Field `TDLBA` reader - Task Descriptor List Base Address This register stores the MSB bits (bits 63:32) of the byte address of the head of the Task Descriptor List in system memory. The size of the task descriptor list is 32 * (Task Descriptor size + Transfer Descriptor size) as configured by Host driver. This register is reserved when using 32-bit addressing mode."]
pub type TdlbaR = crate::FieldReader<u32>;
#[doc = "Field `TDLBA` writer - Task Descriptor List Base Address This register stores the MSB bits (bits 63:32) of the byte address of the head of the Task Descriptor List in system memory. The size of the task descriptor list is 32 * (Task Descriptor size + Transfer Descriptor size) as configured by Host driver. This register is reserved when using 32-bit addressing mode."]
pub type TdlbaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Task Descriptor List Base Address This register stores the MSB bits (bits 63:32) of the byte address of the head of the Task Descriptor List in system memory. The size of the task descriptor list is 32 * (Task Descriptor size + Transfer Descriptor size) as configured by Host driver. This register is reserved when using 32-bit addressing mode."]
    #[inline(always)]
    pub fn tdlba(&self) -> TdlbaR {
        TdlbaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Task Descriptor List Base Address This register stores the MSB bits (bits 63:32) of the byte address of the head of the Task Descriptor List in system memory. The size of the task descriptor list is 32 * (Task Descriptor size + Transfer Descriptor size) as configured by Host driver. This register is reserved when using 32-bit addressing mode."]
    #[inline(always)]
    #[must_use]
    pub fn tdlba(&mut self) -> TdlbaW<EmmccoreCqtdlbauSpec> {
        TdlbaW::new(self, 0)
    }
}
#[doc = "Command queueing task descriptor list base address upper 32bits register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqtdlbau::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqtdlbau::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreCqtdlbauSpec;
impl crate::RegisterSpec for EmmccoreCqtdlbauSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_cqtdlbau::R`](R) reader structure"]
impl crate::Readable for EmmccoreCqtdlbauSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_cqtdlbau::W`](W) writer structure"]
impl crate::Writable for EmmccoreCqtdlbauSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_CQTDLBAU to value 0"]
impl crate::Resettable for EmmccoreCqtdlbauSpec {
    const RESET_VALUE: u32 = 0;
}
