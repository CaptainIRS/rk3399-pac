#[doc = "Register `CLR` reader"]
pub type R = crate::R<ClrSpec>;
#[doc = "Register `CLR` writer"]
pub type W = crate::W<ClrSpec>;
#[doc = "Field `TXC` reader - TX logic clear\n\nThis is a self cleared bit. Write 1 to clear all transmit logic."]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXC` writer - TX logic clear\n\nThis is a self cleared bit. Write 1 to clear all transmit logic."]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXC` reader - RX logic clear\n\nThis is a self cleared bit. Write 1 to clear all receive logic."]
pub type RxcR = crate::BitReader;
#[doc = "Field `RXC` writer - RX logic clear\n\nThis is a self cleared bit. Write 1 to clear all receive logic."]
pub type RxcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX logic clear\n\nThis is a self cleared bit. Write 1 to clear all transmit logic."]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX logic clear\n\nThis is a self cleared bit. Write 1 to clear all receive logic."]
    #[inline(always)]
    pub fn rxc(&self) -> RxcR {
        RxcR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX logic clear\n\nThis is a self cleared bit. Write 1 to clear all transmit logic."]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TxcW<ClrSpec> {
        TxcW::new(self, 0)
    }
    #[doc = "Bit 1 - RX logic clear\n\nThis is a self cleared bit. Write 1 to clear all receive logic."]
    #[inline(always)]
    #[must_use]
    pub fn rxc(&mut self) -> RxcW<ClrSpec> {
        RxcW::new(self, 1)
    }
}
#[doc = "SCLK domain logic clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrSpec;
impl crate::RegisterSpec for ClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr::R`](R) reader structure"]
impl crate::Readable for ClrSpec {}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for ClrSpec {
    const RESET_VALUE: u32 = 0;
}
