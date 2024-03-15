#[doc = "Register `BASE_ADDRESS_1` reader"]
pub type R = crate::R<BaseAddress1Spec>;
#[doc = "Register `BASE_ADDRESS_1` writer"]
pub type W = crate::W<BaseAddress1Spec>;
#[doc = "Field `BAMRW` reader - Base Address - RW part \\[BAMRW\\]
This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
pub type BamrwR = crate::FieldReader<u32>;
#[doc = "Field `BAMRW` writer - Base Address - RW part \\[BAMRW\\]
This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
pub type BamrwW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base Address - RW part \\[BAMRW\\]
This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    pub fn bamrw(&self) -> BamrwR {
        BamrwR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base Address - RW part \\[BAMRW\\]
This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    #[must_use]
    pub fn bamrw(&mut self) -> BamrwW<BaseAddress1Spec> {
        BamrwW::new(self, 0)
    }
}
#[doc = "Base Address Register 1 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_address_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`base_address_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaseAddress1Spec;
impl crate::RegisterSpec for BaseAddress1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`base_address_1::R`](R) reader structure"]
impl crate::Readable for BaseAddress1Spec {}
#[doc = "`write(|w| ..)` method takes [`base_address_1::W`](W) writer structure"]
impl crate::Writable for BaseAddress1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BASE_ADDRESS_1 to value 0"]
impl crate::Resettable for BaseAddress1Spec {
    const RESET_VALUE: u32 = 0;
}
