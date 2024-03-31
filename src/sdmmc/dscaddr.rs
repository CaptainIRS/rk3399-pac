#[doc = "Register `DSCADDR` reader"]
pub type R = crate::R<DscaddrSpec>;
#[doc = "Register `DSCADDR` writer"]
pub type W = crate::W<DscaddrSpec>;
#[doc = "Field `HDA` reader - Host Descriptor Address Pointer. Cleared on reset. Pointer\n\nupdated by IDMAC during operation. This register points to the\n\nstart address of the current descriptor read by the IDMAC."]
pub type HdaR = crate::FieldReader<u32>;
#[doc = "Field `HDA` writer - Host Descriptor Address Pointer. Cleared on reset. Pointer\n\nupdated by IDMAC during operation. This register points to the\n\nstart address of the current descriptor read by the IDMAC."]
pub type HdaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host Descriptor Address Pointer. Cleared on reset. Pointer\n\nupdated by IDMAC during operation. This register points to the\n\nstart address of the current descriptor read by the IDMAC."]
    #[inline(always)]
    pub fn hda(&self) -> HdaR {
        HdaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Host Descriptor Address Pointer. Cleared on reset. Pointer\n\nupdated by IDMAC during operation. This register points to the\n\nstart address of the current descriptor read by the IDMAC."]
    #[inline(always)]
    #[must_use]
    pub fn hda(&mut self) -> HdaW<DscaddrSpec> {
        HdaW::new(self, 0)
    }
}
#[doc = "Current host descriptor address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dscaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DscaddrSpec;
impl crate::RegisterSpec for DscaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dscaddr::R`](R) reader structure"]
impl crate::Readable for DscaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dscaddr::W`](W) writer structure"]
impl crate::Writable for DscaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSCADDR to value 0"]
impl crate::Resettable for DscaddrSpec {
    const RESET_VALUE: u32 = 0;
}
