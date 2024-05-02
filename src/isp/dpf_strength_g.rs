#[doc = "Register `DPF_STRENGTH_G` reader"]
pub type R = crate::R<DpfStrengthGSpec>;
#[doc = "Register `DPF_STRENGTH_G` writer"]
pub type W = crate::W<DpfStrengthGSpec>;
#[doc = "Field `INV_WEIGHT_G` reader - Filter strength of the filter is determined by weight.\n\nDefault is a weight of 1. A higher weight increases the filter\n\nstrength. In this register the unsigned 8 bit value\n\n64/weight is stored.\n\n\n\nThe following values show examples: weight=0.251\n\n-> 255, weight=0.5 -> 128, weight=1 -> 64 *default*\n\nweight=1.25 -> 51, weight=1.5 -> 42,\n\nweight=1.75 -> 37, weight=2 -> 32\n\n"]
pub type InvWeightGR = crate::FieldReader;
#[doc = "Field `INV_WEIGHT_G` writer - Filter strength of the filter is determined by weight.\n\nDefault is a weight of 1. A higher weight increases the filter\n\nstrength. In this register the unsigned 8 bit value\n\n64/weight is stored.\n\n\n\nThe following values show examples: weight=0.251\n\n-> 255, weight=0.5 -> 128, weight=1 -> 64 *default*\n\nweight=1.25 -> 51, weight=1.5 -> 42,\n\nweight=1.75 -> 37, weight=2 -> 32\n\n"]
pub type InvWeightGW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Filter strength of the filter is determined by weight.\n\nDefault is a weight of 1. A higher weight increases the filter\n\nstrength. In this register the unsigned 8 bit value\n\n64/weight is stored.\n\n\n\nThe following values show examples: weight=0.251\n\n-> 255, weight=0.5 -> 128, weight=1 -> 64 *default*\n\nweight=1.25 -> 51, weight=1.5 -> 42,\n\nweight=1.75 -> 37, weight=2 -> 32\n\n"]
    #[inline(always)]
    pub fn inv_weight_g(&self) -> InvWeightGR {
        InvWeightGR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Filter strength of the filter is determined by weight.\n\nDefault is a weight of 1. A higher weight increases the filter\n\nstrength. In this register the unsigned 8 bit value\n\n64/weight is stored.\n\n\n\nThe following values show examples: weight=0.251\n\n-> 255, weight=0.5 -> 128, weight=1 -> 64 *default*\n\nweight=1.25 -> 51, weight=1.5 -> 42,\n\nweight=1.75 -> 37, weight=2 -> 32\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn inv_weight_g(&mut self) -> InvWeightGW<DpfStrengthGSpec> {
        InvWeightGW::new(self, 0)
    }
}
#[doc = "filter strength of the GREEN filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpf_strength_g::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpf_strength_g::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpfStrengthGSpec;
impl crate::RegisterSpec for DpfStrengthGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpf_strength_g::R`](R) reader structure"]
impl crate::Readable for DpfStrengthGSpec {}
#[doc = "`write(|w| ..)` method takes [`dpf_strength_g::W`](W) writer structure"]
impl crate::Writable for DpfStrengthGSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPF_STRENGTH_G to value 0x40"]
impl crate::Resettable for DpfStrengthGSpec {
    const RESET_VALUE: u32 = 0x40;
}
