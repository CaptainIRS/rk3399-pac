#[doc = "Register `I2S_CLR` reader"]
pub type R = crate::R<I2sClrSpec>;
#[doc = "Register `I2S_CLR` writer"]
pub type W = crate::W<I2sClrSpec>;
#[doc = "Field `TXC` reader - TX logic clear This is a self cleared bit. Write 1 to clear all transmit logic."]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXC` writer - TX logic clear This is a self cleared bit. Write 1 to clear all transmit logic."]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXC` reader - RX logic clear This is a self cleared bit. Write 1 to clear all receive logic."]
pub type RxcR = crate::BitReader;
#[doc = "Field `RXC` writer - RX logic clear This is a self cleared bit. Write 1 to clear all receive logic."]
pub type RxcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX logic clear This is a self cleared bit. Write 1 to clear all transmit logic."]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX logic clear This is a self cleared bit. Write 1 to clear all receive logic."]
    #[inline(always)]
    pub fn rxc(&self) -> RxcR {
        RxcR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX logic clear This is a self cleared bit. Write 1 to clear all transmit logic."]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TxcW<I2sClrSpec> {
        TxcW::new(self, 0)
    }
    #[doc = "Bit 1 - RX logic clear This is a self cleared bit. Write 1 to clear all receive logic."]
    #[inline(always)]
    #[must_use]
    pub fn rxc(&mut self) -> RxcW<I2sClrSpec> {
        RxcW::new(self, 1)
    }
}
#[doc = "SCLK domain logic clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sClrSpec;
impl crate::RegisterSpec for I2sClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_clr::R`](R) reader structure"]
impl crate::Readable for I2sClrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2s_clr::W`](W) writer structure"]
impl crate::Writable for I2sClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2S_CLR to value 0"]
impl crate::Resettable for I2sClrSpec {
    const RESET_VALUE: u32 = 0;
}
