#[doc = "Register `HTX` reader"]
pub type R = crate::R<HtxSpec>;
#[doc = "Register `HTX` writer"]
pub type W = crate::W<HtxSpec>;
#[doc = "Field `HALT_TX_EN` reader - This register is use to halt transmissions for testing, so that the\n\ntransmit FIFO can be filled by the master when FIFOs are\n\nimplemented and enabled.\n\n0 = Halt TX disabled\n\n1 = Halt TX enabled"]
pub type HaltTxEnR = crate::BitReader;
#[doc = "Field `HALT_TX_EN` writer - This register is use to halt transmissions for testing, so that the\n\ntransmit FIFO can be filled by the master when FIFOs are\n\nimplemented and enabled.\n\n0 = Halt TX disabled\n\n1 = Halt TX enabled"]
pub type HaltTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This register is use to halt transmissions for testing, so that the\n\ntransmit FIFO can be filled by the master when FIFOs are\n\nimplemented and enabled.\n\n0 = Halt TX disabled\n\n1 = Halt TX enabled"]
    #[inline(always)]
    pub fn halt_tx_en(&self) -> HaltTxEnR {
        HaltTxEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This register is use to halt transmissions for testing, so that the\n\ntransmit FIFO can be filled by the master when FIFOs are\n\nimplemented and enabled.\n\n0 = Halt TX disabled\n\n1 = Halt TX enabled"]
    #[inline(always)]
    #[must_use]
    pub fn halt_tx_en(&mut self) -> HaltTxEnW<HtxSpec> {
        HaltTxEnW::new(self, 0)
    }
}
#[doc = "Halt TX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HtxSpec;
impl crate::RegisterSpec for HtxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`htx::R`](R) reader structure"]
impl crate::Readable for HtxSpec {}
#[doc = "`write(|w| ..)` method takes [`htx::W`](W) writer structure"]
impl crate::Writable for HtxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HTX to value 0"]
impl crate::Resettable for HtxSpec {
    const RESET_VALUE: u32 = 0;
}
