#[doc = "Register `EMMCCORE_CQTDLBA` reader"]
pub type R = crate::R<EmmccoreCqtdlbaSpec>;
#[doc = "Register `EMMCCORE_CQTDLBA` writer"]
pub type W = crate::W<EmmccoreCqtdlbaSpec>;
#[doc = "Field `TDLBA` reader - Task Descriptor List Base Address\n\nThis register stores the LSB bits (bits 31:0) of the byte address of\n\nthe head of the Task Descriptor List in system memory.\n\nThe size of the task descriptor list is 32 * (Task Descriptor size +\n\nTransfer Descriptor size) as configured by Host driver.\n\nThis address shall be set on Byte1 KByte boundary. The lower 10\n\nbits of this register shall be set to 0 by software and shall be\n\nignored by CQE."]
pub type TdlbaR = crate::FieldReader<u32>;
#[doc = "Field `TDLBA` writer - Task Descriptor List Base Address\n\nThis register stores the LSB bits (bits 31:0) of the byte address of\n\nthe head of the Task Descriptor List in system memory.\n\nThe size of the task descriptor list is 32 * (Task Descriptor size +\n\nTransfer Descriptor size) as configured by Host driver.\n\nThis address shall be set on Byte1 KByte boundary. The lower 10\n\nbits of this register shall be set to 0 by software and shall be\n\nignored by CQE."]
pub type TdlbaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Task Descriptor List Base Address\n\nThis register stores the LSB bits (bits 31:0) of the byte address of\n\nthe head of the Task Descriptor List in system memory.\n\nThe size of the task descriptor list is 32 * (Task Descriptor size +\n\nTransfer Descriptor size) as configured by Host driver.\n\nThis address shall be set on Byte1 KByte boundary. The lower 10\n\nbits of this register shall be set to 0 by software and shall be\n\nignored by CQE."]
    #[inline(always)]
    pub fn tdlba(&self) -> TdlbaR {
        TdlbaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Task Descriptor List Base Address\n\nThis register stores the LSB bits (bits 31:0) of the byte address of\n\nthe head of the Task Descriptor List in system memory.\n\nThe size of the task descriptor list is 32 * (Task Descriptor size +\n\nTransfer Descriptor size) as configured by Host driver.\n\nThis address shall be set on Byte1 KByte boundary. The lower 10\n\nbits of this register shall be set to 0 by software and shall be\n\nignored by CQE."]
    #[inline(always)]
    #[must_use]
    pub fn tdlba(&mut self) -> TdlbaW<EmmccoreCqtdlbaSpec> {
        TdlbaW::new(self, 0)
    }
}
#[doc = "Command queueing task descriptor list base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqtdlba::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqtdlba::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreCqtdlbaSpec;
impl crate::RegisterSpec for EmmccoreCqtdlbaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_cqtdlba::R`](R) reader structure"]
impl crate::Readable for EmmccoreCqtdlbaSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_cqtdlba::W`](W) writer structure"]
impl crate::Writable for EmmccoreCqtdlbaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_CQTDLBA to value 0"]
impl crate::Resettable for EmmccoreCqtdlbaSpec {
    const RESET_VALUE: u32 = 0;
}
