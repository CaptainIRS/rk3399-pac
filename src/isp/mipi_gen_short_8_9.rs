#[doc = "Register `MIPI_GEN_SHORT_8_9` reader"]
pub type R = crate::R<MipiGenShort8_9Spec>;
#[doc = "Field `data_field_8` reader - 16 bit user defined data field from last generic\n\nshort packet of data type 0x8"]
pub type DataField8R = crate::FieldReader<u16>;
#[doc = "Field `data_field_9` reader - 16 bit user defined data field from last generic\n\nshort packet of data type 0x9"]
pub type DataField9R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 16 bit user defined data field from last generic\n\nshort packet of data type 0x8"]
    #[inline(always)]
    pub fn data_field_8(&self) -> DataField8R {
        DataField8R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 16 bit user defined data field from last generic\n\nshort packet of data type 0x9"]
    #[inline(always)]
    pub fn data_field_9(&self) -> DataField9R {
        DataField9R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "data field for generic short packets of data type 0x8 and 0x9\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_gen_short_8_9::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiGenShort8_9Spec;
impl crate::RegisterSpec for MipiGenShort8_9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_gen_short_8_9::R`](R) reader structure"]
impl crate::Readable for MipiGenShort8_9Spec {}
#[doc = "`reset()` method sets MIPI_GEN_SHORT_8_9 to value 0"]
impl crate::Resettable for MipiGenShort8_9Spec {
    const RESET_VALUE: u32 = 0;
}
