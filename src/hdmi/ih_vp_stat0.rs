#[doc = "Register `IH_VP_STAT0` reader"]
pub type R = crate::R<IhVpStat0Spec>;
#[doc = "Register `IH_VP_STAT0` writer"]
pub type W = crate::W<IhVpStat0Spec>;
#[doc = "Field `FIFOEMPTYREMAP` reader - Video Packetizer pixel YCC 422 re-mapper FIFO\n\nempty interrupt"]
pub type FifoemptyremapR = crate::BitReader;
#[doc = "Field `FIFOEMPTYREMAP` writer - Video Packetizer pixel YCC 422 re-mapper FIFO\n\nempty interrupt"]
pub type FifoemptyremapW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FIFOFULLREMAP` reader - Video Packetizer pixel YCC 422 re-mapper FIFO full\n\ninterrupt"]
pub type FifofullremapR = crate::BitReader;
#[doc = "Field `FIFOFULLREMAP` writer - Video Packetizer pixel YCC 422 re-mapper FIFO full\n\ninterrupt"]
pub type FifofullremapW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FIFOEMPTYPP` reader - Video Packetizer pixel packing FIFO empty\n\ninterrupt"]
pub type FifoemptyppR = crate::BitReader;
#[doc = "Field `FIFOEMPTYPP` writer - Video Packetizer pixel packing FIFO empty\n\ninterrupt"]
pub type FifoemptyppW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FIFOFULLPP` reader - Video Packetizer pixel packing FIFO full interrupt"]
pub type FifofullppR = crate::BitReader;
#[doc = "Field `FIFOFULLPP` writer - Video Packetizer pixel packing FIFO full interrupt"]
pub type FifofullppW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FIFOEMPTYREPET` reader - Video Packetizer pixel repeater FIFO empty\n\ninterrupt"]
pub type FifoemptyrepetR = crate::BitReader;
#[doc = "Field `FIFOEMPTYREPET` writer - Video Packetizer pixel repeater FIFO empty\n\ninterrupt"]
pub type FifoemptyrepetW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FIFOFULLREPET` reader - Video Packetizer pixel repeater FIFO full interrupt"]
pub type FifofullrepetR = crate::BitReader;
#[doc = "Field `FIFOFULLREPET` writer - Video Packetizer pixel repeater FIFO full interrupt"]
pub type FifofullrepetW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 2 - Video Packetizer pixel YCC 422 re-mapper FIFO\n\nempty interrupt"]
    #[inline(always)]
    pub fn fifoemptyremap(&self) -> FifoemptyremapR {
        FifoemptyremapR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Video Packetizer pixel YCC 422 re-mapper FIFO full\n\ninterrupt"]
    #[inline(always)]
    pub fn fifofullremap(&self) -> FifofullremapR {
        FifofullremapR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Video Packetizer pixel packing FIFO empty\n\ninterrupt"]
    #[inline(always)]
    pub fn fifoemptypp(&self) -> FifoemptyppR {
        FifoemptyppR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Video Packetizer pixel packing FIFO full interrupt"]
    #[inline(always)]
    pub fn fifofullpp(&self) -> FifofullppR {
        FifofullppR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Video Packetizer pixel repeater FIFO empty\n\ninterrupt"]
    #[inline(always)]
    pub fn fifoemptyrepet(&self) -> FifoemptyrepetR {
        FifoemptyrepetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Video Packetizer pixel repeater FIFO full interrupt"]
    #[inline(always)]
    pub fn fifofullrepet(&self) -> FifofullrepetR {
        FifofullrepetR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Video Packetizer pixel YCC 422 re-mapper FIFO\n\nempty interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fifoemptyremap(&mut self) -> FifoemptyremapW<IhVpStat0Spec> {
        FifoemptyremapW::new(self, 2)
    }
    #[doc = "Bit 3 - Video Packetizer pixel YCC 422 re-mapper FIFO full\n\ninterrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fifofullremap(&mut self) -> FifofullremapW<IhVpStat0Spec> {
        FifofullremapW::new(self, 3)
    }
    #[doc = "Bit 4 - Video Packetizer pixel packing FIFO empty\n\ninterrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fifoemptypp(&mut self) -> FifoemptyppW<IhVpStat0Spec> {
        FifoemptyppW::new(self, 4)
    }
    #[doc = "Bit 5 - Video Packetizer pixel packing FIFO full interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fifofullpp(&mut self) -> FifofullppW<IhVpStat0Spec> {
        FifofullppW::new(self, 5)
    }
    #[doc = "Bit 6 - Video Packetizer pixel repeater FIFO empty\n\ninterrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fifoemptyrepet(&mut self) -> FifoemptyrepetW<IhVpStat0Spec> {
        FifoemptyrepetW::new(self, 6)
    }
    #[doc = "Bit 7 - Video Packetizer pixel repeater FIFO full interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fifofullrepet(&mut self) -> FifofullrepetW<IhVpStat0Spec> {
        FifofullrepetW::new(self, 7)
    }
}
#[doc = "Video Packetizer Interrupt Status Register (FIFO Full and Empty Interrupts)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_vp_stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_vp_stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhVpStat0Spec;
impl crate::RegisterSpec for IhVpStat0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_vp_stat0::R`](R) reader structure"]
impl crate::Readable for IhVpStat0Spec {}
#[doc = "`write(|w| ..)` method takes [`ih_vp_stat0::W`](W) writer structure"]
impl crate::Writable for IhVpStat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0xfc;
}
#[doc = "`reset()` method sets IH_VP_STAT0 to value 0"]
impl crate::Resettable for IhVpStat0Spec {
    const RESET_VALUE: u8 = 0;
}
