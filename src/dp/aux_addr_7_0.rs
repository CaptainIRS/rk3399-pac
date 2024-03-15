#[doc = "Register `AUX_ADDR_7_0` reader"]
pub type R = crate::R<AuxAddr7_0Spec>;
#[doc = "Register `AUX_ADDR_7_0` writer"]
pub type W = crate::W<AuxAddr7_0Spec>;
#[doc = "Field `AUX_ADDR_7_0` reader - AUX_ADDR\\[7:0\\], Register control AUX CH address"]
pub type AuxAddr7_0R = crate::FieldReader;
#[doc = "Field `AUX_ADDR_7_0` writer - AUX_ADDR\\[7:0\\], Register control AUX CH address"]
pub type AuxAddr7_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - AUX_ADDR\\[7:0\\], Register control AUX CH address"]
    #[inline(always)]
    pub fn aux_addr_7_0(&self) -> AuxAddr7_0R {
        AuxAddr7_0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - AUX_ADDR\\[7:0\\], Register control AUX CH address"]
    #[inline(always)]
    #[must_use]
    pub fn aux_addr_7_0(&mut self) -> AuxAddr7_0W<AuxAddr7_0Spec> {
        AuxAddr7_0W::new(self, 0)
    }
}
#[doc = "DP AUX CH Address Register #0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aux_addr_7_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aux_addr_7_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxAddr7_0Spec;
impl crate::RegisterSpec for AuxAddr7_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aux_addr_7_0::R`](R) reader structure"]
impl crate::Readable for AuxAddr7_0Spec {}
#[doc = "`write(|w| ..)` method takes [`aux_addr_7_0::W`](W) writer structure"]
impl crate::Writable for AuxAddr7_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUX_ADDR_7_0 to value 0"]
impl crate::Resettable for AuxAddr7_0Spec {
    const RESET_VALUE: u32 = 0;
}
