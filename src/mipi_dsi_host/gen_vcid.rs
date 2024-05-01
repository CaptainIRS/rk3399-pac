#[doc = "Register `GEN_VCID` reader"]
pub type R = crate::R<GenVcidSpec>;
#[doc = "Register `GEN_VCID` writer"]
pub type W = crate::W<GenVcidSpec>;
#[doc = "Field `GEN_VCID_RX` reader - gen_vcid_rx\n\nThis field indicates the Generic interface read-back virtual channel\n\nidentification."]
pub type GenVcidRxR = crate::FieldReader;
#[doc = "Field `GEN_VCID_RX` writer - gen_vcid_rx\n\nThis field indicates the Generic interface read-back virtual channel\n\nidentification."]
pub type GenVcidRxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - gen_vcid_rx\n\nThis field indicates the Generic interface read-back virtual channel\n\nidentification."]
    #[inline(always)]
    pub fn gen_vcid_rx(&self) -> GenVcidRxR {
        GenVcidRxR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - gen_vcid_rx\n\nThis field indicates the Generic interface read-back virtual channel\n\nidentification."]
    #[inline(always)]
    #[must_use]
    pub fn gen_vcid_rx(&mut self) -> GenVcidRxW<GenVcidSpec> {
        GenVcidRxW::new(self, 0)
    }
}
#[doc = "Generic Interface Virtual Channel Id Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_vcid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_vcid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GenVcidSpec;
impl crate::RegisterSpec for GenVcidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen_vcid::R`](R) reader structure"]
impl crate::Readable for GenVcidSpec {}
#[doc = "`write(|w| ..)` method takes [`gen_vcid::W`](W) writer structure"]
impl crate::Writable for GenVcidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GEN_VCID to value 0"]
impl crate::Resettable for GenVcidSpec {
    const RESET_VALUE: u32 = 0;
}
