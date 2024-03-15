#[doc = "Register `DP_IRQ_VECTOR` reader"]
pub type R = crate::R<DpIrqVectorSpec>;
#[doc = "Register `DP_IRQ_VECTOR` writer"]
pub type W = crate::W<DpIrqVectorSpec>;
#[doc = "Field `DP_IRQ_VECTOR` reader - Irq_vector"]
pub type DpIrqVectorR = crate::FieldReader;
#[doc = "Field `DP_IRQ_VECTOR` writer - Irq_vector"]
pub type DpIrqVectorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Irq_vector"]
    #[inline(always)]
    pub fn dp_irq_vector(&self) -> DpIrqVectorR {
        DpIrqVectorR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Irq_vector"]
    #[inline(always)]
    #[must_use]
    pub fn dp_irq_vector(&mut self) -> DpIrqVectorW<DpIrqVectorSpec> {
        DpIrqVectorW::new(self, 0)
    }
}
#[doc = "DP Irq Vector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_irq_vector::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_irq_vector::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpIrqVectorSpec;
impl crate::RegisterSpec for DpIrqVectorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_irq_vector::R`](R) reader structure"]
impl crate::Readable for DpIrqVectorSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_irq_vector::W`](W) writer structure"]
impl crate::Writable for DpIrqVectorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets DP_IRQ_VECTOR to value 0"]
impl crate::Resettable for DpIrqVectorSpec {
    const RESET_VALUE: u32 = 0;
}
