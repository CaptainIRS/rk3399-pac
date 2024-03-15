#[doc = "Register `IH_FC_STAT1` reader"]
pub type R = crate::R<IhFcStat1Spec>;
#[doc = "Register `IH_FC_STAT1` writer"]
pub type W = crate::W<IhFcStat1Spec>;
#[doc = "Field `GCP` reader - Active after successful transmission of an General Control Packet."]
pub type GcpR = crate::BitReader;
#[doc = "Field `GCP` writer - Active after successful transmission of an General Control Packet."]
pub type GcpW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AVI` reader - Active after successful transmission of an AVI InfoFrame packet."]
pub type AviR = crate::BitReader;
#[doc = "Field `AVI` writer - Active after successful transmission of an AVI InfoFrame packet."]
pub type AviW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AMP` reader - Active after successful transmission of an Audio Metadata packet"]
pub type AmpR = crate::BitReader;
#[doc = "Field `AMP` writer - Active after successful transmission of an Audio Metadata packet"]
pub type AmpW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SPD` reader - Active after successful transmission of an Source Product Descriptor InfoFrame packet."]
pub type SpdR = crate::BitReader;
#[doc = "Field `SPD` writer - Active after successful transmission of an Source Product Descriptor InfoFrame packet."]
pub type SpdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `VSD` reader - Active after successful transmission of an Vendor Specific Data InfoFrame packet."]
pub type VsdR = crate::BitReader;
#[doc = "Field `VSD` writer - Active after successful transmission of an Vendor Specific Data InfoFrame packet."]
pub type VsdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ISCR2` reader - Active after successful transmission of an International Standard Recording Code 2 packet"]
pub type Iscr2R = crate::BitReader;
#[doc = "Field `ISCR2` writer - Active after successful transmission of an International Standard Recording Code 2 packet"]
pub type Iscr2W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ISCR1` reader - Active after successful transmission of an International Standard Recording Code 1 packet."]
pub type Iscr1R = crate::BitReader;
#[doc = "Field `ISCR1` writer - Active after successful transmission of an International Standard Recording Code 1 packet."]
pub type Iscr1W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `GMD` reader - Active after successful transmission of an Gamut metadata packet."]
pub type GmdR = crate::BitReader;
#[doc = "Field `GMD` writer - Active after successful transmission of an Gamut metadata packet."]
pub type GmdW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Active after successful transmission of an General Control Packet."]
    #[inline(always)]
    pub fn gcp(&self) -> GcpR {
        GcpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active after successful transmission of an AVI InfoFrame packet."]
    #[inline(always)]
    pub fn avi(&self) -> AviR {
        AviR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Active after successful transmission of an Audio Metadata packet"]
    #[inline(always)]
    pub fn amp(&self) -> AmpR {
        AmpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Active after successful transmission of an Source Product Descriptor InfoFrame packet."]
    #[inline(always)]
    pub fn spd(&self) -> SpdR {
        SpdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Active after successful transmission of an Vendor Specific Data InfoFrame packet."]
    #[inline(always)]
    pub fn vsd(&self) -> VsdR {
        VsdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Active after successful transmission of an International Standard Recording Code 2 packet"]
    #[inline(always)]
    pub fn iscr2(&self) -> Iscr2R {
        Iscr2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Active after successful transmission of an International Standard Recording Code 1 packet."]
    #[inline(always)]
    pub fn iscr1(&self) -> Iscr1R {
        Iscr1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Active after successful transmission of an Gamut metadata packet."]
    #[inline(always)]
    pub fn gmd(&self) -> GmdR {
        GmdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active after successful transmission of an General Control Packet."]
    #[inline(always)]
    #[must_use]
    pub fn gcp(&mut self) -> GcpW<IhFcStat1Spec> {
        GcpW::new(self, 0)
    }
    #[doc = "Bit 1 - Active after successful transmission of an AVI InfoFrame packet."]
    #[inline(always)]
    #[must_use]
    pub fn avi(&mut self) -> AviW<IhFcStat1Spec> {
        AviW::new(self, 1)
    }
    #[doc = "Bit 2 - Active after successful transmission of an Audio Metadata packet"]
    #[inline(always)]
    #[must_use]
    pub fn amp(&mut self) -> AmpW<IhFcStat1Spec> {
        AmpW::new(self, 2)
    }
    #[doc = "Bit 3 - Active after successful transmission of an Source Product Descriptor InfoFrame packet."]
    #[inline(always)]
    #[must_use]
    pub fn spd(&mut self) -> SpdW<IhFcStat1Spec> {
        SpdW::new(self, 3)
    }
    #[doc = "Bit 4 - Active after successful transmission of an Vendor Specific Data InfoFrame packet."]
    #[inline(always)]
    #[must_use]
    pub fn vsd(&mut self) -> VsdW<IhFcStat1Spec> {
        VsdW::new(self, 4)
    }
    #[doc = "Bit 5 - Active after successful transmission of an International Standard Recording Code 2 packet"]
    #[inline(always)]
    #[must_use]
    pub fn iscr2(&mut self) -> Iscr2W<IhFcStat1Spec> {
        Iscr2W::new(self, 5)
    }
    #[doc = "Bit 6 - Active after successful transmission of an International Standard Recording Code 1 packet."]
    #[inline(always)]
    #[must_use]
    pub fn iscr1(&mut self) -> Iscr1W<IhFcStat1Spec> {
        Iscr1W::new(self, 6)
    }
    #[doc = "Bit 7 - Active after successful transmission of an Gamut metadata packet."]
    #[inline(always)]
    #[must_use]
    pub fn gmd(&mut self) -> GmdW<IhFcStat1Spec> {
        GmdW::new(self, 7)
    }
}
#[doc = "Active after successful transmission of an General Control Packet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_fc_stat1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_fc_stat1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhFcStat1Spec;
impl crate::RegisterSpec for IhFcStat1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_fc_stat1::R`](R) reader structure"]
impl crate::Readable for IhFcStat1Spec {}
#[doc = "`write(|w| ..)` method takes [`ih_fc_stat1::W`](W) writer structure"]
impl crate::Writable for IhFcStat1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0xff;
}
#[doc = "`reset()` method sets IH_FC_STAT1 to value 0"]
impl crate::Resettable for IhFcStat1Spec {
    const RESET_VALUE: u8 = 0;
}
