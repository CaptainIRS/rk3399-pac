#[doc = "Register `AUX_ADDR_15_8` reader"]
pub type R = crate::R<AuxAddr15_8Spec>;
#[doc = "Register `AUX_ADDR_15_8` writer"]
pub type W = crate::W<AuxAddr15_8Spec>;
#[doc = "Field `AUX_ADDR_15_8` reader - AUX_ADDR\\[15:8\\], Register control AUX \n\nCH address"]
pub type AuxAddr15_8R = crate::FieldReader;
#[doc = "Field `AUX_ADDR_15_8` writer - AUX_ADDR\\[15:8\\], Register control AUX \n\nCH address"]
pub type AuxAddr15_8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - AUX_ADDR\\[15:8\\], Register control AUX \n\nCH address"]
    #[inline(always)]
    pub fn aux_addr_15_8(&self) -> AuxAddr15_8R {
        AuxAddr15_8R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - AUX_ADDR\\[15:8\\], Register control AUX \n\nCH address"]
    #[inline(always)]
    #[must_use]
    pub fn aux_addr_15_8(&mut self) -> AuxAddr15_8W<AuxAddr15_8Spec> {
        AuxAddr15_8W::new(self, 0)
    }
}
#[doc = "DP AUX CH Address Register #1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aux_addr_15_8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aux_addr_15_8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxAddr15_8Spec;
impl crate::RegisterSpec for AuxAddr15_8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aux_addr_15_8::R`](R) reader structure"]
impl crate::Readable for AuxAddr15_8Spec {}
#[doc = "`write(|w| ..)` method takes [`aux_addr_15_8::W`](W) writer structure"]
impl crate::Writable for AuxAddr15_8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUX_ADDR_15_8 to value 0"]
impl crate::Resettable for AuxAddr15_8Spec {
    const RESET_VALUE: u32 = 0;
}
