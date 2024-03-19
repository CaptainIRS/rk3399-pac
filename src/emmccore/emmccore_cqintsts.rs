#[doc = "Register `EMMCCORE_CQINTSTS` reader"]
pub type R = crate::R<EmmccoreCqintstsSpec>;
#[doc = "Register `EMMCCORE_CQINTSTS` writer"]
pub type W = crate::W<EmmccoreCqintstsSpec>;
#[doc = "Field `HAC` reader - Halt Complete Interrupt\n\nThis status bit is asserted (if CQISTE.HAC=1) when halt bit in\n\nCQCTL register transitions from 0 to 1 indicating that host\n\ncontroller has completed its current ongoing task and has entered\n\nhalt state."]
pub type HacR = crate::BitReader;
#[doc = "Field `HAC` writer - Halt Complete Interrupt\n\nThis status bit is asserted (if CQISTE.HAC=1) when halt bit in\n\nCQCTL register transitions from 0 to 1 indicating that host\n\ncontroller has completed its current ongoing task and has entered\n\nhalt state."]
pub type HacW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TCC` reader - Task Complete Interrupt\n\nThis status bit is asserted(if CQISTE.TCC=1) when at least one of\n\nthe following two conditions are met:\n\na. A task is completed and the INT bit is set in its Task Descriptor\n\nb. Interrupt caused by Interrupt Coalescing logic"]
pub type TccR = crate::BitReader;
#[doc = "Field `TCC` writer - Task Complete Interrupt\n\nThis status bit is asserted(if CQISTE.TCC=1) when at least one of\n\nthe following two conditions are met:\n\na. A task is completed and the INT bit is set in its Task Descriptor\n\nb. Interrupt caused by Interrupt Coalescing logic"]
pub type TccW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RED` reader - Response Error Detected Interrupt\n\nThis status bit is asserted (if CQISTE.RED=1) when a response is\n\nreceived with an error bit set in the device status field.\n\nSoftware uses CQRMEM register to configure which device status\n\nbit fields may trigger an interrupt, and which are masked."]
pub type RedR = crate::BitReader;
#[doc = "Field `RED` writer - Response Error Detected Interrupt\n\nThis status bit is asserted (if CQISTE.RED=1) when a response is\n\nreceived with an error bit set in the device status field.\n\nSoftware uses CQRMEM register to configure which device status\n\nbit fields may trigger an interrupt, and which are masked."]
pub type RedW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TCL` reader - Task Cleared\n\nThis status bit is asserted (if CQISTE.TCL=1) when a task clear\n\noperation is completed by CQE. The completed task clear\n\noperation is either an individual task clear (CQTCLR) or clearing\n\nof all tasks (CQCTL)."]
pub type TclR = crate::BitReader;
#[doc = "Field `TCL` writer - Task Cleared\n\nThis status bit is asserted (if CQISTE.TCL=1) when a task clear\n\noperation is completed by CQE. The completed task clear\n\noperation is either an individual task clear (CQTCLR) or clearing\n\nof all tasks (CQCTL)."]
pub type TclW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TERR` reader - Task Error Interrupt\n\nThis bit is asserted when task error is detected due to invalid task\n\ndescriptor"]
pub type TerrR = crate::BitReader;
#[doc = "Field `TERR` writer - Task Error Interrupt\n\nThis bit is asserted when task error is detected due to invalid task\n\ndescriptor"]
pub type TerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Halt Complete Interrupt\n\nThis status bit is asserted (if CQISTE.HAC=1) when halt bit in\n\nCQCTL register transitions from 0 to 1 indicating that host\n\ncontroller has completed its current ongoing task and has entered\n\nhalt state."]
    #[inline(always)]
    pub fn hac(&self) -> HacR {
        HacR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Task Complete Interrupt\n\nThis status bit is asserted(if CQISTE.TCC=1) when at least one of\n\nthe following two conditions are met:\n\na. A task is completed and the INT bit is set in its Task Descriptor\n\nb. Interrupt caused by Interrupt Coalescing logic"]
    #[inline(always)]
    pub fn tcc(&self) -> TccR {
        TccR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Response Error Detected Interrupt\n\nThis status bit is asserted (if CQISTE.RED=1) when a response is\n\nreceived with an error bit set in the device status field.\n\nSoftware uses CQRMEM register to configure which device status\n\nbit fields may trigger an interrupt, and which are masked."]
    #[inline(always)]
    pub fn red(&self) -> RedR {
        RedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Task Cleared\n\nThis status bit is asserted (if CQISTE.TCL=1) when a task clear\n\noperation is completed by CQE. The completed task clear\n\noperation is either an individual task clear (CQTCLR) or clearing\n\nof all tasks (CQCTL)."]
    #[inline(always)]
    pub fn tcl(&self) -> TclR {
        TclR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Task Error Interrupt\n\nThis bit is asserted when task error is detected due to invalid task\n\ndescriptor"]
    #[inline(always)]
    pub fn terr(&self) -> TerrR {
        TerrR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Halt Complete Interrupt\n\nThis status bit is asserted (if CQISTE.HAC=1) when halt bit in\n\nCQCTL register transitions from 0 to 1 indicating that host\n\ncontroller has completed its current ongoing task and has entered\n\nhalt state."]
    #[inline(always)]
    #[must_use]
    pub fn hac(&mut self) -> HacW<EmmccoreCqintstsSpec> {
        HacW::new(self, 0)
    }
    #[doc = "Bit 1 - Task Complete Interrupt\n\nThis status bit is asserted(if CQISTE.TCC=1) when at least one of\n\nthe following two conditions are met:\n\na. A task is completed and the INT bit is set in its Task Descriptor\n\nb. Interrupt caused by Interrupt Coalescing logic"]
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TccW<EmmccoreCqintstsSpec> {
        TccW::new(self, 1)
    }
    #[doc = "Bit 2 - Response Error Detected Interrupt\n\nThis status bit is asserted (if CQISTE.RED=1) when a response is\n\nreceived with an error bit set in the device status field.\n\nSoftware uses CQRMEM register to configure which device status\n\nbit fields may trigger an interrupt, and which are masked."]
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RedW<EmmccoreCqintstsSpec> {
        RedW::new(self, 2)
    }
    #[doc = "Bit 3 - Task Cleared\n\nThis status bit is asserted (if CQISTE.TCL=1) when a task clear\n\noperation is completed by CQE. The completed task clear\n\noperation is either an individual task clear (CQTCLR) or clearing\n\nof all tasks (CQCTL)."]
    #[inline(always)]
    #[must_use]
    pub fn tcl(&mut self) -> TclW<EmmccoreCqintstsSpec> {
        TclW::new(self, 3)
    }
    #[doc = "Bit 4 - Task Error Interrupt\n\nThis bit is asserted when task error is detected due to invalid task\n\ndescriptor"]
    #[inline(always)]
    #[must_use]
    pub fn terr(&mut self) -> TerrW<EmmccoreCqintstsSpec> {
        TerrW::new(self, 4)
    }
}
#[doc = "Command queueing interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqintsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqintsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreCqintstsSpec;
impl crate::RegisterSpec for EmmccoreCqintstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_cqintsts::R`](R) reader structure"]
impl crate::Readable for EmmccoreCqintstsSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_cqintsts::W`](W) writer structure"]
impl crate::Writable for EmmccoreCqintstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x1f;
}
#[doc = "`reset()` method sets EMMCCORE_CQINTSTS to value 0"]
impl crate::Resettable for EmmccoreCqintstsSpec {
    const RESET_VALUE: u32 = 0;
}
