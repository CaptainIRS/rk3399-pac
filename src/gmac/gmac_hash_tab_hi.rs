#[doc = "Register `GMAC_HASH_TAB_HI` reader"]
pub type R = crate::R<GmacHashTabHiSpec>;
#[doc = "Register `GMAC_HASH_TAB_HI` writer"]
pub type W = crate::W<GmacHashTabHiSpec>;
#[doc = "Field `HTH` reader - Hash Table High\n\nThis field contains the upper 32 bits of Hash table"]
pub type HthR = crate::FieldReader<u32>;
#[doc = "Field `HTH` writer - Hash Table High\n\nThis field contains the upper 32 bits of Hash table"]
pub type HthW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash Table High\n\nThis field contains the upper 32 bits of Hash table"]
    #[inline(always)]
    pub fn hth(&self) -> HthR {
        HthR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Table High\n\nThis field contains the upper 32 bits of Hash table"]
    #[inline(always)]
    #[must_use]
    pub fn hth(&mut self) -> HthW<GmacHashTabHiSpec> {
        HthW::new(self, 0)
    }
}
#[doc = "Hash Table High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_hash_tab_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_hash_tab_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacHashTabHiSpec;
impl crate::RegisterSpec for GmacHashTabHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_hash_tab_hi::R`](R) reader structure"]
impl crate::Readable for GmacHashTabHiSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_hash_tab_hi::W`](W) writer structure"]
impl crate::Writable for GmacHashTabHiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_HASH_TAB_HI to value 0"]
impl crate::Resettable for GmacHashTabHiSpec {
    const RESET_VALUE: u32 = 0;
}
