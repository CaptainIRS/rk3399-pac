#[doc = "Register `MIPI_GEN_SHORT_C_D` reader"]
pub type R = crate::R<MipiGenShortCDSpec>;
#[doc = "Field `data_field_C` reader - 16 bit user defined data field from last generic\n\nshort packet of data type 0xC"]
pub type DataFieldCR = crate::FieldReader<u16>;
#[doc = "Field `data_field_D` reader - 16 bit user defined data field from last generic\n\nshort packet of data type 0xD"]
pub type DataFieldDR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 16 bit user defined data field from last generic\n\nshort packet of data type 0xC"]
    #[inline(always)]
    pub fn data_field_c(&self) -> DataFieldCR {
        DataFieldCR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 16 bit user defined data field from last generic\n\nshort packet of data type 0xD"]
    #[inline(always)]
    pub fn data_field_d(&self) -> DataFieldDR {
        DataFieldDR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "data field for generic short packets of data type 0xC and 0xD\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_gen_short_c_d::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiGenShortCDSpec;
impl crate::RegisterSpec for MipiGenShortCDSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_gen_short_c_d::R`](R) reader structure"]
impl crate::Readable for MipiGenShortCDSpec {}
#[doc = "`reset()` method sets MIPI_GEN_SHORT_C_D to value 0"]
impl crate::Resettable for MipiGenShortCDSpec {
    const RESET_VALUE: u32 = 0;
}
