#[doc = "Register `RXDR` reader"]
pub type R = crate::R<RxdrSpec>;
#[doc = "Register `RXDR` writer"]
pub type W = crate::W<RxdrSpec>;
#[doc = "Field `RXDR` reader - Receive FIFO Data Register.\n\nWhen the register is read, data in the receive FIFO is accessed."]
pub type RxdrR = crate::FieldReader<u16>;
#[doc = "Field `RXDR` writer - Receive FIFO Data Register.\n\nWhen the register is read, data in the receive FIFO is accessed."]
pub type RxdrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Receive FIFO Data Register.\n\nWhen the register is read, data in the receive FIFO is accessed."]
    #[inline(always)]
    pub fn rxdr(&self) -> RxdrR {
        RxdrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive FIFO Data Register.\n\nWhen the register is read, data in the receive FIFO is accessed."]
    #[inline(always)]
    #[must_use]
    pub fn rxdr(&mut self) -> RxdrW<RxdrSpec> {
        RxdrW::new(self, 0)
    }
}
#[doc = "Receive FIFO Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdrSpec;
impl crate::RegisterSpec for RxdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdr::R`](R) reader structure"]
impl crate::Readable for RxdrSpec {}
#[doc = "`write(|w| ..)` method takes [`rxdr::W`](W) writer structure"]
impl crate::Writable for RxdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXDR to value 0"]
impl crate::Resettable for RxdrSpec {
    const RESET_VALUE: u32 = 0;
}
