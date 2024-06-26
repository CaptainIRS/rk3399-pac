#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `CCI` writer - Clear Combined Interrupt\n\nWrite 1 to Clear Combined Interrupt"]
pub type CciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRFUI` writer - Clear Receive FIFO Underflow Interrupt\n\nWrite 1 to Clear Receive FIFO Underflow Interrupt"]
pub type CrfuiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRFOI` writer - Clear Receive FIFO Overflow Interrupt\n\nWrite 1 to Clear Receive FIFO Overflow Interrupt"]
pub type CrfoiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTFOI` writer - Clear Transmit FIFO Overflow Interrupt\n\nWrite 1 to Clear Transmit FIFO Overflow Interrupt"]
pub type CtfoiW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear Combined Interrupt\n\nWrite 1 to Clear Combined Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cci(&mut self) -> CciW<IcrSpec> {
        CciW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Receive FIFO Underflow Interrupt\n\nWrite 1 to Clear Receive FIFO Underflow Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn crfui(&mut self) -> CrfuiW<IcrSpec> {
        CrfuiW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Receive FIFO Overflow Interrupt\n\nWrite 1 to Clear Receive FIFO Overflow Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn crfoi(&mut self) -> CrfoiW<IcrSpec> {
        CrfoiW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Transmit FIFO Overflow Interrupt\n\nWrite 1 to Clear Transmit FIFO Overflow Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ctfoi(&mut self) -> CtfoiW<IcrSpec> {
        CtfoiW::new(self, 3)
    }
}
#[doc = "Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {
    const RESET_VALUE: u32 = 0;
}
