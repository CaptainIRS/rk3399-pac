#[doc = "Register `GMAC_AN_EXP` reader"]
pub type R = crate::R<GmacAnExpSpec>;
#[doc = "Field `NPR` reader - New Page Received\n\nWhen set, this bit indicates that a new page has been received by\n\nthe GMAC. This bit will be cleared when read."]
pub type NprR = crate::BitReader;
#[doc = "Field `NPA` reader - Next Page Ability\n\nThis bit is tied to low, because the GMAC does not support next\n\npage function."]
pub type NpaR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - New Page Received\n\nWhen set, this bit indicates that a new page has been received by\n\nthe GMAC. This bit will be cleared when read."]
    #[inline(always)]
    pub fn npr(&self) -> NprR {
        NprR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Next Page Ability\n\nThis bit is tied to low, because the GMAC does not support next\n\npage function."]
    #[inline(always)]
    pub fn npa(&self) -> NpaR {
        NpaR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Auto Negotiation Expansion Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_an_exp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacAnExpSpec;
impl crate::RegisterSpec for GmacAnExpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_an_exp::R`](R) reader structure"]
impl crate::Readable for GmacAnExpSpec {}
#[doc = "`reset()` method sets GMAC_AN_EXP to value 0"]
impl crate::Resettable for GmacAnExpSpec {
    const RESET_VALUE: u32 = 0;
}
