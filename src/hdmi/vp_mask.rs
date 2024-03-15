#[doc = "Register `VP_MASK` reader"]
pub type R = crate::R<VpMaskSpec>;
#[doc = "Register `VP_MASK` writer"]
pub type W = crate::W<VpMaskSpec>;
#[doc = "Field `SPARE_1` reader - Reserved as “spare” bit with no associated functionality."]
pub type Spare1R = crate::BitReader;
#[doc = "Field `SPARE_1` writer - Reserved as “spare” bit with no associated functionality."]
pub type Spare1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE_2` reader - Reserved as “spare” bit with no associated functionality."]
pub type Spare2R = crate::BitReader;
#[doc = "Field `SPARE_2` writer - Reserved as “spare” bit with no associated functionality."]
pub type Spare2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OINTEMPTYREMAP` reader - Mask bit for Video Packetizer pixel YCC 422 re- mapper FIFO empty"]
pub type OintemptyremapR = crate::BitReader;
#[doc = "Field `OINTEMPTYREMAP` writer - Mask bit for Video Packetizer pixel YCC 422 re- mapper FIFO empty"]
pub type OintemptyremapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OINTFULLREMAP` reader - Mask bit for Video Packetizer pixel YCC 422 re- mapper FIFO full"]
pub type OintfullremapR = crate::BitReader;
#[doc = "Field `OINTFULLREMAP` writer - Mask bit for Video Packetizer pixel YCC 422 re- mapper FIFO full"]
pub type OintfullremapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OINTEMPTYPP` reader - Mask bit for Video Packetizer pixel packing FIFO empty"]
pub type OintemptyppR = crate::BitReader;
#[doc = "Field `OINTEMPTYPP` writer - Mask bit for Video Packetizer pixel packing FIFO empty"]
pub type OintemptyppW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OINTFULLPP` reader - Mask bit for Video Packetizer pixel packing FIFO full"]
pub type OintfullppR = crate::BitReader;
#[doc = "Field `OINTFULLPP` writer - Mask bit for Video Packetizer pixel packing FIFO full"]
pub type OintfullppW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OINTEMPTYREPET` reader - Mask bit for Video Packetizer pixel repeater FIFO empty"]
pub type OintemptyrepetR = crate::BitReader;
#[doc = "Field `OINTEMPTYREPET` writer - Mask bit for Video Packetizer pixel repeater FIFO empty"]
pub type OintemptyrepetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OINTFULLREPET` reader - Mask bit for Video Packetizer pixel repeater FIFO full"]
pub type OintfullrepetR = crate::BitReader;
#[doc = "Field `OINTFULLREPET` writer - Mask bit for Video Packetizer pixel repeater FIFO full"]
pub type OintfullrepetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved as “spare” bit with no associated functionality."]
    #[inline(always)]
    pub fn spare_1(&self) -> Spare1R {
        Spare1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved as “spare” bit with no associated functionality."]
    #[inline(always)]
    pub fn spare_2(&self) -> Spare2R {
        Spare2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for Video Packetizer pixel YCC 422 re- mapper FIFO empty"]
    #[inline(always)]
    pub fn ointemptyremap(&self) -> OintemptyremapR {
        OintemptyremapR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask bit for Video Packetizer pixel YCC 422 re- mapper FIFO full"]
    #[inline(always)]
    pub fn ointfullremap(&self) -> OintfullremapR {
        OintfullremapR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask bit for Video Packetizer pixel packing FIFO empty"]
    #[inline(always)]
    pub fn ointemptypp(&self) -> OintemptyppR {
        OintemptyppR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask bit for Video Packetizer pixel packing FIFO full"]
    #[inline(always)]
    pub fn ointfullpp(&self) -> OintfullppR {
        OintfullppR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask bit for Video Packetizer pixel repeater FIFO empty"]
    #[inline(always)]
    pub fn ointemptyrepet(&self) -> OintemptyrepetR {
        OintemptyrepetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask bit for Video Packetizer pixel repeater FIFO full"]
    #[inline(always)]
    pub fn ointfullrepet(&self) -> OintfullrepetR {
        OintfullrepetR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved as “spare” bit with no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn spare_1(&mut self) -> Spare1W<VpMaskSpec> {
        Spare1W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved as “spare” bit with no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn spare_2(&mut self) -> Spare2W<VpMaskSpec> {
        Spare2W::new(self, 1)
    }
    #[doc = "Bit 2 - Mask bit for Video Packetizer pixel YCC 422 re- mapper FIFO empty"]
    #[inline(always)]
    #[must_use]
    pub fn ointemptyremap(&mut self) -> OintemptyremapW<VpMaskSpec> {
        OintemptyremapW::new(self, 2)
    }
    #[doc = "Bit 3 - Mask bit for Video Packetizer pixel YCC 422 re- mapper FIFO full"]
    #[inline(always)]
    #[must_use]
    pub fn ointfullremap(&mut self) -> OintfullremapW<VpMaskSpec> {
        OintfullremapW::new(self, 3)
    }
    #[doc = "Bit 4 - Mask bit for Video Packetizer pixel packing FIFO empty"]
    #[inline(always)]
    #[must_use]
    pub fn ointemptypp(&mut self) -> OintemptyppW<VpMaskSpec> {
        OintemptyppW::new(self, 4)
    }
    #[doc = "Bit 5 - Mask bit for Video Packetizer pixel packing FIFO full"]
    #[inline(always)]
    #[must_use]
    pub fn ointfullpp(&mut self) -> OintfullppW<VpMaskSpec> {
        OintfullppW::new(self, 5)
    }
    #[doc = "Bit 6 - Mask bit for Video Packetizer pixel repeater FIFO empty"]
    #[inline(always)]
    #[must_use]
    pub fn ointemptyrepet(&mut self) -> OintemptyrepetW<VpMaskSpec> {
        OintemptyrepetW::new(self, 6)
    }
    #[doc = "Bit 7 - Mask bit for Video Packetizer pixel repeater FIFO full"]
    #[inline(always)]
    #[must_use]
    pub fn ointfullrepet(&mut self) -> OintfullrepetW<VpMaskSpec> {
        OintfullrepetW::new(self, 7)
    }
}
#[doc = "Reserved as “spare” bit with no associated functionality.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vp_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vp_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VpMaskSpec;
impl crate::RegisterSpec for VpMaskSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vp_mask::R`](R) reader structure"]
impl crate::Readable for VpMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`vp_mask::W`](W) writer structure"]
impl crate::Writable for VpMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets VP_MASK to value 0"]
impl crate::Resettable for VpMaskSpec {
    const RESET_VALUE: u8 = 0;
}
