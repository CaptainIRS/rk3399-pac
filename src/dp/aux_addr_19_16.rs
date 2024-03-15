#[doc = "Register `AUX_ADDR_19_16` reader"]
pub type R = crate::R<AuxAddr19_16Spec>;
#[doc = "Register `AUX_ADDR_19_16` writer"]
pub type W = crate::W<AuxAddr19_16Spec>;
#[doc = "Field `AUX_ADDR_19_16` reader - AUX_ADDR\\[7:0\\], Register control AUX CH address"]
pub type AuxAddr19_16R = crate::FieldReader;
#[doc = "Field `AUX_ADDR_19_16` writer - AUX_ADDR\\[7:0\\], Register control AUX CH address"]
pub type AuxAddr19_16W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - AUX_ADDR\\[7:0\\], Register control AUX CH address"]
    #[inline(always)]
    pub fn aux_addr_19_16(&self) -> AuxAddr19_16R {
        AuxAddr19_16R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AUX_ADDR\\[7:0\\], Register control AUX CH address"]
    #[inline(always)]
    #[must_use]
    pub fn aux_addr_19_16(&mut self) -> AuxAddr19_16W<AuxAddr19_16Spec> {
        AuxAddr19_16W::new(self, 0)
    }
}
#[doc = "DP AUX CH Address Register #2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aux_addr_19_16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aux_addr_19_16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxAddr19_16Spec;
impl crate::RegisterSpec for AuxAddr19_16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aux_addr_19_16::R`](R) reader structure"]
impl crate::Readable for AuxAddr19_16Spec {}
#[doc = "`write(|w| ..)` method takes [`aux_addr_19_16::W`](W) writer structure"]
impl crate::Writable for AuxAddr19_16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUX_ADDR_19_16 to value 0"]
impl crate::Resettable for AuxAddr19_16Spec {
    const RESET_VALUE: u32 = 0;
}
