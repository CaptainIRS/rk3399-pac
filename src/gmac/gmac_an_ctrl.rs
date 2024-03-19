#[doc = "Register `GMAC_AN_CTRL` reader"]
pub type R = crate::R<GmacAnCtrlSpec>;
#[doc = "Register `GMAC_AN_CTRL` writer"]
pub type W = crate::W<GmacAnCtrlSpec>;
#[doc = "Field `RAN` reader - Restart Auto-Negotiation\n\nWhen set, will cause auto-negotiation to restart if the ANE is set.\n\nThis bit is self-clearing after auto-negotiation starts. This bit\n\nshould be cleared for normal operation."]
pub type RanR = crate::BitReader;
#[doc = "Field `RAN` writer - Restart Auto-Negotiation\n\nWhen set, will cause auto-negotiation to restart if the ANE is set.\n\nThis bit is self-clearing after auto-negotiation starts. This bit\n\nshould be cleared for normal operation."]
pub type RanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANE` reader - Auto-Negotiation Enable\n\nWhen set, will enable the GMAC to perform auto-negotiation with\n\nthe link partner.\n\nClearing this bit will disable auto-negotiation."]
pub type AneR = crate::BitReader;
#[doc = "Field `ANE` writer - Auto-Negotiation Enable\n\nWhen set, will enable the GMAC to perform auto-negotiation with\n\nthe link partner.\n\nClearing this bit will disable auto-negotiation."]
pub type AneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 9 - Restart Auto-Negotiation\n\nWhen set, will cause auto-negotiation to restart if the ANE is set.\n\nThis bit is self-clearing after auto-negotiation starts. This bit\n\nshould be cleared for normal operation."]
    #[inline(always)]
    pub fn ran(&self) -> RanR {
        RanR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Auto-Negotiation Enable\n\nWhen set, will enable the GMAC to perform auto-negotiation with\n\nthe link partner.\n\nClearing this bit will disable auto-negotiation."]
    #[inline(always)]
    pub fn ane(&self) -> AneR {
        AneR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Restart Auto-Negotiation\n\nWhen set, will cause auto-negotiation to restart if the ANE is set.\n\nThis bit is self-clearing after auto-negotiation starts. This bit\n\nshould be cleared for normal operation."]
    #[inline(always)]
    #[must_use]
    pub fn ran(&mut self) -> RanW<GmacAnCtrlSpec> {
        RanW::new(self, 9)
    }
    #[doc = "Bit 12 - Auto-Negotiation Enable\n\nWhen set, will enable the GMAC to perform auto-negotiation with\n\nthe link partner.\n\nClearing this bit will disable auto-negotiation."]
    #[inline(always)]
    #[must_use]
    pub fn ane(&mut self) -> AneW<GmacAnCtrlSpec> {
        AneW::new(self, 12)
    }
}
#[doc = "AN Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_an_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_an_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacAnCtrlSpec;
impl crate::RegisterSpec for GmacAnCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_an_ctrl::R`](R) reader structure"]
impl crate::Readable for GmacAnCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_an_ctrl::W`](W) writer structure"]
impl crate::Writable for GmacAnCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_AN_CTRL to value 0"]
impl crate::Resettable for GmacAnCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
