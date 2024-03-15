#[doc = "Register `VF_BASE_ADDRESS_3` reader"]
pub type R = crate::R<VfBaseAddress3Spec>;
#[doc = "Register `VF_BASE_ADDRESS_3` writer"]
pub type W = crate::W<VfBaseAddress3Spec>;
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
    pub fn bamrw(&mut self) -> BamrwW<VfBaseAddress3Spec> {
        BamrwW::new(self, 0)
    }
}
#[doc = "VF Base Address Register 3 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vf_base_address_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vf_base_address_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfBaseAddress3Spec;
impl crate::RegisterSpec for VfBaseAddress3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vf_base_address_3::R`](R) reader structure"]
impl crate::Readable for VfBaseAddress3Spec {}
#[doc = "`write(|w| ..)` method takes [`vf_base_address_3::W`](W) writer structure"]
impl crate::Writable for VfBaseAddress3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VF_BASE_ADDRESS_3 to value 0"]
impl crate::Resettable for VfBaseAddress3Spec {
    const RESET_VALUE: u32 = 0;
}
