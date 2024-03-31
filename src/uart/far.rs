#[doc = "Register `FAR` reader"]
pub type R = crate::R<FarSpec>;
#[doc = "Register `FAR` writer"]
pub type W = crate::W<FarSpec>;
#[doc = "Field `FIFO_ACCESS_TEST_EN` reader - This register is use to enable a FIFO access mode for testing, so\n\nthat the receive FIFO can be written by the master and the\n\ntransmit FIFO can be read by the master when FIFOs are\n\nimplemented and enabled. When FIFOs are not enabled it allows\n\nthe RBR to be written by the master and the THR to be read by\n\nthe master.\n\n0 = FIFO access mode disabled\n\n1 = FIFO access mode enabled"]
pub type FifoAccessTestEnR = crate::BitReader;
#[doc = "Field `FIFO_ACCESS_TEST_EN` writer - This register is use to enable a FIFO access mode for testing, so\n\nthat the receive FIFO can be written by the master and the\n\ntransmit FIFO can be read by the master when FIFOs are\n\nimplemented and enabled. When FIFOs are not enabled it allows\n\nthe RBR to be written by the master and the THR to be read by\n\nthe master.\n\n0 = FIFO access mode disabled\n\n1 = FIFO access mode enabled"]
pub type FifoAccessTestEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This register is use to enable a FIFO access mode for testing, so\n\nthat the receive FIFO can be written by the master and the\n\ntransmit FIFO can be read by the master when FIFOs are\n\nimplemented and enabled. When FIFOs are not enabled it allows\n\nthe RBR to be written by the master and the THR to be read by\n\nthe master.\n\n0 = FIFO access mode disabled\n\n1 = FIFO access mode enabled"]
    #[inline(always)]
    pub fn fifo_access_test_en(&self) -> FifoAccessTestEnR {
        FifoAccessTestEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This register is use to enable a FIFO access mode for testing, so\n\nthat the receive FIFO can be written by the master and the\n\ntransmit FIFO can be read by the master when FIFOs are\n\nimplemented and enabled. When FIFOs are not enabled it allows\n\nthe RBR to be written by the master and the THR to be read by\n\nthe master.\n\n0 = FIFO access mode disabled\n\n1 = FIFO access mode enabled"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_access_test_en(&mut self) -> FifoAccessTestEnW<FarSpec> {
        FifoAccessTestEnW::new(self, 0)
    }
}
#[doc = "FIFO Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`far::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`far::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FarSpec;
impl crate::RegisterSpec for FarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`far::R`](R) reader structure"]
impl crate::Readable for FarSpec {}
#[doc = "`write(|w| ..)` method takes [`far::W`](W) writer structure"]
impl crate::Writable for FarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FAR to value 0"]
impl crate::Resettable for FarSpec {
    const RESET_VALUE: u32 = 0;
}
