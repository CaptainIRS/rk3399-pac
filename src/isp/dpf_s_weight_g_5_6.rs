#[doc = "Register `DPF_S_WEIGHT_G_5_6` reader"]
pub type R = crate::R<DpfSWeightG5_6Spec>;
#[doc = "Register `DPF_S_WEIGHT_G_5_6` writer"]
pub type W = crate::W<DpfSWeightG5_6Spec>;
#[doc = "Field `S_WEIGHT_G5` reader - Filter Coefficient green channel S_WEIGHT_G5 5 bit\n\nunsigned, value range 1/16 to 16/16 Default value is\n\n16/16 (*Default*)"]
pub type SWeightG5R = crate::FieldReader;
#[doc = "Field `S_WEIGHT_G5` writer - Filter Coefficient green channel S_WEIGHT_G5 5 bit\n\nunsigned, value range 1/16 to 16/16 Default value is\n\n16/16 (*Default*)"]
pub type SWeightG5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `S_WEIGHT_G6` reader - Filter Coefficient green channel S_WEIGHT_G6 5 bit\n\nunsigned, value range 1/16 to 16/16 Default value is\n\n16/16 (*Default*)"]
pub type SWeightG6R = crate::FieldReader;
#[doc = "Field `S_WEIGHT_G6` writer - Filter Coefficient green channel S_WEIGHT_G6 5 bit\n\nunsigned, value range 1/16 to 16/16 Default value is\n\n16/16 (*Default*)"]
pub type SWeightG6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Filter Coefficient green channel S_WEIGHT_G5 5 bit\n\nunsigned, value range 1/16 to 16/16 Default value is\n\n16/16 (*Default*)"]
    #[inline(always)]
    pub fn s_weight_g5(&self) -> SWeightG5R {
        SWeightG5R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Filter Coefficient green channel S_WEIGHT_G6 5 bit\n\nunsigned, value range 1/16 to 16/16 Default value is\n\n16/16 (*Default*)"]
    #[inline(always)]
    pub fn s_weight_g6(&self) -> SWeightG6R {
        SWeightG6R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Filter Coefficient green channel S_WEIGHT_G5 5 bit\n\nunsigned, value range 1/16 to 16/16 Default value is\n\n16/16 (*Default*)"]
    #[inline(always)]
    #[must_use]
    pub fn s_weight_g5(&mut self) -> SWeightG5W<DpfSWeightG5_6Spec> {
        SWeightG5W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Filter Coefficient green channel S_WEIGHT_G6 5 bit\n\nunsigned, value range 1/16 to 16/16 Default value is\n\n16/16 (*Default*)"]
    #[inline(always)]
    #[must_use]
    pub fn s_weight_g6(&mut self) -> SWeightG6W<DpfSWeightG5_6Spec> {
        SWeightG6W::new(self, 8)
    }
}
#[doc = "Spatial Weights green channel 5 6\n\nNote: The value zero (0/16) disables the filter tap. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpf_s_weight_g_5_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpf_s_weight_g_5_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpfSWeightG5_6Spec;
impl crate::RegisterSpec for DpfSWeightG5_6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpf_s_weight_g_5_6::R`](R) reader structure"]
impl crate::Readable for DpfSWeightG5_6Spec {}
#[doc = "`write(|w| ..)` method takes [`dpf_s_weight_g_5_6::W`](W) writer structure"]
impl crate::Writable for DpfSWeightG5_6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPF_S_WEIGHT_G_5_6 to value 0x1010"]
impl crate::Resettable for DpfSWeightG5_6Spec {
    const RESET_VALUE: u32 = 0x1010;
}
