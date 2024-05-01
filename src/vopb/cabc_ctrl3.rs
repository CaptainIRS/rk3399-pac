#[doc = "Register `CABC_CTRL3` reader"]
pub type R = crate::R<CabcCtrl3Spec>;
#[doc = "Register `CABC_CTRL3` writer"]
pub type W = crate::W<CabcCtrl3Spec>;
#[doc = "Field `CABC_GLOBAL_DN` reader - cabc global scale down value."]
pub type CabcGlobalDnR = crate::FieldReader;
#[doc = "Field `CABC_GLOBAL_DN` writer - cabc global scale down value."]
pub type CabcGlobalDnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CABC_GLOBAL_DN_LIMIT_EN` reader - cabc global scale down limit enable."]
pub type CabcGlobalDnLimitEnR = crate::BitReader;
#[doc = "Field `CABC_GLOBAL_DN_LIMIT_EN` writer - cabc global scale down limit enable."]
pub type CabcGlobalDnLimitEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - cabc global scale down value."]
    #[inline(always)]
    pub fn cabc_global_dn(&self) -> CabcGlobalDnR {
        CabcGlobalDnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - cabc global scale down limit enable."]
    #[inline(always)]
    pub fn cabc_global_dn_limit_en(&self) -> CabcGlobalDnLimitEnR {
        CabcGlobalDnLimitEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - cabc global scale down value."]
    #[inline(always)]
    #[must_use]
    pub fn cabc_global_dn(&mut self) -> CabcGlobalDnW<CabcCtrl3Spec> {
        CabcGlobalDnW::new(self, 0)
    }
    #[doc = "Bit 8 - cabc global scale down limit enable."]
    #[inline(always)]
    #[must_use]
    pub fn cabc_global_dn_limit_en(&mut self) -> CabcGlobalDnLimitEnW<CabcCtrl3Spec> {
        CabcGlobalDnLimitEnW::new(self, 8)
    }
}
#[doc = "Content Adaptive Backlight Control register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cabc_ctrl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cabc_ctrl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CabcCtrl3Spec;
impl crate::RegisterSpec for CabcCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cabc_ctrl3::R`](R) reader structure"]
impl crate::Readable for CabcCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`cabc_ctrl3::W`](W) writer structure"]
impl crate::Writable for CabcCtrl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CABC_CTRL3 to value 0"]
impl crate::Resettable for CabcCtrl3Spec {
    const RESET_VALUE: u32 = 0;
}
