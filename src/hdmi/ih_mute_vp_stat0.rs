#[doc = "Register `IH_MUTE_VP_STAT0` reader"]
pub type R = crate::R<IhMuteVpStat0Spec>;
#[doc = "Register `IH_MUTE_VP_STAT0` writer"]
pub type W = crate::W<IhMuteVpStat0Spec>;
#[doc = "Field `SPARE_1` reader - Reserved as “spare” bit with no associated functionality."]
pub type Spare1R = crate::BitReader;
#[doc = "Field `SPARE_1` writer - Reserved as “spare” bit with no associated functionality."]
pub type Spare1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE_2` reader - Reserved as “spare” bit with no associated functionality."]
pub type Spare2R = crate::BitReader;
#[doc = "Field `SPARE_2` writer - Reserved as “spare” bit with no associated functionality."]
pub type Spare2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOEMPTYREMAP` reader - When set to 1, mutes ih_vp_stat0\\[2\\]"]
pub type FifoemptyremapR = crate::BitReader;
#[doc = "Field `FIFOEMPTYREMAP` writer - When set to 1, mutes ih_vp_stat0\\[2\\]"]
pub type FifoemptyremapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOFULLREMAP` reader - When set to 1, mutes ih_vp_stat0\\[3\\]"]
pub type FifofullremapR = crate::BitReader;
#[doc = "Field `FIFOFULLREMAP` writer - When set to 1, mutes ih_vp_stat0\\[3\\]"]
pub type FifofullremapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOEMPTYPP` reader - When set to 1, mutes ih_vp_stat0\\[4\\]"]
pub type FifoemptyppR = crate::BitReader;
#[doc = "Field `FIFOEMPTYPP` writer - When set to 1, mutes ih_vp_stat0\\[4\\]"]
pub type FifoemptyppW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOFULLPP` reader - When set to 1, mutes ih_vp_stat0\\[5\\]"]
pub type FifofullppR = crate::BitReader;
#[doc = "Field `FIFOFULLPP` writer - When set to 1, mutes ih_vp_stat0\\[5\\]"]
pub type FifofullppW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOEMPTYREPET` reader - When set to 1, mutes ih_vp_stat0\\[6\\]"]
pub type FifoemptyrepetR = crate::BitReader;
#[doc = "Field `FIFOEMPTYREPET` writer - When set to 1, mutes ih_vp_stat0\\[6\\]"]
pub type FifoemptyrepetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOFULLREPET` reader - When set to 1, mutes ih_vp_stat0\\[7\\]"]
pub type FifofullrepetR = crate::BitReader;
#[doc = "Field `FIFOFULLREPET` writer - When set to 1, mutes ih_vp_stat0\\[7\\]"]
pub type FifofullrepetW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - When set to 1, mutes ih_vp_stat0\\[2\\]"]
    #[inline(always)]
    pub fn fifoemptyremap(&self) -> FifoemptyremapR {
        FifoemptyremapR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set to 1, mutes ih_vp_stat0\\[3\\]"]
    #[inline(always)]
    pub fn fifofullremap(&self) -> FifofullremapR {
        FifofullremapR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When set to 1, mutes ih_vp_stat0\\[4\\]"]
    #[inline(always)]
    pub fn fifoemptypp(&self) -> FifoemptyppR {
        FifoemptyppR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When set to 1, mutes ih_vp_stat0\\[5\\]"]
    #[inline(always)]
    pub fn fifofullpp(&self) -> FifofullppR {
        FifofullppR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When set to 1, mutes ih_vp_stat0\\[6\\]"]
    #[inline(always)]
    pub fn fifoemptyrepet(&self) -> FifoemptyrepetR {
        FifoemptyrepetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When set to 1, mutes ih_vp_stat0\\[7\\]"]
    #[inline(always)]
    pub fn fifofullrepet(&self) -> FifofullrepetR {
        FifofullrepetR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved as “spare” bit with no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn spare_1(&mut self) -> Spare1W<IhMuteVpStat0Spec> {
        Spare1W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved as “spare” bit with no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn spare_2(&mut self) -> Spare2W<IhMuteVpStat0Spec> {
        Spare2W::new(self, 1)
    }
    #[doc = "Bit 2 - When set to 1, mutes ih_vp_stat0\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn fifoemptyremap(&mut self) -> FifoemptyremapW<IhMuteVpStat0Spec> {
        FifoemptyremapW::new(self, 2)
    }
    #[doc = "Bit 3 - When set to 1, mutes ih_vp_stat0\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn fifofullremap(&mut self) -> FifofullremapW<IhMuteVpStat0Spec> {
        FifofullremapW::new(self, 3)
    }
    #[doc = "Bit 4 - When set to 1, mutes ih_vp_stat0\\[4\\]"]
    #[inline(always)]
    #[must_use]
    pub fn fifoemptypp(&mut self) -> FifoemptyppW<IhMuteVpStat0Spec> {
        FifoemptyppW::new(self, 4)
    }
    #[doc = "Bit 5 - When set to 1, mutes ih_vp_stat0\\[5\\]"]
    #[inline(always)]
    #[must_use]
    pub fn fifofullpp(&mut self) -> FifofullppW<IhMuteVpStat0Spec> {
        FifofullppW::new(self, 5)
    }
    #[doc = "Bit 6 - When set to 1, mutes ih_vp_stat0\\[6\\]"]
    #[inline(always)]
    #[must_use]
    pub fn fifoemptyrepet(&mut self) -> FifoemptyrepetW<IhMuteVpStat0Spec> {
        FifoemptyrepetW::new(self, 6)
    }
    #[doc = "Bit 7 - When set to 1, mutes ih_vp_stat0\\[7\\]"]
    #[inline(always)]
    #[must_use]
    pub fn fifofullrepet(&mut self) -> FifofullrepetW<IhMuteVpStat0Spec> {
        FifofullrepetW::new(self, 7)
    }
}
#[doc = "Reserved as “spare” bit with no associated functionality.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute_vp_stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute_vp_stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhMuteVpStat0Spec;
impl crate::RegisterSpec for IhMuteVpStat0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_mute_vp_stat0::R`](R) reader structure"]
impl crate::Readable for IhMuteVpStat0Spec {}
#[doc = "`write(|w| ..)` method takes [`ih_mute_vp_stat0::W`](W) writer structure"]
impl crate::Writable for IhMuteVpStat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IH_MUTE_VP_STAT0 to value 0"]
impl crate::Resettable for IhMuteVpStat0Spec {
    const RESET_VALUE: u8 = 0;
}
