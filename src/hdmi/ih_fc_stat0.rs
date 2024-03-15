#[doc = "Register `IH_FC_STAT0` reader"]
pub type R = crate::R<IhFcStat0Spec>;
#[doc = "Register `IH_FC_STAT0` writer"]
pub type W = crate::W<IhFcStat0Spec>;
#[doc = "Field `NULL` reader - Active after successful transmission of an Null packet. Due to high number of audio sample packets transmitted, this interrupt is by default masked at frame composer."]
pub type NullR = crate::BitReader;
#[doc = "Field `NULL` writer - Active after successful transmission of an Null packet. Due to high number of audio sample packets transmitted, this interrupt is by default masked at frame composer."]
pub type NullW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ACR` reader - Active after successful transmission of an Audio Clock Regeneration (N/ CTS transmission) packet."]
pub type AcrR = crate::BitReader;
#[doc = "Field `ACR` writer - Active after successful transmission of an Audio Clock Regeneration (N/ CTS transmission) packet."]
pub type AcrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AUDS` reader - Active after successful transmission of an Audio Sample packet. Due to high number of audio sample packets transmitted, this interrupt is by default masked at frame composer."]
pub type AudsR = crate::BitReader;
#[doc = "Field `AUDS` writer - Active after successful transmission of an Audio Sample packet. Due to high number of audio sample packets transmitted, this interrupt is by default masked at frame composer."]
pub type AudsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `NVBI` reader - Active after successful transmission of an NTSC VBI packet"]
pub type NvbiR = crate::BitReader;
#[doc = "Field `NVBI` writer - Active after successful transmission of an NTSC VBI packet"]
pub type NvbiW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MAS` reader - Active after successful transmission of an MultiStream Audio packet"]
pub type MasR = crate::BitReader;
#[doc = "Field `MAS` writer - Active after successful transmission of an MultiStream Audio packet"]
pub type MasW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HBR` reader - Active after successful transmission of an Audio HBR packet."]
pub type HbrR = crate::BitReader;
#[doc = "Field `HBR` writer - Active after successful transmission of an Audio HBR packet."]
pub type HbrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ACP` reader - Active after successful transmission of an Audio Content Protection packet."]
pub type AcpR = crate::BitReader;
#[doc = "Field `ACP` writer - Active after successful transmission of an Audio Content Protection packet."]
pub type AcpW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AUDI` reader - Active after successful transmission of an Audio InfoFrame packet."]
pub type AudiR = crate::BitReader;
#[doc = "Field `AUDI` writer - Active after successful transmission of an Audio InfoFrame packet."]
pub type AudiW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Active after successful transmission of an Null packet. Due to high number of audio sample packets transmitted, this interrupt is by default masked at frame composer."]
    #[inline(always)]
    pub fn null(&self) -> NullR {
        NullR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active after successful transmission of an Audio Clock Regeneration (N/ CTS transmission) packet."]
    #[inline(always)]
    pub fn acr(&self) -> AcrR {
        AcrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Active after successful transmission of an Audio Sample packet. Due to high number of audio sample packets transmitted, this interrupt is by default masked at frame composer."]
    #[inline(always)]
    pub fn auds(&self) -> AudsR {
        AudsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Active after successful transmission of an NTSC VBI packet"]
    #[inline(always)]
    pub fn nvbi(&self) -> NvbiR {
        NvbiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Active after successful transmission of an MultiStream Audio packet"]
    #[inline(always)]
    pub fn mas(&self) -> MasR {
        MasR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Active after successful transmission of an Audio HBR packet."]
    #[inline(always)]
    pub fn hbr(&self) -> HbrR {
        HbrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Active after successful transmission of an Audio Content Protection packet."]
    #[inline(always)]
    pub fn acp(&self) -> AcpR {
        AcpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Active after successful transmission of an Audio InfoFrame packet."]
    #[inline(always)]
    pub fn audi(&self) -> AudiR {
        AudiR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active after successful transmission of an Null packet. Due to high number of audio sample packets transmitted, this interrupt is by default masked at frame composer."]
    #[inline(always)]
    #[must_use]
    pub fn null(&mut self) -> NullW<IhFcStat0Spec> {
        NullW::new(self, 0)
    }
    #[doc = "Bit 1 - Active after successful transmission of an Audio Clock Regeneration (N/ CTS transmission) packet."]
    #[inline(always)]
    #[must_use]
    pub fn acr(&mut self) -> AcrW<IhFcStat0Spec> {
        AcrW::new(self, 1)
    }
    #[doc = "Bit 2 - Active after successful transmission of an Audio Sample packet. Due to high number of audio sample packets transmitted, this interrupt is by default masked at frame composer."]
    #[inline(always)]
    #[must_use]
    pub fn auds(&mut self) -> AudsW<IhFcStat0Spec> {
        AudsW::new(self, 2)
    }
    #[doc = "Bit 3 - Active after successful transmission of an NTSC VBI packet"]
    #[inline(always)]
    #[must_use]
    pub fn nvbi(&mut self) -> NvbiW<IhFcStat0Spec> {
        NvbiW::new(self, 3)
    }
    #[doc = "Bit 4 - Active after successful transmission of an MultiStream Audio packet"]
    #[inline(always)]
    #[must_use]
    pub fn mas(&mut self) -> MasW<IhFcStat0Spec> {
        MasW::new(self, 4)
    }
    #[doc = "Bit 5 - Active after successful transmission of an Audio HBR packet."]
    #[inline(always)]
    #[must_use]
    pub fn hbr(&mut self) -> HbrW<IhFcStat0Spec> {
        HbrW::new(self, 5)
    }
    #[doc = "Bit 6 - Active after successful transmission of an Audio Content Protection packet."]
    #[inline(always)]
    #[must_use]
    pub fn acp(&mut self) -> AcpW<IhFcStat0Spec> {
        AcpW::new(self, 6)
    }
    #[doc = "Bit 7 - Active after successful transmission of an Audio InfoFrame packet."]
    #[inline(always)]
    #[must_use]
    pub fn audi(&mut self) -> AudiW<IhFcStat0Spec> {
        AudiW::new(self, 7)
    }
}
#[doc = "Active after successful transmission of an Null packet. Due to high number of audio sample packets transmitted, this interrupt is by default masked at frame composer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_fc_stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_fc_stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhFcStat0Spec;
impl crate::RegisterSpec for IhFcStat0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_fc_stat0::R`](R) reader structure"]
impl crate::Readable for IhFcStat0Spec {}
#[doc = "`write(|w| ..)` method takes [`ih_fc_stat0::W`](W) writer structure"]
impl crate::Writable for IhFcStat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0xff;
}
#[doc = "`reset()` method sets IH_FC_STAT0 to value 0"]
impl crate::Resettable for IhFcStat0Spec {
    const RESET_VALUE: u8 = 0;
}
