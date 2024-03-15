#[doc = "Register `GMAC_AN_STATUS` reader"]
pub type R = crate::R<GmacAnStatusSpec>;
#[doc = "Register `GMAC_AN_STATUS` writer"]
pub type W = crate::W<GmacAnStatusSpec>;
#[doc = "Field `LS` reader - Link Status When set, this bit indicates that the link is up. When cleared, this bit indicates that the link is down."]
pub type LsR = crate::BitReader;
#[doc = "Field `LS` writer - Link Status When set, this bit indicates that the link is up. When cleared, this bit indicates that the link is down."]
pub type LsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANA` reader - Auto-Negotiation Ability This bit is always high, because the GMAC supports auto- negotiation."]
pub type AnaR = crate::BitReader;
#[doc = "Field `ANC` reader - Auto-Negotiation Complete When set, this bit indicates that the auto-negotiation process is completed. This bit is cleared when auto-negotiation is reinitiated."]
pub type AncR = crate::BitReader;
impl R {
    #[doc = "Bit 2 - Link Status When set, this bit indicates that the link is up. When cleared, this bit indicates that the link is down."]
    #[inline(always)]
    pub fn ls(&self) -> LsR {
        LsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auto-Negotiation Ability This bit is always high, because the GMAC supports auto- negotiation."]
    #[inline(always)]
    pub fn ana(&self) -> AnaR {
        AnaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Auto-Negotiation Complete When set, this bit indicates that the auto-negotiation process is completed. This bit is cleared when auto-negotiation is reinitiated."]
    #[inline(always)]
    pub fn anc(&self) -> AncR {
        AncR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Link Status When set, this bit indicates that the link is up. When cleared, this bit indicates that the link is down."]
    #[inline(always)]
    #[must_use]
    pub fn ls(&mut self) -> LsW<GmacAnStatusSpec> {
        LsW::new(self, 2)
    }
}
#[doc = "AN Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_an_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_an_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacAnStatusSpec;
impl crate::RegisterSpec for GmacAnStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_an_status::R`](R) reader structure"]
impl crate::Readable for GmacAnStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_an_status::W`](W) writer structure"]
impl crate::Writable for GmacAnStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_AN_STATUS to value 0x08"]
impl crate::Resettable for GmacAnStatusSpec {
    const RESET_VALUE: u32 = 0x08;
}
