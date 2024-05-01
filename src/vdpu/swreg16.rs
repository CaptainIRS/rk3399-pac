#[doc = "Register `SWREG16` reader"]
pub type R = crate::R<Swreg16Spec>;
#[doc = "Register `SWREG16` writer"]
pub type W = crate::W<Swreg16Spec>;
#[doc = "Field `SW_PADD_R` reader - the total num of padded in front of R componet"]
pub type SwPaddRR = crate::FieldReader;
#[doc = "Field `SW_PADD_R` writer - the total num of padded in front of R componet"]
pub type SwPaddRW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SW_PADD_G` reader - the total num of padded in front of G componet\n\nthe total num of padded in front of G componet"]
pub type SwPaddGR = crate::FieldReader;
#[doc = "Field `SW_PADD_G` writer - the total num of padded in front of G componet\n\nthe total num of padded in front of G componet"]
pub type SwPaddGW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SW_PADD_B` reader - the total num of padded in front of B componet"]
pub type SwPaddBR = crate::FieldReader;
#[doc = "Field `SW_PADD_B` writer - the total num of padded in front of B componet"]
pub type SwPaddBW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - the total num of padded in front of R componet"]
    #[inline(always)]
    pub fn sw_padd_r(&self) -> SwPaddRR {
        SwPaddRR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - the total num of padded in front of G componet\n\nthe total num of padded in front of G componet"]
    #[inline(always)]
    pub fn sw_padd_g(&self) -> SwPaddGR {
        SwPaddGR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - the total num of padded in front of B componet"]
    #[inline(always)]
    pub fn sw_padd_b(&self) -> SwPaddBR {
        SwPaddBR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - the total num of padded in front of R componet"]
    #[inline(always)]
    #[must_use]
    pub fn sw_padd_r(&mut self) -> SwPaddRW<Swreg16Spec> {
        SwPaddRW::new(self, 0)
    }
    #[doc = "Bits 8:12 - the total num of padded in front of G componet\n\nthe total num of padded in front of G componet"]
    #[inline(always)]
    #[must_use]
    pub fn sw_padd_g(&mut self) -> SwPaddGW<Swreg16Spec> {
        SwPaddGW::new(self, 8)
    }
    #[doc = "Bits 16:20 - the total num of padded in front of B componet"]
    #[inline(always)]
    #[must_use]
    pub fn sw_padd_b(&mut self) -> SwPaddBW<Swreg16Spec> {
        SwPaddBW::new(self, 16)
    }
}
#[doc = "total num of padded for RGB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg16Spec;
impl crate::RegisterSpec for Swreg16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg16::R`](R) reader structure"]
impl crate::Readable for Swreg16Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg16::W`](W) writer structure"]
impl crate::Writable for Swreg16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG16 to value 0"]
impl crate::Resettable for Swreg16Spec {
    const RESET_VALUE: u32 = 0;
}
