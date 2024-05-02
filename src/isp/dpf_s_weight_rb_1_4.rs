#[doc = "Register `DPF_S_WEIGHT_RB_1_4` reader"]
pub type R = crate::R<DpfSWeightRb1_4Spec>;
#[doc = "Register `DPF_S_WEIGHT_RB_1_4` writer"]
pub type W = crate::W<DpfSWeightRb1_4Spec>;
#[doc = "Field `S_WEIGHT_RB1` reader - Filter Coefficient red/blue channels S_WEIGHT_RB1 5\n\nbit unsigned, value range 1/16 to 16/16\n\nDefault value is 16/16 (*Default*)"]
pub type SWeightRb1R = crate::FieldReader;
#[doc = "Field `S_WEIGHT_RB1` writer - Filter Coefficient red/blue channels S_WEIGHT_RB1 5\n\nbit unsigned, value range 1/16 to 16/16\n\nDefault value is 16/16 (*Default*)"]
pub type SWeightRb1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `S_WEIGHT_RB2` reader - Filter Coefficient red/blue channels S_WEIGHT_RB2 5\n\nbit unsigned, value range 1/16 to 16/16\n\nDefault value is 16/16 (*Default*)"]
pub type SWeightRb2R = crate::FieldReader;
#[doc = "Field `S_WEIGHT_RB2` writer - Filter Coefficient red/blue channels S_WEIGHT_RB2 5\n\nbit unsigned, value range 1/16 to 16/16\n\nDefault value is 16/16 (*Default*)"]
pub type SWeightRb2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `S_WEIGHT_RB3` reader - Filter Coefficient red/blue channels S_WEIGHT_RB3 5\n\nbit unsigned, value range 1/16 to 16/16\n\nDefault value is 16/16 (*Default*)"]
pub type SWeightRb3R = crate::FieldReader;
#[doc = "Field `S_WEIGHT_RB3` writer - Filter Coefficient red/blue channels S_WEIGHT_RB3 5\n\nbit unsigned, value range 1/16 to 16/16\n\nDefault value is 16/16 (*Default*)"]
pub type SWeightRb3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `S_WEIGHT_RB4` reader - Filter Coefficient red/blue channels S_WEIGHT_RB4 5\n\nbit unsigned, value range 1/16 to 16/16\n\nDefault value is 16/16 (*Default*)"]
pub type SWeightRb4R = crate::FieldReader;
#[doc = "Field `S_WEIGHT_RB4` writer - Filter Coefficient red/blue channels S_WEIGHT_RB4 5\n\nbit unsigned, value range 1/16 to 16/16\n\nDefault value is 16/16 (*Default*)"]
pub type SWeightRb4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Filter Coefficient red/blue channels S_WEIGHT_RB1 5\n\nbit unsigned, value range 1/16 to 16/16\n\nDefault value is 16/16 (*Default*)"]
    #[inline(always)]
    pub fn s_weight_rb1(&self) -> SWeightRb1R {
        SWeightRb1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Filter Coefficient red/blue channels S_WEIGHT_RB2 5\n\nbit unsigned, value range 1/16 to 16/16\n\nDefault value is 16/16 (*Default*)"]
    #[inline(always)]
    pub fn s_weight_rb2(&self) -> SWeightRb2R {
        SWeightRb2R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Filter Coefficient red/blue channels S_WEIGHT_RB3 5\n\nbit unsigned, value range 1/16 to 16/16\n\nDefault value is 16/16 (*Default*)"]
    #[inline(always)]
    pub fn s_weight_rb3(&self) -> SWeightRb3R {
        SWeightRb3R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Filter Coefficient red/blue channels S_WEIGHT_RB4 5\n\nbit unsigned, value range 1/16 to 16/16\n\nDefault value is 16/16 (*Default*)"]
    #[inline(always)]
    pub fn s_weight_rb4(&self) -> SWeightRb4R {
        SWeightRb4R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Filter Coefficient red/blue channels S_WEIGHT_RB1 5\n\nbit unsigned, value range 1/16 to 16/16\n\nDefault value is 16/16 (*Default*)"]
    #[inline(always)]
    #[must_use]
    pub fn s_weight_rb1(&mut self) -> SWeightRb1W<DpfSWeightRb1_4Spec> {
        SWeightRb1W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Filter Coefficient red/blue channels S_WEIGHT_RB2 5\n\nbit unsigned, value range 1/16 to 16/16\n\nDefault value is 16/16 (*Default*)"]
    #[inline(always)]
    #[must_use]
    pub fn s_weight_rb2(&mut self) -> SWeightRb2W<DpfSWeightRb1_4Spec> {
        SWeightRb2W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Filter Coefficient red/blue channels S_WEIGHT_RB3 5\n\nbit unsigned, value range 1/16 to 16/16\n\nDefault value is 16/16 (*Default*)"]
    #[inline(always)]
    #[must_use]
    pub fn s_weight_rb3(&mut self) -> SWeightRb3W<DpfSWeightRb1_4Spec> {
        SWeightRb3W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Filter Coefficient red/blue channels S_WEIGHT_RB4 5\n\nbit unsigned, value range 1/16 to 16/16\n\nDefault value is 16/16 (*Default*)"]
    #[inline(always)]
    #[must_use]
    pub fn s_weight_rb4(&mut self) -> SWeightRb4W<DpfSWeightRb1_4Spec> {
        SWeightRb4W::new(self, 24)
    }
}
#[doc = "Spatial Weights red/blue channels 1 2 3 4\n\nNote: The value zero (0/16) disables the filter tap. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpf_s_weight_rb_1_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpf_s_weight_rb_1_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpfSWeightRb1_4Spec;
impl crate::RegisterSpec for DpfSWeightRb1_4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpf_s_weight_rb_1_4::R`](R) reader structure"]
impl crate::Readable for DpfSWeightRb1_4Spec {}
#[doc = "`write(|w| ..)` method takes [`dpf_s_weight_rb_1_4::W`](W) writer structure"]
impl crate::Writable for DpfSWeightRb1_4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPF_S_WEIGHT_RB_1_4 to value 0x1010_1010"]
impl crate::Resettable for DpfSWeightRb1_4Spec {
    const RESET_VALUE: u32 = 0x1010_1010;
}
