#[doc = "Register `EP_INBOUND_BAR_ADDRESS_TRANSLATION_1` reader"]
pub type R = crate::R<EpInboundBarAddressTranslation1Spec>;
#[doc = "Register `EP_INBOUND_BAR_ADDRESS_TRANSLATION_1` writer"]
pub type W = crate::W<EpInboundBarAddressTranslation1Spec>;
#[doc = "Field `data` reader - Address bits \\[63:32\\]
\\[data\\]
Bits \\[63:32\\]
of Address Register for BAR N"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `data` writer - Address bits \\[63:32\\]
\\[data\\]
Bits \\[63:32\\]
of Address Register for BAR N"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address bits \\[63:32\\]
\\[data\\]
Bits \\[63:32\\]
of Address Register for BAR N"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address bits \\[63:32\\]
\\[data\\]
Bits \\[63:32\\]
of Address Register for BAR N"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<EpInboundBarAddressTranslation1Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "EP Inbound BAR Address Translation 1 Bits \\[63:32\\]
of Address Register for BAR N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_inbound_bar_address_translation_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_inbound_bar_address_translation_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpInboundBarAddressTranslation1Spec;
impl crate::RegisterSpec for EpInboundBarAddressTranslation1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep_inbound_bar_address_translation_1::R`](R) reader structure"]
impl crate::Readable for EpInboundBarAddressTranslation1Spec {}
#[doc = "`write(|w| ..)` method takes [`ep_inbound_bar_address_translation_1::W`](W) writer structure"]
impl crate::Writable for EpInboundBarAddressTranslation1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EP_INBOUND_BAR_ADDRESS_TRANSLATION_1 to value 0"]
impl crate::Resettable for EpInboundBarAddressTranslation1Spec {
    const RESET_VALUE: u32 = 0;
}
